//! Remove left-over {{#include <file>.md}}

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
///
/// See the [mdBook documentation](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files).
///
/// markdown_src_dir_path: path to the source directory containing the
/// Markdown files.
pub fn remove_includes_in_all_markdown_files_in<P>(markdown_src_dir_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    // Locate the Markdown files with the src directory
    let paths = crate::fs::find_markdown_files_in(markdown_src_dir_path.as_ref())?;

    // Process each .md file
    for p in paths {
        info!("Looking into {p:?}");
        let buf = fs::read_to_string(p.as_path())?;
        if REGEX.is_match(&buf) {
            let mut new_txt = buf.clone();
            for cap in REGEX.captures_iter(&buf) {
                new_txt = new_txt.replace(cap.get(0).unwrap().as_str(), "");
            }
            if new_txt != buf {
                // debug!("{}",  new_txt);
                File::create(p)?.write_all(new_txt.as_bytes())?;
            }
        }
    }
    Ok(())
}
