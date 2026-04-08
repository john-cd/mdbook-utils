//! Generate index files for crates and categories
//! in Markdown format.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

use crate::fs;
use crate::parser;

fn is_valid_name(name: &str) -> bool {
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Generate a category index and write to a Markdown file.
#[tracing::instrument]
pub fn generate_categories<P1: AsRef<Path> + std::fmt::Debug, P2: AsRef<Path> + std::fmt::Debug>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()> {
    fs::create_parent_dir_for(dest_file_path.as_ref())?;
    let mut f =
        File::create(dest_file_path.as_ref()).context("Failed to create categories file.")?;
    writeln!(f, "# Categories\n")?;

    let src_dir_path = fs::check_is_dir(&src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(&src_dir_path)?;
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
                    && name
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
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
#[tracing::instrument]
pub fn generate_crates<P1: AsRef<Path> + std::fmt::Debug, P2: AsRef<Path> + std::fmt::Debug>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()> {
    fs::create_parent_dir_for(dest_file_path.as_ref())?;
    let mut f = File::create(dest_file_path.as_ref()).context("Failed to create crates file.")?;
    writeln!(f, "# Crates\n")?;

    let src_dir_path = fs::check_is_dir(&src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(&src_dir_path)?;
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
                    && name
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
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

#[cfg(test)]
mod test {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

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
    fn test_generate_categories_invalid_dir() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("non_existent_src");
        let dest_file = dir.path().join("categories.md");

        let result = generate_categories(&src_dir, &dest_file);
        assert!(result.is_err());
        Ok(())
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

    #[test]
    fn test_generate_crates_happy_path() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md1 = src_dir.join("1.md");
        fs::write(
            &md1,
            "Here is [crate one](https://crates.io/crates/crate1) and [another](https://crates.io/crates/crate2?version=1.0) and [trailing slash](https://crates.io/crates/crate3/).",
        )?;

        let md2 = src_dir.join("2.md");
        fs::write(
            &md2,
            "Duplicate [crate1](https://crates.io/crates/crate1), and an unrelated [link](https://example.com).",
        )?;

        let dest_file = dir.path().join("crates.md");
        generate_crates(&src_dir, &dest_file)?;

        let content = std::fs::read_to_string(&dest_file)?;
        let expected = "# Crates\n\n- [crate1](https://crates.io/crates/crate1)\n- [crate2](https://crates.io/crates/crate2)\n- [crate3](https://crates.io/crates/crate3)\n";
        assert_eq!(content, expected);

        Ok(())
    }

    #[test]
    fn test_generate_crates_invalid_dir() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("non_existent_src");
        let dest_file = dir.path().join("crates.md");

        let result = generate_crates(&src_dir, &dest_file);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_generate_crates_edge_cases() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md1 = src_dir.join("1.md");
        fs::write(
            &md1,
            "Here is [empty 1](https://crates.io/crates) and [empty 2](https://crates.io/crates/).",
        )?;

        let dest_file = dir.path().join("crates.md");
        generate_crates(&src_dir, &dest_file)?;

        let content = std::fs::read_to_string(&dest_file)?;
        let expected = "# Crates\n\n";
        assert_eq!(content, expected);

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
}
