// TODO Reviewed ??-??-2026
//! Extract Rust code examples from Markdown
use std::fs;
use std::io::Write;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use regex::Regex;
use tempfile::NamedTempFile;
use tracing::info;

/// Embedded Rust code extraction from Markdown
static EXTRACT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)```rust.*?\n(?<code>.*?)```").unwrap());

/// Regex to match hidden lines in Rust code blocks.
///
/// In mdBook, lines starting with `# ` (pound sign followed by a space) are
/// hidden in the rendered output but remain part of the code. This pattern
/// matches such lines and captures the rest of the line (after `# `) in the
/// named group `rest`, so they can be included verbatim in extracted `.rs`
/// files.
static REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^# (?<rest>.*)$").unwrap());

/// Regex to match and capture entire Rust code blocks, including their
/// boundaries and content.
///
/// Uses the `(?s)` (DOTALL) flag so that `.` matches newlines, allowing the
/// pattern to span multiple lines. It captures three named groups:
/// - `first`: the opening fence line (e.g. `` ```rust ``) up to and including
///   the newline.
/// - `code`: the body of the code block.
/// - `last`: the closing `` ``` `` fence.
///
/// This is used to locate complete code blocks when replacing inline code with
/// `{{#include ...}}` directives.
static REG2: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)(?<first>```rust.*?\n)(?<code>.*?\n)(?<last>```)").unwrap());

struct PlannedFile {
    path: PathBuf,
    contents: Vec<u8>,
}

/// Return the stable output path for one code block in a Markdown chapter.
///
/// Keeping the complete Markdown path in a per-chapter directory prevents
/// chapters with the same basename (for example, `a/index.md` and
/// `b/index.md`) from sharing output files.
fn code_block_path(
    markdown_root: &Path,
    markdown_file: &Path,
    code_root: &Path,
    block: usize,
) -> Result<PathBuf> {
    let relative_markdown = markdown_file.strip_prefix(markdown_root).with_context(|| {
        format!(
            "Markdown file {} is outside source directory {}",
            markdown_file.display(),
            markdown_root.display()
        )
    })?;
    let chapter_dir = code_root.join(relative_markdown.with_extension("md.d"));
    Ok(chapter_dir.join(format!("block-{block:03}.rs")))
}

/// Build a relative path from `from_dir` to `target` on the same volume.
fn relative_path(from_dir: &Path, target: &Path) -> Result<PathBuf> {
    let from_components: Vec<_> = from_dir.components().collect();
    let target_components: Vec<_> = target.components().collect();
    let common = from_components
        .iter()
        .zip(&target_components)
        .take_while(|(from, target)| from == target)
        .count();

    if common == 0 {
        bail!(
            "Cannot express include path from {} to {} because the paths have no common root",
            from_dir.display(),
            target.display()
        );
    }

    let mut relative = PathBuf::new();
    for component in &from_components[common..] {
        if !matches!(component, Component::Normal(_)) {
            bail!(
                "Cannot construct a relative include path from {}",
                from_dir.display()
            );
        }
        relative.push("..");
    }
    for component in &target_components[common..] {
        if !matches!(component, Component::Normal(_)) {
            bail!(
                "Cannot construct a relative include path to {}",
                target.display()
            );
        }
        relative.push(component.as_os_str());
    }

    if relative.as_os_str().is_empty() {
        relative.push(".");
    }
    Ok(relative)
}

fn markdown_files_under(root: &Path) -> Result<Vec<(PathBuf, PathBuf)>> {
    let root = root.canonicalize().with_context(|| {
        format!(
            "Failed to resolve Markdown source directory {}",
            root.display()
        )
    })?;
    crate::fs::find_markdown_files_in(&root)?
        .into_iter()
        .map(|path| {
            let canonical = path
                .canonicalize()
                .with_context(|| format!("Failed to resolve Markdown file {}", path.display()))?;
            if !canonical.starts_with(&root) {
                bail!(
                    "Markdown file {} resolves outside source directory {}",
                    path.display(),
                    root.display()
                );
            }
            Ok((root.clone(), canonical))
        })
        .collect()
}

