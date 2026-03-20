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
mod generate;
mod link;
/// Markdown manipulation modules
pub mod markdown;
mod parser;
mod sitemap;
/// Example Markdown for testing
pub mod test_markdown;
mod write_from_parser;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
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

/// Test function that uses fake Markdown and writes events to `./book/temp/test.log`.
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

    let broken_links = handler.broken_links.lock().unwrap().clone();

    link::write_broken_links_to(broken_links, &mut f)?;

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
    //     tracing::info!("{d:?}");
    // }
    let mut new_links = generate::generate_refdefs_from(deps);

    // TODO can we read just the *-refs.md files?
    helper(
        markdown_dir_path,
        refdef_dest_file_path,
        move |parser, f| {
            // Read existing ref defs
            let existing_links: Vec<link::Link<'_>> = parser::extract_links(parser);
            let existing_links_static: Vec<link::Link<'static>> =
                existing_links.into_iter().map(|l| l.to_static()).collect();

            let links = generate::merge_links(existing_links_static, &mut new_links);
            link::write_refdefs_to(links, f)?;
            Ok(())
        },
    )?;
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
    let markdown = std::fs::read_to_string(summary_md_path.clone()).with_context(|| {
        format!(
            "[generate_sitemap] Could not read {}. Does the file exist?",
            summary_md_path.display()
        )
    })?;
    let mut parser = parser::get_parser(markdown.as_str());
    let links: Vec<link::Link<'_>> = parser::extract_links(&mut parser);

    sitemap::generate_sitemap(links, base_url, &mut f, map_index)?;

    Ok(())
}

// MARKDOWN GENERATION

/// Generate a listing of crates.io categories
/// and write to a Markdown file.
pub fn generate_categories<P1: AsRef<Path>, P2: AsRef<Path>>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()> {
    fs::create_parent_dir_for(dest_file_path.as_ref())?;
    let mut f = File::create(dest_file_path).context("Failed to create categories file.")?;
    writeln!(f, "# Categories\n")?;

    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let links = parser::extract_links(&mut parser);

    let mut categories = std::collections::BTreeSet::new();
    for l in links {
        let url = l.get_url();
        if url.contains("crates.io/categories/") {
            let mut path = url.split('?').next().unwrap_or("");
            if path.ends_with('/') {
                path = &path[..path.len() - 1];
            }
            if let Some(name) = path.split('/').next_back() {
                if !name.is_empty() && name != "categories" {
                    categories.insert(name.to_string());
                }
            }
        }
    }

    for c in categories {
        writeln!(f, "- [{c}](https://crates.io/categories/{c})")?;
    }

    Ok(())
}

/// Generate a crate index and write to a Markdown file.
pub fn generate_crates<P1: AsRef<Path>, P2: AsRef<Path>>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()> {
    fs::create_parent_dir_for(dest_file_path.as_ref())?;
    let mut f = File::create(dest_file_path).context("Failed to create crates file.")?;
    writeln!(f, "# Crates\n")?;

    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let links = parser::extract_links(&mut parser);

    let mut crates = std::collections::BTreeSet::new();
    for l in links {
        let url = l.get_url();
        if url.contains("crates.io/crates/") {
            if let Some(name) = url.split('/').next_back() {
                crates.insert(name.to_string());
            }
        }
    }

    for c in crates {
        writeln!(f, "- [{c}](https://crates.io/crates/{c})")?;
    }

    Ok(())
}

/// Identify .md files not in SUMMARY.md
// TODO: Handle nested directories more accurately in SUMMARY.md link parsing.
pub fn identify_files_not_in_summary<P: AsRef<Path>>(
    markdown_src_dir_path: P,
) -> Result<Vec<PathBuf>> {
    let markdown_src_dir_path = fs::check_is_dir(markdown_src_dir_path)?;
    let all_files = fs::find_markdown_files_in(&markdown_src_dir_path)?;

    let summary_path = markdown_src_dir_path.join("SUMMARY.md");
    if !summary_path.exists() {
        bail!(
            "SUMMARY.md not found in {}",
            markdown_src_dir_path.display()
        );
    }

    let summary_content = std::fs::read_to_string(&summary_path)?;
    let mut parser = parser::get_parser(&summary_content);
    let links = parser::extract_links(&mut parser);

    let mut files_in_summary = std::collections::HashSet::new();
    for l in links {
        let url = l.get_url();
        if !url.starts_with("http") && url.ends_with(".md") {
            // Remove any leading ./ or /
            let clean_url = url.trim_start_matches("./").trim_start_matches('/');
            let path = markdown_src_dir_path.join(clean_url);
            if let Ok(canon) = path.canonicalize() {
                files_in_summary.insert(canon);
            }
        }
    }

    let mut missing = Vec::new();
    for f in all_files {
        if let Ok(canon) = f.canonicalize() {
            if !files_in_summary.contains(&canon) && f.file_name().unwrap() != "SUMMARY.md" {
                missing.push(f);
            }
        }
    }

    Ok(missing)
}

/// Identify .rs examples not used in Markdown files
// TODO: Support other ways of including/using .rs files beyond {{#include
// ...}}.
pub fn identify_unused_rs_examples<P1: AsRef<Path>, P2: AsRef<Path>>(
    markdown_src_dir_path: P1,
    code_dir_path: P2,
) -> Result<Vec<PathBuf>> {
    let markdown_src_dir_path = fs::check_is_dir(markdown_src_dir_path)?;
    let code_dir_path = fs::check_is_dir(code_dir_path)?;

    let mut all_rs_files = Vec::new();
    for entry in walkdir::WalkDir::new(&code_dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "rs"))
    {
        all_rs_files.push(entry.path().to_path_buf().canonicalize()?);
    }

    let mut used_rs_files = std::collections::HashSet::new();
    let md_files = fs::find_markdown_files_in(&markdown_src_dir_path)?;

    let re = regex::Regex::new(
        r"\{\{#(?:rustdoc_include|playground_include|include)\s+(?P<path>\S+\.rs)\s*\}\}",
    )?;

    for md_file in md_files {
        let content = std::fs::read_to_string(&md_file)?;
        for cap in re.captures_iter(&content) {
            let rel_path = &cap["path"];
            let abs_path = md_file.parent().unwrap().join(rel_path);
            if let Ok(canon) = abs_path.canonicalize() {
                used_rs_files.insert(canon);
            }
        }
    }

    let mut unused = Vec::new();
    for f in all_rs_files {
        if !used_rs_files.contains(&f) {
            unused.push(f);
        }
    }

    Ok(unused)
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
