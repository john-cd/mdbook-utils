use std::fs::File;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;

use crate::fs;
use crate::link;
use crate::parser;
use crate::sitemap as sitemap_mod;

// SITEMAP

/// Create a sitemap.xml file from the list of Markdown files in a
/// source directory.
///
/// src_dir_path: path to the source directory.
///
/// domain: base URL e.g. <https://john-cd.com/rust_howto/>.
///
/// dest_file_path: the path to the destination file e.g.
/// book/html/sitemap.xml.
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
    // Returns an error whether the base URL is a 'cannot-be-a-base' URL,
    // meaning that parsing a relative URL string with this URL
    // as the base will return an error.
    if base_url.cannot_be_a_base() {
        bail!("Invalid URL - cannot be a base: {base_url}");
    }

    // Verify source path.
    let markdown_src_dir_path = fs::check_is_dir(markdown_src_dir_path)?;

    // Create the parent folders of the destination file, if needed.
    fs::create_parent_dir_for(sitemap_dest_file_path.as_ref())?;

    // Create the `sitemap.xml` file.
    // `File::create` will create a file if it does not exist,
    // and will truncate it if it does.
    let mut f = File::create(sitemap_dest_file_path.as_ref()).with_context(|| {
        format!(
            "Failed to create the sitemap file {}. The full directory path may not exist or required permissions may be missing.",
            sitemap_dest_file_path.as_ref().display()
        )
    })?;

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

    sitemap_mod::generate_sitemap(links, base_url, &mut f, map_index)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use url::Url;
    use std::path::PathBuf;

    #[test]
    fn test_generate_sitemap_invalid_base_url() {
        let markdown_src_dir_path = PathBuf::from("non_existent_src");
        let base_url = Url::parse("mailto:test@example.com").unwrap();
        let sitemap_dest_file_path = PathBuf::from("sitemap.xml");
        let map_index = None;

        let result = generate_sitemap(
            markdown_src_dir_path,
            base_url,
            sitemap_dest_file_path,
            map_index,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid URL - cannot be a base"));
    }
}
