//! Get the book's examples' dependencies
//!
//! TODO Reviewed ??-??-2026

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Write as _;
use std::path::Path;
use std::process::Command;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use serde::Deserialize;
use tempfile::Builder;
use tracing::debug;
use tracing::warn;

/// Stores a dependency to a crate
#[derive(Debug, Deserialize)]
pub(crate) struct Dependency<'a> {
    /// Package description as printed by `cargo tree` (`name vX.Y.Z ...`).
    package: Cow<'a, str>,
    /// Name of the dependency crate.
    pub(crate) library_name: Cow<'a, str>,
    /// URL of the crate's package repository, when `cargo tree` reports one.
    pub(crate) package_repo_url: Option<Cow<'a, str>>,
}

impl Dependency<'_> {
    /// Name of the package on crates.io, which differs from the library name
    /// for crates such as `md-5` (library `md5`).
    pub(crate) fn package_name(&self) -> &str {
        self.package.split_whitespace().next().unwrap_or_default()
    }
}

/// Parse `Cargo.toml` and returns the list of dependencies:
///
/// Calls cargo tree --depth 1 --edges normal --prefix none --format
/// {p},{lib},{r} --locked (immediate children, no-dev/build, flat list,
/// crate name and package repository URL)
///
/// `dir_path`: Path to the directory containing the Cargo.toml file.
pub(crate) fn get_dependencies<P: AsRef<Path>, Q: AsRef<Path>>(
    cargo_toml_dir_path: P,
    log_file_path: Option<Q>,
) -> Result<BTreeMap<Cow<'static, str>, Dependency<'static>>> {
    let output = Command::new("cargo")
        .args([
            "tree",
            "--depth",
            "1",
            "--edges",
            "normal",
            "--prefix",
            "none",
            "--format",
            "{p},{lib},{r}",
            "--locked",
        ])
        .current_dir(cargo_toml_dir_path)
        .output()?; // returns if failed to execute Command

    write_log(
        &output.stdout,
        &output.stderr,
        log_file_path.as_ref().map(std::convert::AsRef::as_ref),
    )?;

    if !output.status.success() {
        return Err(anyhow!(
            "`cargo tree` failed. Status: {}. Output: {}\n {}",
            output.status,
            String::from_utf8(output.stdout.clone())?,
            String::from_utf8(output.stderr)?
        ));
    }

    parse_cargo_tree(output.stdout)
}

/// Parse the `{p},{lib},{r}` lines printed by `cargo tree`.
fn parse_cargo_tree(stdout: Vec<u8>) -> Result<BTreeMap<Cow<'static, str>, Dependency<'static>>> {
    // Useful wrapper that implements Read for Vec<u8>
    let cursor = std::io::Cursor::new(stdout);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(cursor);

    let mut results: BTreeMap<_, _> = BTreeMap::new();

    for res in rdr.deserialize::<Dependency<'_>>() {
        let dep = res.map_err(|e| anyhow!("Failed to parse cargo tree output: {e}"))?;
        // The first line is the root package itself.
        if dep.library_name != "deps" {
            results.insert(dep.library_name.clone(), dep);
        }
    }

    Ok(results)
}

/// Write e.g. stdout / stderr to a file.
///
/// An explicitly requested log file must be inside the current directory and
/// writable; failures are returned. The default temporary log is best effort.
fn write_log(out: &[u8], err: &[u8], log_file_path: Option<&Path>) -> Result<()> {
    let Some(path) = log_file_path else {
        write_default_log(out, err);
        return Ok(());
    };

    let current_dir = std::env::current_dir()
        .context("Failed to get the current directory to validate the log file path")?;
    crate::fs::is_path_within(&current_dir, path)
        .with_context(|| format!("Refusing to write the log file {}", path.display()))?;

    let mut contents = Vec::with_capacity(out.len() + err.len());
    contents.extend_from_slice(out);
    contents.extend_from_slice(err);
    crate::fs::write_atomically(path, &contents)
        .with_context(|| format!("Failed to write the log file {}", path.display()))?;
    debug!("Dependencies log written to {}", path.display());
    Ok(())
}

/// Keep the `cargo tree` output in a temporary file; failures are only logged.
fn write_default_log(out: &[u8], err: &[u8]) {
    let result = Builder::new()
        .prefix("mdbook-utils-dependencies-")
        .suffix(".log")
        .tempfile()
        .map_err(anyhow::Error::from)
        .and_then(|tf| {
            let (mut file, path) = tf.keep().map_err(anyhow::Error::from)?;
            file.write_all(out)?;
            file.write_all(err)?;
            file.flush()?;
            Ok(path)
        });
    match result {
        Ok(path) => debug!("Dependencies log written to {}", path.display()),
        Err(e) => warn!("Failed to write the temporary dependencies log: {e}"),
    }
}

#[cfg(test)]
mod tests {
    /// The package name, not the library name, is what crates.io knows.
    #[test]
    fn parses_package_and_library_names() -> Result<()> {
        let out = "deps v0.1.0 (/tmp/deps),deps,\n\
                   md-5 v0.10.6,md5,https://github.com/RustCrypto/hashes\n\
                   serde v1.0.0,serde,\n";
        let deps = parse_cargo_tree(out.as_bytes().to_vec())?;
        assert_eq!(deps.len(), 2);
        let md5 = &deps["md5"];
        assert_eq!(md5.package_name(), "md-5");
        assert_eq!(
            md5.package_repo_url.as_deref(),
            Some("https://github.com/RustCrypto/hashes")
        );
        assert_eq!(deps["serde"].package_name(), "serde");
        assert_eq!(deps["serde"].package_repo_url.as_deref().unwrap_or(""), "");
        Ok(())
    }

    use super::*;

    /// A log path outside the current directory is rejected, not ignored.
    #[test]
    fn rejects_log_path_outside_current_directory() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let outside = dir.path().join("cargo.log");
        assert!(write_log(b"out", b"err", Some(&outside)).is_err());
        assert!(!outside.exists());
        Ok(())
    }

    /// An explicit log inside the current directory receives stdout and stderr.
    #[test]
    fn writes_explicit_log_inside_current_directory() -> Result<()> {
        let dir = tempfile::tempdir_in(std::env::current_dir()?)?;
        let log = dir.path().join("cargo.log");
        write_log(b"out", b"err", Some(&log))?;
        assert_eq!(std::fs::read_to_string(log)?, "outerr");
        Ok(())
    }
}
