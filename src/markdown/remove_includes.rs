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
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_remove_includes_in_all_markdown_files_in() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        // 1. Markdown file with single include
        let md_file1 = src_dir.join("test1.md");
        fs::write(
            &md_file1,
            r#"# Test 1
Here is an include:
{{#include file1.rs}}
End.
"#,
        )?;

        // 2. Markdown file with multiple includes, same line and different lines
        let md_file2 = src_dir.join("test2.md");
        fs::write(
            &md_file2,
            r#"# Test 2
{{#include a.rs}} and {{#include b.rs}}
Another line.
{{#include c.rs}}
"#,
        )?;

        // 3. Markdown file with no includes
        let md_file3 = src_dir.join("test3.md");
        fs::write(
            &md_file3,
            r#"# Test 3
No includes here.
"#,
        )?;

        // 4. Non-markdown file (should be ignored)
        let txt_file = src_dir.join("test.txt");
        fs::write(
            &txt_file,
            r#"# Test text
{{#include ignore.rs}}
"#,
        )?;

        // Run the function
        let modified = remove_includes_in_all_markdown_files_in(&src_dir, "REPLACED")?;

        // Check the returned list of modified files
        assert_eq!(modified.len(), 2);
        assert!(modified.contains(&md_file1));
        assert!(modified.contains(&md_file2));

        // Verify contents of md_file1
        let content1 = fs::read_to_string(&md_file1)?;
        assert_eq!(
            content1,
            r#"# Test 1
Here is an include:
REPLACED
End.
"#
        );

        // Verify contents of md_file2
        let content2 = fs::read_to_string(&md_file2)?;
        assert_eq!(
            content2,
            r#"# Test 2
REPLACED and REPLACED
Another line.
REPLACED
"#
        );

        // Verify contents of md_file3 are unchanged
        let content3 = fs::read_to_string(&md_file3)?;
        assert_eq!(
            content3,
            r#"# Test 3
No includes here.
"#
        );

        // Verify contents of txt_file are unchanged
        let content_txt = fs::read_to_string(&txt_file)?;
        assert_eq!(
            content_txt,
            r#"# Test text
{{#include ignore.rs}}
"#
        );

        // Test with empty replacement string
        let md_file4 = src_dir.join("test4.md");
        fs::write(&md_file4, "Hello {{#include something.rs}} World!")?;

        let modified_empty = remove_includes_in_all_markdown_files_in(&src_dir, "")?;
        assert_eq!(modified_empty.len(), 1);
        assert!(modified_empty.contains(&md_file4));

        let content4 = fs::read_to_string(&md_file4)?;
        assert_eq!(content4, "Hello  World!");

        Ok(())
    }
}
