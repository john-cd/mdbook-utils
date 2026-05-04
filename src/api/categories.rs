use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

use crate::fs;
use crate::parser;

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
            if let Some(name) = path.split('/').next_back()
                && !name.is_empty()
                && name != "categories"
                && name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                categories.insert(name.to_string());
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
            let mut path = url.split('?').next().unwrap_or("");
            if path.ends_with('/') {
                path = &path[..path.len() - 1];
            }
            if let Some(name) = path.split('/').next_back()
                && !name.is_empty()
                && name != "crates"
                && name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            {
                crates.insert(name.to_string());
            }
        }
    }

    for c in crates {
        writeln!(f, "- [{c}](https://crates.io/crates/{c})")?;
    }

    Ok(())
}