fn check_plan_collisions(plans: &mut [PlannedFile]) -> Result<()> {
    plans.sort_by(|left, right| left.path.cmp(&right.path));
    for pair in plans.windows(2) {
        if pair[0].path == pair[1].path {
            bail!(
                "Multiple Markdown code blocks map to {}",
                pair[0].path.display()
            );
        }
    }
    Ok(())
}

fn existing_file_matches(path: &Path, contents: &[u8]) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            bail!(
                "Refusing to use a symlink as generated output: {}",
                path.display()
            );
        }
        Ok(metadata) if !metadata.is_file() => {
            bail!(
                "Generated output path is not a regular file: {}",
                path.display()
            );
        }
        Ok(_) => {
            let existing = fs::read(path)
                .with_context(|| format!("Failed to read existing output {}", path.display()))?;
            if existing == contents {
                Ok(true)
            } else {
                bail!("Refusing to overwrite existing output {}", path.display());
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => {
            Err(error).with_context(|| format!("Failed to inspect output {}", path.display()))
        }
    }
}

/// Write a generated file without replacing unrelated user data.
///
/// Repeating extraction with identical source is idempotent. If an output
/// already exists with different contents, the operation fails before any
/// output is written.
fn write_generated_file(path: &Path, contents: &[u8]) -> Result<()> {
    if existing_file_matches(path, contents)? {
        return Ok(());
    }

    let parent = path
        .parent()
        .context("Expected generated output parent directory")?;
    fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create output directory {}", parent.display()))?;
    let mut staged = NamedTempFile::new_in(parent)
        .with_context(|| format!("Failed to stage generated output for {}", path.display()))?;
    staged
        .write_all(contents)
        .with_context(|| format!("Failed to write staged output for {}", path.display()))?;
    staged
        .flush()
        .with_context(|| format!("Failed to flush staged output for {}", path.display()))?;

    match staged.persist_noclobber(path) {
        Ok(_) => Ok(()),
        Err(error) if error.error.kind() == std::io::ErrorKind::AlreadyExists => {
            if existing_file_matches(path, contents)? {
                Ok(())
            } else {
                bail!("Refusing to overwrite existing output {}", path.display());
            }
        }
        Err(error) => Err(error.error)
            .with_context(|| format!("Failed to create generated output {}", path.display())),
    }
}

/// Replace an existing file after its complete new contents have been staged.
fn replace_file(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .context("Expected source file parent directory")?;
    let mut staged = NamedTempFile::new_in(parent)
        .with_context(|| format!("Failed to stage replacement for {}", path.display()))?;
    staged
        .write_all(contents)
        .with_context(|| format!("Failed to write staged replacement for {}", path.display()))?;
    staged
        .flush()
        .with_context(|| format!("Failed to flush staged replacement for {}", path.display()))?;
    staged
        .persist(path)
        .map(|_| ())
        .map_err(|error| error.error)
        .with_context(|| format!("Failed to replace {}", path.display()))
}

