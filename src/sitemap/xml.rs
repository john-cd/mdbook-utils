//! Write links to sitemap.xml
//!
//! Inspired by <https://github.com/rxdn/mdbook-sitemap-generator/tree/master>
//! Consider using <https://docs.rs/sitewriter/1.0.4/sitewriter/>.
//! or <https://crates.io/crates/sitemap> instead.

use std::io::Write;

use anyhow::anyhow;
use quick_xml::events::BytesText;
use quick_xml::writer::Writer;

// Write in the sitemap.xml format to a file, given a list of links.
pub(super) fn write_xml<W: Write>(links: Vec<String>, w: &mut W) -> anyhow::Result<()> {
    let mut writer = Writer::new_with_indent(w, b' ', 2);

    writer.write_bom().map_err(|_e| {
        anyhow!("[write_xml] Failed to write byte-order-marks to the XML document.")
    })?;
    // Insert <?xml version="1.0" encoding="UTF-8"?>
    writer
        .get_mut()
        .write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")
        .map_err(|_e| anyhow!("[write_xml] Failed to write to the XML document."))?;
    // <urlset>
    writer
        .create_element("urlset")
        .with_attribute(("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"))
        .write_inner_content(|writer| {
            for link in links.iter() {
                // <url><loc>
                writer.create_element("url").write_inner_content(|w| {
                    let escaped = quick_xml::escape::escape(link.as_str());
                    w.create_element("loc")
                        .write_text_content(BytesText::from_escaped(quick_xml::escape::escape(link.as_str())))?;
                    Ok(())
                })?;
            }
            Ok(())
        })
        .map_err(|_e| anyhow!("[write_xml] Failed to write the url set."))?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write_xml_escapes_url() {
        let links = vec!["http://example.com/test?a=1&b=2<script>alert(1)</script>'\"".to_string()];
        let mut w = Cursor::new(Vec::new());
        write_xml(links, &mut w).unwrap();

        let result = String::from_utf8(w.into_inner()).unwrap();
        // The URL should be properly escaped
        assert!(result.contains("<loc>http://example.com/test?a=1&amp;b=2&lt;script&gt;alert(1)&lt;/script&gt;&apos;&quot;</loc>"));
    }
}
