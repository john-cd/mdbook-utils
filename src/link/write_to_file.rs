/// Write links and reference definitions to file(s)
use std::io::Write;

use anyhow::Result;
use enumflags2::bitflags;
use enumflags2::BitFlags;
use tracing::debug;

use super::Link;

// PUBLIC FUNCTIONS

/// Write links to a writer (e.g. file)
pub(crate) fn write_links_to<W>(links: Vec<Link<'_>>, link_writer: &mut W) -> Result<()>
where
    W: Write,
{
    writeln!(link_writer, "# Links\n")?;

    for l in links.iter() {
        write_link_to(l, link_writer)?;
    }
    Ok(())
}

/// Write a reference definition to a writer (e.g. file)
pub(crate) fn write_ref_defs_to<W>(links: Vec<Link<'_>>, ref_def_writer: &mut W) -> Result<()>
where
    W: Write,
{
    writeln!(ref_def_writer, "# Reference Definitions\n")?;

    for l in links.iter() {
        write_ref_def_to(l, ref_def_writer)?;
    }
    Ok(())
}

// /// Write reference definitions and links to a writer (e.g. file)
// pub(crate) fn write_ref_defs_and_links_to<W>(links: Vec<Link<'_>>, writer:
// &mut W) -> Result<()> where
//     W: Write,
// {
//     writeln!(writer, "# Links and Reference Definitions\n")?;
//     for l in links.iter() {
//         write_link_to(l, writer)?;
//         write_ref_def_to(l, writer)?;
//     }
//     Ok(())
// }

/// Write a badge-style link and associated reference definitions
/// to two separate writers
pub(crate) fn write_badge_refdefs_and_links_to_two<W1, W2>(
    links: Vec<Link<'_>>,
    link_writer: &mut W1,
    ref_def_writer: &mut W2,
) -> Result<()>
where
    W1: Write,
    W2: Write,
{
    writeln!(link_writer, "# Links and Reference Definitions\n")?;

    let link_flags = LinkWrite::LinkWithBadge.into();
    let refdef_flags =
        (LinkWrite::ReferenceDefinition | LinkWrite::BadgeReferenceDefinition).into();
    for l in links.iter() {
        write(l, &link_flags, link_writer)?;
        write(l, &refdef_flags, ref_def_writer)?;
    }
    Ok(())
}

// TODO use write()
/// Write a reference definition to a writer (e.g. file)
#[inline]
fn write_ref_def_to<W>(l: &Link<'_>, ref_def_writer: &mut W) -> Result<()>
where
    W: Write,
{
    debug!("Link: {:?}", l);

    writeln!(
        ref_def_writer,
        "{}\n{}",
        l.to_reference_definition(),
        l.to_badge_reference_definition()
    )?;
    Ok(())
}

// TODO use write()
/// Write a link to a writer (e.g. file)
#[inline]
fn write_link_to<W>(l: &Link<'_>, link_writer: &mut W) -> Result<()>
where
    W: Write,
{
    debug!("Link: {:?}", l);
    writeln!(
        link_writer,
        "{}\n{}\n{}\n{}\n{}\n",
        l.to_inline_link(),
        l.to_reference_link(),
        l.to_reference_definition(),
        l.to_link_with_badge(),
        l.to_badge_reference_definition()
    )?;
    Ok(())
}

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
    if flags.contains(LinkWrite::ReferenceDefinition) {
        writeln!(w, "{}", l.to_reference_definition())?;
    }
    if flags.contains(LinkWrite::LinkWithBadge) {
        writeln!(w, "{}", l.to_link_with_badge())?;
    }
    if flags.contains(LinkWrite::BadgeReferenceDefinition) {
        writeln!(w, "{}", l.to_badge_reference_definition())?;
    }
    Ok(())
}
