//! Write a file without exposing partial contents
//!
//! TODO Reviewed ??-??-2026.

use std::io::Write as _;
use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;
use tempfile::NamedTempFile;

/// Replace or create `path` with `contents`.
///
/// The contents are staged in a temporary file next to `path` and moved into
/// place, so a failure leaves any existing file untouched. Missing parent
/// directories are created. Existing regular files keep their permissions;
/// new files keep the temporary file's restrictive default permissions. A
/// symlink path is replaced itself, without following or changing its target.
/// On Windows, read-only destinations are rejected and left unchanged.
///
/// # Errors
///
/// Returns an error if the parent directory or the staged file cannot be
/// created or written, or if the file cannot be moved into place.
pub(crate) fn write_atomically(path: &Path, contents: &[u8]) -> Result<()> {
    super::create_parent_dir_for(path)?;
    let existing_permissions = match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() => Some(metadata.permissions()),
        Ok(_) => None,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => {
            return Err(error).with_context(|| format!("Failed to inspect {}", path.display()));
        }
    };
    #[cfg(windows)]
    if existing_permissions
        .as_ref()
        .is_some_and(std::fs::Permissions::readonly)
    {
        anyhow::bail!("Destination is read-only: {}", path.display());
    }

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
    if let Some(permissions) = existing_permissions {
        staged
            .as_file()
            .set_permissions(permissions)
            .with_context(|| format!("Failed to preserve permissions for {}", path.display()))?;
    }
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

    /// A newly created Unix file keeps the temporary file's private default mode.
    #[cfg(unix)]
    #[test]
    fn new_unix_file_keeps_restrictive_mode() -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir()?;
        let path = dir.path().join("new.md");
        write_atomically(&path, b"new")?;
        assert_eq!(fs::metadata(path)?.permissions().mode() & 0o777, 0o600);
        Ok(())
    }

    /// Replacing a Unix symlink replaces the link itself and leaves its target intact.
    #[cfg(unix)]
    #[test]
    fn replaces_symlink_without_following_target() -> Result<()> {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir()?;
        let target = dir.path().join("target.md");
        let link = dir.path().join("linked.md");
        fs::write(&target, "target")?;
        symlink(&target, &link)?;

        write_atomically(&link, b"replacement")?;

        assert!(!fs::symlink_metadata(&link)?.file_type().is_symlink());
        assert_eq!(fs::read_to_string(link)?, "replacement");
        assert_eq!(fs::read_to_string(target)?, "target");
        Ok(())
    }

    /// A Windows read-only destination is rejected without changing its content or attribute.
    #[cfg(windows)]
    // Windows clears only FILE_ATTRIBUTE_READONLY here; it does not broaden Unix mode bits.
    #[expect(
        clippy::permissions_set_readonly_false,
        reason = "Windows clears only FILE_ATTRIBUTE_READONLY, not Unix mode bits"
    )]
    #[test]
    fn read_only_windows_destination_is_left_unchanged() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("readonly.md");
        fs::write(&path, "original")?;
        let mut permissions = fs::metadata(&path)?.permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&path, permissions)?;

        assert!(write_atomically(&path, b"replacement").is_err());
        assert_eq!(fs::read_to_string(&path)?, "original");
        assert!(fs::metadata(&path)?.permissions().readonly());

        let mut cleanup_permissions = fs::metadata(&path)?.permissions();
        cleanup_permissions.set_readonly(false);
        fs::set_permissions(&path, cleanup_permissions)?;
        Ok(())
    }
}
