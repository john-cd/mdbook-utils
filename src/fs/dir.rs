//! Directory handling
//!
//! TODO Reviewed ??-??-2026.

use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Error;
use anyhow::bail;
use tracing::info;

/// Returns an owned copy of `dir_path` if it identifies an existing directory.
///
/// # Errors
///
/// Returns an error if the path cannot be verified as a directory.
pub(crate) fn check_is_dir<P>(dir_path: P) -> Result<PathBuf, Error>
where
    P: AsRef<Path>,
{
    let dir_path = dir_path.as_ref();
    if !dir_path.is_dir() {
        bail!(
            "{} should be a folder and exist on disk!",
            dir_path.display()
        );
    }
    Ok(dir_path.to_path_buf())
}

/// Creates missing ancestors of `file_path`. A bare filename needs no directory.
///
/// # Errors
///
/// Returns an error if the parent path cannot be checked or created.
pub(crate) fn create_parent_dir_for<P>(file_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    match AsRef::<Path>::as_ref(&file_path).parent() {
        Some(dir) if dir != Path::new("") => {
            create_dir(dir)?;
        }
        _ => {}
    }
    Ok(())
}

/// Creates `dir_path` and any missing parents when the path is absent.
///
/// # Errors
///
/// Returns an error if the existence check or directory creation fails.
pub(crate) fn create_dir<P>(dir_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    match dir_path.as_ref().try_exists() {
        Ok(false) => {
            std::fs::create_dir_all(dir_path.as_ref())
                .with_context(|| format!("[create_dir] Failed to create {} or one of its parents. Do you have appropriate permissions?", dir_path.as_ref().display()))?;
            info!("{} created", dir_path.as_ref().display());
        }
        Ok(true) => {
            // debug: tracing::debug!("{} already exists", dest_dir);
        }
        Err(_) => {
            bail!(
                "{}'s existence can neither be confirmed nor denied.",
                dir_path.as_ref().display()
            );
        }
    }
    Ok(())
}

/// Tests for directory validation and creation.
#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    /// Accepts directories and rejects files.
    #[test]
    fn test_check_is_dir() -> Result<(), Error> {
        let dir = tempdir()?;
        let res = check_is_dir(dir.path())?;
        assert_eq!(res, dir.path());
        assert!(res.is_dir());

        let file_path = dir.path().join("file.txt");
        fs::write(&file_path, "test")?;
        check_is_dir(&file_path).expect_err("input should be rejected");
        Ok(())
    }

    /// Creates missing parent directories.
    #[test]
    fn test_create_parent_dir_for() -> Result<(), Error> {
        let dir = tempdir()?;
        let file_path = dir.path().join("sub").join("file.txt");
        create_parent_dir_for(&file_path)?;
        assert!(dir.path().join("sub").is_dir());
        Ok(())
    }

    /// Creates a directory and tolerates an existing one.
    #[test]
    fn test_create_dir() -> Result<(), Error> {
        let dir = tempdir()?;
        let new_dir = dir.path().join("new_dir");
        create_dir(&new_dir)?;
        assert!(new_dir.is_dir());
        // Should not error if it already exists
        create_dir(&new_dir)?;
        Ok(())
    }
}
