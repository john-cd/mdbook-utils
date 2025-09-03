//! Extract Rust code examples from Markdown
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::Result;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use regex::Regex;
use tracing::info;

/// Embedded Rust code extraction from Markdown
static EXTRACT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)```rust.*?\n(?<code>.*?)```").unwrap());

// TODO document
static REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^(?:#\s)(?<rest>.*)$").unwrap());

/// Extract code examples from all Markdown files within a source
/// directory and write them to separate files.
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
    P2: AsRef<Path>,
{
    // Locate the Markdown files with the e.g. src/ directory
    let markdown_file_paths = crate::fs::find_markdown_files_in(markdown_src_dir_path.as_ref())?;

    // Create the destination directory if it doesn't exist
    crate::fs::create_dir(code_dest_dir_path.as_ref())?;

    // Process each .md file
    for p in markdown_file_paths {
        info!("{p:?}");
        let buf = fs::read_to_string(p.as_path())?;
        let random_string = Alphanumeric.sample_string(&mut rand::rng(), 5);

        // debug!("{p:?}: length = {}", buf.len());
        for (number, (_, [code])) in EXTRACT_REGEX
            .captures_iter(&buf)
            .map(|c| c.extract())
            .enumerate()
        {
            // remove "# " at beginning of lines
            let code = REG.replace_all(code, "$rest");
            let code_filename = format!(
                "{}{}{}",
                p.file_stem()
                    .unwrap_or(random_string.as_ref())
                    .to_string_lossy(),
                if number == 0 {
                    String::new()
                } else {
                    number.to_string()
                },
                ".rs"
            );
            let code_path = code_dest_dir_path.as_ref().join(code_filename);
            info!(" {number}: {code_path:?}\n");
            File::create(code_path)?.write_all(code.as_bytes())?;
        }
    }
    Ok(())
}

// TODO document
static REG2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)(?<first>```rust.*?\n)(?<code>.+?)(?<last>```)").unwrap());

// TODO
/// Remove Rust code blocks from Markdown files,
/// replacing each by an {{#include ... }} statement.
///
/// Note: the curent code does not handle multiple examples in one
/// file well. You may need to number includes manually.
///
/// markdown_src_dir_path: path to the source directory containing the
/// Markdown files
///
/// code_dir_path: path to the folder containing the Rust code.
pub fn remove_code_from_all_markdown_files_in<P1, P2>(
    markdown_src_dir_path: P1,
    code_dir_path: P2,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    // Locate the Markdown files with the src directory
    let markdown_file_paths = crate::fs::find_markdown_files_in(markdown_src_dir_path)?;

    // Process each .md file
    for p in markdown_file_paths {
        info!("{p:?}");
        let buf = fs::read_to_string(p.as_path())?;
        if REG2.is_match(&buf) {
            let replacement = format!(
                "$first{{#include {}.rs}}\n$last",
                PathBuf::from(code_dir_path.as_ref())
                    .join(p.file_stem().ok_or(anyhow!(
                        "[remove_code_from_all_markdown_files_in] There is no file name."
                    ))?)
                    .display()
            );
            let new_txt = REG2.replace_all(&buf, replacement);
            // debug!("{}", new_txt);
            File::create(p)?.write_all(new_txt.as_bytes())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // TODO
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
