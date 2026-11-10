//! Get the book's examples' dependencies
//!
//! TODO Reviewed ??-??-2026.

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

/// Stores a resolved direct dependency to a crate.
#[derive(Debug)]
pub(crate) struct Dependency {
    /// Package name, which differs from the library name for crates such as `md-5` (`md5`).
    package_name: Cow<'static, str>,
    /// Name of the dependency crate.
    pub(crate) library_name: Cow<'static, str>,
    /// URL of the crate's package repository, when Cargo metadata reports one.
    pub(crate) package_repo_url: Option<Cow<'static, str>>,
}

impl Dependency {
    /// Returns the crates.io package name, which may differ from the library name.
    pub(crate) fn package_name(&self) -> &str {
        self.package_name.as_ref()
    }
}

/// Cargo metadata package information needed to identify selected workspace members and libraries.
#[derive(Debug, Deserialize)]
struct CargoMetadata {
    /// Canonical path to the workspace root.
    workspace_root: std::path::PathBuf,
    /// Package identifiers included by Cargo when invoked at the workspace root.
    workspace_default_members: Vec<String>,
    /// Resolved packages available to the workspace.
    packages: Vec<CargoPackage>,
    /// Resolved package dependency graph.
    resolve: Option<CargoResolve>,
}

/// Cargo package fields used to report dependency references.
#[derive(Debug, Deserialize)]
struct CargoPackage {
    /// Stable Cargo package identifier, including source and version.
    id: String,
    /// Registry package name.
    name: String,
    /// Package source repository URL.
    repository: Option<String>,
    /// Absolute Cargo manifest path.
    manifest_path: std::path::PathBuf,
    /// Package targets, including its library target when present.
    targets: Vec<CargoTarget>,
}

/// Cargo target fields needed to identify importable libraries.
#[derive(Debug, Deserialize)]
struct CargoTarget {
    /// Target name used as the Rust library name.
    name: String,
    /// Cargo target kinds, such as `lib` or `proc-macro`.
    kind: Vec<String>,
}

/// Resolved dependency graph from Cargo metadata.
#[derive(Debug, Deserialize)]
struct CargoResolve {
    /// Resolved workspace package nodes.
    nodes: Vec<CargoNode>,
}

/// A package node and its direct resolved dependencies.
#[derive(Debug, Deserialize)]
struct CargoNode {
    /// Package identifier for this node.
    id: String,
    /// Direct dependency edges.
    deps: Vec<CargoNodeDependency>,
}

/// A dependency edge from one resolved package to another.
#[derive(Debug, Deserialize)]
struct CargoNodeDependency {
    /// Package identifier of the dependency target.
    pkg: String,
    /// Kinds under which this edge is used.
    dep_kinds: Vec<CargoDependencyKind>,
}

/// A dependency edge's Cargo dependency kind.
#[derive(Debug, Deserialize)]
struct CargoDependencyKind {
    /// `None` denotes a normal dependency; other values include `dev` and `build`.
    kind: Option<String>,
}

/// Extracts selected Cargo packages' resolved direct normal dependencies.
///
/// Cargo metadata supplies package identities, library targets, and repository URLs as JSON.
/// The selected package's resolved direct normal dependencies are returned; dev and build
/// dependencies are excluded. At a workspace root, Cargo's default members are selected.
/// If multiple resolved packages expose the same library name, the package with the
/// lexicographically smallest Cargo package ID wins so results do not depend on output order.
/// The `--locked` option prevents Cargo from modifying the lockfile.
///
/// `dir_path`: Path to the directory containing the Cargo.toml file.
#[expect(
    clippy::needless_pass_by_value,
    reason = "public-facing signature takes Option<impl AsRef<Path>> by value"
)]
pub(crate) fn get_dependencies<P: AsRef<Path>, Q: AsRef<Path>>(
    cargo_toml_dir_path: P,
    log_file_path: Option<Q>,
) -> Result<BTreeMap<Cow<'static, str>, Dependency>> {
    let output = Command::new("cargo")
        .args(["metadata", "--format-version", "1", "--locked"])
        .current_dir(cargo_toml_dir_path.as_ref())
        .output()?;

    write_log(
        &output.stdout,
        &output.stderr,
        log_file_path.as_ref().map(std::convert::AsRef::as_ref),
    )?;

    if !output.status.success() {
        return Err(anyhow!(
            "`cargo metadata` failed. Status: {}. Output: {}\n {}",
            output.status,
            String::from_utf8(output.stdout.clone())?,
            String::from_utf8(output.stderr)?
        ));
    }

    parse_cargo_metadata(output.stdout, cargo_toml_dir_path.as_ref())
}

