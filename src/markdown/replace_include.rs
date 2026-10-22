// TODO Reviewed ??-??-2026
//! Replace {{#include file.md}} by the file contents
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use tempfile::NamedTempFile;
use tracing::info;

/// Regex to find {{#include \<file\>.md}}.
static INSERT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\{\{#include (?<filepath>\S+?\.md)(?<selector>:[^}\s]+)?\}\}").unwrap()
});
static ANCHOR_START_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"ANCHOR:\s*(?<name>[\w_-]+)").unwrap());
static ANCHOR_END_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"ANCHOR_END:\s*(?<name>[\w_-]+)").unwrap());

struct PlannedReplacement {
    path: PathBuf,
    contents: Vec<u8>,
}

fn selected_contents<'a>(content: &'a str, selector: Option<&str>, path: &Path) -> Result<&'a str> {
    let Some(selector) = selector else {
        return Ok(content);
    };
    let selector = selector
        .strip_prefix(':')
        .context("Include selector must start with a colon")?;
    let lines: Vec<_> = content.split_inclusive('\n').collect();

    let selected_lines = |first: usize, end: usize| -> Result<&'a str> {
        if first > end || end > lines.len() {
            anyhow::bail!(
                "Line selection {selector:?} is outside {} ({} lines)",
                path.display(),
                lines.len()
            );
        }
        let start_byte = lines[..first].iter().map(|line| line.len()).sum();
        let end_byte = start_byte
            + lines[first..end]
                .iter()
                .map(|line| line.len())
                .sum::<usize>();
        Ok(&content[start_byte..end_byte])
    };

    let segments: Vec<_> = selector.split(':').collect();
    if segments.len() == 1 {
        if let Ok(line_number) = segments[0].parse::<usize>() {
            if line_number == 0 {
                anyhow::bail!("Include line numbers start at 1 in {}", path.display());
            }
            return selected_lines(line_number - 1, line_number);
        }

        let start = lines.iter().enumerate().find_map(|(index, line)| {
            ANCHOR_START_REGEX
                .captures(line)
                .filter(|captures| &captures["name"] == segments[0])
                .map(|_| index)
        });
        let Some(start) = start else {
            anyhow::bail!(
                "Include anchor {:?} was not found in {}",
                segments[0],
                path.display()
            );
        };
        let end = lines
            .iter()
            .enumerate()
            .skip(start + 1)
            .find_map(|(index, line)| {
                ANCHOR_END_REGEX
                    .captures(line)
                    .filter(|captures| &captures["name"] == segments[0])
                    .map(|_| index)
            });
        let Some(end) = end else {
            anyhow::bail!(
                "End of include anchor {:?} was not found in {}",
                segments[0],
                path.display()
            );
        };
        return selected_lines(start + 1, end);
    }

    if segments.len() == 2 {
        let first = if segments[0].is_empty() {
            1
        } else {
            segments[0]
                .parse::<usize>()
                .with_context(|| format!("Invalid start line in include selector {selector:?}"))?
        };
        let end = if segments[1].is_empty() {
            lines.len()
        } else {
            segments[1]
                .parse::<usize>()
                .with_context(|| format!("Invalid end line in include selector {selector:?}"))?
        };
        if first == 0 {
            anyhow::bail!("Include line numbers start at 1 in {}", path.display());
        }
        return selected_lines(first - 1, end);
    }

    anyhow::bail!(
        "Unsupported include selector {selector:?} in {}",
        path.display()
    );
}

