//! Merge existing reference definitions and new ones
//! and write the result to a file

use crate::link::Link;

/// Append, sort and dedupe reference definitions.
pub(crate) fn merge_links<'a>(
    existing_links: Vec<Link<'a>>,
    new_links: &mut Vec<Link<'a>>,
) -> Vec<Link<'a>> {
    let mut buf = existing_links.clone();
    buf.append(new_links);

    // `Link` has a custom Ord / Eq implementation,
    // thus we can sort them.
    buf.sort();
    buf.dedup();
    buf
}

#[cfg(test)]
mod test {
    use crate::link::LinkBuilder;
    use pulldown_cmark::LinkType;

    use super::*;

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
        assert_eq!(merged[0].to_reference_definition(), "[label1]: url1 \"title1\"");
        assert_eq!(merged[1].to_reference_definition(), "[label2]: url2 \"title2\"");
        assert_eq!(merged[2].to_reference_definition(), "[label3]: url3 \"title3\"");
    }

    #[test]
    fn test_merge_links_empty() {
        let existing = vec![];
        let mut new = vec![];
        let merged = merge_links(existing, &mut new);
        assert!(merged.is_empty());
    }
}