/// Parse Cargo metadata and collect direct normal dependencies for the selected package roots.
fn parse_cargo_metadata(
    stdout: Vec<u8>,
    cargo_toml_dir_path: &Path,
) -> Result<BTreeMap<Cow<'static, str>, Dependency>> {
    let metadata: CargoMetadata =
        serde_json::from_slice(&stdout).context("Failed to parse cargo metadata output")?;
    let requested_dir = std::fs::canonicalize(cargo_toml_dir_path).with_context(|| {
        format!(
            "Failed to resolve Cargo directory {}",
            cargo_toml_dir_path.display()
        )
    })?;
    let metadata_workspace_root = canonical_or_original(&metadata.workspace_root);
    let selected_roots = if requested_dir == metadata_workspace_root {
        metadata.workspace_default_members.clone()
    } else {
        let selected_package = metadata
            .packages
            .iter()
            .find(|package| {
                package
                    .manifest_path
                    .parent()
                    .is_some_and(|path| canonical_or_original(path) == requested_dir)
            })
            .with_context(|| {
                format!(
                    "No Cargo package found in selected directory {}",
                    requested_dir.display()
                )
            })?;
        vec![selected_package.id.clone()]
    };

    let packages: BTreeMap<_, _> = metadata
        .packages
        .iter()
        .map(|package| (package.id.as_str(), package))
        .collect();
    let resolve = metadata
        .resolve
        .context("Cargo metadata did not include a resolved dependency graph")?;
    let nodes: BTreeMap<_, _> = resolve
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let mut results: BTreeMap<Cow<'static, str>, (String, Dependency)> = BTreeMap::new();

    for root_id in selected_roots {
        let node = nodes.get(root_id.as_str()).with_context(|| {
            format!("Cargo metadata did not include selected package {root_id}")
        })?;
        for edge in &node.deps {
            if !edge.dep_kinds.iter().any(|kind| {
                kind.kind
                    .as_deref()
                    .is_none_or(|dependency_kind| dependency_kind == "normal")
            }) {
                continue;
            }
            let package = packages.get(edge.pkg.as_str()).with_context(|| {
                format!("Cargo metadata omitted dependency package {}", edge.pkg)
            })?;
            let Some(library) = package.targets.iter().find(|target| {
                target
                    .kind
                    .iter()
                    .any(|kind| kind == "lib" || kind == "proc-macro")
            }) else {
                continue;
            };
            let library_name: Cow<'static, str> = Cow::Owned(library.name.clone());
            let dependency = Dependency {
                package_name: Cow::Owned(package.name.clone()),
                library_name: library_name.clone(),
                package_repo_url: package.repository.clone().map(Cow::Owned),
            };
            let should_replace = results
                .get(&library_name)
                .is_none_or(|(package_id, _)| package.id.as_str() < package_id.as_str());
            if should_replace {
                results.insert(library_name, (package.id.clone(), dependency));
            }
        }
    }

    Ok(results
        .into_iter()
        .map(|(library_name, (_, dependency))| (library_name, dependency))
        .collect())
}

/// Canonicalizes paths from Cargo when possible so Windows extended paths compare consistently.
fn canonical_or_original(path: &Path) -> std::path::PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
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

