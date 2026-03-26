//! Path validation utilities
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use anyhow::bail;

/// Returns the canonicalized path if it is within the canonicalized base_dir.
/// Otherwise returns an error.
pub(crate) fn is_path_within(base_dir: &Path, path: &Path) -> Result<PathBuf> {
    let base_dir_canon = base_dir.canonicalize()?;
    let path_canon = path.canonicalize()?;

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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_is_path_within() -> Result<()> {
        let dir = tempdir()?;
        let base_dir = dir.path().join("base");
        fs::create_dir(&base_dir)?;

        let safe_file = base_dir.join("safe.txt");
        fs::write(&safe_file, "safe")?;

        let unsafe_file = dir.path().join("unsafe.txt");
        fs::write(&unsafe_file, "unsafe")?;

        // Positive case
        assert!(is_path_within(&base_dir, &safe_file).is_ok());

        // Negative case
        assert!(is_path_within(&base_dir, &unsafe_file).is_err());

        Ok(())
    }
}
