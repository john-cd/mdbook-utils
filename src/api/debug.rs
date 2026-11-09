//! Public operation that writes raw Markdown parser events for debugging.
//!
//! TODO Reviewed ??-??-2026.

use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;

use crate::fs;
use crate::parser;
use crate::write_from_parser;

// DEBUG

/// Render the raw parser events for all Markdown files in a source directory.
///
/// The caller chooses where and how to write the returned diagnostic text.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the diagnostic
/// output cannot be rendered.
pub fn render_debug_events<P: AsRef<Path>>(src_dir_path: P) -> Result<String> {
    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let mut output = Vec::new();
    write_from_parser::write_raw_to(&mut parser, &mut output)?;
    String::from_utf8(output).context("rendered parser events were not valid UTF-8")
}

/// Parse Markdown sources and atomically write raw parser events to a file.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the destination
/// file cannot be created or written.
pub fn debug_parse_to<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let output = render_debug_events(src_dir_path)?;
    fs::write_atomically(dest_file_path.as_ref(), output.as_bytes())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    /// Raw parser events are written to the destination file.
    #[test]
    fn writes_raw_events() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let src = dir.path().join("src");
        fs::create_dir(&src)?;
        fs::write(src.join("a.md"), "# Title\n")?;
        let dest = dir.path().join("out").join("events.txt");

        debug_parse_to(&src, &dest)?;

        let text = fs::read_to_string(&dest)?;
        assert!(text.contains("Title"), "{text}");
        Ok(())
    }

    /// A missing source directory is an error and writes nothing.
    #[test]
    fn missing_source_directory_is_an_error() {
        let dir = tempfile::tempdir().expect("temp dir");
        let dest = dir.path().join("events.txt");

        assert!(debug_parse_to(dir.path().join("nope"), &dest).is_err());
        assert!(!dest.exists());
    }
}
