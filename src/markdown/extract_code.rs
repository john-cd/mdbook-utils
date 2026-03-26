//! Extract Rust code examples from Markdown
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::Result;
use once_cell::sync::Lazy;
use rand::distr::Alphanumeric;
use rand::distr::SampleString;
use regex::Regex;
use tracing::info;

/// Embedded Rust code extraction from Markdown
static EXTRACT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)```rust.*?\n(?<code>.*?)```").unwrap());

/// Regex to remove "# " from the beginning of lines in Rust code blocks.
/// These lines are hidden in the rendered mdBook but should be included in the
/// extracted .rs files.
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

/// Regex to identify Rust code blocks for replacement by includes.
static REG2: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)(?<first>```rust.*?\n)(?<code>.*?\n)(?<last>```)").unwrap());

/// Remove Rust code blocks from Markdown files,
/// replacing each by an {{#include ... }} statement.
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

        let mut new_txt = String::with_capacity(buf.len());
        let mut last_match = 0;
        let mut counter = 0;

        for caps in REG2.captures_iter(&buf) {
            let m = caps.get(0).unwrap();
            new_txt.push_str(&buf[last_match..m.start()]);

            let first = caps.name("first").unwrap().as_str();
            let last = caps.name("last").unwrap().as_str();
            let file_stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("example");

            let filename = if counter == 0 {
                format!("{file_stem}.rs")
            } else {
                format!("{file_stem}{counter}.rs")
            };

            let include_path = PathBuf::from(code_dir_path.as_ref()).join(filename);
            new_txt.push_str(&format!(
                "{first}{{{{#include {}}}}}\n{last}",
                include_path.display()
            ));

            last_match = m.end();
            counter += 1;
        }

        if counter > 0 {
            new_txt.push_str(&buf[last_match..]);
            File::create(p)?.write_all(new_txt.as_bytes())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_extract_code_from_all_markdown_files_in() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        let code_dir = dir.path().join("code");
        fs::create_dir(&src_dir)?;

        let md_file = src_dir.join("test.md");
        fs::write(
            &md_file,
            r#"# Test
```rust
fn main() {}
```
"#,
        )?;

        extract_code_from_all_markdown_files_in(&src_dir, &code_dir)?;

        let extracted_file = code_dir.join("test.rs");
        assert!(extracted_file.exists());
        let content = fs::read_to_string(extracted_file)?;
        assert_eq!(content, "fn main() {}\n");
        Ok(())
    }

    #[test]
    fn test_remove_code_from_all_markdown_files_in() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        let code_dir = dir.path().join("code");
        fs::create_dir(&src_dir)?;

        let md_file = src_dir.join("test.md");
        fs::write(
            &md_file,
            r#"# Test
```rust
fn main() {}
```
"#,
        )?;

        remove_code_from_all_markdown_files_in(&src_dir, &code_dir)?;

        let content = fs::read_to_string(md_file)?;
        assert!(content.contains("{{#include"));
        assert!(content.contains("test.rs"));
        Ok(())
    }
}
#[cfg(test)]
mod additional_tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_extract_and_remove_multiple() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        let code_dir = dir.path().join("code");
        fs::create_dir(&src_dir)?;

        let md_file = src_dir.join("test.md");
        fs::write(
            &md_file,
            r#"# Test
```rust
fn main() {}
```
```rust
fn foo() {}
```
"#,
        )?;

        extract_code_from_all_markdown_files_in(&src_dir, &code_dir)?;

        let extracted_file1 = code_dir.join("test.rs");
        assert!(extracted_file1.exists());
        let content1 = fs::read_to_string(extracted_file1)?;
        assert_eq!(content1, "fn main() {}\n");

        let extracted_file2 = code_dir.join("test1.rs");
        assert!(extracted_file2.exists());
        let content2 = fs::read_to_string(extracted_file2)?;
        assert_eq!(content2, "fn foo() {}\n");

        remove_code_from_all_markdown_files_in(&src_dir, &code_dir)?;

        let content = fs::read_to_string(md_file)?;
        assert!(content.contains("{{#include"));
        assert!(content.contains("test.rs"));
        assert!(content.contains("test1.rs"));
        Ok(())
    }
}
