//! Write reference definitions
//!
//! TODO Reviewed ??-??-2026

use std::io::Write;

use anyhow::Result;
use pulldown_cmark::Parser;

/// Write reference definitions parsed from a Markdown parser to a
/// file / writer.
///
/// parser: Markdown parser.
///
/// w: Writer e.g. File
pub(crate) fn write_refdefs_to<W>(parser: &mut Parser<'_>, w: &mut W) -> Result<()>
where
    W: Write,
{
    let sorted_linkdefs: std::collections::BTreeMap<_, _> =
        parser.reference_definitions().iter().collect();

    for (s, linkdef) in sorted_linkdefs {
        if let Some(t) = &linkdef.title {
            writeln!(w, "[{s}]: {} \"{}\"", linkdef.dest, t.replace('"', "\\\""))?;
        } else {
            writeln!(w, "[{s}]: {}", linkdef.dest)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::get_parser;

    /// Titles are written as plain quoted strings, with quotes escaped.
    #[test]
    fn writes_titles_without_debug_formatting() -> Result<()> {
        let md = "[a]: http://x.com \"My Title\"\n[b]: http://y.com\n";
        let mut parser = get_parser(md);
        let mut out = Vec::new();
        write_refdefs_to(&mut parser, &mut out)?;
        assert_eq!(
            String::from_utf8(out)?,
            "[a]: http://x.com \"My Title\"\n[b]: http://y.com\n"
        );
        Ok(())
    }
}
