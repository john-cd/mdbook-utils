//! Functions to identify unused files
//! like examples and files not in SUMMARY.md

use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;

use crate::fs;
use crate::parser;

/// Identify .md files not in SUMMARY.md
// TODO: Handle nested directories more accurately in SUMMARY.md link parsing.
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
        if !url.starts_with("http") && url.ends_with(".md") {
            // Remove any leading ./ or /
            let clean_url = url.trim_start_matches("./").trim_start_matches('/');
            let path = markdown_src_dir_path.join(clean_url);
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

    Ok(missing)
}

/// Identify .rs examples not used in Markdown files
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

    /// Regex to match Rust files in Markdown
    static RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"(?P<path>[a-zA-Z0-9_.\-\/]+\.rs)").unwrap());

    for md_file in md_files {
        let content = std::fs::read_to_string(&md_file)?;
        for cap in RE.captures_iter(&content) {
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

    Ok(unused)
}

#[cfg(test)]
mod test {
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

    #[test]
    fn test_identify_unused_rs_examples_invalid_markdown_dir() {
        let dir = tempdir().unwrap();
        let markdown_dir = dir.path().join("non_existent_markdown");
        let code_dir = dir.path().join("code");
        fs::create_dir(&code_dir).unwrap();

        let result = identify_unused_rs_examples(&markdown_dir, &code_dir);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("should be a folder and exist on disk!"));
    }

    #[test]
    fn test_identify_unused_rs_examples_invalid_code_dir() {
        let dir = tempdir().unwrap();
        let markdown_dir = dir.path().join("markdown");
        fs::create_dir(&markdown_dir).unwrap();
        let code_dir = dir.path().join("non_existent_code");

        let result = identify_unused_rs_examples(&markdown_dir, &code_dir);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("should be a folder and exist on disk!"));
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
        fs::write(&md_file, "Check this: ../code/used.rs")?;

        let unused_files = identify_unused_rs_examples(&markdown_dir, &code_dir)?;

        assert_eq!(unused_files.len(), 1);
        let canon_unused = unused_rs.canonicalize()?;
        assert_eq!(unused_files[0], canon_unused);

        Ok(())
    }
}
