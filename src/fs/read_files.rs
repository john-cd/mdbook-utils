// TODO Reviewed ??-??-2026
//! Read one or multiple files into a string or vector of strings

use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

// // Read a single file to String
// pub(crate) fn read_to_string<P: AsRef<Path>>(path: P) -> String {
// let mut file = File::open(path).expect(format!("{path:?} should
// exist.").as_str());
// let mut buf = String::new();
// // or String::with_capacity(50000);
// file.read_to_string(&mut buf).expect("file should be readable");
// buf
// }

/// Read all Markdown files in a directory into one big string.
pub(crate) fn read_to_string_all_markdown_files_in<'a, P>(
    markdown_root_dir_path: P,
) -> Result<Cow<'a, str>>
where
    P: AsRef<Path>,
{
    // Locate the Markdown files with the src directory
    let paths = super::find_markdown_files::find_markdown_files_in(markdown_root_dir_path)?;

    // Read all .md files into one big String
    let mut all_markdown = String::with_capacity(1024 * 1024);
    for p in paths {
        let mut file = File::open(p.as_path()).with_context(|| {
            format!(
                "[read_to_string_all_markdown_files_in] Could not open {}. Does the file exist?",
                p.display()
            )
        })?;
        file.read_to_string(&mut all_markdown).with_context(|| {
            format!(
                "[read_to_string_all_markdown_files_in] Could not read {}. Does the file exist?",
                p.display()
            )
        })?;
        // Keep the last line of a file from merging with the next file.
        if !all_markdown.is_empty() && !all_markdown.ends_with('\n') {
            all_markdown.push('\n');
        }
    }

    Ok(Cow::from(all_markdown))
}

/// Read a file line by line into a vector of strings.
///
/// Returns an error if the file does not exist or cannot be read.
/// # Example
/// ```ignore
/// let lines = read_lines("test.txt").unwrap();
/// for line in lines {
///     println!("{line}"");
/// }
#[allow(dead_code)]
#[tracing::instrument(skip(file_path))]
pub(crate) fn read_lines<P>(file_path: P) -> Result<Vec<Cow<'static, str>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path.as_ref()).with_context(|| {
        format!(
            "[read_lines] Could not open {}. Does the file exist?",
            file_path.as_ref().display()
        )
    })?;
    // Returns an Iterator to the Reader of the lines of the file.
    Ok(io::BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(Cow::from)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A file without a trailing newline does not merge with the next file.
    #[test]
    fn separates_files_that_lack_a_trailing_newline() -> Result<()> {
        let dir = tempfile::tempdir()?;
        std::fs::write(dir.path().join("a.md"), "see [a]")?;
        std::fs::write(dir.path().join("b.md"), "[a]: http://x.com\n")?;
        let all = read_to_string_all_markdown_files_in(dir.path())?;
        assert_eq!(all, "see [a]\n[a]: http://x.com\n");
        Ok(())
    }
}
