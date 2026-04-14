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

pub mod api;
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
use std::sync::LazyLock;

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

/// Test function that uses fake Markdown and writes events to
/// `./book/temp/test.log`.
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
///
/// log_file_path: optional path to a log file where the output of `cargo tree`
/// will be written.
pub fn generate_refdefs_to<P1, P2, P3, P4>(
    cargo_toml_dir_path: P1,
    markdown_dir_path: P2,
    refdef_dest_file_path: P3,
    log_file_path: Option<P4>,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
    P4: AsRef<Path>,
{
    // Generate ref defs from dependencies
    let deps = dependencies::get_dependencies(&cargo_toml_dir_path, log_file_path)?;
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
                if !name.is_empty()
                    && name != "categories"
                    && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
                {
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
    use std::io::Write;

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
            let mut path = url.split('?').next().unwrap_or("");
            if path.ends_with('/') {
                path = &path[..path.len() - 1];
            }
            if let Some(name) = path.split('/').next_back() {
                if !name.is_empty()
                    && name != "crates"
                    && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
                {
                    crates.insert(name.to_string());
                }
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
            if let Ok(canon) = fs::is_path_within(&markdown_src_dir_path, &path) {
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
        match entry.path().canonicalize() {
            Ok(canon) => all_rs_files.push(canon),
            Err(e) => {
                tracing::warn!("Failed to canonicalize {:?}: {}", entry.path(), e);
                continue;
            }
        }
    }

    let mut used_rs_files = std::collections::HashSet::new();
    let md_files = fs::find_markdown_files_in(&markdown_src_dir_path)?;

    /// Regex to match Rust files in Markdown
    static RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"(?P<path>[a-zA-Z0-9_.\-\/]+\.rs)").unwrap());

    for md_file in md_files {
        let content = std::fs::read_to_string(&md_file)?;
        for cap in RE.captures_iter(&content) {
            let rel_path = Path::new(&cap["path"]);
            let abs_path = md_file.parent().unwrap().join(rel_path);
            if let Ok(canon) = fs::is_path_within(&code_dir_path, &abs_path) {
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
    use std::fs;
    use std::io::Write;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_generate_categories() -> Result<()> {
        let temp_dir = tempdir()?;
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md_file_path = src_dir.join("test.md");
        let mut md_file = File::create(&md_file_path)?;
        writeln!(md_file, "This is a test file.")?;
        writeln!(md_file, "[parsing](https://crates.io/categories/parsing)")?;
        writeln!(
            md_file,
            "Another category: [development-tools](https://crates.io/categories/development-tools/)"
        )?;
        writeln!(
            md_file,
            "Duplicate category: [parsing](https://crates.io/categories/parsing?sort=recent-updates)"
        )?;
        writeln!(md_file, "A crate: [serde](https://crates.io/crates/serde)")?;

        let dest_file_path = temp_dir.path().join("categories.md");

        generate_categories(&src_dir, &dest_file_path)?;

        let content = fs::read_to_string(&dest_file_path)?;

        assert!(content.contains("# Categories"));
        assert!(
            content
                .contains("- [development-tools](https://crates.io/categories/development-tools)")
        );
        assert!(content.contains("- [parsing](https://crates.io/categories/parsing)"));
        assert!(!content.contains("- [serde]"));

        // Ensure they are sorted and deduplicated
        let lines: Vec<&str> = content.lines().filter(|l| l.starts_with("-")).collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            "- [development-tools](https://crates.io/categories/development-tools)"
        );
        assert_eq!(
            lines[1],
            "- [parsing](https://crates.io/categories/parsing)"
        );

        Ok(())
    }

    #[test]
    fn test_generate_crates() -> Result<()> {
        let temp_dir = tempdir()?;
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md_file_path = src_dir.join("test.md");
        let mut md_file = File::create(&md_file_path)?;
        writeln!(md_file, "This is a test file.")?;
        writeln!(md_file, "[serde](https://crates.io/crates/serde)")?;
        writeln!(
            md_file,
            "Another crate: [anyhow](https://crates.io/crates/anyhow)"
        )?;
        writeln!(
            md_file,
            "Duplicate crate: [serde](https://crates.io/crates/serde)"
        )?;
        writeln!(
            md_file,
            "A category: [parsing](https://crates.io/categories/parsing)"
        )?;

        let dest_file_path = temp_dir.path().join("crates.md");

        generate_crates(&src_dir, &dest_file_path)?;

        let content = fs::read_to_string(&dest_file_path)?;

        assert!(content.contains("# Crates"));
        assert!(content.contains("- [anyhow](https://crates.io/crates/anyhow)"));
        assert!(content.contains("- [serde](https://crates.io/crates/serde)"));
        assert!(!content.contains("- [parsing]"));

        // Ensure they are sorted and deduplicated
        let lines: Vec<&str> = content.lines().filter(|l| l.starts_with("-")).collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "- [anyhow](https://crates.io/crates/anyhow)");
        assert_eq!(lines[1], "- [serde](https://crates.io/crates/serde)");
        Ok(())
    }

    #[test]
    fn test_generate_categories_injection() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md1 = src_dir.join("1.md");
        fs::write(
            &md1,
            "Malicious [link](https://crates.io/categories/cat1\\\");alert(1);(\\\"\\\")",
        )?;

        let dest_file = dir.path().join("categories.md");
        generate_categories(&src_dir, &dest_file)?;

        let content = fs::read_to_string(&dest_file)?;
        let expected = "# Categories\n\n";
        assert_eq!(content, expected);

        Ok(())
    }

    #[test]
    fn test_generate_crates_injection() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md1 = src_dir.join("1.md");
        fs::write(
            &md1,
            "Malicious [link](https://crates.io/crates/crate1\\\");alert(1);(\\\"\\\")",
        )?;

        let dest_file = dir.path().join("crates.md");
        generate_crates(&src_dir, &dest_file)?;

        let content = std::fs::read_to_string(&dest_file)?;
        let expected = "# Crates\n\n";
        assert_eq!(content, expected);

        Ok(())
    }


    #[test]
    fn test_identify_unused_rs_examples() {
        let dir = tempdir().unwrap();
        let markdown_dir = dir.path().join("markdown");
        let code_dir = dir.path().join("code");

        fs::create_dir(&markdown_dir).unwrap();
        fs::create_dir(&code_dir).unwrap();

        // Create Rust files
        let used1_rs = code_dir.join("used1.rs");
        let used2_rs = code_dir.join("used2.rs");
        let unused_rs = code_dir.join("unused.rs");
        fs::write(&used1_rs, "fn main() {}").unwrap();
        fs::write(&used2_rs, "fn main() {}").unwrap();
        fs::write(&unused_rs, "fn main() {}").unwrap();

        // Create Markdown file using some of the Rust files
        let md_file = markdown_dir.join("test.md");
        let mut md = fs::File::create(&md_file).unwrap();
        writeln!(md, "Some text").unwrap();
        writeln!(md, "{{{{#include ../code/used1.rs}}}}").unwrap();
        writeln!(md, "{{{{#rustdoc_include ../code/used2.rs}}}}").unwrap();

        // Call the function
        let mut unused_files = identify_unused_rs_examples(&markdown_dir, &code_dir).unwrap();
        unused_files.sort();

        // Only unused.rs should be identified as unused
        let expected = unused_rs.canonicalize().unwrap();
        assert_eq!(unused_files.len(), 1);
        assert_eq!(unused_files[0], expected);
    }

    #[test]
    fn test_identify_unused_rs_examples_invalid_dir() {
        let dir = tempdir().unwrap();
        let markdown_dir = dir.path().join("markdown"); // Does not exist
        let code_dir = dir.path().join("code"); // Does not exist

        // Should return an error because the directories don't exist
        let result = identify_unused_rs_examples(&markdown_dir, &code_dir);
        assert!(result.is_err());
    }

    #[test]
    fn test_identify_files_not_in_summary_all_included() {
        let dir = tempdir().unwrap();
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root).unwrap();

        let summary_path = root.join("SUMMARY.md");
        fs::write(&summary_path, "[Page 1](./page1.md)\n[Page 2](page2.md)").unwrap();

        fs::write(root.join("page1.md"), "# Page 1").unwrap();
        fs::write(root.join("page2.md"), "# Page 2").unwrap();

        let missing = identify_files_not_in_summary(&root).unwrap();
        assert!(
            missing.is_empty(),
            "Expected no missing files, but got {:?}",
            missing
        );
    }

    #[test]
    fn test_identify_files_not_in_summary_some_missing() {
        let dir = tempdir().unwrap();
        let markdown_src_dir_path = dir.path();

        // Let's create a non-hidden sub directory, and use that as the root,
        // since `tempdir` returns a hidden directory like `/tmp/.tmpxxx`.
        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root).unwrap();

        let summary_path = root.join("SUMMARY.md");
        fs::write(&summary_path, "[Page 1](./page1.md)").unwrap();

        fs::write(root.join("page1.md"), "# Page 1").unwrap();
        fs::write(root.join("page2.md"), "# Page 2").unwrap();

        let missing = identify_files_not_in_summary(&root).unwrap();
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].file_name().unwrap(), "page2.md");
    }

    #[test]
    fn test_identify_files_not_in_summary_no_summary() {
        let dir = tempdir().unwrap();
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root).unwrap();

        fs::write(root.join("page1.md"), "# Page 1").unwrap();

        let result = identify_files_not_in_summary(&root);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("SUMMARY.md not found in")
        );
    }

    #[test]
    fn test_identify_files_not_in_summary_nested_files() {
        let dir = tempdir().unwrap();
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root).unwrap();

        let sub_dir = root.join("sub");
        fs::create_dir(&sub_dir).unwrap();

        let summary_path = root.join("SUMMARY.md");
        fs::write(
            &summary_path,
            "[Page 1](./page1.md)\n[Sub Page](sub/page2.md)",
        )
        .unwrap();

        fs::write(root.join("page1.md"), "# Page 1").unwrap();
        fs::write(sub_dir.join("page2.md"), "# Page 2").unwrap();
        fs::write(sub_dir.join("page3.md"), "# Page 3").unwrap(); // missing

        let missing = identify_files_not_in_summary(&root).unwrap();
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].file_name().unwrap(), "page3.md");
    }

    #[test]
    fn test_generate_categories_happy_path() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md1 = src_dir.join("1.md");
        fs::write(
            &md1,
            "Here is [category one](https://crates.io/categories/cat1) and [another](https://crates.io/categories/cat2?sort=recent) and [trailing slash](https://crates.io/categories/cat3/).",
        )?;

        let md2 = src_dir.join("2.md");
        fs::write(
            &md2,
            "Duplicate [cat1](https://crates.io/categories/cat1), and an unrelated [link](https://example.com).",
        )?;

        let dest_file = dir.path().join("categories.md");
        generate_categories(&src_dir, &dest_file)?;

        let content = fs::read_to_string(&dest_file)?;
        let expected = "# Categories\n\n- [cat1](https://crates.io/categories/cat1)\n- [cat2](https://crates.io/categories/cat2)\n- [cat3](https://crates.io/categories/cat3)\n";
        assert_eq!(content, expected);

        Ok(())
    }

    #[test]
    fn test_generate_categories_invalid_dir() {
        let dir = tempdir().unwrap();
        let src_dir = dir.path().join("non_existent_src");
        let dest_file = dir.path().join("categories.md");

        let result = generate_categories(&src_dir, &dest_file);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_categories_edge_cases() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md1 = src_dir.join("1.md");
        fs::write(
            &md1,
            "Here is [empty 1](https://crates.io/categories) and [empty 2](https://crates.io/categories/).",
        )?;

        let dest_file = dir.path().join("categories.md");
        generate_categories(&src_dir, &dest_file)?;

        let content = fs::read_to_string(&dest_file)?;
        let expected = "# Categories\n\n";
        assert_eq!(content, expected);

        Ok(())
    }
}
