//! CLI regression tests for `links write-inline` external-link output.

use std::fs;
use std::process::Command;
use std::process::Output;

use anyhow::Result;
use tempfile::tempdir;

/// Runs `links write-inline` on a fixture with external and internal links.
fn run_write_inline(extra: &[&str]) -> Result<(Output, String)> {
    let dir = tempdir()?;
    let src = dir.path().join("src");
    fs::create_dir(&src)?;
    fs::write(
        src.join("page.md"),
        "[ext](https://example.com/a) <https://example.org/b>\n\n\
         [rel](other.md) [frag](#top)\n\n```md\n[code](https://example.net/c)\n```\n",
    )?;
    let out_path = dir.path().join("out.md");
    let output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["links", "write-inline", "--markdown-dir"])
        .arg(&src)
        .arg("--output")
        .arg(&out_path)
        .args(extra)
        .current_dir(dir.path())
        .env_remove("BOOK_ROOT_DIR_PATH")
        .env_remove("MARKDOWN_DIR_PATH")
        .env_remove("DEFAULT_DEST_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?;
    let content = fs::read_to_string(&out_path).unwrap_or_default();
    Ok((output, content))
}

/// Reference output lists external links only, with definitions.
#[test]
fn write_inline_defaults_to_reference_format_for_external_links() -> Result<()> {
    let (output, content) = run_write_inline(&[])?;
    assert!(output.status.success());
    assert!(content.contains("]: https://example.com/a"), "{content}");
    for excluded in ["other.md", "#top", "example.net"] {
        assert!(!content.contains(excluded), "{content}");
    }
    Ok(())
}

/// `--link-format autolink` writes `<url>` lines.
#[test]
fn write_inline_link_format_autolink() -> Result<()> {
    let (output, content) = run_write_inline(&["--link-format", "autolink"])?;
    assert!(output.status.success());
    assert!(content.contains("<https://example.com/a>"), "{content}");
    assert!(!content.contains("]: "), "{content}");
    Ok(())
}

/// An unknown format is rejected with a non-zero exit status.
#[test]
fn write_inline_rejects_unknown_link_format() -> Result<()> {
    let (output, _) = run_write_inline(&["--link-format", "bogus"])?;
    assert!(!output.status.success());
    Ok(())
}
