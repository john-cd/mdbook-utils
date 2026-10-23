//! Sitemap file generation.
//!
//! TODO Reviewed ??-??-2026

mod xml;

use std::collections::BTreeSet;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use percent_encoding::percent_decode_str;
use tracing::info;
use url::Url;

/// Placeholder origin used to resolve source paths without touching the real
/// base URL.
const SOURCE_ROOT_URL: &str = "https://mdbook-utils.invalid/__mdbook_source__/";

/// Return the published page URLs referenced by SUMMARY.md.
///
/// A source page is included only when the selected renderer output contains
/// its corresponding HTML file. This excludes chapters omitted by a
/// preprocessor and source fragments that are not published pages.
pub(crate) fn collect_page_urls(
    links: Vec<crate::link::Link<'_>>,
    source_root: &Path,
    rendered_root: &Path,
    base_url: Url,
    map_index: Option<(String, String)>,
) -> Result<Vec<String>> {
    let base_url = normalize_base_url(base_url)?;
    validate_index_mapping(map_index.as_ref())?;

    let source_root = source_root.canonicalize().with_context(|| {
        format!(
            "Could not resolve Markdown source directory {}",
            source_root.display()
        )
    })?;
    let rendered_root = rendered_root.canonicalize().with_context(|| {
        format!(
            "Could not resolve rendered HTML directory {}",
            rendered_root.display()
        )
    })?;
    if !rendered_root.is_dir() {
        bail!(
            "Rendered HTML output {} is not a directory",
            rendered_root.display()
        );
    }

    let mut page_urls = BTreeSet::new();
    for link in links {
        let url = link.get_url();
        let Some(source_relative_path) = source_markdown_path(url.as_ref(), &base_url)? else {
            continue;
        };

        let source_path = source_root.join(&source_relative_path);
        let source_path = crate::fs::is_path_within(&source_root, &source_path)
            .with_context(|| format!("Invalid local page in SUMMARY.md: {url}"))?;
        if !source_path.is_file() {
            bail!(
                "SUMMARY.md references a missing Markdown page: {}",
                source_path.display()
            );
        }

        let mut rendered_relative_path = source_relative_path;
        // mdBook renders `README.md` as `index.html`.
        if rendered_relative_path
            .file_name()
            .and_then(|name| name.to_str())
            == Some("README.md")
        {
            rendered_relative_path.set_file_name("index.md");
        }
        if let Some((from, to)) = &map_index
            && rendered_relative_path
                .file_name()
                .and_then(|name| name.to_str())
                == Some(from)
        {
            rendered_relative_path.set_file_name(to);
        }
        rendered_relative_path.set_extension("html");

        let rendered_path = rendered_root.join(&rendered_relative_path);
        let rendered_path = crate::fs::is_path_within(&rendered_root, &rendered_path)
            .with_context(|| format!("Invalid rendered path for SUMMARY.md entry: {url}"))?;
        if !rendered_path.is_file() {
            tracing::warn!(
                page = "local SUMMARY entry",
                rendered_path = %rendered_path.display(),
                "Skipping SUMMARY.md page absent from the selected renderer output"
            );
            continue;
        }

        page_urls.insert(hosted_page_url(&base_url, &rendered_relative_path)?);
    }

    Ok(page_urls.into_iter().collect())
}

/// Writes the page URLs as a `sitemap.xml` document.
///
/// # Errors
///
/// Returns an error if the XML cannot be written.
pub(crate) fn write_sitemap<W: std::io::Write>(page_urls: Vec<String>, w: &mut W) -> Result<()> {
    xml::write_xml(page_urls, w).context("[generate_sitemap] Failed to write the XML.")?;
    info!("sitemap.xml created.");
    Ok(())
}

/// Validates the sitemap base URL and ensures its path ends with `/`.
///
/// # Errors
///
/// Returns an error unless the URL is an HTTP(S) directory URL without query or
/// fragment.
pub(crate) fn normalize_base_url(mut base_url: Url) -> Result<Url> {
    if base_url.cannot_be_a_base() || !matches!(base_url.scheme(), "http" | "https") {
        bail!("Invalid sitemap base URL; expected an HTTP or HTTPS directory URL: {base_url}");
    }
    if base_url.query().is_some() || base_url.fragment().is_some() {
        bail!("Sitemap base URL must not contain a query or fragment: {base_url}");
    }
    if !base_url.path().ends_with('/') {
        let mut path = base_url.path().to_owned();
        path.push('/');
        base_url.set_path(&path);
    }
    Ok(base_url)
}

/// Checks that an index mapping, if any, names plain file names.
///
/// # Errors
///
/// Returns an error if a name is empty or contains path components.
fn validate_index_mapping(map_index: Option<&(String, String)>) -> Result<()> {
    if let Some((from, to)) = map_index {
        for name in [from, to] {
            let mut components = Path::new(name).components();
            if name.is_empty()
                || !matches!(components.next(), Some(Component::Normal(_)))
                || components.next().is_some()
                || name.contains(['/', '\\'])
            {
                bail!("Invalid sitemap index mapping filename: {name:?}");
            }
        }
    }
    Ok(())
}

