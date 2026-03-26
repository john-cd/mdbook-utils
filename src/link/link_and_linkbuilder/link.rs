use std::borrow::Cow;
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;

use heck::ToKebabCase;
use pulldown_cmark::LinkType;

/// `Link` is a structure that collects all necessary information to
/// write Markdown (inline or reference-style) links and reference
/// definitions, including badges.
#[derive(Debug, Default, Clone)]
pub(crate) struct Link<'a> {
    pub(crate) link_type: Option<LinkType>,
    pub(crate) text: Option<Cow<'a, str>>,  // [text](...)
    pub(crate) label: Option<Cow<'a, str>>, // [...][label] and [label]: ...
    pub(crate) url: Option<Cow<'a, str>>,   // [...]: url or [...](url) or <url>
    // parsed_url: Option<Url>, Url::parse( )?
    pub(crate) title: Option<Cow<'a, str>>, // [...]: url "title" or [...](url "title")

    // [![image_alt_text][image_label]][...]
    // [image_label]: image_url "image_title"
    #[allow(dead_code)]
    pub(crate) image_link_type: Option<LinkType>,
    pub(crate) image_alt_text: Option<Cow<'a, str>>,
    pub(crate) image_label: Option<Cow<'a, str>>,
    pub(crate) image_url: Option<Cow<'a, str>>,
    pub(crate) image_title: Option<Cow<'a, str>>,
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

    // Methods that write Markdown directly

    /// Returns the link type
    pub(crate) fn get_link_type(&self) -> Option<LinkType> {
        self.link_type
    }

    /// Return the link's text
    fn get_text(&self) -> Cow<'a, str> {
        self.text.clone().unwrap_or(Cow::from(""))
    }

    /// Returns the link's url
    pub(crate) fn get_url(&self) -> Cow<'a, str> {
        if let Some(u) = &self.url {
            u.clone()
        } else {
            Cow::from(String::new())
        }
    }

    /// Returns the link's url (and title if present)
    fn get_url_and_title(&self) -> Cow<'a, str> {
        if let Some(u) = &self.url {
            if let Some(t) = &self.title {
                format!("{u} \"{t}\"").into()
            } else {
                u.clone()
            }
        } else {
            Cow::from(String::new())
        }
    }

    /// Returns the link's reference label, if it exists, or the
    /// kebab-cased link's text
    fn get_label(&self) -> Cow<'a, str> {
        if let Some(label) = &self.label {
            label.clone()
        } else if let Some(txt) = &self.text {
            txt.to_kebab_case().into()
        } else {
            "".into()
        }
    }

    /// Return a Markdown inline link:
    /// [text](url) or [text](url "title")
    pub(crate) fn to_inline_link(&self) -> Cow<'a, str> {
        format!("[{}]( {} )", self.get_text(), self.get_url_and_title()).into()
    }

    /// Return a reference-style Markdown link:
    /// \[text\]\[label\] or \[text/label\]
    pub(crate) fn to_reference_link(&self) -> Cow<'a, str> {
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
    pub(crate) fn to_reference_definition(&self) -> Cow<'a, str> {
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
