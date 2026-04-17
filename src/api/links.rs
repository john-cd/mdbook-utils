use std::fs::File;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use pulldown_cmark::LinkType;

use crate::fs;
use crate::helper;
use crate::link;
use crate::parser;

// LINKS

/// Parse Markdown from all .md files in a given source directory,
/// write all inline links and autolinks (i.e., not written as
/// reference-style links) found therein to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
pub fn write_inline_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(src_dir_path, dest_file_path, |parser, f| {
        let links: Vec<link::Link<'_>> = parser::extract_links(parser);
        let links: Vec<_> = links
            .into_iter()
            .filter(|l| {
                [LinkType::Inline, LinkType::Autolink]
                    .iter()
                    .any(|&x| l.get_link_type().unwrap() == x)
            })
            .filter(|l| {
                let url = l.get_url();
                url.starts_with("http")
            })
            .collect();
        link::write_reference_style_links_to(links, f)?;
        Ok(())
    })?;

    Ok(())
}

/// Parse Markdown from all .md files in a given source directory,
/// write all links found therein to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
pub fn write_all_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(src_dir_path, dest_file_path, |parser, f| {
        let links: Vec<link::Link<'_>> = parser::extract_links(parser);
        let links: Vec<_> = links
            .into_iter()
            .filter(|l| {
                let url = l.get_url();
                url.starts_with("http")
            })
            .collect();
        link::write_reference_style_links_to(links, f)?;
        Ok(())
    })?;

    Ok(())
}

/// Parse Markdown from all .md files in a given source directory,
/// write duplicated links found therein to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
pub fn write_duplicate_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(src_dir_path, dest_file_path, |parser, f| {
        let links: Vec<link::Link<'_>> = parser::extract_links(parser);
        let links: Vec<_> = links
            .into_iter()
            .filter(|l| {
                let url = l.get_url();
                url.starts_with("http")
            })
            .collect();
        let mut counts = std::collections::HashMap::new();
        for l in &links {
            *counts.entry(l.clone()).or_insert(0) += 1;
        }
        let duplicates: Vec<_> = links.into_iter().filter(|l| counts[l] > 1).collect();
        link::write_duplicate_links_to(duplicates, f)?;
        Ok(())
    })?;

    Ok(())
}

/// Parse Markdown from all .md files in a given source directory,
/// write broken links found therein to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
pub fn write_broken_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    fs::create_parent_dir_for(dest_file_path.as_ref())?;

    let mut f = File::create(dest_file_path.as_ref()).with_context(|| {
        format!(
            "[write_broken_links] Could not create file {}",
            dest_file_path.as_ref().display()
        )
    })?;

    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let handler = parser::Handler::new();
    let parser = parser::get_parser_with_broken_links_handler(&all_markdown, handler.clone());

    // We need to consume the parser to trigger the callbacks
    for _ in parser {}

    let broken_links = handler.broken_links.lock().unwrap_or_else(|e| e.into_inner()).clone();

    link::write_broken_links_to(broken_links, &mut f)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

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
        assert!(content.contains("Type: ShortcutUnknown"));

        Ok(())
    }
}
