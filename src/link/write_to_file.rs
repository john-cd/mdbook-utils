//! Write [links][Link] and reference definitions to file(s)
//!
//! TODO Reviewed ??-??-2026

use std::io::Write;

use anyhow::Context as _;
use anyhow::Result;
use enumflags2::BitFlags;
use enumflags2::bitflags;

use super::Link;
use super::LinkFormat;

// PUBLIC FUNCTIONS

/// Write links to a writer (e.g. file)
pub(crate) fn write_reference_style_links_to<W>(
    links: Vec<Link<'_>>,
    link_writer: &mut W,
) -> Result<()>
where
    W: Write,
{
    writeln!(link_writer, "# Links\n")
        .context("[write_reference_style_links_to] Failed to write links.")?;

    let link_flags = LinkWrite::ReferenceLink | LinkWrite::ReferenceDefinition;
    for l in &links {
        write(l, &link_flags, link_writer)?;
        writeln!(link_writer)?;
    }
    Ok(())
}

/// Write links to a writer (e.g. file) in the requested [`LinkFormat`].
///
/// Reference format writes each reference link followed by its definition,
/// inline format writes `[text](url "title")`, and autolink format writes
/// `<url>`.
pub(crate) fn write_links_in_format_to<W>(
    links: Vec<Link<'_>>,
    format: LinkFormat,
    link_writer: &mut W,
) -> Result<()>
where
    W: Write,
{
    if format == LinkFormat::Reference {
        return write_reference_style_links_to(links, link_writer);
    }
    writeln!(link_writer, "# Links\n")
        .context("[write_links_in_format_to] Failed to write links.")?;

    let link_flags = match format {
        LinkFormat::Inline => LinkWrite::InlineLink,
        _ => LinkWrite::Autolink,
    }
    .into();
    for l in &links {
        write(l, &link_flags, link_writer)?;
        writeln!(link_writer)?;
    }
    Ok(())
}

