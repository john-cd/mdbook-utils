/// Write links and reference definitions to file(s)
use std::io::Write;

use anyhow::Context;
use anyhow::Result;
use enumflags2::bitflags;
use enumflags2::BitFlags;

use super::Link;

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
    for l in links.iter() {
        write(l, &link_flags, link_writer)?;
        writeln!(link_writer)?
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

    for l in links.iter() {
        write(l, &refdef_flags, refdef_writer)?;
        writeln!(refdef_writer)?
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
    for l in links.iter() {
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
//     tracing::debug!("[write_refdef_to] {:?}", l);
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
//     tracing::debug!("[write_link_to] {:?}", l);
//     let link_flags = (LinkWrite::ReferenceLink |
// LinkWrite::ReferenceDefinition ).into();     write(l, &link_flags,
// link_writer).context("[write_link_to] Failed to write a link.")?;     Ok(())
// }

// PRIVATE FUNCTIONS

#[bitflags(default = ReferenceLink | ReferenceDefinition)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum LinkWrite {
    InlineLink = 1,
    ReferenceLink = 2,
    ReferenceDefinition = 4,
    LinkWithBadge = 8,
    BadgeReferenceDefinition = 16,
}

/// Write a link to a writer in the format specified by flags.
///
/// l: the Link struct
///
/// flags: Bitflags
///
/// w: the writer e.g. a File or Vec<u8>
#[inline]
fn write<W: Write>(l: &Link<'_>, flags: &BitFlags<LinkWrite>, w: &mut W) -> Result<()> {
    if flags.contains(LinkWrite::InlineLink) {
        writeln!(w, "{}", l.to_inline_link())?;
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
