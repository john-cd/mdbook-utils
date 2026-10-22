// TODO Reviewed ??-??-2026
//! Functions to identify unused files
//! like examples and files not in SUMMARY.md

use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use percent_encoding::percent_decode_str;
use regex::Regex;

use crate::fs;
use crate::parser;

/// Regex for mdBook helpers that refer to Rust source files.
static RUST_FILE_REFERENCE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\{\{\s*#(?:include|rustdoc_include|playground)\s+(?P<path>[^{}\s]+?\.rs)(?::[^{}\s]*)?(?:\s+[^{}]*)?\s*\}\}",
    )
    .expect("Rust file reference regex is valid")
});

/// Return whether a helper match is inside a quoted string or inline code span.
fn is_inside_literal(content: &str, match_start: usize) -> bool {
    let line_start = content[..match_start]
        .rfind('\n')
        .map_or(0, |newline| newline + 1);
    let mut in_literal = false;
    let mut escaped = false;
    for character in content[line_start..match_start].chars() {
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == '"' || character == '`' {
            in_literal = !in_literal;
        }
    }
    in_literal
}

/// Return a normalized relative chapter path from a SUMMARY link, if supported.
fn summary_chapter_path(url: &str) -> Option<PathBuf> {
    let path_end = url.find(['?', '#']).unwrap_or(url.len());
    let url_path = &url[..path_end];
    if url_path.is_empty() || url_path.starts_with("//") {
        return None;
    }

    if url::Url::parse(url_path).is_ok() {
        return None;
    }

    let decoded = percent_decode_str(url_path).decode_utf8().ok()?;
    let relative_path = decoded.strip_prefix('/').unwrap_or(&decoded);
    let path = Path::new(relative_path);
    if path.is_absolute() || path.extension().is_none_or(|extension| extension != "md") {
        return None;
    }

    Some(path.to_path_buf())
}

/// Identify .md files not in SUMMARY.md
///
/// # Errors
///
/// Returns an error if the source directory is unavailable or its
/// `SUMMARY.md` file cannot be read.
#[tracing::instrument]
pub fn identify_files_not_in_summary<P: AsRef<Path> + std::fmt::Debug>(
    markdown_src_dir_path: P,
) -> Result<Vec<PathBuf>> {
    let markdown_src_dir_path = fs::check_is_dir(&markdown_src_dir_path)?;
    let all_files = fs::find_markdown_files_in(&markdown_src_dir_path)?;

    let summary_path = markdown_src_dir_path.join("SUMMARY.md");
    if !summary_path.exists() {
        bail!(
            "SUMMARY.md not found in {}",
            markdown_src_dir_path.display()
        );
    }

    let summary_content = std::fs::read_to_string(&summary_path)?;
    let mut parser = parser::get_parser(&summary_content);
    let links = parser::extract_links(&mut parser);

    let mut files_in_summary = std::collections::HashSet::new();
    for l in links {
        let url = l.get_url();
        if let Some(relative_path) = summary_chapter_path(&url) {
            let path = markdown_src_dir_path.join(relative_path);
            if let Ok(canon) = fs::is_path_within(&markdown_src_dir_path, &path) {
                files_in_summary.insert(canon);
            }
        }
    }

    let mut missing = Vec::new();
    for f in all_files {
        if let Ok(canon) = f.canonicalize() {
            let file_name = f.file_name().context("Failed to get file name")?;
            if !files_in_summary.contains(&canon) && file_name.to_string_lossy() != "SUMMARY.md" {
                missing.push(f);
            }
        }
    }

    tracing::debug!(
        count = missing.len(),
        "identified markdown files not referenced in SUMMARY.md"
    );
    Ok(missing)
}

/// Identify .rs examples not used in Markdown files
///
/// # Errors
///
/// Returns an error if either source directory is unavailable or a Markdown
/// file cannot be read.
#[tracing::instrument]
pub fn identify_unused_rs_examples<
    P1: AsRef<Path> + std::fmt::Debug,
    P2: AsRef<Path> + std::fmt::Debug,
