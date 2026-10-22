// TODO Reviewed ??-??-2026
//! CLI regression tests for configurable GitHub badge generation.

use std::fs;
use std::process::Command;

use anyhow::Result;
use tempfile::tempdir;

/// Lists the supported Shields.io styles in refdefs generate help.
#[test]
fn refdefs_generate_help_lists_badge_styles() -> Result<()> {
    let output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["refdefs", "generate", "--help"])
        .env("RUST_LOG", "off")
        .output()?;

    assert!(output.status.success());
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("--badge-style <STYLE>"));
    for style in ["flat", "flat-square", "plastic", "for-the-badge", "social"] {
        assert!(help.contains(style), "help omitted style {style:?}");
    }
    Ok(())
}

/// Generates badges using an explicit style and escapes GitHub name separators.
#[test]
fn refdefs_generate_applies_badge_style_without_the_badges_flag() -> Result<()> {
    let dir = tempdir()?;
    let markdown_dir = dir.path().join("src");
    fs::create_dir(&markdown_dir)?;
    fs::write(
        markdown_dir.join("github.md"),
        "[project]: https://github.com/john-cd/my_repo-with-dash\n",
    )?;
    let destination = dir.path().join("badges.md");

    let output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args([
            "refdefs",
            "generate",
            "--badge-style",
            "flat-square",
            "--markdown-dir",
        ])
        .arg(&markdown_dir)
        .arg("--output")
        .arg(&destination)
        .current_dir(dir.path())
        .env("RUST_LOG", "off")
        .output()?;

    assert!(
        output.status.success(),
        "CLI failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let badges = fs::read_to_string(destination)?;
    assert!(badges.contains(
        "https://img.shields.io/badge/my__repo--with--dash-steelblue?logo=github&style=flat-square"
    ));
    Ok(())
}
