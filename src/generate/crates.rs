//! Generate index files for crates and categories
//! in Markdown format.

use std::fs::File;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

use crate::fs;
use crate::generate::index::IndexKind;
use crate::generate::index::extract_index_names;
use crate::generate::index::write_index;
use crate::parser;

/// Generate a category index and write to a Markdown file.
#[tracing::instrument]
pub fn generate_categories<P1: AsRef<Path> + std::fmt::Debug, P2: AsRef<Path> + std::fmt::Debug>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()> {
    fs::create_parent_dir_for(dest_file_path.as_ref())?;
    let f = File::create(dest_file_path.as_ref()).context("Failed to create categories file.")?;

    let src_dir_path = fs::check_is_dir(&src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(&src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let links = parser::extract_links(&mut parser);
    let categories = extract_index_names(links, IndexKind::Categories);
    tracing::info!(count = categories.len(), "writing categories index");
    write_index(f, IndexKind::Categories, categories)?;

    Ok(())
}

/// Generate a crate index and write to a Markdown file.
#[tracing::instrument]
pub fn generate_crates<P1: AsRef<Path> + std::fmt::Debug, P2: AsRef<Path> + std::fmt::Debug>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()> {
    fs::create_parent_dir_for(dest_file_path.as_ref())?;
    let f = File::create(dest_file_path.as_ref()).context("Failed to create crates file.")?;

    let src_dir_path = fs::check_is_dir(&src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(&src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let links = parser::extract_links(&mut parser);
    let crates = extract_index_names(links, IndexKind::Crates);
    tracing::info!(count = crates.len(), "writing crates index");
    write_index(f, IndexKind::Crates, crates)?;

    Ok(())
}

#[cfg(test)]
mod tests {
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
    fn test_generate_crates_injection() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let md1 = src_dir.join("1.md");
        fs::write(
            &md1,
            "Malicious crate: [crate](https://crates.io/crates/mycrate](javascript:alert(1))/).",
        )?;

        let dest_file = dir.path().join("crates.md");
        generate_crates(&src_dir, &dest_file)?;

        let content = fs::read_to_string(&dest_file)?;
        // If vulnerable, it would contain: - [mycrate](javascript:alert(1))](https://crates.io/crates/mycrate](javascript:alert(1)))
        // After fix, it should skip it or sanitize it.
        // Given our planned fix is to skip it, we expect no crates.
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
            "Malicious category: [cat](https://crates.io/categories/mycat](javascript:alert(1))/).",
        )?;

        let dest_file = dir.path().join("categories.md");
        generate_categories(&src_dir, &dest_file)?;

        let content = fs::read_to_string(&dest_file)?;
        let expected = "# Categories\n\n";
        assert_eq!(content, expected);

        Ok(())
    }
}