/// Maps a link destination to the Markdown source path it points to, or `None`
/// for external or non-source links.
///
/// # Errors
///
/// Returns an error if the destination cannot be resolved against the base URL.
fn source_markdown_path(raw_url: &str, base_url: &Url) -> Result<Option<PathBuf>> {
    let source_root = Url::parse(SOURCE_ROOT_URL).expect("constant source URL is valid");
    let target = match Url::parse(raw_url) {
        Ok(target) => {
            if target.origin() != base_url.origin() || !target.path().starts_with(base_url.path()) {
                return Ok(None);
            }
            target
        }
        Err(url::ParseError::RelativeUrlWithoutBase) if raw_url.starts_with("//") => {
            return Ok(None);
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let relative_url = if let Some(path) = raw_url.strip_prefix('/') {
                path
            } else {
                raw_url
            };
            source_root
                .join(relative_url)
                .with_context(|| format!("Invalid URL in SUMMARY.md: {raw_url:?}"))?
        }
        Err(error) => {
            return Err(error).with_context(|| format!("Invalid URL in SUMMARY.md: {raw_url:?}"));
        }
    };

    let encoded_relative_path = if target.origin() == base_url.origin() {
        target.path().strip_prefix(base_url.path())
    } else {
        target.path().strip_prefix("/__mdbook_source__/")
    };
    let Some(encoded_relative_path) = encoded_relative_path else {
        bail!("Local SUMMARY.md URL escapes the book source directory: {raw_url:?}");
    };
    if !encoded_relative_path.ends_with(".md") {
        return Ok(None);
    }

    let mut relative_path = PathBuf::new();
    for encoded_segment in encoded_relative_path.split('/') {
        let decoded_segment = percent_decode_str(encoded_segment)
            .decode_utf8()
            .with_context(|| {
                format!("Invalid percent-encoded path in SUMMARY.md URL: {raw_url:?}")
            })?;
        let mut components = Path::new(decoded_segment.as_ref()).components();
        if !matches!(components.next(), Some(Component::Normal(_)))
            || components.next().is_some()
            || decoded_segment.contains(['/', '\\'])
        {
            bail!("Unsafe local page path in SUMMARY.md URL: {raw_url:?}");
        }
        relative_path.push(decoded_segment.as_ref());
    }
    Ok(Some(relative_path))
}

