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
pub fn include_in_all_markdown_files_in<P>(markdown_src_dir_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    // Locate the Markdown files with the src directory
    let paths = crate::fs::find_markdown_files_in(markdown_src_dir_path.as_ref())?;

    // Process each .md file
    for p in paths {
        info!("Looking into {p:?}");
        let parent_dir = p.parent().unwrap().to_string_lossy();
        let buf = fs::read_to_string(p.as_path())?;
        if INSERT_REGEX.is_match(&buf) {
            let mut new_txt = String::with_capacity(buf.len());
            let mut last_match = 0;
            let mut modified_file = false;

            for cap in INSERT_REGEX.captures_iter(&buf) {
                let m = cap.get(0).unwrap();
                new_txt.push_str(&buf[last_match..m.start()]);

                let rel_file_path = cap.name("filepath").unwrap().as_str();
                if !rel_file_path.ends_with("refs.md") {
                    let path_file_to_insert = Path::new(parent_dir.as_ref()).join(rel_file_path);
                    info!("Insert {path_file_to_insert:?}");
                    let contents_to_insert = fs::read_to_string(path_file_to_insert)?;
                    new_txt.push_str(&contents_to_insert);
                    modified_file = true;
                } else {
                    info!("Ignored");
                    new_txt.push_str(m.as_str());
                }
                last_match = m.end();
            }
            new_txt.push_str(&buf[last_match..]);

            if modified_file {
                File::create(p)?.write_all(new_txt.as_bytes())?;
            }
        }
    }
    Ok(())
}

// TODO write tests
#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
