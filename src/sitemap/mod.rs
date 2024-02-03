mod xml;

use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use tracing::error;
use tracing::info;

// Create a sitemap.xml file from the  list of markdown files
// in a source directory.
pub(crate) fn generate_sitemap<P1, W>(src_dir_path: P1, base_url: url::Url, w: &mut W) -> Result<()>
where
    P1: AsRef<Path>,
    W: Write,
{
    // Locate the Markdown files
    let paths: Vec<PathBuf> = crate::fs::find_markdown_files_in(src_dir_path.as_ref())?;

    // Remove a few exceptions
    let exclude = ["refs.md", "SUMMARY.md"];
    let l = paths.into_iter().filter(|p| {
        !exclude.iter().any(|&ex| {
            p.file_name()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap_or_default()
                .ends_with(ex)
        })
    }); // p.ends_with(ex) did not work here for some reason
    // debug: let l = l.map(|path| { tracing::debug!("{:?}", path); path
    // });

    let l = l.map(|p: PathBuf| {
        p.with_extension("html")
            .strip_prefix(src_dir_path.as_ref()) // Result<&Path, _>
            .map_err(anyhow::Error::from)
            .and_then(|p| p.to_str().ok_or(anyhow!("Non UTF-8 path: {:?}", p)))
            .map(|s| format!("{base_url}{s}")) // Prefix with domain
            .map(|s| s.replace("intro.html", "index.html"))
    });

    // Separate links from errors and print errors if any
    let (links, errors): (Vec<Result<_, _>>, Vec<Result<_, _>>) = l.partition(Result::is_ok);
    let mut links: Vec<String> = links.into_iter().map(Result::unwrap).collect();
    let errors: Vec<Error> = errors.into_iter().map(Result::unwrap_err).collect();
    // debug: tracing::debug!("Links: {:?}", links);
    if !errors.is_empty() {
        error!("Errors: {:?}", errors);
    }

    links.dedup();
    links.sort();

    // Write the sitemap
    xml::write_xml(links, w).context("[generate_sitemap] Failed to write the XML.")?;
    info!("sitemap.xml created.");
    Ok(())
}
