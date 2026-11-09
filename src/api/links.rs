//! Public operations that write reports about the links in Markdown sources.
//!
//! TODO Reviewed ??-??-2026.

use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;
use pulldown_cmark::LinkType;

use crate::fs;
use crate::link;
pub use crate::link::LinkFormat;
use crate::parser;

// LINKS

/// Render external inline links and autolinks from Markdown sources.
///
/// Internal links (relative paths, fragments, and non-HTTP schemes) are
/// excluded. `format` selects the Markdown syntax for the output.
///
/// The caller chooses where and how to write the returned Markdown.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the Markdown
/// output cannot be rendered.
pub fn render_inline_links<P: AsRef<Path>>(src_dir_path: P, format: LinkFormat) -> Result<String> {
    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let links: Vec<link::Link<'_>> = parser::extract_links(&mut parser);
    let links: Vec<_> = links
        .into_iter()
        .filter(|link| {
            matches!(
                link.get_link_type(),
                Some(LinkType::Inline | LinkType::Autolink)
            )
        })
        .filter(|link| is_external_url(&link.get_url()))
        .collect();

    let mut output = Vec::new();
    link::write_links_in_format_to(links, format, &mut output)?;
    String::from_utf8(output).context("rendered links were not valid UTF-8")
}

/// Parse Markdown sources and atomically write the rendered external inline
/// links and autolinks to a file.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the destination
/// file cannot be created or written.
pub fn write_inline_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    write_inline_links_with_format(src_dir_path, dest_file_path, LinkFormat::Reference)
}

/// Same as [`render_inline_links`], but atomically writes the result to a file.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read, the Markdown
/// output cannot be rendered, or the destination file cannot be written.
pub fn write_inline_links_with_format<P1, P2>(
    src_dir_path: P1,
    dest_file_path: P2,
    format: LinkFormat,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let output = render_inline_links(src_dir_path, format)?;
    fs::write_atomically(dest_file_path.as_ref(), output.as_bytes())
}

/// Returns true if the destination is an absolute `http` or `https` URL,
/// i.e. a link to an external site rather than a relative path, fragment,
/// or other scheme.
fn is_external_url(url: &str) -> bool {
    let lower = url.trim().to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

/// Render HTTP and HTTPS links found in all Markdown files in a source directory.
///
/// The caller chooses where and how to write the returned Markdown.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the Markdown
/// output cannot be rendered.
pub fn render_all_links<P: AsRef<Path>>(src_dir_path: P) -> Result<String> {
    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let links: Vec<link::Link<'_>> = parser::extract_links(&mut parser);
    let links: Vec<_> = links
        .into_iter()
        .filter(|link| link.get_url().starts_with("http"))
        .collect();

    let mut output = Vec::new();
    link::write_reference_style_links_to(links, &mut output)?;
    String::from_utf8(output).context("rendered links were not valid UTF-8")
}

/// Parse Markdown sources and atomically write the rendered links to a file.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the destination
/// file cannot be created or written.
pub fn write_all_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let output = render_all_links(src_dir_path)?;
    fs::write_atomically(dest_file_path.as_ref(), output.as_bytes())
}

/// Render duplicate links found in all Markdown files in a source directory.
///
/// Two links are duplicates when they have the same label, URL and title.
/// Inline links and autolinks have no label, so they are duplicates when they
/// point to the same URL, whatever their text.
///
/// The caller chooses where and how to write the returned Markdown.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the Markdown
/// output cannot be rendered.
pub fn render_duplicate_links<P: AsRef<Path>>(src_dir_path: P) -> Result<String> {
    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let links: Vec<link::Link<'_>> = parser::extract_links(&mut parser);
    let links: Vec<_> = links
        .into_iter()
        .filter(|link| link.get_url().starts_with("http"))
        .collect();
    let mut counts = std::collections::HashMap::new();
    for link in &links {
        *counts.entry(link.clone()).or_insert(0) += 1;
    }
    let duplicates: Vec<_> = links.into_iter().filter(|link| counts[link] > 1).collect();

    let mut output = Vec::new();
    link::write_duplicate_links_to(&duplicates, &mut output)?;
    String::from_utf8(output).context("rendered duplicate links were not valid UTF-8")
}

/// Parse Markdown sources and atomically write duplicate links to a file.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the destination
/// file cannot be created or written.
pub fn write_duplicate_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let output = render_duplicate_links(src_dir_path)?;
    fs::write_atomically(dest_file_path.as_ref(), output.as_bytes())
}

/// Render broken reference links found in all Markdown files in a source directory.
///
/// The caller chooses where and how to write the returned Markdown.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the Markdown
/// output cannot be rendered.
pub fn render_broken_links<P: AsRef<Path>>(src_dir_path: P) -> Result<String> {
    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let sources = fs::read_markdown_sources_in(src_dir_path)?;
    let handler = parser::Handler::new();
    let parser = parser::get_parser_with_broken_links_handler(&sources.text, handler.clone());

    for _ in parser {}

    let broken_links = handler
        .broken_links
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .iter()
        .map(|broken| {
            let location = sources
                .locate(broken.offset)
                .map(|(path, line)| format!("{}:{line}", path.display()))
                .unwrap_or_default();
            (broken.reference.clone(), location, broken.link_type.clone())
        })
        .collect();

    let mut buffer = Vec::new();
    link::write_broken_links_to(broken_links, &mut buffer)?;
    String::from_utf8(buffer).context("rendered broken links were not valid UTF-8")
}

