//! Get the book's examples' dependencies
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::Builder;

use anyhow::Result;
use tracing::debug;
use tracing::warn;
use anyhow::anyhow;
use serde::Deserialize;

/// Stores a dependency to a crate
#[derive(Debug, Deserialize)]
pub(crate) struct Dependency<'a> {
    pub(crate) library_name: Cow<'a, str>,
    pub(crate) package_repo_url: Option<Cow<'a, str>>,
}

/// Parse `Cargo.toml` and returns the list of dependencies:
///
/// Calls cargo tree --depth 1 --edges normal --prefix none --format
/// {lib},{r} --locked (immediate children, no-dev/build, flat list,
/// crate name and package repository URL)
///
/// dir_path: Path to the directory containing the Cargo.toml file.
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
            "{lib},{r}",
            "--locked",
        ])
        .current_dir(cargo_toml_dir_path)
        .output()?; // returns if failed to execute Command

    write_log(
        &output.stdout,
        &output.stderr,
        log_file_path.as_ref().map(|p| p.as_ref()),
    )?;

    if !output.status.success() {
        return Err(anyhow!(
            "`cargo tree` failed. Status: {}. Output: {}\n {}",
            output.status,
            String::from_utf8(output.stdout.clone())?,
            String::from_utf8(output.stderr)?
        ));
    }

    // Useful wrapper that implements Read for Vec<u8>
    let cursor = std::io::Cursor::new(output.stdout);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(cursor);

    // Read cargo tree output
    let mut results: BTreeMap<_, _> = BTreeMap::new();

    while let Some(res) = rdr.deserialize::<Dependency<'_>>().next() {
        if let Ok(dep) = res {
            if dep.library_name != "deps" {
                results.insert(dep.library_name.clone(), dep);
            }
        } else {
            return Err(anyhow!("Failed to parse cargo tree output."));
        }
    }

    Ok(results)
}

/// Write e.g. stdout / stderr to a file.
fn write_log(out: &[u8], err: &[u8], log_file_path: Option<&Path>) -> Result<()> {
    let (file, actual_path) = match log_file_path {
        Some(path) => match File::create(path) {
            Ok(f) => (f, path.to_path_buf()),
            Err(e) => {
                warn!("Failed to create log file {}: {}", path.display(), e);
                return Ok(());
            }
        },
        None => match Builder::new()
            .prefix("mdbook-utils-dependencies-")
            .suffix(".log")
            .tempfile()
        {
            Ok(tf) => match tf.keep() {
                Ok((f, p)) => (f, p),
                Err(e) => {
                    warn!("Failed to keep temporary log file: {}", e);
                    return Ok(());
                }
            },
            Err(e) => {
                warn!("Failed to create temporary log file: {}", e);
                return Ok(());
            }
        },
    };

    let mut buffer = BufWriter::new(file);
    if let Err(e) = buffer
        .write_all(out)
        .and_then(|_| buffer.write_all(err))
        .and_then(|_| buffer.flush())
    {
        warn!(
            "Failed to write to log file {}: {}",
            actual_path.display(),
            e
        );
    } else {
        debug!("Dependencies log written to {}", actual_path.display());
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
