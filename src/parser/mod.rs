/// Functions that create parsers, and
/// extract reference definitions and links
/// from said parser
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

/// Handler for broken references
#[derive(Debug)]
pub(crate) struct Handler<'input> {
    markdown_input: &'input str,
    pub broken_links: Vec<(String, String, String)>,
}

impl<'input> Handler<'input> {
    fn new(markdown_input: &'input str) -> Self {
        let broken_links = Vec::new();
        Self {
            markdown_input,
            broken_links,
        }
    }
}

/// Implement the trait required by `new_with_broken_link_callback`
impl<'input> BrokenLinkCallback<'input> for Handler<'input> {
    /// In case the parser encounters any potential links that have a broken
    /// reference (e.g [foo] when there is no [foo]:  entry at the bottom) the
    /// provided callback will be called with the reference name, and the
    /// returned pair will be used as the link URL and title if it is not None.
    fn handle_broken_link(
        &mut self,
        link: BrokenLink<'input>,
    ) -> Option<(CowStr<'input>, CowStr<'input>)> {
        let txt: &str = self.markdown_input.get(link.span).unwrap_or("");
        warn!(
            "Issue with the markdown: reference: {}, `{}`, type: {:?}",
            link.reference, txt, link.link_type,
        );
        self.broken_links.push((
            link.reference.into_string(),
            txt.into(),
            format!("{:?}", link.link_type),
        ));
        Some(("http://TODO".into(), ":BROKEN_LINK:".into()))
        // or simply return None
    }
}

// TODO use this function
/// Return a parser with suitable options and a broken link handler.
///
/// markdown_input: the Markdown contents to parse
#[allow(dead_code)]
pub(crate) fn get_parser_with_broken_links_handler<'input>(
    markdown_input: &'input str,
) -> Parser<'_, Handler<'_>> {
    Parser::<'input, Handler<'_>>::new_with_broken_link_callback(
        markdown_input,
        get_options(),
        Some(Handler::<'_>::new(markdown_input)),
    )
    // Alternative with a closure:
    // let parser = Parser::new_with_broken_link_callback(
    //     markdown_input.as_ref(),
    //     get_options(),
    //     Some(&mut |broken_link: BrokenLink| { callback(broken_link,
    // markdown_input.as_ref()) }), )
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
//     println!("{} events", count);
// }
