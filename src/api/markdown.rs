//! Markdown manipulation modules
pub use crate::fs::unused::identify_files_not_in_summary;
pub use crate::fs::unused::identify_unused_rs_examples;

#[cfg(test)]
mod test {
    use std::fs;
    use std::path::Path;

    use anyhow::Result;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_identify_files_not_in_summary_all_included() -> Result<()> {
        let dir = tempdir()?;
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root)?;

        let summary_path = root.join("SUMMARY.md");
        fs::write(&summary_path, "[Page 1](./page1.md)\n[Page 2](page2.md)")?;

        fs::write(root.join("page1.md"), "# Page 1")?;
        fs::write(root.join("page2.md"), "# Page 2")?;

        let missing = identify_files_not_in_summary(&root)?;
        assert!(
            missing.is_empty(),
            "Expected no missing files, but got {:?}",
            missing
        );
        Ok(())
    }

    #[test]
    fn test_identify_files_not_in_summary_some_missing() -> Result<()> {
        let dir = tempdir()?;
        let markdown_src_dir_path = dir.path();

        // Let's create a non-hidden sub directory, and use that as the root,
        // since `tempdir` returns a hidden directory like `/tmp/.tmpxxx`.
        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root)?;

        let summary_path = root.join("SUMMARY.md");
        fs::write(&summary_path, "[Page 1](./page1.md)")?;

        fs::write(root.join("page1.md"), "# Page 1")?;
        fs::write(root.join("page2.md"), "# Page 2")?;

        let missing = identify_files_not_in_summary(&root)?;
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].file_name().unwrap(), "page2.md");
        Ok(())
    }

    #[test]
    fn test_identify_files_not_in_summary_no_summary() -> Result<()> {
        let dir = tempdir()?;
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root)?;

        fs::write(root.join("page1.md"), "# Page 1")?;

        let result = identify_files_not_in_summary(&root);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("SUMMARY.md not found in")
        );
        Ok(())
    }

    #[test]
    fn test_identify_files_not_in_summary_nested_files() -> Result<()> {
        let dir = tempdir()?;
        let markdown_src_dir_path = dir.path();

        let root = markdown_src_dir_path.join("src");
        fs::create_dir(&root)?;

        let sub_dir = root.join("sub");
        fs::create_dir(&sub_dir)?;

        let summary_path = root.join("SUMMARY.md");
        fs::write(
            &summary_path,
            "[Page 1](./page1.md)\n[Sub Page](sub/page2.md)",
        )?;

        fs::write(root.join("page1.md"), "# Page 1")?;
        fs::write(sub_dir.join("page2.md"), "# Page 2")?;
        fs::write(sub_dir.join("page3.md"), "# Page 3")?; // missing

        let missing = identify_files_not_in_summary(&root)?;
        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].file_name().unwrap(), "page3.md");
        Ok(())
    }
}
