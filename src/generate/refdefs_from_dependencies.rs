//! Create reference definitions from dependencies.
//!
//! TODO Reviewed ??-??-2026

use std::borrow::Cow;
use std::collections::BTreeMap;

use tracing::info;

use crate::dependencies::Dependency;
use crate::link::Link;

/// Create reference definitions from dependencies
///
/// dependencies: sorted map of dependencies
pub(crate) fn generate_refdefs_from(
    dependencies: BTreeMap<Cow<'_, str>, Dependency<'_>>,
) -> Vec<Link<'static>> {
    let mut buf = Vec::new();
    info!(
        dependency_count = dependencies.len(),
        "generating dependency references"
    );
    for (_, dep) in dependencies {
        buf.push(generate_refdefs_for_one_library(
            &dep.library_name,
            dep.package_name(),
            dep.package_repo_url.as_deref(),
        ));
    }
    buf.into_iter().flatten().collect::<Vec<_>>()
}

/// Create, for a given crate, multiple reference definitions for
/// common websites such as docs.rs, crates.io, github,
/// and the associated badge URLs
fn generate_refdefs_for_one_library(
    library_name: &str,
    package_name: &str,
    package_repo_url: Option<&str>,
) -> Vec<Link<'static>> {
    let mut buf = Vec::new();

    // [arrow-rs]: https://docs.rs/arrow/
    let docs_rs_url = format!("https://docs.rs/{package_name}");
    buf.push(
        crate::link::LinkBuilder::default()
            .set_label(library_name.to_owned().into())
            .set_url(docs_rs_url.into())
            .build(),
    );

    // [config-crate]: https://crates.io/crates/config/
    let crates_io_url = format!("https://crates.io/crates/{package_name}");
    buf.push(
        crate::link::LinkBuilder::default()
            .set_label(format!("{library_name}-crate").into())
            .set_url(crates_io_url.into())
            .build(),
    );

    if let Some(url) = package_repo_url {
        // [arrow-rs-github]: https://github.com/apache/arrow-rs/
        buf.push(
            crate::link::LinkBuilder::default()
                .set_label(format!("{library_name}-github").into())
                .set_url(url.to_owned().into())
                .build(),
        );
    }

    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    /// URLs use the package name; labels use the library name.
    #[test]
    fn urls_use_the_package_name() {
        let links = generate_refdefs_for_one_library(
            "md5",
            "md-5",
            Some("https://github.com/RustCrypto/hashes"),
        );
        let got: Vec<_> = links
            .iter()
            .map(|l| (l.label.as_deref().unwrap_or(""), l.get_url().into_owned()))
            .collect();
        assert_eq!(
            got,
            [
                ("md5", "https://docs.rs/md-5".to_owned()),
                ("md5-crate", "https://crates.io/crates/md-5".to_owned()),
                (
                    "md5-github",
                    "https://github.com/RustCrypto/hashes".to_owned()
                ),
            ]
        );
    }
}
