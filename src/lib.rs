//! # mdbook-utils
//!
//! For installation and usage instructions for the `mdbook-utils` command-line
//! tool, consult the [User Guide](https://john-cd.com/mdbook-utils/).
//!
//! A list of available commands is displayed when entering `mdbook-utils` at a
//! shell prompt.
//!
//! The following (<https://docs.rs/mdbook-utils/>) contains the **library API** doc.
//! Please consult the [Public API](https://john-cd.com/mdbook-utils/public_api.html) page as well.
//!
//! You will want to use the Public API (over the CLI) to:
//!
//! - Integrate it in your project, for example call it from a `build.rs` build
//!   script,
//! - Extend its capabilities,
//! - ...

#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
// #![doc(html_favicon_url = "https://example.com/favicon.ico")]
// #![doc(html_logo_url = "https://example.com/logo.jpg")]

mod build_book;
mod dependencies;
mod fs;
mod gen;
mod link;
pub mod markdown;
mod parser;
mod sitemap;
pub mod test_markdown;
mod write_from_parser;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use pulldown_cmark::LinkType;
use pulldown_cmark::Parser;

/// Helper function:
///
/// Checks if the source directory exists,
/// create the destination directory if it doesn't exist,
/// create the destination file,
/// parse all the Markdown files in the source directory,
/// and invoke a closure that uses the parser to write to the file.
fn helper<P1, P2, F>(src_dir_path: P1, dest_file_path: P2, func: F) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    F: for<'a, 'b> FnOnce(&'a mut Parser<'a>, &'b mut File) -> Result<()>,
{
    let src_dir_path = fs::check_is_dir(src_dir_path)?;

    fs::create_parent_dir_for(dest_file_path.as_ref())?;

    let mut f = File::create(dest_file_path.as_ref()).with_context(|| {
        format!(
            "[helper] Could not create file {}",
            dest_file_path.as_ref().display()
        )
    })?;

    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());

    func(&mut parser, &mut f)?;
    Ok(())
}

// Public Functions

// DEBUG

/// Parse Markdown from all .md files in a given source directory and
/// write all raw events to a file for debugging purposes.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
pub fn debug_parse_to<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_raw_to,
    )?;
    Ok(())
}

/// Test function that uses fake Markdown.
pub fn test() -> Result<()> {
    fs::create_dir("./book/temp/")?;

    let dest_file_path = "./book/temp/test.log";
    let mut f = BufWriter::new(File::create(dest_file_path).context(
        "[test] Failed to create the destination file. Does the full directory path exist?",
    )?);

    let test_markdown = test_markdown::get_test_markdown();
    let mut parser = parser::get_parser(test_markdown.as_ref());
    write_from_parser::write_raw_to(&mut parser, &mut f)?;
    f.flush()
        .context("Not all bytes could be written due to I/O errors or EOF being reached.")?;
    Ok(())
}

// REFERENCE DEFINITIONS

/// Parse Markdown from all .md files in a given source directory
/// and write reference definitions found therein to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
pub fn write_refdefs_to<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_refdefs_to,
    )?;
    Ok(())
}

/// Parse Markdown from all .md files in a given source directory,
/// extract existing reference definitions,
/// identify URLs that are GitHub repos,
/// create badge URLs for these links,
/// and write to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
pub fn generate_badges<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_github_repo_badge_refdefs,
    )?;
    Ok(())
}

// LINKS

// TODO need to remove internal links

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
    helper(src_dir_path, dest_file_path, |parser, _f| {
        let _links: Vec<link::Link<'_>> = parser::extract_links(parser);
        // TODO ::write_duplicate_links_to(links, f)?;
        println!("NOT IMPLEMENTED!");
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
pub fn write_broken_links<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(src_dir_path, dest_file_path, |parser, _f| {
        let _links: Vec<link::Link<'_>> = parser::extract_links(parser);
        // TODO ::write_broken_links_to(links, f)?;
        println!("NOT IMPLEMENTED!");
        Ok(())
    })?;

    Ok(())
}

// GENERATE REF DEFS FROM DEPENDENCIES

/// Given a Cargo.toml path,
/// generate reference definitions from code dependencies
/// and write them to a file.
///
/// cargo_toml_dir_path: path to the directory containing `Cargo.toml`.
///
/// markdown_dir_path: path to the directory containing Markdown files.
///
/// refdef_dest_file_path: path to the file to create and
/// write into.
pub fn generate_refdefs_to<P1, P2, P3>(
    cargo_toml_dir_path: P1,
    markdown_dir_path: P2,
    refdef_dest_file_path: P3,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
{
    // Generate ref defs from dependencies
    let deps = dependencies::get_dependencies(&cargo_toml_dir_path)?;
    // for (_, d) in &deps {
    //     tracing::info!("{:?}", d);
    // }
    let mut new_links = gen::generate_refdefs_from(deps);

    // TODO can we read just the *-refs.md files?
    helper(markdown_dir_path, refdef_dest_file_path, |parser, f| {
        // Read existing ref defs
        let _sorted_linkdefs: std::collections::BTreeMap<_, _> =
            parser.reference_definitions().iter().collect();
        // TODO
        println!("NOT IMPLEMENTED!");
        let existing_links = Vec::new();

        let links = gen::merge_links(existing_links, &mut new_links);
        link::write_refdefs_to(links, f)?;
        Ok(())
    })?;
    Ok(())
}

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
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    // Returns an error whether the base URL is a cannot-be-a-base URL,
    // meaning that parsing a relative URL string with this URL
    // as the base will return an error.
    if base_url.cannot_be_a_base() {
        bail!("Invalid URL - cannot be a base: {}", base_url);
    }

    // Verify source path
    let markdown_src_dir_path = fs::check_is_dir(markdown_src_dir_path)?;

    // Create the parent folders of the destination file, if needed
    fs::create_parent_dir_for(sitemap_dest_file_path.as_ref())?;

    // Create the `sitemap.xml` file
    // File::create will create a file if it does not exist,
    // and will truncate it if it does.
    let mut f = File::create(sitemap_dest_file_path.as_ref()).with_context(|| {
        format!(
            "Failed to create the sitemap file {}. The full directory path may not exist or required permissions may be missing.",
            sitemap_dest_file_path.as_ref().display()
        )
    })?;

    let summary_md_path = markdown_src_dir_path.join("SUMMARY.md");
    tracing::debug!("SUMMARY.md path: {}", summary_md_path.display());
    let markdown = std::fs::read_to_string(summary_md_path.clone()).with_context(|| {
        format!(
            "[generate_sitemap] Could not read {}. Does the file exist?",
            summary_md_path.display()
        )
    })?;
    let mut parser = parser::get_parser(markdown.as_str());
    let links: Vec<link::Link<'_>> = parser::extract_links(&mut parser);

    sitemap::generate_sitemap(links, base_url, &mut f)?;

    Ok(())
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
