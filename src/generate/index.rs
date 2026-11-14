//! Generation of crates.io category and crate index pages.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeSet;
use std::io::Write;

use crate::link::Link;

/// Kind of crates.io index to generate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IndexKind {
    /// Index of crates.io categories.
    Categories,
    /// Index of crates.io crates.
    Crates,
}

impl IndexKind {
    ///
    /// # Returns
    /// The &'static str computed for the calling workflow.
    pub(crate) fn title(self) -> &'static str {
        match self {
            Self::Categories => "Categories",
            Self::Crates => "Crates",
        }
    }

    ///
    /// # Arguments
    /// * `name` - Human-readable value used to identify or display the selected index kind.
    ///
    /// # Returns
    /// The formatted text produced for the caller.
    pub(crate) fn list_url(self, name: &str) -> String {
        match self {
            Self::Categories => format!("https://crates.io/categories/{name}"),
            Self::Crates => format!("https://crates.io/crates/{name}"),
        }
    }

    /// First path segment of a crates.io URL that introduces this kind of name.
    ///
    /// # Returns
    /// The &'static str computed for the calling workflow.
    fn path_prefix(self) -> &'static str {
        match self {
            Self::Categories => "categories",
            Self::Crates => "crates",
        }
    }
}

/// Return whether `name` is a plausible crate or category name
/// (categories use `::` between a category and its subcategory).
///
/// # Arguments
/// * `name` - Human-readable value used to identify or display the selected index kind.
/// * `kind` - Index kind carrying the kind information consumed by the index kind workflow.
///
/// # Returns
/// Whether the condition described above holds.
fn is_valid_name(name: &str, kind: IndexKind) -> bool {
    !name.is_empty()
        && name.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || c == '-'
                || c == '_'
                || (kind == IndexKind::Categories && c == ':')
        })
}

/// Return the crate or category name of a crates.io URL, if it has one.
/// Only the segment right after `/crates/` or `/categories/` counts, so
/// `https://crates.io/crates/serde/versions` yields `serde`.
///
/// # Arguments
/// * `url` - Str carrying the url information consumed by the index kind workflow.
/// * `kind` - Index kind carrying the kind information consumed by the index kind workflow.
///
/// # Returns
/// The matching value when available; `None` when no value satisfies the lookup or selection rules.
fn name_from_url(url: &str, kind: IndexKind) -> Option<String> {
    let url = url::Url::parse(url).ok()?;
    if !matches!(url.host_str(), Some("crates.io" | "www.crates.io")) {
        return None;
    }
    let mut segments = url.path_segments()?;
    if segments.next()? != kind.path_prefix() {
        return None;
    }
    let name = segments.next()?;
    is_valid_name(name, kind).then(|| name.to_owned())
}

/// Collects the sorted, deduplicated names of the given kind from crates.io
/// link URLs, ignoring query strings, trailing slashes, deeper pages, listing
/// pages, other hosts and invalid names.
pub(crate) fn extract_index_names<'a>(
    links: impl IntoIterator<Item = Link<'a>>,
    kind: IndexKind,
) -> BTreeSet<String> {
    links
        .into_iter()
        .filter_map(|link| name_from_url(&link.get_url(), kind))
        .collect()
}
/// crates.io.
/// # Errors
/// Returns an error if writing to `writer` fails.
///
/// # Arguments
/// * `writer` - Destination that receives the serialized or formatted result.
/// * `kind` - Index kind carrying the kind information consumed by the index kind workflow.
/// * `names` - Impl into iterator carrying the names information consumed by the index kind workflow.

pub(crate) fn write_index<W: Write>(
    mut writer: W,
    kind: IndexKind,
    names: impl IntoIterator<Item = String>,
) -> anyhow::Result<()> {
    writeln!(writer, "# {}\n", kind.title())?;
    for name in names {
        writeln!(writer, "- [{name}]({})", kind.list_url(&name))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;
    use crate::link::LinkBuilder;

    ///
    /// # Arguments
    /// * `url` - Str carrying the url information consumed by the index kind workflow.
    ///
    /// # Returns
    /// The link computed for the calling workflow.
    fn link(url: &str) -> Link<'_> {
        LinkBuilder::default()
            .set_label(Cow::Borrowed("x"))
            .set_url(Cow::Owned(url.to_owned()))
            .build()
    }
    /// Keeps one entry per valid crate name and drops malformed URLs.

    #[test]
    fn test_extract_index_names_skips_invalid_and_duplicates() {
        let links = vec![
            link("https://crates.io/crates/serde"),
            link("https://crates.io/crates/serde?sort=recent"),
            link("https://crates.io/crates/"),
            link("https://example.com/serde"),
            link("https://crates.io/crates/bad]()"),
            link("https://crates.io/crates/anyhow/"),
        ];

        let names = extract_index_names(links, IndexKind::Crates);
        let expected = BTreeSet::from(["anyhow".to_owned(), "serde".to_owned()]);
        assert_eq!(names, expected);
    }

    /// Only the segment after `/crates/` is a crate name; deeper pages and
    /// other hosts that merely contain the string are ignored.
    ///
    /// # Returns
    /// The successful result value produced by this operation.
    ///
    /// # Errors
    /// Returns an error if a fallible operation cannot complete.
    #[test]
    fn test_crate_name_is_the_segment_after_crates() {
        let links = vec![
            link("https://crates.io/crates/serde/versions"),
            link("https://crates.io/crates/tokio/1.0.0"),
            link("https://notcrates.io/crates/fake"),
        ];
        let names = extract_index_names(links, IndexKind::Crates);
        assert_eq!(
            names,
            BTreeSet::from(["serde".to_owned(), "tokio".to_owned()])
        );
    }

    /// Subcategories such as `web-programming::websocket` are kept.
    ///
    /// # Returns
    /// The successful result value produced by this operation.
    ///
    /// # Errors
    /// Returns an error if a fallible operation cannot complete.
    #[test]
    fn test_subcategories_are_kept() {
        let links = vec![link(
            "https://crates.io/categories/web-programming::websocket",
        )];
        let names = extract_index_names(links, IndexKind::Categories);
        assert_eq!(
            names,
            BTreeSet::from(["web-programming::websocket".to_owned()])
        );
    }

    /// Checks the write index writes header and links behavior and its externally visible result in the index workflow. It checks the expected equality outcomes.
    ///
    /// # Returns
    /// The successful result value produced by this operation.
    ///
    /// # Errors
    /// An error is returned if write_index, from_utf8 fails.
    #[test]
    fn test_write_index_writes_header_and_links() -> anyhow::Result<()> {
        let mut out = Vec::new();
        write_index(
            &mut out,
            IndexKind::Categories,
            vec!["parsing".to_owned(), "development-tools".to_owned()],
        )?;

        let content = String::from_utf8(out)?;
        let expected = "# Categories\n\n- [parsing](https://crates.io/categories/parsing)\n- [development-tools](https://crates.io/categories/development-tools)\n";
        assert_eq!(content, expected);
        Ok(())
    }
}
