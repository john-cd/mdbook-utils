use std::borrow::Cow;

use pulldown_cmark::LinkType;

use super::link::Link;

/// Link builder that progressively construct a [Link]
/// from pieces of information
#[derive(Debug, Default)]
pub(crate) struct LinkBuilder<'a> {
    link: Link<'a>,
}

impl<'a> LinkBuilder<'a> {
    pub(crate) fn from_type_url_title(
        link_type: LinkType,
        url: Cow<'a, str>,
        title: Cow<'a, str>,
        label: Cow<'a, str>,
    ) -> Self {
        Self {
            link: Link {
                link_type: Some(link_type),
                url: if !url.is_empty() { Some(url) } else { None },
                title: if !title.is_empty() { Some(title) } else { None },
                label: if !label.is_empty() { Some(label) } else { None },
                ..Link::default()
            },
        }
    }

    pub(crate) fn set_url(mut self, url: Cow<'a, str>) -> Self {
        if !url.is_empty() {
            self.link.url = Some(url);
        }
        self
    }

    pub(crate) fn add_text(mut self, text: Cow<'a, str>) -> Self {
        if !text.is_empty() {
            self.link.text = Some(format!("{}{text}", self.link.text.unwrap_or_default()).into());
        }
        self
    }

    pub(crate) fn set_label(mut self, label: Cow<'a, str>) -> Self {
        if !label.is_empty() {
            self.link.label = Some(label);
        }
        self
    }

    pub(crate) fn set_image(
        self,
        image_link_type: LinkType,
        image_url: Cow<'a, str>,
        image_title: Cow<'a, str>,
        image_label: Cow<'a, str>,
    ) -> Self {
        Self {
            link: Link {
                image_link_type: Some(image_link_type),
                image_url: if !image_url.is_empty() {
                    Some(image_url)
                } else {
                    self.link.image_url
                },
                image_title: if !image_title.is_empty() {
                    Some(image_title)
                } else {
                    self.link.image_title
                },
                image_label: if !image_label.is_empty() {
                    Some(image_label)
                } else {
                    self.link.image_label
                },
                ..self.link
            },
        }
    }

    pub(crate) fn set_image_url(mut self, image_url: Cow<'a, str>) -> Self {
        if !image_url.is_empty() {
            self.link.image_url = Some(image_url);
        }
        self
    }

    pub(crate) fn add_image_alt_text(mut self, image_alt_text: Cow<'a, str>) -> Self {
        if !image_alt_text.is_empty() {
            self.link.image_alt_text =
                Some(self.link.image_alt_text.unwrap_or_default() + image_alt_text)
        }
        self
    }

    pub(crate) fn build(self) -> Link<'a> {
        self.link
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_equality_and_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;
        use std::hash::Hasher;

        let link1 = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url1".into(),
            "title1".into(),
            "label1".into(),
        )
        .build();
        let link2 = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url1".into(),
            "title1".into(),
            "label1".into(),
        )
        .build();
        let link3 = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url2".into(),
            "title1".into(),
            "label1".into(),
        )
        .build();

        assert_eq!(link1, link2);
        assert_ne!(link1, link3);

        let mut hasher1 = DefaultHasher::new();
        link1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        link2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_link_ordering() {
        let link_a = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url".into(),
            "title".into(),
            "a".into(),
        )
        .build();
        let link_b = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url".into(),
            "title".into(),
            "b".into(),
        )
        .build();
        let link_a_url2 = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url2".into(),
            "title".into(),
            "a".into(),
        )
        .build();

        assert!(link_a < link_b);
        assert!(link_a < link_a_url2);
    }

    #[test]
    fn test_to_reference_definition() {
        let link = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url".into(),
            "title".into(),
            "label".into(),
        )
        .build();
        assert_eq!(link.to_reference_definition(), "[label]: url \"title\"");

        let link_no_title = LinkBuilder::from_type_url_title(
            LinkType::Shortcut,
            "url".into(),
            "".into(),
            "label".into(),
        )
        .build();
        assert_eq!(link_no_title.to_reference_definition(), "[label]: url");
    }
}
