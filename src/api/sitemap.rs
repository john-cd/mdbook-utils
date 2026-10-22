//! Public operation that generates a `sitemap.xml` for a book.
//!
//! TODO Reviewed ??-??-2026

use std::fs::File;
use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;

use crate::fs;
use crate::link;
use crate::parser;
use crate::sitemap as sitemap_mod;

/// Create a sitemap from published HTML pages linked in SUMMARY.md.
///
/// The parent directory of `sitemap_dest_file_path` must be the existing output
/// directory for the selected HTML renderer, such as book/html.
///
/// Pages are included only when their Markdown source exists and the matching
/// HTML file is present in that renderer output. Unrendered chapters and
/// shared source fragments are omitted. URLs in `base_url` are rooted at the
/// hosted book directory, for example <https://john-cd.com/rust_howto/>.
///
/// # Errors
///
/// Returns an error if `base_url` is invalid for a sitemap, the source or
/// renderer directory is unavailable, `SUMMARY.md` cannot be read, or the
/// destination file cannot be created or written.
pub fn generate_sitemap<P1, P2>(
    markdown_src_dir_path: P1,
    base_url: url::Url,
    sitemap_dest_file_path: P2,
    map_index: Option<(String, String)>,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let base_url = sitemap_mod::normalize_base_url(base_url)?;
    let markdown_src_dir_path = fs::check_is_dir(markdown_src_dir_path)?;
    let sitemap_dest_file_path = sitemap_dest_file_path.as_ref();
    let rendered_root = sitemap_dest_file_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let rendered_root = fs::check_is_dir(rendered_root)
        .context("The sitemap destination must be inside an existing renderer output directory")?;

    let summary_md_path = markdown_src_dir_path.join("SUMMARY.md");
    tracing::debug!("SUMMARY.md path: {}", summary_md_path.display());
    let markdown = std::fs::read_to_string(&summary_md_path).with_context(|| {
        format!(
            "[generate_sitemap] Could not read {}. Does the file exist?",
            summary_md_path.display()
        )
    })?;
    let mut parser = parser::get_parser(markdown.as_str());
    let links: Vec<link::Link<'_>> = parser::extract_links(&mut parser);
    let page_urls = sitemap_mod::collect_page_urls(
        links,
        &markdown_src_dir_path,
        &rendered_root,
        base_url,
        map_index,
    )?;

    let mut f = File::create(sitemap_dest_file_path).with_context(|| {
        format!(
            "Failed to create the sitemap file {}. The full directory path may not exist or required permissions may be missing.",
            sitemap_dest_file_path.display()
        )
    })?;
    sitemap_mod::write_sitemap(page_urls, &mut f)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tempfile::tempdir;
    use url::Url;

    use super::*;

    /// Rejects a base URL that cannot be a base, such as `mailto:`.
    #[test]
    fn test_generate_sitemap_invalid_base_url() {
        let markdown_src_dir_path = PathBuf::from("non_existent_src");
        let base_url = Url::parse("mailto:test@example.com").expect("input should parse");
        let sitemap_dest_file_path = PathBuf::from("sitemap.xml");

        let result = generate_sitemap(
            markdown_src_dir_path,
            base_url,
            sitemap_dest_file_path,
            None,
        );

        assert!(result.is_err());
        assert!(
            result
                .expect_err("input should be rejected")
                .to_string()
                .contains("Invalid sitemap base URL")
        );
    }

    /// Reads pages from the configured renderer's output rather than the
    /// default `html` directory.
    #[test]
    fn sitemap_uses_the_selected_custom_renderer_directory() -> Result<()> {
        let temp = tempdir()?;
        let source = temp.path().join("src");
        let custom_html = temp.path().join("custom-build").join("rendered-html");
        let other_html = temp.path().join("default-build").join("html");
        fs::create_dir_all(&source)?;
        fs::create_dir_all(&custom_html)?;
        fs::create_dir_all(&other_html)?;
        fs::write(
            source.join("SUMMARY.md"),
            "[Home](index.md) [Missing](missing.md)",
        )?;
        fs::write(source.join("index.md"), "# Home")?;
        fs::write(source.join("missing.md"), "# Missing")?;
        fs::write(custom_html.join("index.html"), "custom build")?;
        fs::write(other_html.join("missing.html"), "other renderer")?;

        let sitemap_path = custom_html.join("sitemap.xml");
        generate_sitemap(
            &source,
            Url::parse("https://example.com/book")?,
            &sitemap_path,
            None,
        )?;

        let sitemap = fs::read_to_string(sitemap_path)?;
        assert!(sitemap.contains("<loc>https://example.com/book/index.html</loc>"));
        assert!(!sitemap.contains("missing.html"));
        Ok(())
    }
}