/// Builds the published URL of a page from the base URL and its relative path.
///
/// # Errors
///
/// Returns an error if the base URL cannot accept path segments.
fn hosted_page_url(base_url: &Url, relative_path: &Path) -> Result<String> {
    let mut url = base_url.clone();
    {
        let mut segments = url.path_segments_mut().map_err(|()| {
            anyhow::anyhow!("Sitemap base URL cannot accept page paths: {base_url}")
        })?;
        segments.pop_if_empty();
        for component in relative_path.components() {
            match component {
                Component::Normal(segment) => {
                    segments.push(segment.to_str().context("Page path is not valid UTF-8")?);
                }
                _ => bail!("Invalid rendered page path: {}", relative_path.display()),
            }
        }
    }
    Ok(url.into())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;
    use url::Url;

    use super::*;
    use crate::parser;

    /// Parses links from a Markdown string for sitemap tests.
    fn links(markdown: &str) -> Vec<crate::link::Link<'_>> {
        let mut parser = parser::get_parser(markdown);
        parser::extract_links(&mut parser)
    }

    /// URLs come from the rendered pages, are normalized and appear once.
    #[test]
    fn sitemap_urls_follow_rendered_inventory_and_are_normalized_and_deduplicated() -> Result<()> {
        let temp = tempdir()?;
        let source = temp.path().join("src");
        let custom_html = temp.path().join("custom-output").join("html");
        let other_html = temp.path().join("other-output").join("html");
        fs::create_dir_all(source.join("nested"))?;
        fs::create_dir_all(source.join("folder.md"))?;
        fs::create_dir_all(custom_html.join("nested"))?;
        fs::create_dir_all(custom_html.join("folder.md"))?;
        fs::create_dir_all(&other_html)?;

        fs::write(
            source.join("SUMMARY.md"),
            "# Summary\n\
             [Intro](intro.md)\n\
             [Nested](nested/space%20page.md#topic)\n\
             [A](a.md)\n\
             [B](b.md)\n\
             [A again](./a.md)\n\
             [Mapped name](nested/my-intro.md)\n\
             [Extension](folder.md/page.md)\n\
             [External](https://outside.example/page.md)\n\
             [Anchor only](#details)\n\
             [Unrendered](hidden.md)\n",
        )?;
        for file in ["intro.md", "a.md", "b.md", "hidden.md"] {
            fs::write(source.join(file), "# Page")?;
        }
        for file in ["space page.md", "my-intro.md"] {
            fs::write(source.join("nested").join(file), "# Page")?;
        }
        fs::write(source.join("folder.md").join("page.md"), "# Page")?;
        fs::write(source.join("shared.incl.md"), "# Shared fragment")?;
        fs::write(custom_html.join("index.html"), "published")?;
        fs::write(custom_html.join("a.html"), "published")?;
        fs::write(custom_html.join("b.html"), "published")?;
        fs::write(
            custom_html.join("nested").join("space page.html"),
            "published",
        )?;
        fs::write(
            custom_html.join("nested").join("my-intro.html"),
            "published",
        )?;
        fs::write(custom_html.join("folder.md").join("page.html"), "published")?;
        fs::write(
            custom_html.join("shared.incl.html"),
            "not a summary chapter",
        )?;
        fs::write(other_html.join("hidden.html"), "other renderer only")?;

        let summary_markdown = fs::read_to_string(source.join("SUMMARY.md"))?;
        let mut summary = parser::get_parser(&summary_markdown);
        let links = parser::extract_links(&mut summary);
        let urls = collect_page_urls(
            links,
            &source,
            &custom_html,
            Url::parse("https://example.com/rust_howto")?,
            Some(("intro.md".to_owned(), "index.md".to_owned())),
        )?;

        assert_eq!(
            urls,
            [
                "https://example.com/rust_howto/a.html",
                "https://example.com/rust_howto/b.html",
                "https://example.com/rust_howto/folder.md/page.html",
                "https://example.com/rust_howto/index.html",
                "https://example.com/rust_howto/nested/my-intro.html",
                "https://example.com/rust_howto/nested/space%20page.html",
            ]
        );
        assert!(!urls.iter().any(|url| url.contains("hidden")));
        assert!(!urls.iter().any(|url| url.contains("shared.incl")));
        Ok(())
    }

    /// Malformed URLs and pages missing from the output are reported.
    #[test]
    fn malformed_urls_and_missing_summary_pages_are_reported() -> Result<()> {
        let temp = tempdir()?;
        let source = temp.path().join("src");
        let html = temp.path().join("html");
        fs::create_dir_all(&source)?;
        fs::create_dir_all(&html)?;

        let malformed = links("[Bad](https://[::1/page.md)");
        collect_page_urls(
            malformed,
            &source,
            &html,
            Url::parse("https://example.com/book/")?,
            None,
        )
        .expect_err("input should be rejected");

        let missing = links("[Missing](missing.md)");
        collect_page_urls(
            missing,
            &source,
            &html,
            Url::parse("https://example.com/book/")?,
            None,
        )
        .expect_err("input should be rejected");
        Ok(())
    }

    /// The index mapping rewrites only a matching final path component.
    #[test]
    fn index_mapping_changes_only_the_matching_file_component() -> Result<()> {
        let temp = tempdir()?;
        let source = temp.path().join("src");
        let html = temp.path().join("html");
        fs::create_dir_all(source.join("nested"))?;
        fs::create_dir_all(html.join("nested"))?;
        fs::write(source.join("intro.md"), "# Intro")?;
        fs::write(source.join("my-intro.md"), "# Intro")?;
        fs::write(source.join("nested").join("intro.md"), "# Intro")?;
        fs::write(html.join("index.html"), "index")?;
        fs::write(html.join("my-intro.html"), "named")?;
        fs::write(html.join("nested").join("index.html"), "nested index")?;

        let urls = collect_page_urls(
            links("[Mapped](intro.md) [Similar](my-intro.md) [Nested](nested/intro.md)"),
            &source,
            &html,
            Url::parse("https://example.com/book/")?,
            Some(("intro.md".to_owned(), "index.md".to_owned())),
        )?;
        assert_eq!(
            urls,
            [
                "https://example.com/book/index.html",
                "https://example.com/book/my-intro.html",
                "https://example.com/book/nested/index.html",
            ]
        );
        Ok(())
    }

    /// `README.md` is rendered as `index.html` without any configured mapping.
    #[test]
    fn readme_is_rendered_as_index() -> Result<()> {
        let temp = tempdir()?;
        let source = temp.path().join("src");
        let html = temp.path().join("html");
        fs::create_dir_all(source.join("nested"))?;
        fs::create_dir_all(html.join("nested"))?;
        fs::write(source.join("README.md"), "# Home")?;
        fs::write(source.join("nested").join("README.md"), "# Nested")?;
        fs::write(html.join("index.html"), "index")?;
        fs::write(html.join("nested").join("index.html"), "nested index")?;

        let urls = collect_page_urls(
            links("[Home](README.md) [Nested](nested/README.md)"),
            &source,
            &html,
            Url::parse("https://example.com/book/")?,
            None,
        )?;
        assert_eq!(
            urls,
            [
                "https://example.com/book/index.html",
                "https://example.com/book/nested/index.html",
            ]
        );
        Ok(())
    }
}
