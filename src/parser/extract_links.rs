//! Use a Markdown parser to extract links
//!
//! TODO Reviewed ??-??-2026

use std::borrow::Cow;

use pulldown_cmark::Event;
use pulldown_cmark::LinkType;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use pulldown_cmark::TagEnd;
use tracing::debug;

use super::super::link::Link;
use super::super::link::LinkBuilder;

/// Tracks where we are in the Markdown parser event stream.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Where {
    /// Not inside a link.
    Elsewhere,
    /// Inside a link, outside any image.
    InLink, // between Start(Tag::Link(...)) and End(Tag::Link(..))
    /// Inside an image that is itself inside a link (badge).
    InImageInLink, /* between Start(Tag::Image(...)) and
                    * End(Tag::Image(..)), within a link */
}

/// Return the reference definitions (`[label]: url "title"`) of the parsed
/// text, and nothing else: inline links and autolinks are not included.
pub(crate) fn extract_reference_definitions(parser: &Parser<'_>) -> Vec<Link<'static>> {
    parser
        .reference_definitions()
        .iter()
        .map(|(label, def)| {
            LinkBuilder::from_type_url_title(
                LinkType::Shortcut,
                def.dest.to_string().into(),
                def.title.as_deref().unwrap_or_default().to_owned().into(),
                label.to_owned().into(),
            )
            .build()
        })
        .collect()
}

/// Read from a Markdown parser, extract links from the event stream,
/// and return said links.
pub(crate) fn extract_links<'input>(parser: &mut Parser<'input>) -> Vec<Link<'input>> {
    let mut state: Vec<(Where, LinkBuilder<'input>)> = Vec::new();
    let mut links: Vec<Link<'input>> = Vec::new();

    for event in parser {
        match event {
            // Start of a link
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) => {
                debug!(
                    ?link_type,
                    has_title = !title.is_empty(),
                    has_reference_label = !id.is_empty(),
                    "parsed link event"
                );
                state.push((
                    Where::InLink,
                    LinkBuilder::from_type_url_title(
                        link_type,
                        dest_url.into(),
                        title.into(),
                        id.into(),
                    ),
                ));
            }

            // End of the link
            Event::End(TagEnd::Link) => {
                // We pop until we get the corresponding InLink?
                // Wait, Start(Link) always pushes.
                // End(Link) always pops. So they must be balanced if the parser
                // works correctly.
                if let Some((whr, link_builder)) = state.pop() {
                    if whr == Where::InLink {
                        links.push(link_builder.build());
                    } else {
                        tracing::warn!("Expected InLink state, found {:?}", whr);
                    }
                } else {
                    tracing::warn!("Unbalanced Link End tag");
                }
            }

            // Inspect events while in the link
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) if !state.is_empty() => {
                debug!(
                    ?link_type,
                    has_title = !title.is_empty(),
                    has_reference_label = !id.is_empty(),
                    "parsed image inside link"
                );
                if let Some((whr, link_builder)) = state.pop() {
                    if whr == Where::InLink {
                        state.push((
                            Where::InImageInLink,
                            link_builder.set_image(
                                link_type,
                                dest_url.into(),
                                title.into(),
                                id.into(),
                            ),
                        ));
                    } else {
                        tracing::warn!("Expected InLink state, found {:?}", whr);
                        state.push((whr, link_builder)); // restore state
                    }
                }
            }

            Event::End(TagEnd::Image) if !state.is_empty() => {
                if let Some((whr, link_builder)) = state.pop() {
                    if whr == Where::InImageInLink {
                        state.push((Where::InLink, link_builder));
                    } else {
                        tracing::warn!("Expected InImageInLink state, found {:?}", whr);
                        state.push((whr, link_builder)); // restore state
                    }
                }
            }
            // Text of an Image
            Event::Text(t)
                if state.last().map_or(Where::Elsewhere, |s| s.0) == Where::InImageInLink =>
            {
                debug!(byte_length = t.len(), "read image alternative text");
                if let Some((whr, link_builder)) = state.pop() {
                    if whr == Where::InImageInLink {
                        state.push((whr, link_builder.add_image_alt_text(Cow::from(t))));
                    } else {
                        tracing::warn!("Expected InImageInLink state, found {:?}", whr);
                        state.push((whr, link_builder)); // restore state
                    }
                }
            }
            // Text of a Link
            Event::Text(t) if !state.is_empty() => {
                debug!(byte_length = t.len(), "read link text");
                if let Some((whr, link_builder)) = state.pop() {
                    if whr == Where::InLink {
                        state.push((whr, link_builder.add_text(Cow::from(t))));
                    } else {
                        tracing::warn!("Expected InLink state, found {:?}", whr);
                        state.push((whr, link_builder)); // restore state
                    }
                }
            }

            Event::Code(c) if !state.is_empty() => {
                debug!(byte_length = c.len(), "read code inside link");
                if let Some((whr, link_builder)) = state.pop() {
                    if whr == Where::InImageInLink {
                        state.push((whr, link_builder.add_image_alt_text(c.into())));
                    } else if whr == Where::InLink {
                        state.push((whr, link_builder.add_text(c.into())));
                    } else {
                        tracing::warn!("Expected InImageInLink or InLink state, found {:?}", whr);
                        state.push((whr, link_builder)); // restore state
                    }
                }
            }

            // To robustly handle nested structures like bold, italics, etc, we should simply
            // ignore Start and End tags that aren't Link or Image while in state.
            Event::Start(_) | Event::End(_) if !state.is_empty() => {
                // Ignore nested formatting tags like Strong, Emphasis, etc.
                debug!("Ignoring nested formatting tag");
            }

            _ if !state.is_empty() => {
                // Ignore other nested things, like Rule, SoftBreak, HardBreak,
                // Html. Or maybe SoftBreak/HardBreak should add
                // a space? Text gets added normally.
                // We shouldn't panic or error out here.
            }
            _ => {}
        }
    }

    if !state.is_empty() {
        tracing::warn!("State was not empty at end of parsing. Unbalanced markdown tags detected.");
    }

    links
}

