//! Read one or multiple files into a string or vector of strings
//!
//! TODO Reviewed ??-??-2026

use std::borrow::Cow;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

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

/// All Markdown files of a directory joined into one string, with the
/// offset at which each file starts.
pub(crate) struct MarkdownSources {
    /// The concatenated contents, each file ending with a newline.
    pub(crate) text: String,
    /// Each file and the byte offset of its first line in `text`.
    files: Vec<(PathBuf, usize)>,
}

impl MarkdownSources {
    /// Return the file and 1-based line that contain the byte `offset` of
    /// [`MarkdownSources::text`].
    pub(crate) fn locate(&self, offset: usize) -> Option<(&Path, usize)> {
        let index = self.files.partition_point(|(_, start)| *start <= offset);
        let (path, start) = self.files.get(index.checked_sub(1)?)?;
        let end = offset.min(self.text.len());
        let line = self.text[*start..end].matches('\n').count() + 1;
        Some((path.as_path(), line))
    }
}

/// Read all Markdown files in a directory, remembering where each file starts.
pub(crate) fn read_markdown_sources_in<P>(markdown_root_dir_path: P) -> Result<MarkdownSources>
where
    P: AsRef<Path>,
{
    // Locate the Markdown files with the src directory
    let paths = super::find_markdown_files::find_markdown_files_in(markdown_root_dir_path)?;

    // Read all .md files into one big String
    let mut text = String::with_capacity(1024 * 1024);
    let mut files = Vec::with_capacity(paths.len());
    for p in paths {
        files.push((p.clone(), text.len()));
        let mut file = File::open(p.as_path()).with_context(|| {
            format!(
                "[read_to_string_all_markdown_files_in] Could not open {}. Does the file exist?",
                p.display()
            )
        })?;
        file.read_to_string(&mut text).with_context(|| {
            format!(
                "[read_to_string_all_markdown_files_in] Could not read {}. Does the file exist?",
                p.display()
            )
        })?;
        // Keep the last line of a file from merging with the next file.
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
    }

    Ok(MarkdownSources { text, files })
}

/// Read all Markdown files in a directory into one big string.
pub(crate) fn read_to_string_all_markdown_files_in<'a, P>(
    markdown_root_dir_path: P,
) -> Result<Cow<'a, str>>
where
    P: AsRef<Path>,
{
    Ok(Cow::from(read_markdown_sources_in(markdown_root_dir_path)?.text))
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

    /// Offsets map back to the file and line they came from.
    #[test]
    fn locates_offsets_in_the_originating_file() -> Result<()> {
        let dir = tempfile::tempdir()?;
        std::fs::write(dir.path().join("a.md"), "one\ntwo")?;
        std::fs::write(dir.path().join("b.md"), "three\nfour\n")?;
        let sources = read_markdown_sources_in(dir.path())?;
        assert_eq!(sources.text, "one\ntwo\nthree\nfour\n");

        let (path, line) = sources.locate(5).context("offset in a.md")?;
        assert_eq!((path.file_name().and_then(|n| n.to_str()), line), (Some("a.md"), 2));
        let (path, line) = sources.locate(15).context("offset in b.md")?;
        assert_eq!((path.file_name().and_then(|n| n.to_str()), line), (Some("b.md"), 2));
        Ok(())
    }
}