>(
    markdown_src_dir_path: P1,
    code_dir_path: P2,
) -> Result<Vec<PathBuf>> {
    let markdown_src_dir_path = fs::check_is_dir(&markdown_src_dir_path)?;
    let code_dir_path = fs::check_is_dir(&code_dir_path)?;

    let mut all_rs_files = Vec::new();
    for entry in walkdir::WalkDir::new(&code_dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "rs"))
    {
        match entry.path().canonicalize() {
            Ok(canon) => all_rs_files.push(canon),
            Err(e) => {
                tracing::warn!("Failed to canonicalize {:?}: {}", entry.path(), e);
                continue;
            }
        }
    }

    let mut used_rs_files = std::collections::HashSet::new();
    let md_files = fs::find_markdown_files_in(&markdown_src_dir_path)?;

    for md_file in md_files {
        let content = std::fs::read_to_string(&md_file)?;
        for cap in RUST_FILE_REFERENCE_REGEX.captures_iter(&content) {
            if is_inside_literal(
                &content,
                cap.get(0).context("Missing Rust helper match")?.start(),
            ) {
                continue;
            }
            let rel_path = Path::new(&cap["path"]);
            if let Some(parent) = md_file.parent() {
                let abs_path = parent.join(rel_path);
                if let Ok(canon) = fs::is_path_within(&code_dir_path, &abs_path) {
                    used_rs_files.insert(canon);
                }
            }
        }
    }

    let mut unused = Vec::new();
    for f in all_rs_files {
        if !used_rs_files.contains(&f) {
            unused.push(f);
        }
    }

    tracing::debug!(count = unused.len(), "identified unused rust example files");
    Ok(unused)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_identify_files_not_in_summary_all_included() -> Result<()> {
        let dir = tempdir()?;
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root)?;

        let summary_path = root.join("SUMMARY.md");
        fs::write(&summary_path, "[Page 1](./page1.md)\n[Page 2](page2.md)")?;

        fs::write(root.join("page1.md"), "# Page 1")?;
        fs::write(root.join("page2.md"), "# Page 2")?;

        let missing = identify_files_not_in_summary(&root)?;
        assert!(
            missing.is_empty(),
            "Expected no missing files, but got {:?}",
            missing
        );
        Ok(())
    }

    #[test]
    fn test_identify_files_not_in_summary_some_missing() -> Result<()> {
        let dir = tempdir()?;
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root)?;

        let summary_path = root.join("SUMMARY.md");
        fs::write(&summary_path, "[Page 1](./page1.md)")?;

        fs::write(root.join("page1.md"), "# Page 1")?;
        fs::write(root.join("page2.md"), "# Page 2")?;

        let missing = identify_files_not_in_summary(&root)?;
        assert_eq!(missing.len(), 1);
        assert_eq!(
            missing[0].file_name().unwrap().to_string_lossy(),
            "page2.md"
        );
        Ok(())
    }

    #[test]
    fn test_identify_files_not_in_summary_no_summary() -> Result<()> {
        let dir = tempdir()?;
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root)?;

        fs::write(root.join("page1.md"), "# Page 1")?;

        let result = identify_files_not_in_summary(&root);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("SUMMARY.md not found in")
        );
        Ok(())
    }

    #[test]
    fn test_identify_files_not_in_summary_nested_files() -> Result<()> {
        let dir = tempdir()?;
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root)?;

        let sub_dir = root.join("sub");
        fs::create_dir(&sub_dir)?;

        let summary_path = root.join("SUMMARY.md");
        fs::write(
            &summary_path,
            "[Page 1](./page1.md)\n[Sub Page](sub/page2.md)",
        )?;

        fs::write(root.join("page1.md"), "# Page 1")?;
        fs::write(sub_dir.join("page2.md"), "# Page 2")?;
        fs::write(sub_dir.join("page3.md"), "# Page 3")?; // missing

        let missing = identify_files_not_in_summary(&root)?;
        assert_eq!(missing.len(), 1);
        assert_eq!(
            missing[0].file_name().unwrap().to_string_lossy(),
            "page3.md"
        );
        Ok(())
    }

    /// Resolves nested, encoded, and fragment-bearing SUMMARY links while
    /// rejecting traversal.
    #[test]
    fn test_identify_files_not_in_summary_nested_encoded_and_traversal_paths() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path().join("src");
        fs::create_dir_all(root.join("nested/sub"))?;
        fs::create_dir_all(root.join("nested pages"))?;

        fs::write(
            root.join("SUMMARY.md"),
            "\
[Nested](./nested/sub/index.md#chapter-intro)\n\
[Encoded](nested%20pages/caf%C3%A9.md?print=1#details)\n\
[Outside](../../outside.md#hidden)\n\
",
        )?;
        fs::write(root.join("nested/sub/index.md"), "# Nested")?;
        fs::write(root.join("nested pages/café.md"), "# Encoded")?;
        fs::write(dir.path().join("outside.md"), "# Outside")?;
        fs::write(root.join("unlisted.md"), "# Unlisted")?;

        let missing = identify_files_not_in_summary(&root)?;
        assert_eq!(missing, vec![root.join("unlisted.md")]);
        Ok(())
    }

    #[test]
    fn test_identify_unused_rs_examples_invalid_markdown_dir() {
        let dir = tempdir().unwrap();
        let markdown_dir = dir.path().join("non_existent_markdown");
        let code_dir = dir.path().join("code");
        fs::create_dir(&code_dir).unwrap();

        let result = identify_unused_rs_examples(&markdown_dir, &code_dir);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("should be a folder and exist on disk!")
        );
    }

    #[test]
    fn test_identify_unused_rs_examples_invalid_code_dir() {
        let dir = tempdir().unwrap();
        let markdown_dir = dir.path().join("markdown");
        fs::create_dir(&markdown_dir).unwrap();
        let code_dir = dir.path().join("non_existent_code");

        let result = identify_unused_rs_examples(&markdown_dir, &code_dir);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("should be a folder and exist on disk!")
        );
    }

    #[test]
    fn test_identify_unused_rs_examples_happy_path() -> Result<()> {
        let dir = tempdir()?;
        let markdown_dir = dir.path().join("markdown");
        let code_dir = dir.path().join("code");

        fs::create_dir(&markdown_dir)?;
        fs::create_dir(&code_dir)?;

        // Create Rust files
        let used_rs = code_dir.join("used.rs");
        let unused_rs = code_dir.join("unused.rs");
        fs::write(&used_rs, "fn main() {}")?;
        fs::write(&unused_rs, "fn main() {}")?;

        // Create Markdown file using the used Rust file
        let md_file = markdown_dir.join("test.md");
        // Using a relative path that points into code_dir
        fs::write(&md_file, "{{#include ../code/used.rs}}")?;

        let unused_files = identify_unused_rs_examples(&markdown_dir, &code_dir)?;

        assert_eq!(unused_files.len(), 1);
        let canon_unused = unused_rs.canonicalize()?;
        assert_eq!(unused_files[0], canon_unused);

        Ok(())
    }

    /// Recognizes mdBook Rust helpers and ignores incidental paths in prose and
    /// code.
    #[test]
    fn test_identify_unused_rs_examples_supported_helpers_and_incidental_paths() -> Result<()> {
        let dir = tempdir()?;
        let markdown_dir = dir.path().join("markdown");
        let code_dir = dir.path().join("code");
        fs::create_dir(&markdown_dir)?;
        fs::create_dir(&code_dir)?;

        let included = code_dir.join("included.rs");
        let rustdoc = code_dir.join("rustdoc.rs");
        let playground = code_dir.join("playground.rs");
        let incidental = code_dir.join("incidental.rs");
        let literal_helper = code_dir.join("literal_helper.rs");
        for path in [
            &included,
            &rustdoc,
            &playground,
            &incidental,
            &literal_helper,
        ] {
            fs::write(path, "fn example() {}")?;
        }

        fs::write(
            markdown_dir.join("chapter.md"),
            "\
{{#include ../code/included.rs}}\n\
```rust\n\
{{#rustdoc_include ../code/rustdoc.rs:sample}}\n\
{{#playground ../code/playground.rs editable}}\n\
let path = \"../code/incidental.rs\";\n\
let directive = \"{{#include ../code/literal_helper.rs}}\";\n\
```\n\
The literal path `../code/incidental.rs` is documentation.\n\
",
        )?;

        let mut unused = identify_unused_rs_examples(&markdown_dir, &code_dir)?;
        unused.sort();
        let mut expected = vec![incidental.canonicalize()?, literal_helper.canonicalize()?];
        expected.sort();
        assert_eq!(unused, expected);
        Ok(())
    }
}
