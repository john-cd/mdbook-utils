//! Executable regression tests for CLI behavior and generated file output.

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

use anyhow::Result;
use tempfile::tempdir;

/// Returns a fixture path inside this package's test data directory.
fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("issue_198")
        .join(name)
}

/// Runs one Markdown index command with explicit source and destination paths.
fn run_markdown_index(
    subcommand: &str,
    markdown_dir: &Path,
    output_path: &Path,
    working_dir: &Path,
) -> Result<Output> {
    Ok(Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["markdown", subcommand, "--markdown-dir"])
        .arg(markdown_dir)
        .arg("--output")
        .arg(output_path)
        .current_dir(working_dir)
        .env_remove("BOOK_ROOT_DIR_PATH")
        .env_remove("MARKDOWN_DIR_PATH")
        .env_remove("DEFAULT_DEST_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?)
}

/// Checks a successful CLI result has the expected completion text and no error
/// output.
fn assert_successful_output(output: &Output, action: &str) {
    assert!(
        output.status.success(),
        "{action} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("Done."),
        "{action} did not report completion: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.stderr.is_empty(),
        "{action} wrote unexpected stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Runs links and Markdown scan commands against the repository's test_book
/// fixtures.
#[test]
fn test_book_fixtures_run_through_links_and_markdown_cli() -> Result<()> {
    let temp = tempdir()?;
    let book_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test_book");
    let source = book_root.join("src");
    let summary_path = source.join("SUMMARY.md");
    let original_summary = fs::read_to_string(&summary_path)?;
    let links_path = temp.path().join("test-book-links.md");

    let links_output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["links", "write-all", "--output"])
        .arg(&links_path)
        .current_dir(temp.path())
        .env("BOOK_ROOT_DIR_PATH", &book_root)
        .env_remove("MARKDOWN_DIR_PATH")
        .env_remove("DEFAULT_DEST_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?;
    assert_successful_output(&links_output, "test_book links command");
    assert!(
        links_path.is_file(),
        "links command did not create its output"
    );
    assert_eq!(
        fs::read_to_string(&links_path)?,
        "# Links\n\n\
         [Inline][inline]\n\n\
         [inline]: https://inline\n\n\
         [Reference-style][ref]\n\n\
         [ref]: https://reference-definition/\n\n\
         [Link to Github][github]\n\n\
         [github]: https://github.com/john-cd/mdbook-utils\n\n\
         [https://autolink][https-autolink]\n\n\
         [https-autolink]: https://autolink\n\n"
    );

    let markdown_output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["markdown", "identify-files-not-in-summary"])
        .current_dir(temp.path())
        .env("BOOK_ROOT_DIR_PATH", &book_root)
        .env_remove("MARKDOWN_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?;
    assert!(
        markdown_output.status.success(),
        "test_book Markdown scan failed: {}",
        String::from_utf8_lossy(&markdown_output.stderr)
    );
    let markdown_stdout = String::from_utf8_lossy(&markdown_output.stdout);
    assert!(markdown_stdout.contains("Files not in SUMMARY.md:"));
    for file in [
        format!("chapter_2{}incl.md", std::path::MAIN_SEPARATOR),
        "included.md".to_string(),
        "wont-be-included-refs.md".to_string(),
    ] {
        assert!(
            markdown_stdout.contains(&file),
            "missing fixture {file} in output"
        );
    }
    assert!(markdown_output.stderr.is_empty());
    assert_eq!(fs::read_to_string(summary_path)?, original_summary);
    assert!(!source.join("broken_links.md").exists());
    Ok(())
}

/// Checks category and crate index CLI output against exact fixtures and repeat
/// runs.
#[test]
fn generate_index_cli_outputs_match_fixtures_and_repeat_exactly() -> Result<()> {
    let temp = tempdir()?;
    let source = temp.path().join("source");
    fs::create_dir(&source)?;
    fs::copy(fixture_path("indexes.md"), source.join("indexes.md"))?;

    for (subcommand, output_name, expected_name) in [
        ("generate-categories", "categories.md", "categories.md"),
        ("generate-crates", "crates.md", "crates.md"),
    ] {
        let output_path = temp.path().join(output_name);
        let first = run_markdown_index(subcommand, &source, &output_path, temp.path())?;
        assert_successful_output(&first, subcommand);
        let first_contents = fs::read_to_string(&output_path)?;
        assert_eq!(
            first_contents,
            fs::read_to_string(fixture_path(expected_name))?
        );

        let second = run_markdown_index(subcommand, &source, &output_path, temp.path())?;
        assert_successful_output(&second, subcommand);
        assert_eq!(fs::read_to_string(&output_path)?, first_contents);
    }
    Ok(())
}

/// Checks the sitemap CLI writes the configured output with stable exact XML.
#[test]
fn sitemap_cli_output_matches_exact_xml_on_repeated_runs() -> Result<()> {
    let temp = tempdir()?;
    let book_root = temp.path().join("book");
    let source = book_root.join("chapters");
    let rendered = book_root.join("dist");
    fs::create_dir_all(&source)?;
    fs::create_dir_all(&rendered)?;
    fs::write(
        book_root.join("book.toml"),
        "[book]\nsrc = \"chapters\"\n[build]\nbuild-dir = \"dist\"\n[output.html]\n",
    )?;
    fs::write(source.join("SUMMARY.md"), "[Intro](intro.md)\n")?;
    fs::write(source.join("intro.md"), "# Intro\n")?;
    fs::write(rendered.join("index.html"), "<h1>Intro</h1>")?;

    let run_sitemap = || {
        Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
            .args(["sitemap", "--base-url", "https://example.test/docs/"])
            .current_dir(temp.path())
            .env("BOOK_ROOT_DIR_PATH", &book_root)
            .env_remove("MARKDOWN_DIR_PATH")
            .env_remove("BOOK_HTML_BUILD_DIR_PATH")
            .env("RUST_LOG", "off")
            .output()
    };
    let first = run_sitemap()?;
    assert_successful_output(&first, "sitemap");
    let sitemap_path = rendered.join("sitemap.xml");
    let expected = concat!(
        "\u{feff}<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n",
        "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
        "  <url>\n",
        "    <loc>https://example.test/docs/index.html</loc>\n",
        "  </url>\n",
        "</urlset>",
    );
    let first_contents = fs::read_to_string(&sitemap_path)?;
    assert_eq!(first_contents, expected);

    let second = run_sitemap()?;
    assert_successful_output(&second, "sitemap");
    assert_eq!(fs::read_to_string(sitemap_path)?, first_contents);
    Ok(())
}

/// Checks CLI errors, standard streams, and output-file effects for both
/// command groups.
#[test]
fn links_and_markdown_failures_report_errors_without_partial_outputs() -> Result<()> {
    let temp = tempdir()?;
    let missing_source = temp.path().join("missing-source");
    let links_output_path = temp.path().join("should-not-exist.md");
    let links_output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["links", "write-all", "--markdown-dir"])
        .arg(&missing_source)
        .arg("--output")
        .arg(&links_output_path)
        .current_dir(temp.path())
        .env_remove("BOOK_ROOT_DIR_PATH")
        .env_remove("MARKDOWN_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?;
    assert!(!links_output.status.success());
    assert!(links_output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&links_output.stderr).contains("does not exist"));
    assert!(!links_output_path.exists());

    let markdown_source = temp.path().join("markdown-source");
    fs::create_dir(&markdown_source)?;
    let chapter = markdown_source.join("chapter.md");
    fs::write(&chapter, "# Chapter\n")?;
    let markdown_output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args([
            "markdown",
            "identify-files-not-in-summary",
            "--markdown-dir",
        ])
        .arg(&markdown_source)
        .current_dir(temp.path())
        .env_remove("BOOK_ROOT_DIR_PATH")
        .env_remove("MARKDOWN_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?;
    assert!(!markdown_output.status.success());
    assert!(
        String::from_utf8_lossy(&markdown_output.stdout).contains("Identifying Markdown files")
    );
    assert!(String::from_utf8_lossy(&markdown_output.stderr).contains("SUMMARY.md not found"));
    assert_eq!(fs::read_to_string(chapter)?, "# Chapter\n");
    Ok(())
}
