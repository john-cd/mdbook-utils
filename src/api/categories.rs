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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

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
