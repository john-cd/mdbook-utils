//! Atomic output writes owned by the command-line application.
//!
//! TODO Reviewed ??-??-2026.

use std::io::Write as _;
use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;
use tempfile::NamedTempFile;

/// Replace or create a report file after its contents have been rendered.
///
/// Staging the output next to the destination keeps an existing report intact
/// if rendering or writing fails. Missing parent directories are created.
///
/// # Errors
///
/// Returns an error if the parent directory or staged file cannot be created,
/// the contents cannot be written, or the staged file cannot replace the
/// destination.
pub(crate) fn write_text_atomically(path: &Path, contents: &str) -> Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)
        .with_context(|| format!("Failed to create the output directory {}", parent.display()))?;

    let mut staged = NamedTempFile::new_in(parent)
        .with_context(|| format!("Failed to stage output for {}", path.display()))?;
    staged
        .write_all(contents.as_bytes())
        .with_context(|| format!("Failed to write staged output for {}", path.display()))?;
    staged
        .flush()
        .with_context(|| format!("Failed to flush staged output for {}", path.display()))?;
    staged
        .persist(path)
        .map(|_| ())
        .map_err(|error| error.error)
        .with_context(|| format!("Failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;

    use super::*;

    /// Replaces existing report contents without leaving a staging file.
    #[test]
    fn replaces_existing_report_atomically() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("reports/out.md");
        fs::create_dir_all(path.parent().expect("path has a parent"))?;
        fs::write(&path, "old")?;

        write_text_atomically(&path, "new")?;

        assert_eq!(fs::read_to_string(&path)?, "new");
        assert_eq!(
            fs::read_dir(path.parent().expect("path has a parent"))?.count(),
            1
        );
        Ok(())
    }

    /// A destination that is a directory remains unchanged after a failed replacement.
    #[test]
    fn failed_report_write_preserves_destination_directory() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("out");
        fs::create_dir(&path)?;

        assert!(write_text_atomically(&path, "new").is_err());
        assert!(path.is_dir());
        assert_eq!(fs::read_dir(dir.path())?.count(), 1);
        Ok(())
    }
}