#[cfg(test)]
mod tests {
    use pulldown_cmark::Parser;

    use super::*;

    /// Only reference definitions are returned, not inline links.
    #[test]
    fn extracts_only_reference_definitions() {
        let markdown = "[inline](https://a.example) [ref]\n\n[ref]: https://b.example \"T\"\n";
        let parser = Parser::new(markdown);
        let defs = extract_reference_definitions(&parser);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].label.as_deref(), Some("ref"));
        assert_eq!(defs[0].get_url(), "https://b.example");
        assert_eq!(defs[0].title.as_deref(), Some("T"));
    }

    /// Extracts a link with a title.
    #[test]
    fn test_extract_links_simple() {
        let markdown = "[text](url \"title\")";
        let mut parser = Parser::new(markdown);
        let links = extract_links(&mut parser);
        assert_eq!(links.len(), 1);
        let link = &links[0];
        assert_eq!(link.get_url(), "url");
    }

    /// Extracts an image nested in a link.
    #[test]
    fn test_extract_links_with_image() {
        let markdown = "[![alt](img_url)](url)";
        let mut parser = Parser::new(markdown);
        let links = extract_links(&mut parser);
        assert_eq!(links.len(), 1);
        let link = &links[0];
        assert_eq!(link.get_url(), "url");
        // image alt text is not directly exposed in Link yet via getter
    }

    /// Keeps bold and code inside link text.
    #[test]
    fn test_extract_links_with_nested_formatting() {
        let markdown = "[**bold text** and `code` in link](url \"title\")";
        let mut parser = Parser::new(markdown);
        let links = extract_links(&mut parser);
        assert_eq!(links.len(), 1);
        let link = &links[0];
        assert_eq!(link.get_url(), "url");
        assert_eq!(
            link.to_inline_link(),
            "[bold text and code in link](url \"title\")"
        );
    }

    /// Keeps formatting inside the alt text of an image link.
    #[test]
    fn test_extract_links_with_image_and_nested_formatting() {
        let markdown = "[![**badge** alt](badge_url)](link_url)";
        let mut parser = Parser::new(markdown);
        let links = extract_links(&mut parser);
        assert_eq!(links.len(), 1);
        let link = &links[0];
        assert_eq!(link.get_url(), "link_url");
        assert_eq!(link.to_link_with_badge(), "[![badge alt][badge alt]][]");
    }

    /// Handles several links, images and nested brackets in one document.
    #[test]
    fn test_extract_links_complex() {
        let markdown = "[`code`](url) [![`image_code`](img_url)](url) [[link_in_link](url2)](url) [![[link_in_img](url2)](img_url)](url) [foo **bold**](url)";
        let mut parser = Parser::new(markdown);
        let links = extract_links(&mut parser);
        assert_eq!(links.len(), 5);
        assert_eq!(links[0].get_url(), "url");
        assert_eq!(links[1].get_url(), "url");
        assert_eq!(links[2].get_url(), "url2");
        assert_eq!(links[3].get_url(), "url2");
        assert_eq!(links[4].get_url(), "url");
    }

    /// Extracts `CommonMark` destinations while preserving code and mdBook helper
    /// text.
    #[test]
    fn test_extract_links_commonmark_url_edge_cases_and_code_fences() {
        let markdown = r"
[punctuation](https://example.com/guide_(intro)!.)
[escaped](https://example.com/guide\(draft\))
[unicode](https://例え.テスト/雪)
[encoded](https://example.com/%E9%9B%AA?raw=%E9#item)
[query](https://example.com/search?q=rust%2Fmarkdown#results)
<https://example.com/autolink>
[invalid host](https://?query)
[invalid percent escape](https://example.com/%ZZ)

https://example.com/plain-text-is-not-an-autolink
`[code span](https://example.com/code-span)`

```markdown
[fenced](https://example.com/fenced)
{{#include examples/demo.rs}}
```
";
        let mut parser = super::super::get_parser(markdown);
        let links = extract_links(&mut parser);
        let destinations: Vec<_> = links.iter().map(|link| link.get_url()).collect();

        assert_eq!(
            destinations,
            [
                "https://example.com/guide_(intro)!.",
                "https://example.com/guide(draft)",
                "https://\u{4f8b}\u{3048}.\u{30c6}\u{30b9}\u{30c8}/\u{96ea}",
                "https://example.com/%E9%9B%AA?raw=%E9#item",
                "https://example.com/search?q=rust%2Fmarkdown#results",
                "https://example.com/autolink",
                "https://?query",
                "https://example.com/%ZZ",
            ]
        );
    }
}
