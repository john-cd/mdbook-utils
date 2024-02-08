//! Parse `Cargo.toml`
#![allow(dead_code)]

use std::path::Path;

use anyhow::Error;
use cargo_toml::Manifest;
use tracing::debug;

/// Parse `Cargo.toml` (using the `cargo_toml` crate)
///
/// cargo_toml_dir_path: the path to the directory where Cargo.toml may be found
pub(crate) fn parse_cargo_toml<P: AsRef<Path>>(
    cargo_toml_dir_path: P,
) -> Result<Vec<String>, Error> {
    debug!("Parsing Cargo.toml...");

    // Manifest::from_path calls Manifest::complete_from_path to discover implicit
    // binaries, etc. It will search for a workspace.
    let manifest = Manifest::from_path(cargo_toml_dir_path)?;

    if let Some(package) = manifest.package {
        debug!("Name: {}\n", package.name);
        debug!(
            "Description: {}\n",
            package.description.unwrap_or_default().get()?
        );
        debug!("Authors: {:?}\n", package.authors.get()?);
        debug!(
            "Homepage: {}\n",
            package.homepage.unwrap_or_default().get()?
        );
        debug!(
            "Repository: {}\n",
            package.repository.unwrap_or_default().get()?
        );
        debug!(
            "Documentation: {}\n",
            package.documentation.unwrap_or_default().get()?
        );
        debug!("Keywords: {:?}\n", package.keywords.get()?);
        debug!("Categories: {:?}\n", package.categories.get()?);
    }
    let deps = manifest.dependencies;
    // The keys in this map are not always crate names, this can be overriden by the
    // package field, and there may be multiple copies of the same crate.

    // Return the crate names
    Ok(deps.keys().cloned().collect::<Vec<String>>())
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
