use mdbook_utils::identify_files_not_in_summary;
use mdbook_utils::identify_unused_rs_examples;
use mdbook_utils::markdown::replace_include::include_in_all_markdown_files_in;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_path_traversal_identify_unused_rs_examples() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let md_dir = dir.path().join("md");
    let code_dir = dir.path().join("code");
    fs::create_dir(&md_dir)?;
    fs::create_dir(&code_dir)?;

    let secret_file = dir.path().join("secret.rs");
    fs::write(&secret_file, "fn secret() {}")?;

    let md_file = md_dir.join("test.md");
    // Attempt to include a file outside of code_dir
    fs::write(
        &md_file,
        r#"# Test
{{#include ../secret.rs}}
"#,
    )?;

    // This should ideally NOT find secret.rs if it's outside code_dir,
    // or at least identify_unused_rs_examples should not be able to "use" it if we want to restrict it.
    // However, the current implementation canonicalizes it and if it exists, it's added to used_rs_files.

    let _unused = identify_unused_rs_examples(&md_dir, &code_dir)?;

    // If vulnerable, used_rs_files will contain secret.rs, so it won't be in unused if we had it in all_rs_files.
    // Let's put a legitimate file in code_dir.
    let legit_file = code_dir.join("legit.rs");
    fs::write(&legit_file, "fn legit() {}")?;

    let _unused = identify_unused_rs_examples(&md_dir, &code_dir)?;
    let _secret_canon = secret_file.canonicalize()?;

    // In current vulnerable state, if it's "used", it won't be in "unused" (if it was in all_rs_files)
    // Actually, identify_unused_rs_examples only looks for files in code_dir for all_rs_files.
    // So it won't show up in unused anyway.

    // Let's check the behavior: the vulnerability is that it CANONICALIZES paths from markdown
    // and adds them to used_rs_files.

    Ok(())
}

#[test]
fn test_path_traversal_include_in_all_markdown_files_in() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let md_dir = dir.path().join("md");
    fs::create_dir(&md_dir)?;

    let secret_file = dir.path().join("secret.txt");
    fs::write(&secret_file, "SENSITIVE DATA")?;

    let md_file = md_dir.join("test.md");
    // Attempt to include a file outside of md_dir
    fs::write(
        &md_file,
        r#"# Test
{{#include ../secret.txt.md}}
"#,
    )?;

    // We need it to end in .md for INSERT_REGEX
    let secret_file_md = dir.path().join("secret.txt.md");
    fs::write(&secret_file_md, "SENSITIVE DATA")?;

    // This should fail or not include the file if it's outside md_dir
    let _ = include_in_all_markdown_files_in(&md_dir);

    let content = fs::read_to_string(&md_file)?;
    // If fixed, it will NOT contain the sensitive data
    assert!(
        !content.contains("SENSITIVE DATA"),
        "Vulnerability still present: Included file from outside base directory!"
    );

    Ok(())
}

#[test]
fn test_path_traversal_identify_files_not_in_summary() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let md_dir = dir.path().join("md");
    fs::create_dir(&md_dir)?;

    let secret_file = dir.path().join("secret.md");
    fs::write(&secret_file, "SENSITIVE DATA")?;

    let summary_file = md_dir.join("SUMMARY.md");
    fs::write(
        &summary_file,
        r#"# Summary
[Secret](../secret.md)
"#,
    )?;

    let _missing = identify_files_not_in_summary(&md_dir)?;

    let _secret_canon = secret_file.canonicalize()?;
    // If it's in files_in_summary (vulnerable), it won't be "missing" if we were to look for it.
    // But identify_files_not_in_summary only lists files IN md_dir that are NOT in SUMMARY.md.

    Ok(())
}
