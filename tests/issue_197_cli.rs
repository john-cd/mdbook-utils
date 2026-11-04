// TODO Reviewed ??-??-2026

//! CLI regression tests for `links write-inline` external-link output.

use std::fs;
use std::process::Command;
use std::process::Output;

use anyhow::Context as _;
use anyhow::Result;
use tempfile::tempdir;

/// Runs `links write-inline` on a fixture containing external links and
/// unsupported reference forms.
fn run_write_inline(extra: &[&str]) -> Result<(Output, Option<String>)> {
    let dir = tempdir()?;
    let src = dir.path().join("src");
    fs::create_dir(&src)?;
    fs::write(
        src.join("page.md"),
        "[ext](https://example.com/a \"Example docs\") <https://example.org/b> <https://docs.rs/tokio/latest/tokio/task/index.html> <https://example.com/guide> <https://another.org/guide>\n\n\
         [rel](other.md) [frag](#top)\n\n\
         [full][full-ref] [collapsed][] [shortcut] ![image][image-ref]\n\n\
         [full-ref]: https://example.net/full \"Full reference\"\n\
         [collapsed]: https://example.net/collapsed\n\
         [shortcut]: https://example.net/shortcut\n\
         [image-ref]: https://example.net/image\n\n\
         [docs](https://example.com/guide) [docs](https://example.org/guide)\n\n\
         ```md\n[code](https://example.net/c)\n```\n",
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
    let content = match fs::read_to_string(&out_path) {
        Ok(content) => Some(content),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => return Err(error.into()),
    };
    Ok((output, content))
}

/// Reference output lists external links only, with definitions.
#[test]
fn write_inline_defaults_to_reference_format_for_external_links() -> Result<()> {
    let (output, content) = run_write_inline(&[])?;
    assert!(output.status.success());
    let content = content.context("successful command did not write its report")?;
    assert!(content.contains("]: https://example.com/a"), "{content}");
    assert!(content.contains("[ext]"), "{content}");
    assert!(
        content.contains("[https://docs.rs/tokio/latest/tokio/task/index.html][tokio]"),
        "autolink should retain its URL as visible text and use a URL-derived label: {content}"
    );
    assert!(
        content.contains("[tokio]: https://docs.rs/tokio/latest/tokio/task/index.html"),
        "{content}"
    );
    assert!(
        content.contains("[https://example.com/guide][guide]"),
        "{content}"
    );
    assert!(
        content.contains("[https://another.org/guide][guide-2]"),
        "{content}"
    );
    assert!(
        content.contains("[guide-2]: https://another.org/guide"),
        "{content}"
    );
    assert!(
        content.contains("[docs]\n\n[docs]: https://example.com/guide"),
        "{content}"
    );
    assert!(content.contains("[docs][docs-2]"), "{content}");
    assert!(
        content.contains("[docs-2]: https://example.org/guide"),
        "{content}"
    );
    assert!(content.contains("Example docs"), "{content}");
    for excluded in [
        "other.md",
        "#top",
        "example.net",
        "full-ref",
        "collapsed",
        "shortcut",
        "image-ref",
    ] {
        assert!(!content.contains(excluded), "{content}");
    }
    Ok(())
}

/// `--link-format autolink` writes `<url>` lines.
#[test]
fn write_inline_link_format_autolink() -> Result<()> {
    let (output, content) = run_write_inline(&["--link-format", "autolink"])?;
    assert!(output.status.success());
    let content = content.context("successful command did not write its report")?;
    assert!(content.contains("<https://example.com/a>"), "{content}");
    assert!(!content.contains("]: "), "{content}");
    Ok(())
}

/// `--link-format inline` preserves link text and titles without padding.
#[test]
fn write_inline_link_format_inline_preserves_title() -> Result<()> {
    let (output, content) = run_write_inline(&["--link-format", "inline"])?;
    assert!(output.status.success());
    let content = content.context("successful command did not write its report")?;
    assert!(
        content.contains("[ext](https://example.com/a \"Example docs\")"),
        "{content}"
    );
    assert!(!content.contains("( https://"), "{content}");
    Ok(())
}

/// An unknown format is rejected with a non-zero exit status.
#[test]
fn write_inline_rejects_unknown_link_format() -> Result<()> {
    let (output, content) = run_write_inline(&["--link-format", "bogus"])?;
    assert!(!output.status.success());
    assert!(
        content.is_none(),
        "failed command unexpectedly wrote a report"
    );
    Ok(())
}
