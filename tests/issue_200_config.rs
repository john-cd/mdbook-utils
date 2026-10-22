//! CLI regression tests for mdBook source and renderer path resolution.

use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use tempfile::tempdir;

/// Runs a links command with an explicit output and returns its process output.
fn write_links(
    markdown_dir: Option<&Path>,
    output_path: &Path,
    book_root: &Path,
) -> Result<std::process::Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"));
    command
        .args(["links", "write-all"])
        .arg("--output")
        .arg(output_path)
        .current_dir(book_root.parent().unwrap_or(book_root))
        .env("BOOK_ROOT_DIR_PATH", book_root)
        .env_remove("MARKDOWN_DIR_PATH")
        .env_remove("BOOK_HTML_BUILD_DIR_PATH")
        .env_remove("BOOK_MARKDOWN_BUILD_DIR_PATH")
        .env("RUST_LOG", "off");
    if let Some(markdown_dir) = markdown_dir {
        command.arg("--markdown-dir").arg(markdown_dir);
    }
    Ok(command.output()?)
}

/// Checks command-line, environment, and book.toml Markdown source precedence.
#[test]
fn markdown_source_precedence_is_cli_then_environment_then_book_toml() -> Result<()> {
    let dir = tempdir()?;
    let book_root = dir.path().join("book");
    let toml_src = book_root.join("toml-src");
    let env_src = dir.path().join("env-src");
    let cli_src = dir.path().join("cli-src");
    fs::create_dir_all(&toml_src)?;
    fs::create_dir(&env_src)?;
    fs::create_dir(&cli_src)?;
    fs::write(book_root.join("book.toml"), "[book]\nsrc = \"toml-src\"\n")?;
    fs::write(
        toml_src.join("chapter.md"),
        "[TOML](https://toml.example/)\n",
    )?;
    fs::write(env_src.join("chapter.md"), "[ENV](https://env.example/)\n")?;
    fs::write(cli_src.join("chapter.md"), "[CLI](https://cli.example/)\n")?;

    let toml_output = dir.path().join("toml-links.md");
    let toml_result = write_links(None, &toml_output, &book_root)?;
    assert!(
        toml_result.status.success(),
        "TOML-selected command failed: {}",
        String::from_utf8_lossy(&toml_result.stderr)
    );
    let toml_links = fs::read_to_string(toml_output)?;
    assert!(toml_links.contains("https://toml.example/"));

    let env_output = dir.path().join("env-links.md");
    let env_result = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["links", "write-all", "--output"])
        .arg(&env_output)
        .current_dir(dir.path())
        .env("BOOK_ROOT_DIR_PATH", &book_root)
        .env("MARKDOWN_DIR_PATH", &env_src)
        .env_remove("BOOK_HTML_BUILD_DIR_PATH")
        .env_remove("BOOK_MARKDOWN_BUILD_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?;
    assert!(
        env_result.status.success(),
        "environment-selected command failed: {}",
        String::from_utf8_lossy(&env_result.stderr)
    );
    let env_links = fs::read_to_string(env_output)?;
    assert!(env_links.contains("https://env.example/"));

    let cli_output = dir.path().join("cli-links.md");
    let cli_result = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["links", "write-all", "--markdown-dir"])
        .arg(&cli_src)
        .arg("--output")
        .arg(&cli_output)
        .current_dir(dir.path())
        .env("BOOK_ROOT_DIR_PATH", &book_root)
        .env("MARKDOWN_DIR_PATH", &env_src)
        .env_remove("BOOK_HTML_BUILD_DIR_PATH")
        .env_remove("BOOK_MARKDOWN_BUILD_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?;
    assert!(
        cli_result.status.success(),
        "command-line-selected command failed: {}",
        String::from_utf8_lossy(&cli_result.stderr)
    );
    let cli_links = fs::read_to_string(cli_output)?;
    assert!(cli_links.contains("https://cli.example/"));
    Ok(())
}

/// Checks a configured multi-renderer sitemap path and rejects malformed TOML.
#[test]
fn sitemap_uses_renderer_output_path_and_reports_invalid_config() -> Result<()> {
    let dir = tempdir()?;
    let book_root = dir.path().join("book");
    let markdown_src = book_root.join("chapters");
    let html_output = book_root.join("dist").join("html");
    fs::create_dir_all(&markdown_src)?;
    fs::create_dir_all(&html_output)?;
    fs::write(
        book_root.join("book.toml"),
        "[book]\nsrc = \"chapters\"\n[build]\nbuild-dir = \"dist\"\n[output.html]\n[output.markdown]\n",
    )?;
    fs::write(
        markdown_src.join("SUMMARY.md"),
        "# Summary\n\n[Intro](README.md)\n",
    )?;
    fs::write(markdown_src.join("README.md"), "# Intro\n")?;
    fs::write(html_output.join("index.html"), "<h1>Intro</h1>")?;

    let output = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["sitemap", "--base-url", "https://example.test/book/"])
        .current_dir(dir.path())
        .env("BOOK_ROOT_DIR_PATH", &book_root)
        .env_remove("MARKDOWN_DIR_PATH")
        .env_remove("BOOK_HTML_BUILD_DIR_PATH")
        .env_remove("BOOK_MARKDOWN_BUILD_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?;
    assert!(
        output.status.success(),
        "sitemap command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let sitemap_path = html_output.join("sitemap.xml");
    let sitemap = fs::read_to_string(&sitemap_path)?;
    assert!(
        sitemap.contains("https://example.test/book/index.html"),
        "unexpected sitemap contents: {sitemap}"
    );

    fs::write(book_root.join("book.toml"), "[build\nbuild-dir = 7")?;
    let invalid_config = Command::new(env!("CARGO_BIN_EXE_mdbook-utils"))
        .args(["links", "write-all", "--output"])
        .arg(dir.path().join("invalid-links.md"))
        .current_dir(dir.path())
        .env("BOOK_ROOT_DIR_PATH", &book_root)
        .env_remove("MARKDOWN_DIR_PATH")
        .env("RUST_LOG", "off")
        .output()?;
    assert!(!invalid_config.status.success());
    assert!(String::from_utf8_lossy(&invalid_config.stderr).contains("book.toml"));
    assert!(!dir.path().join("invalid-links.md").exists());
    Ok(())
}