/// Keep Cargo's output in a temporary file; failures are only logged.
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
    use super::*;

    /// Builds a Cargo package record for focused metadata parser tests.
    fn package(
        id: &str,
        name: &str,
        manifest_path: &str,
        library_name: Option<&str>,
        repository: Option<&str>,
    ) -> serde_json::Value {
        let targets = library_name.map_or_else(Vec::new, |library_name| {
            vec![serde_json::json!({
                "name": library_name,
                "kind": ["lib"]
            })]
        });
        serde_json::json!({
            "id": id,
            "name": name,
            "version": "1.0.0",
            "repository": repository,
            "manifest_path": manifest_path,
            "targets": targets
        })
    }

    /// Builds a Cargo dependency edge with one dependency kind.
    fn edge(pkg: &str, kind: Option<&str>) -> serde_json::Value {
        serde_json::json!({
            "pkg": pkg,
            "dep_kinds": [{ "kind": kind, "target": null }]
        })
    }

    /// Parses a JSON fixture as Cargo metadata for the selected directory.
    fn parse_fixture(
        metadata: serde_json::Value,
        selected_dir: &Path,
    ) -> Result<BTreeMap<Cow<'static, str>, Dependency>> {
        std::fs::create_dir_all(selected_dir)?;
        parse_cargo_metadata(serde_json::to_vec(&metadata)?, selected_dir)
    }

    /// Creates a canonical temporary directory for metadata fixtures.
    fn fixture_dir(name: &str) -> Result<(tempfile::TempDir, std::path::PathBuf)> {
        let temp = tempfile::tempdir()?;
        let path = temp.path().join(name);
        std::fs::create_dir_all(&path)?;
        Ok((temp, std::fs::canonicalize(path)?))
    }

    /// Returns a Cargo manifest path beneath a package fixture directory.
    fn manifest_path(directory: &Path) -> String {
        directory.join("Cargo.toml").to_string_lossy().into_owned()
    }

    /// Package names and repository URLs stay intact when names and paths contain punctuation.
    #[test]
    fn extracts_by_package_identity_without_fixture_name_assumptions() -> Result<()> {
        let (_temp, root) = fixture_dir("odd-root")?;
        let dependency_dir = root.join("path, with, commas");
        let metadata = serde_json::json!({
            "workspace_root": root.display().to_string(),
            "workspace_default_members": ["root-id"],
            "packages": [
                package("root-id", "odd-root", &manifest_path(&root), Some("odd_root"), None),
                package(
                    "dep-id",
                    "md-5",
                    &manifest_path(&dependency_dir),
                    Some("md5"),
                    Some("https://example.test/repo,with,commas")
                )
            ],
            "resolve": { "nodes": [
                { "id": "root-id", "deps": [edge("dep-id", None)] },
                { "id": "dep-id", "deps": [] }
            ] }
        });

        let dependencies = parse_fixture(metadata, &root)?;
        assert_eq!(dependencies.len(), 1);
        let md5 = &dependencies["md5"];
        assert_eq!(md5.package_name(), "md-5");
        assert_eq!(
            md5.package_repo_url.as_deref(),
            Some("https://example.test/repo,with,commas")
        );
        Ok(())
    }

    /// Workspace roots collect normal dependencies from all default members and exclude dev/build
    /// edges.
    #[test]
    fn virtual_workspace_collects_default_members_and_normal_dependencies() -> Result<()> {
        let (_temp, workspace) = fixture_dir("workspace")?;
        let member_a = workspace.join("a");
        let member_b = workspace.join("b");
        let excluded = workspace.join("excluded");
        let dependencies_dir = workspace.join("deps");
        let dev_dir = workspace.join("dev");
        let build_dir = workspace.join("build");
        let other_dir = workspace.join("other");
        let metadata = serde_json::json!({
            "workspace_root": workspace.display().to_string(),
            "workspace_default_members": ["member-a", "member-b"],
            "packages": [
                package("member-a", "first-member", &manifest_path(&member_a), Some("first_member"), None),
                package("member-b", "second-member", &manifest_path(&member_b), Some("second_member"), None),
                package("excluded", "excluded-member", &manifest_path(&excluded), Some("excluded_member"), None),
                package("deps-id", "deps-package", &manifest_path(&dependencies_dir), Some("deps"), None),
                package("dev-id", "dev-only", &manifest_path(&dev_dir), Some("dev_only"), None),
                package("build-id", "build-only", &manifest_path(&build_dir), Some("build_only"), None),
                package("other-id", "other-package", &manifest_path(&other_dir), Some("other_package"), None)
            ],
            "resolve": { "nodes": [
                { "id": "member-a", "deps": [
                    edge("deps-id", None),
                    edge("dev-id", Some("dev")),
                    edge("build-id", Some("build"))
                ] },
                { "id": "member-b", "deps": [edge("other-id", None)] },
                { "id": "excluded", "deps": [] },
                { "id": "deps-id", "deps": [] },
                { "id": "dev-id", "deps": [] },
                { "id": "build-id", "deps": [] },
                { "id": "other-id", "deps": [] }
            ] }
        });

        let dependencies = parse_fixture(metadata.clone(), &workspace)?;
        assert_eq!(dependencies.len(), 2);
        assert_eq!(dependencies["deps"].package_name(), "deps-package");
        assert_eq!(
            dependencies["other_package"].package_name(),
            "other-package"
        );
        assert!(!dependencies.contains_key("dev_only"));
        assert!(!dependencies.contains_key("build_only"));
        assert!(!dependencies.contains_key("excluded_member"));

        let member_dependencies = parse_fixture(metadata, &member_a)?;
        assert_eq!(member_dependencies.len(), 1);
        assert_eq!(member_dependencies["deps"].package_name(), "deps-package");
        Ok(())
    }

    /// Repeated library names choose the same package regardless of Cargo node order.
    #[test]
    fn duplicate_library_names_choose_smallest_package_id() -> Result<()> {
        let (_temp, workspace) = fixture_dir("workspace")?;
        let package_z = workspace.join("z-package");
        let package_a = workspace.join("a-package");
        let metadata = serde_json::json!({
            "workspace_root": workspace.display().to_string(),
            "workspace_default_members": ["root-id"],
            "packages": [
                package("root-id", "some-root", &manifest_path(&workspace), Some("some_root"), None),
                package("z-id", "zeta", &manifest_path(&package_z), Some("shared"), None),
                package("a-id", "alpha", &manifest_path(&package_a), Some("shared"), None)
            ],
            "resolve": { "nodes": [
                { "id": "root-id", "deps": [
                    edge("z-id", None),
                    edge("a-id", None)
                ] },
                { "id": "z-id", "deps": [] },
                { "id": "a-id", "deps": [] }
            ] }
        });
        let reversed = serde_json::json!({
            "workspace_root": workspace.display().to_string(),
            "workspace_default_members": ["root-id"],
            "packages": [
                package("a-id", "alpha", &manifest_path(&package_a), Some("shared"), None),
                package("z-id", "zeta", &manifest_path(&package_z), Some("shared"), None),
                package("root-id", "some-root", &manifest_path(&workspace), Some("some_root"), None)
            ],
            "resolve": { "nodes": [
                { "id": "a-id", "deps": [] },
                { "id": "z-id", "deps": [] },
                { "id": "root-id", "deps": [
                    edge("a-id", None),
                    edge("z-id", None)
                ] }
            ] }
        });

        let first = parse_fixture(metadata, &workspace)?;
        let second = parse_fixture(reversed, &workspace)?;
        assert_eq!(first["shared"].package_name(), "alpha");
        assert_eq!(second["shared"].package_name(), "alpha");
        Ok(())
    }

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
