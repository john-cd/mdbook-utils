//! Path validation utilities
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use anyhow::bail;

/// Returns the canonicalized path if it is within the canonicalized base_dir.
/// Otherwise returns an error.
///
/// This function handles non-existent files by validating their parent
/// directory. Dangling symlinks are rejected for security.
pub(crate) fn is_path_within(base_dir: &Path, path: &Path) -> Result<PathBuf> {
    let base_dir_canon = base_dir.canonicalize()?;

    match path.canonicalize() {
        Ok(path_canon) => {
            if path_canon.starts_with(&base_dir_canon) {
                Ok(path_canon)
            } else {
                bail!(
                    "Path traversal detected: {:?} is outside of {:?}",
                    path_canon,
                    base_dir_canon
                );
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // If the path doesn't exist, it might be a dangling symlink or a new file.
            // Check if it's a dangling symlink.
            if path.symlink_metadata().is_ok() {
                bail!("Path traversal detected: {:?} is a dangling symlink", path);
            }

            // Find the first existing ancestor
            let mut ancestor = path;
            while let Some(parent) = ancestor.parent() {
                ancestor = parent;
                // If parent is empty string (relative path in current dir), use "."
                let effective_ancestor = if ancestor.as_os_str().is_empty() {
                    Path::new(".")
                } else {
                    ancestor
                };

                if let Ok(ancestor_canon) = effective_ancestor.canonicalize() {
                    if ancestor_canon.starts_with(&base_dir_canon) {
                        // The ancestor is safe.
                        // We return the path as is if it's within base_dir_canon.
                        // Note: starts_with on non-canonicalized paths can be tricked,
                        // but here we are just validating safety.
                        return Ok(path.to_path_buf());
                    } else {
                        bail!(
                            "Path traversal detected: ancestor {:?} is outside of {:?}",
                            ancestor_canon,
                            base_dir_canon
                        );
                    }
                }
            }
            bail!(
                "Path traversal detected: {:?} has no existing ancestor within base directory",
                path
            );
        }
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_is_path_within() -> Result<()> {
        let dir = tempdir()?;
        let base_dir = dir.path().join("base");
        fs::create_dir_all(&base_dir)?;

        let safe_file = base_dir.join("safe.txt");
        fs::write(&safe_file, "safe")?;

        let unsafe_file = dir.path().join("unsafe.txt");
        fs::write(&unsafe_file, "unsafe")?;

        // Positive case: existing file
        assert!(is_path_within(&base_dir, &safe_file).is_ok());

        // Negative case: existing file outside
        assert!(is_path_within(&base_dir, &unsafe_file).is_err());

        // Positive case: non-existent file in safe dir
        let non_existent_safe = base_dir.join("new.txt");
        assert!(is_path_within(&base_dir, &non_existent_safe).is_ok());

        // Negative case: non-existent file in unsafe dir
        let non_existent_unsafe = dir.path().join("new_unsafe.txt");
        assert!(is_path_within(&base_dir, &non_existent_unsafe).is_err());

        // Negative case: dangling symlink (even if it points to a safe location, we
        // reject it)
        #[cfg(unix)]
        {
            let dangling = base_dir.join("dangling.lnk");
            std::os::unix::fs::symlink(base_dir.join("non_existent"), &dangling)?;
            assert!(is_path_within(&base_dir, &dangling).is_err());
        }

        Ok(())
    }
}
