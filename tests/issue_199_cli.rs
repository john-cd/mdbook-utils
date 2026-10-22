//! CLI regression tests for SUMMARY chapter and Rust example references.
//!
//! TODO Reviewed ??-??-2026

use std::fs;
use std::process::Command;

use anyhow::Result;
use tempfile::tempdir;

/// Prints only unlisted nested Markdown chapters from a SUMMARY scan.
#[test]
fn identify_files_not_in_summary_reports_nested_unlisted_chapters() -> Result<()> {
    let dir = tempdir()?;
    let markdown_dir = dir.path().join("src");
    fs::create_dir_all(markdown_dir.join("nested pages"))?;
    fs::write(
        markdown_dir.join("SUMMARY.md"),
        "\
[Nested](nested%20pages/chapter.md#details)\n\
[Outside](../../outside.md)\n\
",
    )?;
    let listed = markdown_dir.join("nested pages/chapter.md");
    let unlisted = markdown_dir.join("unlisted.md");
    fs::write(&listed, "# Listed")?;
    fs::write(&unlisted, "# Unlisted")?;

    let output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args([
            "markdown",
            "identify-files-not-in-summary",
            "--markdown-dir",
        ])
        .arg(&markdown_dir)
        .current_dir(dir.path())
        .env("RUST_LOG", "off")
        .output()?;

    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    // The command prints canonical paths, whose prefix differs from the
    // temporary directory's on Windows (`\\?\` and short names); compare names.
    assert!(stdout.contains("unlisted.md"), "{stdout}");
    assert!(!stdout.contains("chapter.md"), "{stdout}");
    Ok(())
}

/// Detects all supported mdBook Rust helpers and ignores ordinary path
/// mentions.
#[test]
fn identify_unused_rs_examples_recognizes_mdbook_helpers() -> Result<()> {
    let dir = tempdir()?;
    let markdown_dir = dir.path().join("markdown");
    let code_dir = dir.path().join("code");
    fs::create_dir(&markdown_dir)?;
    fs::create_dir(&code_dir)?;
    for file in [
        "include.rs",
        "rustdoc.rs",
        "playground.rs",
        "unreferenced.rs",
        "literal_helper.rs",
    ] {
        fs::write(code_dir.join(file), "fn example() {}")?;
    }
    fs::write(
        markdown_dir.join("chapter.md"),
        "\
{{#include ../code/include.rs}}\n\
```rust\n\
{{#rustdoc_include ../code/rustdoc.rs:sample}}\n\
{{#playground ../code/playground.rs editable}}\n\
let file = \"../code/unreferenced.rs\";\n\
let directive = \"{{#include ../code/literal_helper.rs}}\";\n\
```\n\
The path `../code/unreferenced.rs` appears here as text.\n\
",
    )?;

    let output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["markdown", "identify-unused-rs-examples", "--markdown-dir"])
        .arg(&markdown_dir)
        .arg("--target-dir")
        .arg(&code_dir)
        .current_dir(dir.path())
        .env("RUST_LOG", "off")
        .output()?;

    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("unreferenced.rs"));
    assert!(stdout.contains("literal_helper.rs"));
    assert!(!stdout.contains("include.rs"));
    assert!(!stdout.contains("rustdoc.rs"));
    assert!(!stdout.contains("playground.rs"));
    Ok(())
}

/// Returns a CLI error when the Markdown directory has no SUMMARY.md.
#[test]
fn identify_files_not_in_summary_reports_missing_summary() -> Result<()> {
    let dir = tempdir()?;
    let markdown_dir = dir.path().join("src");
    fs::create_dir(&markdown_dir)?;
    fs::write(markdown_dir.join("chapter.md"), "# Chapter")?;

    let output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args([
            "markdown",
            "identify-files-not-in-summary",
            "--markdown-dir",
        ])
        .arg(&markdown_dir)
        .current_dir(dir.path())
        .env("RUST_LOG", "off")
        .output()?;

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("SUMMARY.md not found"));
    Ok(())
}
