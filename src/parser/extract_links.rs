//! Use a Markdown parser to extract links
use std::borrow::Cow;

use pulldown_cmark::Event;
use pulldown_cmark::Parser;
use pulldown_cmark::Tag;
use pulldown_cmark::TagEnd;
use tracing::debug;
use tracing::error;

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

    // Retrieve and group all Link-related events
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
                    "Link: link_type: {:?}, url: {}, title: {}, id: {}",
                    link_type, dest_url, title, id
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
            ref e @ Event::End(TagEnd::Link) => {
                debug!("{:?}", e);
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
                    "image: link type: {:?}, image url: {}, image title: {}, label: {}",
                    link_type, dest_url, title, id
                );
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InLink);
                state.push((
                    Where::InImageInLink,
                    link_builder.set_image(link_type, dest_url.into(), title.into(), id.into()),
                ));
            }

            ref e @ Event::End(TagEnd::Image) if !state.is_empty() => {
                debug!("{:?}", e);
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InImageInLink);
                state.push((Where::InLink, link_builder));
            }
            // Text of an Image
            Event::Text(t)
                if state.last().map_or(Where::Elsewhere, |s| s.0) == Where::InImageInLink =>
            {
                debug!("Event::Text({:?})", t);
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InImageInLink);
                state.push((whr, link_builder.add_image_alt_text(Cow::from(t))));
            }
            // Text of a Link
            Event::Text(t) if !state.is_empty() => {
                debug!("Event::Text({:?})", t);
                let (whr, link_builder) = state.pop().unwrap();
                assert_eq!(whr, Where::InLink);
                state.push((whr, link_builder.add_text(Cow::from(t))));
            }

            Event::Code(c) if !state.is_empty() => {
                debug!("code: {}", c);
                let (whr, link_builder) = state.pop().unwrap();
                state.push((whr, link_builder.add_text(c.into())));
            }

            // corner cases: Code within an Image, Link within an Image...
            ref e if !state.is_empty() => {
                error!("Unhandled event while 'in link': {:?}", e);
            }

            ref e => {
                debug!("Ignored: {:?}", e);
            }
        }
    }
    assert!(state.is_empty());

    links
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
