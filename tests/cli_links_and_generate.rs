//! CLI tests for the `links`, `refdefs write`, `debug`, and crates.io index
//! commands: exit codes and the files they write.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

use anyhow::Result;
use tempfile::TempDir;
use tempfile::tempdir;

/// A temporary book with one Markdown file in `src/`.
struct Fixture {
    dir: TempDir,
    src: PathBuf,
    out: PathBuf,
}

/// Creates a fixture whose only source file is `page.md` with the given text.
fn fixture(markdown: &str) -> Result<Fixture> {
    let dir = tempdir()?;
    let src = dir.path().join("src");
    fs::create_dir(&src)?;
    fs::write(src.join("page.md"), markdown)?;
    let out = dir.path().join("out").join("result.md");
    Ok(Fixture { dir, src, out })
}

/// Runs `mdbook-utils <args> --markdown-dir <src> --output <out>` in the
/// fixture directory with a clean environment.
fn run(fixture: &Fixture, args: &[&str]) -> Result<Output> {
    run_with_src(fixture, args, &fixture.src)
}

/// Same as [`run`], with an explicit source directory.
fn run_with_src(fixture: &Fixture, args: &[&str], src: &Path) -> Result<Output> {
    Ok(Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(args)
        .arg("--markdown-dir")
        .arg(src)
        .arg("--output")
        .arg(&fixture.out)
        .current_dir(fixture.dir.path())
        .env_remove("BOOK_ROOT_DIR_PATH")
        .env_remove("MARKDOWN_DIR_PATH")
        .env_remove("DEFAULT_DEST_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?)
}

/// Asserts success and returns the content of the output file.
fn succeeded(fixture: &Fixture, output: &Output) -> Result<String> {
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(fs::read_to_string(&fixture.out)?)
}

/// `links write-all` lists external links as reference links and definitions.
#[test]
fn links_write_all_writes_external_links() -> Result<()> {
    let f = fixture("[Serde](https://serde.rs) and [local](other.md)\n")?;
    let content = succeeded(&f, &run(&f, &["links", "write-all"])?)?;

    assert!(content.starts_with("# Links"), "{content}");
    assert!(content.contains("https://serde.rs"), "{content}");
    assert!(!content.contains("other.md"), "{content}");
    Ok(())
}

/// `links duplicate-links` reports only repeated links.
#[test]
fn links_duplicate_links_reports_only_repeats() -> Result<()> {
    let f = fixture("[a](https://a.example) [a](https://a.example) [b](https://b.example)\n")?;
    let content = succeeded(&f, &run(&f, &["links", "duplicate-links"])?)?;

    assert!(content.contains("# Duplicate Links"), "{content}");
    assert!(content.contains("https://a.example"), "{content}");
    assert!(!content.contains("https://b.example"), "{content}");
    Ok(())
}

/// `links broken-links` reports references that have no definition.
#[test]
fn links_broken_links_reports_missing_definitions() -> Result<()> {
    let f = fixture("[good] and [bad]\n\n[good]: https://example.com\n")?;
    let content = succeeded(&f, &run(&f, &["links", "broken-links"])?)?;

    assert!(content.contains("- Reference: bad"), "{content}");
    assert!(!content.contains("Reference: good"), "{content}");
    Ok(())
}

/// `refdefs write` writes the reference definitions found in the sources.
#[test]
fn refdefs_write_writes_existing_definitions() -> Result<()> {
    let f = fixture("[serde]: https://serde.rs\n")?;
    let content = succeeded(&f, &run(&f, &["refdefs", "write"])?)?;

    assert!(content.contains("[serde]: https://serde.rs"), "{content}");
    Ok(())
}

/// `debug` writes the parser events to the output file.
#[test]
fn debug_writes_parser_events() -> Result<()> {
    let f = fixture("# Heading\n")?;
    let content = succeeded(&f, &run(&f, &["debug"])?)?;

    assert!(content.contains("Heading"), "{content}");
    Ok(())
}

/// `markdown generate-crates` writes a sorted, deduplicated crate index.
#[test]
fn generate_crates_writes_index() -> Result<()> {
    let f = fixture(
        "[b](https://crates.io/crates/b) [a](https://crates.io/crates/a) \
         [a again](https://crates.io/crates/a/)\n",
    )?;
    let content = succeeded(&f, &run(&f, &["markdown", "generate-crates"])?)?;

    assert_eq!(
        content,
        "# Crates\n\n- [a](https://crates.io/crates/a)\n- [b](https://crates.io/crates/b)\n"
    );
    Ok(())
}

/// `markdown generate-categories` writes a category index.
#[test]
fn generate_categories_writes_index() -> Result<()> {
    let f = fixture("[p](https://crates.io/categories/parsing)\n")?;
    let content = succeeded(&f, &run(&f, &["markdown", "generate-categories"])?)?;

    assert_eq!(
        content,
        "# Categories\n\n- [parsing](https://crates.io/categories/parsing)\n"
    );
    Ok(())
}

/// A missing source directory exits non-zero and writes no output file.
#[test]
fn missing_source_directory_fails_without_writing() -> Result<()> {
    let f = fixture("text\n")?;
    let missing = f.dir.path().join("missing");
    for args in [
        &["links", "write-all"][..],
        &["links", "broken-links"][..],
        &["debug"][..],
    ] {
        let output = run_with_src(&f, args, &missing)?;

        assert!(!output.status.success(), "{args:?} should fail");
        assert!(!f.out.exists(), "{args:?} wrote {}", f.out.display());
    }
    Ok(())
}

/// Every command that writes an output file fails on a missing source
/// directory with a nonzero status and leaves an existing output untouched.
#[test]
fn failed_runs_keep_the_existing_output_file() -> Result<()> {
    let commands: &[&[&str]] = &[
        &["links", "write-all"],
        &["links", "write-inline"],
        &["links", "duplicate-links"],
        &["links", "broken-links"],
        &["refdefs", "write"],
        &["debug"],
        &["markdown", "generate-crates"],
        &["markdown", "generate-categories"],
    ];
    let f = fixture("# Page\n")?;
    fs::create_dir_all(f.out.parent().expect("output has a parent"))?;
    let missing = f.dir.path().join("missing");

    for args in commands {
        fs::write(&f.out, "previous")?;
        let output = run_with_src(&f, args, &missing)?;
        assert!(!output.status.success(), "{args:?} succeeded");
        assert_eq!(fs::read_to_string(&f.out)?, "previous", "{args:?}");
    }
    Ok(())
}

/// Extracting then replacing Rust code blocks through the CLI keeps the
/// surrounding Markdown, and a second replacement changes nothing.
#[test]
fn code_example_round_trip_keeps_surrounding_text() -> Result<()> {
    let dir = tempdir()?;
    let drafts = dir.path().join("drafts");
    let code = dir.path().join("code");
    fs::create_dir(&drafts)?;
    let chapter = drafts.join("ch.md");
    fs::write(
        &chapter,
        "# T\n\n```rust\n```\n\ntext\n\n```rust\nfn main() {}\n```\n",
    )?;
    let run = |args: &[&str]| -> Result<Output> {
        Ok(Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
            .args(args)
            .arg("--yes")
            .arg("--markdown-dir")
            .arg(&drafts)
            .arg("--target-dir")
            .arg(&code)
            .current_dir(dir.path())
            .env_remove("BOOK_ROOT_DIR_PATH")
            .env_remove("MARKDOWN_DIR_PATH")
            .env("RUST_LOG", "off")
            .output()?)
    };

    assert!(
        run(&["markdown", "extract-code-examples"])?
            .status
            .success()
    );
    assert!(
        run(&["markdown", "replace-code-examples-by-includes"])?
            .status
            .success()
    );
    let first = fs::read_to_string(&chapter)?;
    assert!(first.contains("\ntext\n"), "{first}");
    assert_eq!(first.matches("{{#include").count(), 2, "{first}");

    assert!(
        run(&["markdown", "replace-code-examples-by-includes"])?
            .status
            .success()
    );
    assert_eq!(fs::read_to_string(&chapter)?, first);
    Ok(())
}

/// `remove-includes` asks for confirmation; without a terminal and without
/// `--yes` it fails and leaves the files alone, and with `--yes` it replaces
/// each include and reports the modified file.
#[test]
fn remove_includes_requires_confirmation_and_replaces_includes() -> Result<()> {
    let dir = tempdir()?;
    let chapter = dir.path().join("ch.md");
    let original = "# T\n\n```rust\n{{#include missing.rs}}\n```\n";
    fs::write(&chapter, original)?;
    let run = |extra: &[&str]| -> Result<Output> {
        Ok(Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
            .args(["markdown", "remove-includes"])
            .args(extra)
            .arg("--markdown-dir")
            .arg(dir.path())
            .current_dir(dir.path())
            .stdin(std::process::Stdio::null())
            .env_remove("BOOK_ROOT_DIR_PATH")
            .env_remove("MARKDOWN_DIR_PATH")
            .env("RUST_LOG", "off")
            .output()?)
    };

    let declined = run(&[])?;
    assert!(!declined.status.success());
    assert!(String::from_utf8_lossy(&declined.stderr).contains("rerun with --yes"));
    assert_eq!(fs::read_to_string(&chapter)?, original);

    let confirmed = run(&["--yes"])?;
    assert!(confirmed.status.success());
    let stdout = String::from_utf8_lossy(&confirmed.stdout);
    assert!(stdout.contains("{{#include ...}}"), "{stdout}");
    assert!(
        stdout.contains("Modified:") && stdout.contains("ch.md"),
        "{stdout}"
    );
    assert_eq!(
        fs::read_to_string(&chapter)?,
        "# T\n\n```rust\n// MISSING INCLUDE FILE\nfn main() {}\n```\n"
    );
    Ok(())
}

/// Every command that modifies Markdown refuses to run without `--yes` when
/// there is no terminal, and leaves the files untouched.
#[test]
fn modifying_commands_fail_without_terminal_or_yes() -> Result<()> {
    let dir = tempdir()?;
    let drafts = dir.path().join("drafts");
    fs::create_dir(&drafts)?;
    let chapter = drafts.join("ch.md");
    let original = "# T\n\n```rust\nfn main() {}\n```\n";
    fs::write(&chapter, original)?;

    for subcommand in [
        "replace-code-examples-by-includes",
        "replace-includes-by-contents",
        "remove-includes",
    ] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"));
        command
            .args(["markdown", subcommand, "--markdown-dir"])
            .arg(&drafts);
        if subcommand == "replace-code-examples-by-includes" {
            command.arg("--target-dir").arg(dir.path().join("code"));
        }
        let output = command
            .current_dir(dir.path())
            .stdin(std::process::Stdio::null())
            .env_remove("BOOK_ROOT_DIR_PATH")
            .env_remove("MARKDOWN_DIR_PATH")
            .env("RUST_LOG", "off")
            .output()?;
        assert!(!output.status.success(), "{subcommand} succeeded");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("rerun with --yes"),
            "{subcommand}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(fs::read_to_string(&chapter)?, original, "{subcommand}");
    }
    Ok(())
}
