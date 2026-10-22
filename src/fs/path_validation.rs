//! Path validation utilities
//!
//! TODO Reviewed ??-??-2026

use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use anyhow::bail;

/// Returns the canonicalized path if it is within the canonicalized `base_dir`.
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
                bail!("Path traversal detected: {path_canon:?} is outside of {base_dir_canon:?}");
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // `..` in a path that does not exist cannot be resolved by the
            // filesystem, so it could climb out of the base directory after
            // the check below.
            if path
                .components()
                .any(|c| c == std::path::Component::ParentDir)
            {
                bail!("Path traversal detected: {path:?} does not exist and contains `..`");
            }

            // If the path doesn't exist, it might be a dangling symlink or a
            // new file. Check if it's a dangling symlink.
            if path.symlink_metadata().is_ok() {
                bail!("Path traversal detected: {path:?} is a dangling symlink");
            }

            // Find the first existing ancestor
            let mut ancestor = path;
            while let Some(parent) = ancestor.parent() {
                ancestor = parent;
                // If parent is empty string (relative path in current dir), use
                // "."
                let effective_ancestor = if ancestor.as_os_str().is_empty() {
                    Path::new(".")
                } else {
                    ancestor
                };

                if let Ok(ancestor_canon) = effective_ancestor.canonicalize() {
                    if ancestor_canon.starts_with(&base_dir_canon) {
                        // The ancestor is safe.
                        // We return the path as is if it's within
                        // base_dir_canon.
                        // Note: starts_with on non-canonicalized paths can be
                        // tricked, but here we are just
                        // validating safety.
                        return Ok(path.to_path_buf());
                    }
                    bail!(
                        "Path traversal detected: ancestor {ancestor_canon:?} is outside of {base_dir_canon:?}"
                    );
                }
            }
            bail!(
                "Path traversal detected: {path:?} has no existing ancestor within base directory"
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

    /// Accepts paths under the base directory and rejects escapes.
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
        is_path_within(&base_dir, &safe_file).expect("input should parse");

        // Negative case: existing file outside
        is_path_within(&base_dir, &unsafe_file).expect_err("input should be rejected");

        // Positive case: non-existent file in safe dir
        let non_existent_safe = base_dir.join("new.txt");
        is_path_within(&base_dir, &non_existent_safe).expect("input should parse");

        // Negative case: non-existent file in unsafe dir
        let non_existent_unsafe = dir.path().join("new_unsafe.txt");
        is_path_within(&base_dir, &non_existent_unsafe).expect_err("input should be rejected");

        // Negative case: dangling symlink (even if it points to a safe
        // location, we reject it)
        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let dangling = base_dir.join("dangling.lnk");
            symlink(base_dir.join("non_existent"), &dangling)?;
            is_path_within(&base_dir, &dangling).expect_err("input should be rejected");
        }

        Ok(())
    }

    /// A not-yet-existing path may not use `..` to leave the base directory.
    #[test]
    fn rejects_parent_components_in_missing_paths() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let base = dir.path().join("base");
        std::fs::create_dir(&base)?;
        is_path_within(&base, &base.join("new/../../escape.txt"))
            .expect_err("input should be rejected");
        is_path_within(&base, &base.join("new/ok.txt")).expect("input should parse");
        Ok(())
    }
}