/// Write a reference definition to a writer (e.g. file)
pub(crate) fn write_refdefs_to<W>(links: Vec<Link<'_>>, refdef_writer: &mut W) -> Result<()>
where
    W: Write,
{
    writeln!(refdef_writer, "# Reference Definitions\n")
        .context("[write_refdefs_to] Failed to write reference definitions.")?;

    let refdef_flags = (LinkWrite::ReferenceDefinition).into();

    for l in &links {
        write(l, &refdef_flags, refdef_writer)?;
        writeln!(refdef_writer)?;
    }
    Ok(())
}

/// Write duplicate links to a writer
pub(crate) fn write_duplicate_links_to<W>(links: Vec<Link<'_>>, link_writer: &mut W) -> Result<()>
where
    W: Write,
{
    writeln!(link_writer, "# Duplicate Links\n")
        .context("[write_duplicate_links_to] Failed to write duplicate links.")?;

    let link_flags = LinkWrite::ReferenceLink | LinkWrite::ReferenceDefinition;
    for l in &links {
        write(l, &link_flags, link_writer)?;
        writeln!(link_writer)?;
    }
    Ok(())
}

/// Write broken links to a writer
pub(crate) fn write_broken_links_to<W>(
    broken_links: Vec<(String, String, String)>,
    link_writer: &mut W,
) -> Result<()>
where
    W: Write,
{
    writeln!(link_writer, "# Broken Links\n")
        .context("[write_broken_links_to] Failed to write broken links.")?;

    for (reference, location, link_type) in broken_links {
        writeln!(link_writer, "- Reference: {reference}")?;
        if !location.is_empty() {
            writeln!(link_writer, "  Location: {location}")?;
        }
        writeln!(link_writer, "  Type: {link_type}")?;
        writeln!(link_writer)?;
    }
    Ok(())
}

// /// Write reference definitions and links to a writer (e.g. file)
// pub(crate) fn write_refdefs_and_links_to<W>(links: Vec<Link<'_>>, writer:
// &mut W) -> Result<()> where
//     W: Write,
// {
//     writeln!(writer, "# Links and Reference
// Definitions\n").context("[write_refdefs_and_links_to] Failed to write links
// and reference definitions.")?;     for l in links.iter() {
//         write_link_to(l, writer)?;
//         write_refdef_to(l, writer)?;
//     }
//     Ok(())
// }

/// Write a badge-style link and associated reference definitions
/// to two separate writers
pub(crate) fn write_badge_refdefs_and_links_to_two<W1, W2>(
    links: Vec<Link<'_>>,
    link_writer: &mut W1,
    refdef_writer: &mut W2,
) -> Result<()>
where
    W1: Write,
    W2: Write,
{
    writeln!(link_writer, "# Links and Reference Definitions\n")
        .context("[write_badge_refdefs_and_links_to_two] Failed to write refdefs and links.")?;

    let link_flags = LinkWrite::LinkWithBadge.into();
    let refdef_flags = LinkWrite::ReferenceDefinition | LinkWrite::BadgeReferenceDefinition;
    for l in &links {
        write(l, &link_flags, link_writer)?;
        write(l, &refdef_flags, refdef_writer)?;
    }
    Ok(())
}

// /// Write a reference definition to a writer (e.g. file)
// #[inline]
// fn write_refdef_to<W>(l: &Link<'_>, refdef_writer: &mut W) -> Result<()>
// where
//     W: Write,
// {
//     tracing::debug!("[write_refdef_to] {l:?}");
//     let refdef_flags = (LinkWrite::ReferenceDefinition).into();
//     write(l, &refdef_flags, refdef_writer).context("[write_refdef_to] Failed
// to write a reference definition.")?;     Ok(())
// }

// /// Write a link to a writer (e.g. file)
// #[inline]
// fn write_link_to<W>(l: &Link<'_>, link_writer: &mut W) -> Result<()>
// where
//     W: Write,
// {
//     tracing::debug!("[write_link_to] {l:?}");
//     let link_flags = (LinkWrite::ReferenceLink |
// LinkWrite::ReferenceDefinition ).into();     write(l, &link_flags,
// link_writer).context("[write_link_to] Failed to write a link.")?;     Ok(())
// }

// PRIVATE FUNCTIONS

/// Type of link or reference definition to write
#[bitflags(default = ReferenceLink | ReferenceDefinition)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum LinkWrite {
    InlineLink = 1,
    ReferenceLink = 2,
    ReferenceDefinition = 4,
    LinkWithBadge = 8,
    BadgeReferenceDefinition = 16,
    Autolink = 32,
}

/// Write a link to a writer in the format specified by flags.
///
/// l: the Link struct
///
/// flags: Bitflags
///
/// w: the writer e.g. a File or Vec\<u8\>
#[inline]
fn write<W: Write>(l: &Link<'_>, flags: &BitFlags<LinkWrite>, w: &mut W) -> Result<()> {
    if flags.contains(LinkWrite::InlineLink) {
        writeln!(w, "{}", l.to_inline_link())?;
    }
    if flags.contains(LinkWrite::Autolink) {
        writeln!(w, "{}", l.to_autolink())?;
    }
    if flags.contains(LinkWrite::ReferenceLink) {
        writeln!(w, "{}", l.to_reference_link())?;
    }
    if flags.contains(LinkWrite::LinkWithBadge) {
        writeln!(w, "{}", l.to_link_with_badge())?;
    }
    if flags.intersects(LinkWrite::ReferenceDefinition | LinkWrite::BadgeReferenceDefinition) {
        // empty line separator
        writeln!(w)?;
    }
    if flags.contains(LinkWrite::ReferenceDefinition) {
        writeln!(w, "{}", l.to_reference_definition())?;
    }
    if flags.contains(LinkWrite::BadgeReferenceDefinition) {
        writeln!(w, "{}", l.to_badge_reference_definition())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use anyhow::Result;
    use pulldown_cmark::LinkType;

    use super::*;
    use crate::link::LinkBuilder;

    /// Builds a link with text, label, URL and title.
    fn link(text: &'static str, label: &'static str, url: &'static str) -> Link<'static> {
        LinkBuilder::from_type_url_title(
            LinkType::Inline,
            Cow::Borrowed(url),
            Cow::Borrowed(""),
            Cow::Borrowed(label),
        )
        .add_text(Cow::Borrowed(text))
        .build()
    }

    /// Reference-style output has a heading, the reference link, and its
    /// definition.
    #[test]
    fn writes_reference_links_and_definitions() -> Result<()> {
        let mut out = Vec::new();
        write_reference_style_links_to(vec![link("Serde", "serde", "https://serde.rs")], &mut out)?;
        let text = String::from_utf8(out)?;

        assert!(text.starts_with("# Links\n"), "{text}");
        assert!(text.contains("[Serde][serde]"), "{text}");
        assert!(text.contains("[serde]: https://serde.rs"), "{text}");
        Ok(())
    }

    /// Reference definition output omits the reference links.
    #[test]
    fn writes_only_reference_definitions() -> Result<()> {
        let mut out = Vec::new();
        write_refdefs_to(vec![link("Serde", "serde", "https://serde.rs")], &mut out)?;
        let text = String::from_utf8(out)?;

        assert!(text.starts_with("# Reference Definitions\n"), "{text}");
        assert!(text.contains("[serde]: https://serde.rs"), "{text}");
        assert!(!text.contains("[Serde][serde]"), "{text}");
        Ok(())
    }

    /// Duplicate-link output uses its own heading.
    #[test]
    fn writes_duplicate_links_heading() -> Result<()> {
        let mut out = Vec::new();
        write_duplicate_links_to(vec![link("A", "a", "https://a.example")], &mut out)?;

        assert!(String::from_utf8(out)?.starts_with("# Duplicate Links\n"));
        Ok(())
    }

    /// Broken links are written as a list of reference, text, and type.
    #[test]
    fn writes_broken_links_list() -> Result<()> {
        let mut out = Vec::new();
        write_broken_links_to(
            vec![("ref".into(), "a.md:3".into(), "Shortcut".into())],
            &mut out,
        )?;

        assert_eq!(
            String::from_utf8(out)?,
            "# Broken Links\n\n- Reference: ref\n  Location: a.md:3\n  Type: Shortcut\n\n"
        );
        Ok(())
    }

    /// An empty list of broken links still writes the heading.
    #[test]
    fn writes_heading_for_no_broken_links() -> Result<()> {
        let mut out = Vec::new();
        write_broken_links_to(Vec::new(), &mut out)?;

        assert_eq!(String::from_utf8(out)?, "# Broken Links\n\n");
        Ok(())
    }

    /// Badge output sends links and definitions to separate writers.
    #[test]
    fn writes_badge_links_and_definitions_to_two_writers() -> Result<()> {
        let (mut links, mut refdefs) = (Vec::new(), Vec::new());
        write_badge_refdefs_and_links_to_two(
            vec![link("Serde", "serde", "https://serde.rs")],
            &mut links,
            &mut refdefs,
        )?;
        let (links, refdefs) = (String::from_utf8(links)?, String::from_utf8(refdefs)?);

        assert!(links.contains("[![serde][serde-badge]][serde]"), "{links}");
        assert!(refdefs.contains("[serde]: https://serde.rs"), "{refdefs}");
        Ok(())
    }
}