/// Parse Markdown sources and atomically write broken links to a file.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the destination
/// file cannot be created or written.
pub fn write_broken_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let output = render_broken_links(src_dir_path)?;
    fs::write_atomically(dest_file_path.as_ref(), output.as_bytes())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    /// Reports a reference link that has no definition.
    #[test]
    fn test_write_broken_links() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md_file_path = src_dir.join("test.md");
        fs::write(
            &md_file_path,
            "Here is a valid link: [valid](http://example.com)\n\nAnd here is a broken one: [broken reference]\n",
        )?;

        let dest_file_path = temp_dir.path().join("broken_links.md");

        write_broken_links(&src_dir, &dest_file_path)?;

        assert!(dest_file_path.exists());
        let content = fs::read_to_string(&dest_file_path)?;

        assert!(content.contains("# Broken Links"));
        assert!(content.contains("- Reference: broken reference"));
        assert!(content.contains("test.md:3"), "{content}");
        assert!(
            content.contains("Type: Shortcut"),
            "Unexpected report:\n{content}"
        );

        Ok(())
    }

    /// Writes a fixture with external, internal, fragment and reference links.
    fn write_inline_fixture(src_dir: &std::path::Path) -> Result<()> {
        fs::create_dir(src_dir)?;
        fs::write(
            src_dir.join("page.md"),
            "[ext](https://example.com/a \"T\") <https://example.org/b>\n\n\
             [rel](other.md) [frag](#section) [up](../x.md#y) [mail](mailto:a@b.c)\n\n\
             [ref][r]\n\n[r]: https://example.net/\n",
        )?;
        Ok(())
    }

    /// Internal links and reference links are excluded from inline output.
    #[test]
    fn test_write_inline_links_excludes_internal_links() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let src_dir = temp_dir.path().join("src");
        write_inline_fixture(&src_dir)?;
        let dest = temp_dir.path().join("inline.md");
        write_inline_links(&src_dir, &dest)?;
        let content = fs::read_to_string(&dest)?;
        assert!(content.contains("https://example.com/a"));
        assert!(content.contains("https://example.org/b"));
        for excluded in ["other.md", "#section", "../x.md", "mailto:", "example.net"] {
            assert!(!content.contains(excluded), "{excluded} in:\n{content}");
        }
        Ok(())
    }

    /// The inline format writes `[text](url)` lines only.
    #[test]
    fn test_write_inline_links_inline_format() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let src_dir = temp_dir.path().join("src");
        write_inline_fixture(&src_dir)?;
        let dest = temp_dir.path().join("inline.md");
        write_inline_links_with_format(&src_dir, &dest, LinkFormat::Inline)?;
        let content = fs::read_to_string(&dest)?;
        assert!(
            content.contains("[ext](https://example.com/a \"T\")"),
            "{content}"
        );
        assert!(!content.contains("]: "), "{content}");
        Ok(())
    }

    /// The autolink format writes `<url>` lines only.
    #[test]
    fn test_write_inline_links_autolink_format() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let src_dir = temp_dir.path().join("src");
        write_inline_fixture(&src_dir)?;
        let dest = temp_dir.path().join("inline.md");
        write_inline_links_with_format(&src_dir, &dest, LinkFormat::Autolink)?;
        let content = fs::read_to_string(&dest)?;
        assert!(content.contains("<https://example.com/a>"), "{content}");
        assert!(content.contains("<https://example.org/b>"), "{content}");
        assert!(!content.contains("other.md"), "{content}");
        Ok(())
    }

    /// A failing run keeps an existing destination file.
    #[test]
    fn test_failed_run_keeps_existing_destination() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let dest = temp_dir.path().join("out.md");
        fs::write(&dest, "previous")?;
        let missing = temp_dir.path().join("missing");

        assert!(write_all_links(&missing, &dest).is_err());
        assert!(write_broken_links(&missing, &dest).is_err());
        assert_eq!(fs::read_to_string(&dest)?, "previous");
        Ok(())
    }

    /// Reports URLs that appear in several links.
    #[test]
    fn test_write_duplicate_links() -> Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md_file_path1 = src_dir.join("test1.md");
        fs::write(
            &md_file_path1,
            "Link 1: [example1](http://example.com/1)\nLink 2: [example2](http://example.com/2)\n",
        )?;

        let md_file_path2 = src_dir.join("test2.md");
        fs::write(
            &md_file_path2,
            "Link 1 again: [example1](http://example.com/1)\nLink 3: [example3](http://example.com/3)\n",
        )?;

        let dest_file_path = temp_dir.path().join("duplicate_links.md");

        write_duplicate_links(&src_dir, &dest_file_path)?;

        assert!(dest_file_path.exists());
        let content = fs::read_to_string(&dest_file_path)?;

        assert!(content.contains("# Duplicate Links"));

        let occurrences = content.matches("http://example.com/1").count();
        assert_eq!(occurrences, 2);

        assert!(!content.contains("http://example.com/2"));
        assert!(!content.contains("http://example.com/3"));

        Ok(())
    }
}