fn expand_includes(
    path: &Path,
    selector: Option<&str>,
    base_dir: &Path,
    active: &mut Vec<(PathBuf, Option<String>)>,
) -> Result<(String, bool)> {
    let canonical_path = crate::fs::is_path_within(base_dir, path)
        .with_context(|| format!("Failed to resolve Markdown file {}", path.display()))?
        .canonicalize()
        .with_context(|| format!("Failed to resolve Markdown file {}", path.display()))?;
    let key = (canonical_path.clone(), selector.map(str::to_owned));
    if active.contains(&key) {
        let cycle = active
            .iter()
            .map(|(path, selector)| match selector {
                Some(selector) => format!("{}:{selector}", path.display()),
                None => path.display().to_string(),
            })
            .chain(std::iter::once(match selector {
                Some(selector) => format!("{}:{selector}", canonical_path.display()),
                None => canonical_path.display().to_string(),
            }))
            .collect::<Vec<_>>()
            .join(" -> ");
        anyhow::bail!("Recursive Markdown include cycle: {cycle}");
    }

    active.push(key);
    let buf = fs::read_to_string(&canonical_path)
        .with_context(|| format!("Failed to read Markdown file {}", canonical_path.display()))?;
    let selected = selected_contents(&buf, selector, &canonical_path)?;
    let mut new_txt = String::with_capacity(selected.len());
    let mut last_match = 0;
    let mut modified = false;

    for cap in INSERT_REGEX.captures_iter(selected) {
        let matched = cap.get(0).context("Missing include match")?;
        new_txt.push_str(&selected[last_match..matched.start()]);

        let rel_file_path = cap
            .name("filepath")
            .context("Missing filepath capture")?
            .as_str();
        let include_selector = cap.name("selector").map(|selector| selector.as_str());
        if rel_file_path.ends_with("refs.md") {
            info!(
                "Leaving reference include unchanged in {}",
                canonical_path.display()
            );
            new_txt.push_str(matched.as_str());
        } else {
            let parent_dir = canonical_path
                .parent()
                .context("Expected parent directory")?;
            let include_path = parent_dir.join(rel_file_path);
            let safe_include =
                crate::fs::is_path_within(base_dir, &include_path).with_context(|| {
                    format!(
                        "Failed to resolve include {} from {}",
                        include_path.display(),
                        canonical_path.display()
                    )
                })?;
            info!("Insert {safe_include:?}");
            let (contents, _) = expand_includes(&safe_include, include_selector, base_dir, active)
                .with_context(|| {
                    format!(
                        "Failed to expand include {}{} from {}",
                        safe_include.display(),
                        include_selector.unwrap_or_default(),
                        canonical_path.display()
                    )
                })?;
            new_txt.push_str(&contents);
            modified = true;
        }
        last_match = matched.end();
    }
    new_txt.push_str(&selected[last_match..]);
    active.pop();

    Ok((new_txt, modified))
}

fn replace_file_atomically(path: &Path, contents: &[u8]) -> Result<()> {
    let parent_dir = path
        .parent()
        .context("Expected source file parent directory")?;
    let mut staged = NamedTempFile::new_in(parent_dir)
        .with_context(|| format!("Failed to stage expanded Markdown for {}", path.display()))?;
    staged
        .write_all(contents)
        .with_context(|| format!("Failed to write staged Markdown for {}", path.display()))?;
    staged
        .flush()
        .with_context(|| format!("Failed to flush staged Markdown for {}", path.display()))?;
    staged
        .persist(path)
        .map(|_| ())
        .map_err(|error| error.error)
        .with_context(|| format!("Failed to replace Markdown file {}", path.display()))
}

