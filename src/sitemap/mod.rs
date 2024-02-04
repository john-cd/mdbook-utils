mod xml;

use std::io::Write;

use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use tracing::info;

/// Create a sitemap.xml file from a list of links and a base URL.
///
/// links: the list of links to Markdown files / book chapters (e.g. from
/// SUMMARY.md)
///
/// base_url: the base URL used as the prefix for HTML files.
///
/// w: a writer (e.g. a File) to write the sitemap to.
pub(crate) fn generate_sitemap<W>(
    links: Vec<crate::link::Link<'_>>,
    base_url: url::Url,
    w: &mut W,
) -> Result<()>
where
    W: Write,
{
    // Remove a few exceptions
    let exclude = ["refs.md", "SUMMARY.md"];
    let ls = links
        .into_iter()
        .filter(|l| !exclude.iter().any(|&ex| l.get_url().ends_with(ex)));
    // debug: let l = l.map(|l| { tracing::debug!("{:?}", l); l });

    // Change the extension and replace intro.html by index.html
    let ls = ls.map(|l| {
        base_url.join(
            l.get_url()
                .replace("intro.md", "index.md")
                .replace(".md", ".html")
                .as_str(),
        )
    });

    // Separate links from errors and print errors if any
    let (links, errors): (Vec<Result<_, _>>, Vec<Result<_, _>>) = ls.partition(Result::is_ok);
    let mut links: Vec<String> = links.into_iter().map(|r| r.unwrap().to_string()).collect();
    let errors: Vec<Error> = errors.into_iter().map(|r| r.unwrap_err().into()).collect();
    // debug: tracing::debug!("Links: {:?}", links);
    if !errors.is_empty() {
        tracing::error!("Errors: {:?}", errors);
    }

    // Deduplicate and sort links
    links.dedup();
    links.sort();

    // Write the sitemap
    xml::write_xml(links, w).context("[generate_sitemap] Failed to write the XML.")?;
    info!("sitemap.xml created.");
    Ok(())
}
