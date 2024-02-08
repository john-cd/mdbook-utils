use std::io::Write;

use anyhow::Result;
use pulldown_cmark::Parser;

/// Write reference definitions parsed from a Markdown parser to a
/// file / writer.
///
/// parser: Markdown parser
/// w: Writer e.g. File
pub(crate) fn write_refdefs_to<W>(parser: &mut Parser<'_>, w: &mut W) -> Result<()>
where
    W: Write,
{
    let sorted_linkdefs: std::collections::BTreeMap<_, _> =
        parser.reference_definitions().iter().collect();

    for (s, linkdef) in sorted_linkdefs {
        if let Some(t) = &linkdef.title {
            writeln!(w, "[{s}]: {} \"{t:?}\"", linkdef.dest)?;
        } else {
            writeln!(w, "[{s}]: {}", linkdef.dest)?;
        }
    }
    Ok(())
}

# [cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
