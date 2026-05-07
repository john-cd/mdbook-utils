use std::collections::BTreeSet;
use std::io::Write;

use crate::link::Link;

#[derive(Clone, Copy, Debug)]
pub(crate) enum IndexKind {
    Categories,
    Crates,
}

impl IndexKind {
    pub(crate) fn title(self) -> &'static str {
        match self {
            Self::Categories => "Categories",
            Self::Crates => "Crates",
        }
    }

    pub(crate) fn list_url(self, name: &str) -> String {
        match self {
            Self::Categories => format!("https://crates.io/categories/{name}"),
            Self::Crates => format!("https://crates.io/crates/{name}"),
        }
    }

    fn segment_prefix(self) -> &'static str {
        match self {
            Self::Categories => "crates.io/categories/",
            Self::Crates => "crates.io/crates/",
        }
    }

    fn segment_exclusion(self) -> &'static str {
        match self {
            Self::Categories => "categories",
            Self::Crates => "crates",
        }
    }
}

fn is_valid_name(name: &str) -> bool {
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

pub(crate) fn extract_index_names<'a>(
    links: impl IntoIterator<Item = Link<'a>>,
    kind: IndexKind,
) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for link in links {
        let url = link.get_url();
        if !url.contains(kind.segment_prefix()) {
            continue;
        }
        let mut path = url.split('?').next().unwrap_or("");
        if path.ends_with('/') {
            path = &path[..path.len() - 1];
        }
        if let Some(name) = path.split('/').next_back()
            && !name.is_empty()
            && name != kind.segment_exclusion()
            && is_valid_name(name)
        {
            names.insert(name.to_string());
        }
    }
    names
}

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

    use crate::link::LinkBuilder;

    use super::*;

    fn link(url: &str) -> Link<'_> {
        LinkBuilder::default()
            .set_label(Cow::Borrowed("x"))
            .set_url(Cow::Owned(url.to_string()))
            .build()
    }

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
        let expected = BTreeSet::from(["anyhow".to_string(), "serde".to_string()]);
        assert_eq!(names, expected);
    }

    #[test]
    fn test_write_index_writes_header_and_links() -> anyhow::Result<()> {
        let mut out = Vec::new();
        write_index(
            &mut out,
            IndexKind::Categories,
            vec!["parsing".to_string(), "development-tools".to_string()],
        )?;

        let content = String::from_utf8(out)?;
        let expected = "# Categories\n\n- [parsing](https://crates.io/categories/parsing)\n- [development-tools](https://crates.io/categories/development-tools)\n";
        assert_eq!(content, expected);
        Ok(())
    }
}
