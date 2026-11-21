// TODO Reviewed ??-??-2026.
//! Regressions for generated mdBook include paths.

use std::fs;

use tempfile::tempdir;

/// Unrepresentable include paths leave every chapter byte-identical.
#[test]
fn include_rewrite_rejects_whitespace_in_generated_paths() -> anyhow::Result<()> {
    use mdbook_utils::markdown::extract_code::extract_code_from_all_markdown_files_in;
    use mdbook_utils::markdown::extract_code::remove_code_from_all_markdown_files_in;

    for (chapter_name, code_dir_name) in
        [("chapter.md", "extracted code"), ("a chapter.md", "code")]
    {
        let dir = tempdir()?;
        let source_dir = dir.path().join("src");
        let code_dir = dir.path().join(code_dir_name);
        fs::create_dir(&source_dir)?;
        let chapter = source_dir.join(chapter_name);
        let other = source_dir.join("first.md");
        let original = b"# Example\n\n```rust\nprintln!(\"sample\");\n```\n";
        fs::write(&chapter, original)?;
        fs::write(&other, original)?;
        extract_code_from_all_markdown_files_in(&source_dir, &code_dir)?;
        let error = remove_code_from_all_markdown_files_in(&source_dir, &code_dir)
            .expect_err("mdBook cannot resolve the generated path");
        assert!(
            error.to_string().contains("relative include path"),
            "{error:#}"
        );
        assert_eq!(fs::read(chapter)?, original);
        assert_eq!(fs::read(other)?, original);
    }
    Ok(())
}
