//! Internal models for a Markdown [Link] and [LinkBuilder]
use std::borrow::Cow;
use std::cmp::Ordering;

use heck::ToKebabCase;
use pulldown_cmark::LinkType;

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

impl<'a> Link<'a> {
    pub(crate) fn to_static(&self) -> Link<'static> {
        Link {
            link_type: self.link_type,
            text: self.text.as_ref().map(|c| Cow::Owned(c.to_string())),
            label: self.label.as_ref().map(|c| Cow::Owned(c.to_string())),
            url: self.url.as_ref().map(|c| Cow::Owned(c.to_string())),
            title: self.title.as_ref().map(|c| Cow::Owned(c.to_string())),
            image_link_type: self.image_link_type,
            image_alt_text: self
                .image_alt_text
                .as_ref()
                .map(|c| Cow::Owned(c.to_string())),
            image_label: self.image_label.as_ref().map(|c| Cow::Owned(c.to_string())),
            image_url: self.image_url.as_ref().map(|c| Cow::Owned(c.to_string())),
            image_title: self.image_title.as_ref().map(|c| Cow::Owned(c.to_string())),
        }
    }
}

// LINK -----------------------------

use std::hash::Hash;
use std::hash::Hasher;

/// `Link` is a structure that collects all necessary information to
/// write Markdown (inline or reference-style) links and reference
/// definitions, including badges.
#[derive(Debug, Default, Clone)]
pub(crate) struct Link<'a> {
    link_type: Option<LinkType>,
    text: Option<Cow<'a, str>>,  // [text](...)
    label: Option<Cow<'a, str>>, // [...][label] and [label]: ...
    url: Option<Cow<'a, str>>,   // [...]: url or [...](url) or <url>
    // parsed_url: Option<Url>, Url::parse( )?
    title: Option<Cow<'a, str>>, // [...]: url "title" or [...](url "title")

    // [![image_alt_text][image_label]][...]
    // [image_label]: image_url "image_title"
    #[allow(dead_code)]
    image_link_type: Option<LinkType>,
    image_alt_text: Option<Cow<'a, str>>,
    image_label: Option<Cow<'a, str>>,
    image_url: Option<Cow<'a, str>>,
    image_title: Option<Cow<'a, str>>,
}

impl<'a> Link<'a> {
    // Methods that write Markdown directly

    /// Returns the link type
    pub(crate) fn get_link_type(&self) -> Option<LinkType> {
        self.link_type
    }

    /// Return the link's text
    fn get_text(&self) -> Cow<'_, str> {
        self.text
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or(Cow::Borrowed(""))
    }

    /// Returns the link's url
    pub(crate) fn get_url(&self) -> Cow<'_, str> {
        self.url
            .as_deref()
            .map(Cow::Borrowed)
            .unwrap_or(Cow::Borrowed(""))
    }

    /// Returns the link's url (and title if present)
    fn get_url_and_title(&self) -> Cow<'_, str> {
        if let Some(u) = &self.url {
            if let Some(t) = &self.title {
                format!("{u} \"{t}\"").into()
            } else {
                Cow::Borrowed(u.as_ref())
            }
        } else {
            Cow::Borrowed("")
        }
    }

    /// Returns the link's reference label, if it exists, or the
    /// kebab-cased link's text
    fn get_label(&self) -> Cow<'_, str> {
        if let Some(label) = &self.label {
            Cow::Borrowed(label.as_ref())
        } else if let Some(txt) = &self.text {
            txt.to_kebab_case().into()
        } else {
            Cow::Borrowed("")
        }
    }

    /// Return a Markdown inline link:
    /// [text](url) or [text](url "title")
    pub(crate) fn to_inline_link(&self) -> Cow<'_, str> {
        format!("[{}]( {} )", self.get_text(), self.get_url_and_title()).into()
    }

    /// Return a reference-style Markdown link:
    /// \[text\]\[label\] or \[text/label\]
    pub(crate) fn to_reference_link(&self) -> Cow<'_, str> {
        let txt: String = self.get_text().into();
        let label: String = self.get_label().into();
        if txt == label {
            format!("[{txt}]").into()
        } else {
            format!("[{txt}][{label}]").into()
        }
    }

    /// Return a Markdown reference definition:
    /// \[label\]: url or \[label\]: url "title"
    pub(crate) fn to_reference_definition(&self) -> Cow<'_, str> {
        format!("[{}]: {}", self.get_label(), self.get_url_and_title()).into()
    }

    // BADGES / IMAGES

    /// Return the badge alt text, if it exists, or the badge's label
    /// or the link's label
    fn get_badge_alt_text(&self) -> Cow<'a, str> {
        if let Some(alt_txt) = &self.image_alt_text {
            alt_txt.clone()
        } else if let Some(img_lbl) = &self.image_label {
            img_lbl.clone()
        } else if let Some(lbl) = &self.label {
            lbl.clone()
        } else {
            "".into()
        }
    }

    /// Return the label for the badge reference
    /// e.g. image_label or \<label\>-badge
    fn get_badge_label(&self) -> Cow<'a, str> {
        if let Some(ref img_lbl) = self.image_label {
            img_lbl.clone()
        } else if let Some(ref lbl) = self.label {
            format!("{lbl}-badge").into()
        } else if let Some(ref alt_txt) = self.image_alt_text {
            alt_txt.clone()
        } else {
            "badge".into()
        }
    }

    /// Return the badge's url and title (if present)
    fn get_badge_url_and_title(&self) -> Cow<'a, str> {
        if let Some(ref u) = self.image_url {
            if let Some(ref t) = self.image_title {
                format!("{u} \"{t}\"").into()
            } else {
                u.clone()
            }
        } else {
            Cow::from(String::new())
        }
    }

    /// Return a badge image that is clickable:
    /// \[ !\[ alt-text \]\[ badge-label \] \]\[ label \]
    pub(crate) fn to_link_with_badge(&self) -> Cow<'a, str> {
        format!(
            "[![{}][{}]][{}]",
            self.get_badge_alt_text(),
            self.get_badge_label(),
            self.get_label()
        )
        .into()
    }

    /// Return the reference definition for a badge image: [badge-label]: https://badge-image-url...  "image_title"
    pub(crate) fn to_badge_reference_definition(&self) -> Cow<'a, str> {
        format!(
            "[{}]: {}",
            self.get_badge_label(),
            self.get_badge_url_and_title()
        )
        .into()
    }
}

impl PartialOrd for Link<'_> {
    /// PartialOrd implementation for Link
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // The type is `Ord``,
        // thus we can implement `partial_cmp` by using `cmp`
        Some(self.cmp(other))
    }
}

impl Ord for Link<'_> {
    /// Ord implementation for Link
    fn cmp(&self, other: &Self) -> Ordering {
        self.label
            .cmp(&other.label)
            .then(self.url.cmp(&other.url))
            .then(self.title.cmp(&other.title))
    }
}

impl PartialEq for Link<'_> {
    /// PartialEq implementation for Link
    fn eq(&self, other: &Self) -> bool {
        (self.label == other.label) && (self.url == other.url) && (self.title == other.title)
    }
}

/// Eq implementation for Link
impl Eq for Link<'_> {}

impl Hash for Link<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
        self.url.hash(state);
        self.title.hash(state);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_link_equality_and_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

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

        let link_no_title =
            LinkBuilder::from_type_url_title(LinkType::Shortcut, "url".into(), "".into(), "label".into())
                .build();
        assert_eq!(link_no_title.to_reference_definition(), "[label]: url");
    }
}
