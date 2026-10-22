//! Functions that create parsers, and
//! extract reference definitions and links
//! from said parser
//!
//! TODO Reviewed ??-??-2026

mod extract_links;

pub(crate) use extract_links::*;
use pulldown_cmark::BrokenLink;
use pulldown_cmark::BrokenLinkCallback;
use pulldown_cmark::CowStr;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use tracing::warn;

// Public Functions

// RETURN A PARSER WITH APPROPRIATE OPTIONS

/// Return a parser with suitable options
///
/// markdown_input: the unprocessed markdown text
pub(crate) fn get_parser(markdown_input: &str) -> Parser<'_> {
    Parser::new_ext(markdown_input, get_options())
}

// Private Functions

/// Return suitable Parser options
fn get_options() -> Options {
    // Set up options and parser.
    // Strikethroughs, etc... are not part of the CommonMark standard
    // and we therefore must enable them explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TABLES);
    options
}

// BROKEN REFERENCES -----------------------------------

use std::sync::Arc;
use std::sync::Mutex;

/// A reference-style link that has no definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BrokenReference {
    /// The undefined reference label.
    pub(crate) reference: String,
    /// The kind of link, as reported by the parser (for example `Shortcut`).
    pub(crate) link_type: String,
    /// Byte offset of the link in the parsed text.
    pub(crate) offset: usize,
}

/// Handler for broken references
#[derive(Debug, Clone, Default)]
pub(crate) struct Handler {
    /// List of broken references found
    pub broken_links: Arc<Mutex<Vec<BrokenReference>>>,
}

impl Handler {
    /// Creates a handler with an empty list of broken links.
    pub(crate) fn new() -> Self {
        Self {
            broken_links: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

/// Implement the trait required by `new_with_broken_link_callback`
impl<'input> BrokenLinkCallback<'input> for Handler {
    /// In case the parser encounters any potential links that have a broken
    /// reference (e.g \[foo\] when there is no \[foo\]:  entry at the bottom)
    /// the provided callback will be called with the reference name, and
    /// the returned pair will be used as the link URL and title if it is
    /// not None.
    fn handle_broken_link(
        &mut self,
        link: BrokenLink<'input>,
    ) -> Option<(CowStr<'input>, CowStr<'input>)> {
        warn!(
            link_type = ?link.link_type,
            "Markdown contains an unresolved reference"
        );
        if let Ok(mut links) = self.broken_links.lock() {
            links.push(BrokenReference {
                offset: link.span.start,
                link_type: format!("{:?}", link.link_type),
                reference: link.reference.into_string(),
            });
        } else {
            tracing::warn!("Failed to lock broken_links");
        }
        None
    }
}

/// Return a parser with suitable options and a broken link handler.
///
/// markdown_input: the Markdown contents to parse
pub(crate) fn get_parser_with_broken_links_handler<'input>(
    markdown_input: &'input str,
    handler: Handler,
) -> Parser<'input, Handler> {
    Parser::<'input, Handler>::new_with_broken_link_callback(
        markdown_input,
        get_options(),
        Some(handler),
    )
}

// Example using `new_with_broken_link_callback` from https://github.com/raphlinus/pulldown-cmark/blob/1a5e54546b40d79eec8001d4e268b436571a78bb/pulldown-cmark/src/main.rs#L33
// fn dry_run(text: &str, opts: Options, broken_links: &mut
// Vec<BrokenLink<'static>>) {     let p =
// Parser::new_with_broken_link_callback(         text,
//         opts,
//         Some(|link: BrokenLink<'_>| {
//             broken_links.push(link.into_static());
//             None
//         }),
//     );
//     let count = p.count();
//     println!("{count} events");
// }

#[cfg(test)]
mod tests {
    use pulldown_cmark::Event;
    use pulldown_cmark::Tag;

    use super::*;

    /// The parser enables strikethrough, tables, and footnotes.
    #[test]
    fn parser_enables_strikethrough_tables_and_footnotes() {
        let md = "~~gone~~\n\n| a |\n|---|\n| b |\n\nNote[^1]\n\n[^1]: text\n";
        let events: Vec<_> = get_parser(md).collect();

        assert!(
            events
                .iter()
                .any(|e| matches!(e, Event::Start(Tag::Strikethrough)))
        );
        assert!(
            events
                .iter()
                .any(|e| matches!(e, Event::Start(Tag::Table(_))))
        );
        assert!(
            events
                .iter()
                .any(|e| matches!(e, Event::FootnoteReference(_)))
        );
    }

    /// Unresolved references are recorded with their label and link type;
    /// resolved ones are not.
    #[test]
    fn handler_records_only_unresolved_references() {
        let handler = Handler::new();
        let md = "[missing] and [ok]\n\n[ok]: https://example.com\n";
        for _ in get_parser_with_broken_links_handler(md, handler.clone()) {}

        let broken = handler
            .broken_links
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone();
        assert_eq!(
            broken,
            vec![BrokenReference {
                reference: "missing".to_string(),
                link_type: "Shortcut".to_string(),
                offset: 0,
            }]
        );
    }

    /// A new handler starts with no broken links.
    #[test]
    fn new_handler_is_empty() {
        let handler = Handler::new();

        assert!(
            handler
                .broken_links
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .is_empty()
        );
    }
}
