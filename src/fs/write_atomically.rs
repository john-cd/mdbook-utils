//! Write a file without exposing partial contents
//!
//! TODO Reviewed ??-??-2026

use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use tempfile::NamedTempFile;

/// Replace or create `path` with `contents`.
///
/// The contents are staged in a temporary file next to `path` and moved into
/// place, so a failure leaves any existing file untouched. Missing parent
/// directories are created.
///
/// # Errors
///
/// Returns an error if the parent directory or the staged file cannot be
/// created or written, or if the file cannot be moved into place.
pub(crate) fn write_atomically(path: &Path, contents: &[u8]) -> Result<()> {
    super::create_parent_dir_for(path)?;
    let parent = match path.parent() {
        Some(parent) if parent != Path::new("") => parent,
        _ => Path::new("."),
    };
    let mut staged = NamedTempFile::new_in(parent)
        .with_context(|| format!("Failed to stage output for {}", path.display()))?;
    staged
        .write_all(contents)
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

    use super::*;

    /// A new file is created together with its missing parent directories.
    #[test]
    fn creates_file_and_parent_directories() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("a/b/out.md");
        write_atomically(&path, b"hello")?;
        assert_eq!(fs::read_to_string(path)?, "hello");
        Ok(())
    }

    /// An existing file is replaced, and no staged files are left behind.
    #[test]
    fn replaces_existing_file_without_leftovers() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("out.md");
        fs::write(&path, "old")?;
        write_atomically(&path, b"new")?;
        assert_eq!(fs::read_to_string(&path)?, "new");
        assert_eq!(fs::read_dir(dir.path())?.count(), 1);
        Ok(())
    }

    /// A failed write (destination is a directory) keeps the original.
    #[test]
    fn failure_leaves_the_destination_unchanged() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("out");
        fs::create_dir(&path)?;
        assert!(write_atomically(&path, b"new").is_err());
        assert!(path.is_dir());
        Ok(())
    }
}
