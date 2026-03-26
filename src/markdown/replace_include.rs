//! Replace {{#include file.md}} by the file contents
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::info;

/// Regex to find {{#include \<file\>.md}}.
static INSERT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{\{#include (?<filepath>\S+?\.md)\}\}").unwrap());

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
use anyhow::Context;

/// Process files and replace include macros
#[tracing::instrument(skip(markdown_src_dir_path))]
pub fn include_in_all_markdown_files_in<P>(markdown_src_dir_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    // Locate the Markdown files with the src directory
    let paths = crate::fs::find_markdown_files_in(markdown_src_dir_path.as_ref())?;

    // Process each .md file
    for p in paths {
        info!("Looking into {p:?}");
        let parent_dir = p
            .parent()
            .context("Expected parent directory")?
            .to_string_lossy();
        let buf = fs::read_to_string(p.as_path())?;
        if INSERT_REGEX.is_match(&buf) {
            let mut new_txt = buf.clone();
            for cap in INSERT_REGEX.captures_iter(&buf) {
                let rel_file_path = cap
                    .name("filepath")
                    .context("Missing filepath capture")?
                    .as_str();
                // debug!("relative file path: {rel_file_path:?}");
                if !rel_file_path.ends_with("refs.md") {
                    let path_file_to_insert = Path::new(parent_dir.as_ref()).join(rel_file_path);
                    info!("Insert {path_file_to_insert:?}");
                    let contents_to_insert = fs::read_to_string(path_file_to_insert)?;
                    // debug!("\n{}", contents_to_insert);
                    // debug!("{}", cap.get(0).unwrap().as_str());
                    new_txt = new_txt.replace(
                        cap.get(0).context("Missing entire match capture")?.as_str(),
                        &contents_to_insert,
                    );
                } else {
                    info!("Ignored");
                }
            }
            if new_txt != buf {
                // debug!("{}",  new_txt);
                File::create(p)?.write_all(new_txt.as_bytes())?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_include_in_all_markdown_files_in() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md1 = src_dir.join("main.md");
        fs::write(
            &md1,
            "Hello\n{{#include include1.md}}\nWorld\n{{#include ignored-refs.md}}",
        )?;

        let md2 = src_dir.join("include1.md");
        fs::write(&md2, "Included Content")?;

        let md3 = src_dir.join("ignored-refs.md");
        fs::write(&md3, "Should be ignored")?;

        include_in_all_markdown_files_in(&src_dir)?;

        let content = fs::read_to_string(&md1)?;
        assert_eq!(
            content,
            "Hello\nIncluded Content\nWorld\n{{#include ignored-refs.md}}"
        );

        Ok(())
    }
}
