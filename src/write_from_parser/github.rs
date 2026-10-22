//! Generate links and reference definitions for GitHub repos.
//!
//! TODO Reviewed ??-??-2026

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::Parser;
use tracing::debug;

use crate::api::BadgeStyle;
use crate::link::Link;
use crate::link::LinkBuilder;
use crate::link::write_badge_refdefs_and_links_to_two;

// [![github][github-badge]][github]
// [github]: https://github.com/john-cd/mdbook-utils
// [github-badge]: https://img.shields.io/badge/mdbook--utils-steelblue?logo=github

/// Get existing reference definitions from a Markdown parser,
/// identify URLs that are GitHub repos, create badge URLs for these
/// links, and write to a writer / file.
///
/// parser: Markdown parser.
///
/// w: writer (e.g. File) to write to.
pub(crate) fn write_github_repo_badge_refdefs<'input, W>(
    parser: &'input mut Parser<'input>,
    w: &mut W,
    style: BadgeStyle,
) -> Result<()>
where
    W: Write,
{
    let sorted_refdefs: BTreeMap<_, _> = parser.reference_definitions().iter().collect();

    let rule = &crate::link::GLOBAL_RULES["github repo"];
    let re = &crate::link::COMPILED_RULES["github repo"];

    let mut links = Vec::new();

    // Iterate through all ref defs
    for (lbl, linkdef) in sorted_refdefs {
        // if the URL is a github repo...
        if let Some(captures) = re.captures(linkdef.dest.as_ref()) {
            debug!("matched a GitHub repository reference");

            let escaped_repo = escape_shields_badge_component(&captures["repo"]);
            let mut badge_image_url = rule.badge_url_pattern.replace("${repo}", &escaped_repo);
            if let Some(style) = style.shields_query_value() {
                badge_image_url.push_str("&style=");
                badge_image_url.push_str(style);
            }

            let link: Link<'input> = LinkBuilder::default()
                .set_label(Cow::from(lbl))
                .set_url(Cow::from(linkdef.dest.as_ref()))
                .set_image_url(Cow::Owned(badge_image_url))
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

/// Escapes Shields.io path separators within a repository name.
fn escape_shields_badge_component(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        if matches!(character, '-' | '_') {
            escaped.push(character);
        }
        escaped.push(character);
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles Shields.io separator characters in repository names.
    #[test]
    fn test_escape_shields_badge_component() {
        assert_eq!(
            escape_shields_badge_component("my_repo-with-dash"),
            "my__repo--with--dash"
        );
        assert_eq!(escape_shields_badge_component("plain"), "plain");
    }
}
