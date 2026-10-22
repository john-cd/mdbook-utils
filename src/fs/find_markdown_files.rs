// TODO Reviewed ??-??-2026
//! Locate Markdown files within a directory

use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use tracing::debug;
use walkdir::DirEntry;
use walkdir::WalkDir;

/// Returns `true`` if the directory entry is hidden (starts with a `.`).
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Locate Markdown files within a directory
///
/// If root_dir_path is a file, then it is the first and only item
/// yielded
pub(crate) fn find_markdown_files_in<P>(root_dir_path: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut paths = Vec::new();

    let walker = WalkDir::new(root_dir_path)
        .sort_by_file_name()
        .into_iter()
        // Skip hidden files and directories efficiently on unix systems,
        // but never the root itself (it may legitimately be e.g. `.book`)
        .filter_entry(|e| e.depth() == 0 || !is_hidden(e));

    for entry in walker {
        let entry =
            entry.context("Failed to read a directory entry while looking for Markdown files")?;
        if !entry.file_type().is_file() {
            continue;
        }
        match entry.path().extension() {
            Some(extension) => {
                if extension == "md" {
                    // debug: println!("{}", entry.path().display());
                    paths.push(entry.into_path());
                } else {
                    debug!("Not a Markdown file: {}", entry.path().display());
                }
            }
            None => {
                debug!("Could not extract extension for {}", entry.path().display());
            }
        }
    }
    Ok(paths)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use tempfile::tempdir;

    use super::*;

    /// Returns only Markdown files, in file-name order, skipping hidden
    /// files and directories and other extensions.
    #[test]
    fn finds_sorted_markdown_files_and_skips_hidden_and_other_files() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();
        fs::create_dir_all(root.join("sub"))?;
        fs::create_dir_all(root.join(".hidden"))?;
        fs::write(root.join("b.md"), "b")?;
        fs::write(root.join("a.md"), "a")?;
        fs::write(root.join("sub").join("c.md"), "c")?;
        fs::write(root.join("notes.txt"), "t")?;
        fs::write(root.join("README"), "r")?;
        fs::write(root.join(".secret.md"), "s")?;
        fs::write(root.join(".hidden").join("d.md"), "d")?;

        let names: Vec<_> = find_markdown_files_in(root)?
            .iter()
            .map(|p| p.strip_prefix(root).map(|r| r.to_path_buf()))
            .collect::<std::result::Result<_, _>>()?;

        assert_eq!(
            names,
            vec![
                PathBuf::from("a.md"),
                PathBuf::from("b.md"),
                PathBuf::from("sub").join("c.md"),
            ]
        );
        Ok(())
    }

    /// A single Markdown file is yielded as the only item.
    #[test]
    fn a_file_root_yields_only_that_file() -> Result<()> {
        let dir = tempdir()?;
        let file = dir.path().join("one.md");
        fs::write(&file, "x")?;

        assert_eq!(find_markdown_files_in(&file)?, vec![file]);
        Ok(())
    }

    /// A root directory whose own name starts with a dot is still searched.
    #[test]
    fn a_hidden_root_directory_is_still_searched() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path().join(".book_src");
        fs::create_dir(&root)?;
        fs::write(root.join("a.md"), "a")?;

        assert_eq!(find_markdown_files_in(&root)?, vec![root.join("a.md")]);
        Ok(())
    }

    /// An empty directory yields no files.
    #[test]
    fn an_empty_directory_yields_nothing() -> Result<()> {
        let dir = tempdir()?;

        assert!(find_markdown_files_in(dir.path())?.is_empty());
        Ok(())
    }
}
