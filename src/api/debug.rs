// TODO Reviewed ??-??-2026
use std::path::Path;

use anyhow::Result;

use crate::helper;
use crate::write_from_parser;

// DEBUG

/// Parse Markdown from all .md files in a given source directory and
/// write all raw events to a file for debugging purposes.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
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
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_raw_to,
    )?;
    Ok(())
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
