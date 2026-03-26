//! Use a Markdown parser to extract links
use std::borrow::Cow;

use pulldown_cmark::Event;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use pulldown_cmark::TagEnd;
use tracing::debug;

use super::super::link::Link;
use super::super::link::LinkBuilder;

/// Tracks where we are in the Markdown parser event stream.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Where {
    Elsewhere,
    InLink, // between Start(Tag::Link(...)) and End(Tag::Link(..))
    InImageInLink, /* between Start(Tag::Image(...)) and
             * End(Tag::Image(..)), within a link */
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
                debug!("Link: link_type: {link_type:?}, url: {dest_url}, title: {title}, id: {id}");
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
            ref e @ Event::End(TagEnd::Link) => {
                debug!("{e:?}");
                // We pop until we get the corresponding InLink?
                // Wait, Start(Link) always pushes.
                // End(Link) always pops. So they must be balanced if the parser works correctly.
                let (whr, link_builder) = state.pop().unwrap(); // Start and End events are balanced
                assert_eq!(whr, Where::InLink);
                links.push(link_builder.build());
            }

            // Inspect events while in the link
            Event::Start(Tag::Image {
                link_type,
                dest_url,
                title,
                id,
            }) if !state.is_empty() => {
                debug!(
                    "image: link type: {link_type:?}, image url: {dest_url}, image title: {title}, label: {id}",
                );
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InLink);
                state.push((
                    Where::InImageInLink,
                    link_builder.set_image(link_type, dest_url.into(), title.into(), id.into()),
                ));
            }

            ref e @ Event::End(TagEnd::Image) if !state.is_empty() => {
                debug!("{e:?}");
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InImageInLink);
                state.push((Where::InLink, link_builder));
            }
            // Text of an Image
            Event::Text(t)
                if state.last().map_or(Where::Elsewhere, |s| s.0) == Where::InImageInLink =>
            {
                debug!("Event::Text({t:?})");
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InImageInLink);
                state.push((whr, link_builder.add_image_alt_text(Cow::from(t))));
            }
            // Text of a Link
            Event::Text(t) if !state.is_empty() => {
                debug!("Event::Text({t:?})");
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InLink);
                state.push((whr, link_builder.add_text(Cow::from(t))));
            }

            Event::Code(c) if !state.is_empty() => {
                debug!("code: {c}");
                let (whr, link_builder) = state.pop().unwrap();
                if whr == Where::InImageInLink {
                    state.push((whr, link_builder.add_image_alt_text(c.into())));
                } else {
                    state.push((whr, link_builder.add_text(c.into())));
                }
            }

            // To robustly handle nested structures like bold, italics, etc, we should simply
            // ignore Start and End tags that aren't Link or Image while in state.
            Event::Start(_) | Event::End(_) if !state.is_empty() => {
                // Ignore nested formatting tags like Strong, Emphasis, etc.
                debug!("Ignoring nested formatting tag");
            }

            ref e if !state.is_empty() => {
                // Ignore other nested things, like Rule, SoftBreak, HardBreak, Html.
                // Or maybe SoftBreak/HardBreak should add a space? Text gets added normally.
                // We shouldn't panic or error out here.
                debug!("Ignored event while 'in link': {e:?}");
            }

            ref e => {
                debug!("Ignored: {e:?}");
            }
        }
    }
    assert!(state.is_empty());

    links
}

#[cfg(test)]
mod test {
    use super::*;
    use pulldown_cmark::Parser;

    #[test]
    fn test_extract_links_simple() {
        let markdown = "[text](url \"title\")";
        let mut parser = Parser::new(markdown);
        let links = extract_links(&mut parser);
        assert_eq!(links.len(), 1);
        let link = &links[0];
        assert_eq!(link.get_url(), "url");
    }

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
            "[bold text and code in link]( url \"title\" )"
        );
    }

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
}
