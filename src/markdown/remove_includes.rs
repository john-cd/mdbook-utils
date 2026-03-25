//! Remove left-over {{#include file.ext}} from mdbook-style Markdown sources.
//!
//! Use to clean up Markdown sources or when the [output.markdown] renderer is
//! enabled in `book.toml`.

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::info;

/// Regex to find {{#include \<file\>.md}}
static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{\{#include.*?\}\}").unwrap());

/// Within each mdBook-style Markdown file in a source directory,
/// remove any left-over {{#include file.md}} statements
/// (and replace by a hard-coded string).
///
/// See the [mdBook documentation](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files).
///
/// markdown_src_dir_path: path to the source directory containing the Markdown
/// files.
///
/// contents_to_insert: replacement of {{#include ...}} statements (same value
/// for all statements). Can be "".
/// Within each mdBook-style Markdown file in a source directory,
/// remove any left-over {{#include file.md}} statements
/// (and replace by a hard-coded string).
pub fn remove_includes_in_all_markdown_files_in<P>(
    markdown_dir_path: P,
    contents_to_insert: &str,
) -> Result<Vec<std::path::PathBuf>>
where
    P: AsRef<Path>,
{
    let mut modified = Vec::new();

    // Locate the Markdown files with the `src`` directory
    let paths = crate::fs::find_markdown_files_in(markdown_dir_path.as_ref())?;

    // TODO LATER: consider inserting contents from a file
    // path_file_to_insert: Option<P2>,
    // let contents_to_insert = if let Some(to_insert) = path_file_to_insert {
    //      fs::read_to_string(to_insert)?
    // } else { String::new( )};
    // // debug!("{}", contents_to_insert);

    // Process each .md file
    for p in paths {
        info!("Looking into {p:?}");
        let buf = fs::read_to_string(p.as_path())?;
        if REGEX.is_match(&buf) {
            let mut new_txt = buf.clone();
            for cap in REGEX.captures_iter(&buf) {
                new_txt = new_txt.replace(cap.get(0).unwrap().as_str(), contents_to_insert);
            }
            if new_txt != buf {
                // tracing::debug!("modified: {}", p.display());
                File::create(p.clone())?.write_all(new_txt.as_bytes())?;
                modified.push(p);
            }
        }
    }
    Ok(modified)
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_remove_includes_in_all_markdown_files_in() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let main_md_path = src_dir.join("main.md");
        let main_content = r#"
# Main
{{#include include1.md}}
Some text
{{#include include2.md}}
"#;
        fs::write(&main_md_path, main_content)?;

        // Test with a replacement string
        let modified = remove_includes_in_all_markdown_files_in(&src_dir, "REPLACED")?;
        assert_eq!(modified.len(), 1);
        assert_eq!(modified[0], main_md_path);

        let updated_content = fs::read_to_string(&main_md_path)?;
        assert!(!updated_content.contains("{{#include include1.md}}"));
        assert!(!updated_content.contains("{{#include include2.md}}"));
        assert_eq!(updated_content.matches("REPLACED").count(), 2);

        // Test with empty string
        fs::write(&main_md_path, main_content)?;
        let modified = remove_includes_in_all_markdown_files_in(&src_dir, "")?;
        assert_eq!(modified.len(), 1);
        let updated_content = fs::read_to_string(&main_md_path)?;
        assert!(!updated_content.contains("{{#include"));
        assert!(updated_content.contains("# Main"));
        assert!(updated_content.contains("Some text"));

        Ok(())
    }
}