/// Extract code examples from all Markdown files within a source
/// directory and write them to separate files.
///
/// Output paths preserve each chapter's source-relative path and use a
/// deterministic block index. Existing identical outputs are kept; conflicting
/// files are reported instead of being truncated.
///
/// markdown_src_dir_path: path to the source directory
///
/// code_dest_dir_path: path to the directory, where destination files
/// will be created
pub fn extract_code_from_all_markdown_files_in<P1, P2>(
    markdown_src_dir_path: P1,
    code_dest_dir_path: P2,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path> + std::marker::Sync,
{
    let markdown_files = markdown_files_under(markdown_src_dir_path.as_ref())?;
    crate::fs::create_dir(code_dest_dir_path.as_ref())?;
    let code_dest_root = code_dest_dir_path
        .as_ref()
        .canonicalize()
        .with_context(|| {
            format!(
                "Failed to resolve code output directory {}",
                code_dest_dir_path.as_ref().display()
            )
        })?;

    let planned: Result<Vec<Vec<PlannedFile>>> = markdown_files
        .into_par_iter()
        .map(|(markdown_root, markdown_file)| {
            info!("{markdown_file:?}");
            let buf = fs::read_to_string(&markdown_file).with_context(|| {
                format!("Failed to read Markdown file {}", markdown_file.display())
            })?;
            EXTRACT_REGEX
                .captures_iter(&buf)
                .enumerate()
                .map(|(number, captures)| {
                    let code = REG.replace_all(&captures[1], "$rest");
                    let output_path =
                        code_block_path(&markdown_root, &markdown_file, &code_dest_root, number)?;
                    crate::fs::is_path_within(&code_dest_root, &output_path).with_context(
                        || {
                            format!(
                                "Generated output {} is outside the destination directory",
                                output_path.display()
                            )
                        },
                    )?;
                    info!(" {number}: {output_path:?}\n");
                    Ok(PlannedFile {
                        path: output_path,
                        contents: code.as_bytes().to_vec(),
                    })
                })
                .collect()
        })
        .collect();

    let mut planned: Vec<PlannedFile> = planned?.into_iter().flatten().collect();
    check_plan_collisions(&mut planned)?;

    // Check every destination before writing any of them, so a collision late
    // in the traversal cannot leave earlier chapters partially extracted.
    for output in &planned {
        if existing_file_matches(&output.path, &output.contents)? {
            continue;
        }
    }
    for output in planned {
        write_generated_file(&output.path, &output.contents)?;
    }
    Ok(())
}

/// Remove Rust code blocks from Markdown files, replacing each by an
/// `{{#include ...}}` statement that points to the corresponding generated
/// code file.
///
/// markdown_src_dir_path: path to the source directory containing the
/// Markdown files
///
/// code_dir_path: path to the folder containing the extracted Rust code
pub fn remove_code_from_all_markdown_files_in<P1, P2>(
    markdown_src_dir_path: P1,
    code_dir_path: P2,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path> + std::marker::Sync,
{
    let markdown_files = markdown_files_under(markdown_src_dir_path.as_ref())?;
    let code_root = code_dir_path.as_ref().canonicalize().with_context(|| {
        format!(
            "Failed to resolve extracted code directory {}",
            code_dir_path.as_ref().display()
        )
    })?;

    let planned: Result<Vec<Option<PlannedFile>>> = markdown_files
        .into_par_iter()
        .map(|(markdown_root, markdown_file)| {
            info!("{markdown_file:?}");
            let buf = fs::read_to_string(&markdown_file).with_context(|| {
                format!("Failed to read Markdown file {}", markdown_file.display())
            })?;
            let mut new_txt = String::with_capacity(buf.len());
            let mut last_match = 0;
            let mut counter = 0;

            for captures in REG2.captures_iter(&buf) {
                let matched = captures.get(0).context("Missing Rust code block match")?;
                new_txt.push_str(&buf[last_match..matched.start()]);

                let code_path =
                    code_block_path(&markdown_root, &markdown_file, &code_root, counter)?;
                let safe_code_path = crate::fs::is_path_within(&code_root, &code_path)
                    .with_context(|| {
                        format!(
                            "Extracted code path {} is outside the code directory",
                            code_path.display()
                        )
                    })?;
                let canonical_code_path = safe_code_path.canonicalize().with_context(|| {
                    format!("Missing extracted code file {}", safe_code_path.display())
                })?;
                if !canonical_code_path.is_file() {
                    bail!(
                        "Extracted code path is not a file: {}",
                        canonical_code_path.display()
                    );
                }
                let source_dir = markdown_file
                    .parent()
                    .context("Expected Markdown file parent directory")?;
                let include_path = relative_path(source_dir, &canonical_code_path)?;
                let include_path = include_path
                    .to_str()
                    .context("Include path contains non-UTF-8 characters")?
                    .replace('\\', "/");
                let first = captures
                    .name("first")
                    .context("Missing Rust code fence")?
                    .as_str();
                let last = captures
                    .name("last")
                    .context("Missing closing code fence")?
                    .as_str();
                new_txt.push_str(&format!("{first}{{{{#include {include_path}}}}}\n{last}"));

                last_match = matched.end();
                counter += 1;
            }

            if counter == 0 {
                return Ok(None);
            }
            new_txt.push_str(&buf[last_match..]);
            Ok(Some(PlannedFile {
                path: markdown_file,
                contents: new_txt.into_bytes(),
            }))
        })
        .collect();

    let mut planned: Vec<PlannedFile> = planned?.into_iter().flatten().collect();
    planned.sort_by(|left, right| left.path.cmp(&right.path));
    for source in planned {
        replace_file(&source.path, &source.contents)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use tempfile::tempdir;

    use super::*;

    fn chapter_with_blocks(blocks: &[&str]) -> String {
        let mut content = String::from("# Chapter\n");
        for block in blocks {
            content.push_str("```rust\n");
            content.push_str(block);
            content.push_str("\n```\n");
        }
        content
    }

    #[test]
    fn extracts_same_basename_chapters_without_collisions_and_is_repeatable() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        let code_dir = src_dir.join("code");
        fs::create_dir_all(src_dir.join("a"))?;
        fs::create_dir_all(src_dir.join("b"))?;
        fs::write(
            src_dir.join("a/index.md"),
            chapter_with_blocks(&["fn a() {}", "# fn hidden() {}"]),
        )?;
        fs::write(
            src_dir.join("b/index.md"),
            chapter_with_blocks(&["fn b() {}"]),
        )?;

        extract_code_from_all_markdown_files_in(&src_dir, &code_dir)?;
        extract_code_from_all_markdown_files_in(&src_dir, &code_dir)?;

        assert_eq!(
            fs::read_to_string(code_dir.join("a/index.md.d/block-000.rs"))?,
            "fn a() {}\n"
        );
        assert_eq!(
            fs::read_to_string(code_dir.join("a/index.md.d/block-001.rs"))?,
            "fn hidden() {}\n"
        );
        assert_eq!(
            fs::read_to_string(code_dir.join("b/index.md.d/block-000.rs"))?,
            "fn b() {}\n"
        );
        Ok(())
    }

    #[test]
    fn rejects_existing_output_conflicts_before_writing_other_chapters() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        let code_dir = src_dir.join("code");
        fs::create_dir_all(src_dir.join("a"))?;
        fs::create_dir_all(src_dir.join("b"))?;
        fs::write(
            src_dir.join("a/index.md"),
            chapter_with_blocks(&["fn a() {}"]),
        )?;
        fs::write(
            src_dir.join("b/index.md"),
            chapter_with_blocks(&["fn b() {}"]),
        )?;
        let conflicting_output = code_dir.join("b/index.md.d/block-000.rs");
        fs::create_dir_all(
            conflicting_output
                .parent()
                .context("expected output parent")?,
        )?;
        fs::write(&conflicting_output, "hand-written content\n")?;

        assert!(extract_code_from_all_markdown_files_in(&src_dir, &code_dir).is_err());
        assert!(!code_dir.join("a/index.md.d/block-000.rs").exists());
        assert_eq!(
            fs::read_to_string(conflicting_output)?,
            "hand-written content\n"
        );
        Ok(())
    }

    #[test]
    fn replacement_uses_the_same_per_chapter_mapping() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        let code_dir = src_dir.join("code");
        fs::create_dir_all(src_dir.join("a"))?;
        let chapter = src_dir.join("a/index.md");
        fs::write(&chapter, chapter_with_blocks(&["fn main() {}"]))?;

        extract_code_from_all_markdown_files_in(&src_dir, &code_dir)?;
        remove_code_from_all_markdown_files_in(&src_dir, &code_dir)?;

        let updated = fs::read_to_string(chapter)?;
        assert!(updated.contains("{{#include ../code/a/index.md.d/block-000.rs}}"));
        assert_eq!(
            fs::read_to_string(code_dir.join("a/index.md.d/block-000.rs"))?,
            "fn main() {}\n"
        );
        Ok(())
    }
}
