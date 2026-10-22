//! Merge existing reference definitions and new ones
//! and write the result to a file
//!
//! TODO Reviewed ??-??-2026

use crate::link::Link;

/// Append, sort and dedupe reference definitions.
pub(crate) fn merge_links<'a>(
    existing_links: Vec<Link<'a>>,
    new_links: &mut Vec<Link<'a>>,
) -> Vec<Link<'a>> {
    // A label can have only one definition (CommonMark keeps the first), so
    // generated definitions never override an existing label.
    let existing_labels: std::collections::HashSet<_> = existing_links
        .iter()
        .filter_map(|link| link.label.clone())
        .collect();
    new_links.retain(|link| {
        link.label
            .as_ref()
            .is_none_or(|l| !existing_labels.contains(l))
    });

    let mut buf = existing_links.clone();
    buf.append(new_links);

    // `Link` has a custom Ord / Eq implementation,
    // thus we can sort them.
    buf.sort();
    buf.dedup();
    buf
}

#[cfg(test)]
mod tests {
    use pulldown_cmark::LinkType;

    use super::*;
    use crate::link::LinkBuilder;

    #[test]
    fn test_merge_links() {
        let link1 = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url1".into(),
            "title1".into(),
            "label1".into(),
        )
        .build();
        let link2 = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url2".into(),
            "title2".into(),
            "label2".into(),
        )
        .build();
        let link3 = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url3".into(),
            "title3".into(),
            "label3".into(),
        )
        .build();
        let link1_dup = link1.clone();

        let existing = vec![link3.clone(), link1.clone()];
        let mut new = vec![link2.clone(), link1_dup];

        let merged = merge_links(existing, &mut new);

        assert_eq!(merged.len(), 3);
        // Sorted by label: label1, label2, label3
        assert_eq!(
            merged[0].to_reference_definition(),
            "[label1]: url1 \"title1\""
        );
        assert_eq!(
            merged[1].to_reference_definition(),
            "[label2]: url2 \"title2\""
        );
        assert_eq!(
            merged[2].to_reference_definition(),
            "[label3]: url3 \"title3\""
        );
    }

    #[test]
    fn test_merge_links_empty() {
        let existing = vec![];
        let mut new = vec![];
        let merged = merge_links(existing, &mut new);
        assert!(merged.is_empty());
    }

    /// Builds a definition with a label and URL.
    fn definition(label: &str, url: &str) -> Link<'static> {
        LinkBuilder::default()
            .set_label(label.to_string().into())
            .set_url(url.to_string().into())
            .build()
    }

    /// A generated definition does not duplicate an existing label.
    #[test]
    fn existing_label_wins_over_generated_definition() {
        let existing = vec![definition("serde", "https://docs.rs/serde/")];
        let mut generated = vec![
            definition("serde", "https://docs.rs/serde"),
            definition("serde-crate", "https://crates.io/crates/serde"),
        ];
        let merged = merge_links(existing, &mut generated);
        let labels: Vec<_> = merged.iter().filter_map(|l| l.label.as_deref()).collect();
        assert_eq!(labels, ["serde", "serde-crate"]);
        assert_eq!(merged[0].get_url(), "https://docs.rs/serde/");
    }
}