///  Within each mdBook-style Markdown file in a source directory,
/// replace {{#include file.md}} statements by the contents of the
/// included file.
///
/// See the [mdBook documentation](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files).
///
/// Note: {{#include *-refs.md}} are excluded.
///
/// markdown_src_dir_path: path to the source directory containing the
/// Markdown files.
/// Process files and replace include macros.
///
/// # Errors
///
/// Returns an error if the Markdown directory cannot be read, an include path
/// or anchor is invalid, an included file cannot be read, or a changed file
/// cannot be written.
#[tracing::instrument(skip(markdown_src_dir_path))]
pub fn include_in_all_markdown_files_in<P>(markdown_src_dir_path: P) -> Result<()>
where
    P: AsRef<Path> + std::marker::Sync,
{
    let base_dir = markdown_src_dir_path
        .as_ref()
        .canonicalize()
        .with_context(|| {
            format!(
                "Failed to resolve Markdown source directory {}",
                markdown_src_dir_path.as_ref().display()
            )
        })?;

    let paths = crate::fs::find_markdown_files_in(&base_dir)?;
    let planned: Result<Vec<Option<PlannedReplacement>>> = paths
        .into_par_iter()
        .map(|path| {
            info!("Looking into {path:?}");
            let canonical_path = path
                .canonicalize()
                .with_context(|| format!("Failed to resolve Markdown file {}", path.display()))?;
            if !canonical_path.starts_with(&base_dir) {
                anyhow::bail!(
                    "Markdown file {} resolves outside source directory {}",
                    path.display(),
                    base_dir.display()
                );
            }
            let (expanded, modified) =
                expand_includes(&canonical_path, None, &base_dir, &mut Vec::new())?;
            Ok(modified.then(|| PlannedReplacement {
                path: canonical_path,
                contents: expanded.into_bytes(),
            }))
        })
        .collect();
    let mut planned: Vec<_> = planned?.into_iter().flatten().collect();
    planned.sort_by(|left, right| left.path.cmp(&right.path));

    // Resolve every include before replacing any Markdown file. This keeps a
    // missing or forbidden include from leaving a partially transformed tree.
    for replacement in planned {
        replace_file_atomically(&replacement.path, &replacement.contents)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_include_in_all_markdown_files_in() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let main_md_path = src_dir.join("main.md");
        let include1_md_path = src_dir.join("include1.md");
        let refs_md_path = src_dir.join("some-refs.md");
        let sub_dir = src_dir.join("sub");
        fs::create_dir(&sub_dir)?;
        let include2_md_path = sub_dir.join("include2.md");

        fs::write(&include1_md_path, "Content of include1")?;
        fs::write(&refs_md_path, "Content of refs")?;
        fs::write(&include2_md_path, "Content of include2")?;

        let main_content = r#"
# Main
{{#include include1.md}}
{{#include some-refs.md}}
{{#include sub/include2.md}}
"#;
        fs::write(&main_md_path, main_content)?;

        include_in_all_markdown_files_in(&src_dir)?;

        let updated_content = fs::read_to_string(&main_md_path)?;
        assert!(updated_content.contains("Content of include1"));
        assert!(updated_content.contains("{{#include some-refs.md}}"));
        assert!(!updated_content.contains("Content of refs"));
        assert!(updated_content.contains("Content of include2"));
        assert!(!updated_content.contains("{{#include include1.md}}"));
        assert!(!updated_content.contains("{{#include sub/include2.md}}"));

        Ok(())
    }

    #[test]
    fn failed_include_expansion_preserves_the_source_file() -> Result<()> {
        for content in [
            "{{#include missing.md}} then {{#include valid.md}}\n",
            "{{#include valid.md}} then {{#include missing.md}}\n",
            "{{#include valid.md}} then {{#include missing.md}} then {{#include missing.md}}\n",
        ] {
            let dir = tempdir()?;
            let src_dir = dir.path().join("src");
            fs::create_dir(&src_dir)?;
            fs::write(src_dir.join("valid.md"), "included\n")?;
            let source = src_dir.join("doc.md");
            fs::write(&source, content)?;

            assert!(include_in_all_markdown_files_in(&src_dir).is_err());
            assert_eq!(fs::read_to_string(source)?, content);
        }

        Ok(())
    }

    #[test]
    fn nested_include_paths_resolve_from_the_including_chapter() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        let chapter_dir = src_dir.join("chapters");
        fs::create_dir_all(chapter_dir.join("shared"))?;
        fs::write(chapter_dir.join("shared/content.md"), "nested content\n")?;
        let source = chapter_dir.join("chapter.md");
        fs::write(&source, "{{#include shared/content.md}}\n")?;

        include_in_all_markdown_files_in(&src_dir)?;

        assert_eq!(fs::read_to_string(source)?, "nested content\n\n");
        Ok(())
    }

    #[test]
    fn recursively_expands_nested_includes_and_rejects_cycles_without_writes() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir_all(&src_dir)?;
        let source = src_dir.join("doc.md");
        let outer = src_dir.join("outer.md");
        fs::write(src_dir.join("inner.md"), "nested content\n")?;
        fs::write(&outer, "before {{#include inner.md}} after\n")?;
        fs::write(&source, "{{#include outer.md}}\n")?;

        include_in_all_markdown_files_in(&src_dir)?;
        assert_eq!(
            fs::read_to_string(&source)?,
            "before nested content\n after\n\n"
        );

        let cycle_a = src_dir.join("cycle-a.md");
        let cycle_b = src_dir.join("cycle-b.md");
        let cycle_a_contents = "a {{#include cycle-b.md}}\n";
        let cycle_b_contents = "b {{#include cycle-a.md}}\n";
        fs::write(&cycle_a, cycle_a_contents)?;
        fs::write(&cycle_b, cycle_b_contents)?;

        assert!(include_in_all_markdown_files_in(&src_dir).is_err());
        assert_eq!(fs::read_to_string(cycle_a)?, cycle_a_contents);
        assert_eq!(fs::read_to_string(cycle_b)?, cycle_b_contents);
        Ok(())
    }

    #[test]
    fn expands_line_ranges_and_named_anchors_without_normalizing_line_endings() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir_all(&src_dir)?;
        fs::write(
            src_dir.join("snippet.md"),
            "before\r\n<!-- ANCHOR: sample -->\r\nfirst\r\nsecond\r\n<!-- ANCHOR_END: sample -->\r\nafter\r\n",
        )?;
        let source = src_dir.join("doc.md");
        fs::write(
            &source,
            "{{#include snippet.md:3:4}}\n{{#include snippet.md:sample}}\n",
        )?;

        include_in_all_markdown_files_in(&src_dir)?;

        assert_eq!(
            fs::read_to_string(source)?,
            "first\r\nsecond\r\n\nfirst\r\nsecond\r\n\n"
        );
        Ok(())
    }
}
