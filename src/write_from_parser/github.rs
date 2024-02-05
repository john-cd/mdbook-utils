//! Generate links and reference definitions
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::Parser;
use regex::Regex;
use tracing::debug;

use crate::link::write_badge_refdefs_and_links_to_two;
use crate::link::Link;
use crate::link::LinkBuilder;

// TODO
// - handle - and _ in github repo names (shields.io conventions)
// - set the image alt-text
//
// [![github][github-badge]][github]
// [github]: https://github.com/john-cd/mdbook-utils
// [github-badge]: https://img.shields.io/badge/mdbook-utils-navy?logo=github

/// Get existing reference definitions from a Markdown parser,
/// identify URLs that are GitHub repos, create badge URLs for these
/// links, and write to a writer / file.
///
/// parser: Markdown parser
///
/// w: Writer (e.g. File) to write to
pub(crate) fn write_github_repo_badge_refdefs<'input, W>(
    parser: &'input mut Parser<'input>,
    w: &mut W,
) -> Result<()>
where
    W: Write,
{
    let sorted_refdefs: BTreeMap<_, _> = parser.reference_definitions().iter().collect();

    let rule = &crate::link::GLOBAL_RULES["github repo"];
    let re = Regex::new(rule.re).unwrap();

    let mut links = Vec::new();

    // Iterate through all ref defs
    for (lbl, linkdef) in sorted_refdefs {
        // if the URL is a github repo...
        if let Some(capture) = re.captures(linkdef.dest.as_ref()) {
            debug!("dest_url: {} -> {:?}", linkdef.dest, capture);

            // ...create the URL for the badge...
            let badge_image_url: Cow<'_, str> =
                re.replace(linkdef.dest.as_ref(), rule.badge_url_pattern);
            debug!("badge_image_url: {}", badge_image_url);

            let link: Link<'input> = LinkBuilder::default()
                .set_label(Cow::from(lbl))
                .set_url(Cow::from(linkdef.dest.as_ref()))
                // .add_image_alt_text( )
                .set_image_url(badge_image_url)
                .build();
            links.push(link);
        }
    }

    // ...and write the reference definition and link to it.
    let mut refdef_buffer = Vec::new();

    write_badge_refdefs_and_links_to_two(links, w, &mut refdef_buffer)?;

    // Write reference definitions after links
    w.write_all(&refdef_buffer)?;
    Ok(())
}
