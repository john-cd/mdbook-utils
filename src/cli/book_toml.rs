// TODO Reviewed ??-??-2026
//! Parse book.toml
//!
//! Any relative path specified in the configuration
//! will always be taken relative from the root of the book
//! where the configuration file is located.

use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use toml::Value;
use tracing::debug;

/// Structs that map to the `book.toml` format
#[derive(Deserialize, Debug)]
pub(crate) struct BookToml {
    // [book] table
    book: Option<Book>,
    // [build] table
    build: Option<Build>,

    // [output.*] tables
    output: Option<Output>,
}

/// [book] table
#[derive(Deserialize, Debug)]
pub(crate) struct Book {
    // Source files
    // [book]
    // src = "src"
    src: Option<PathBuf>,
}

/// [build] table
#[derive(Deserialize, Debug)]
pub(crate) struct Build {
    // Directory where the output is placed
    // [build]
    // build-dir = "book"
    #[serde(rename = "build-dir")]
    build_dir: Option<PathBuf>,
    // We don't care about the rest.
}

/// [output.*] tables
#[derive(Deserialize, Debug)]
pub(crate) struct Output {
    // Capture additional fields
    // https://serde.rs/attr-flatten.html
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

/// Paths resolved from an mdBook configuration file.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BookTomlPaths {
    /// Directory containing the Markdown source files.
    pub(crate) markdown_src: PathBuf,
    /// Directory containing rendered HTML, when the HTML renderer is enabled.
    pub(crate) html_output: Option<PathBuf>,
    /// Directory containing expanded Markdown, when the Markdown renderer is
    /// enabled.
    pub(crate) markdown_output: Option<PathBuf>,
}

/// Parse an optional mdBook `book.toml` and resolve its configured paths.
///
/// Relative paths are resolved from `book_root_dir_path`. A missing file is
/// reported as `Ok(None)`; file access and TOML parsing failures are returned
/// with the configuration path as context.
pub(crate) fn try_parse_book_toml<P: AsRef<Path>>(
    book_root_dir_path: P,
) -> Result<Option<BookTomlPaths>> {
    let book_toml_path = book_root_dir_path.as_ref().join("book.toml");
    debug!(
        book_toml_path = %book_toml_path.display(),
        "reading mdBook configuration"
    );

    let source = match fs::read_to_string(&book_toml_path) {
        Ok(source) => source,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error)
                .with_context(|| format!("Could not read {}", book_toml_path.display()));
        }
    };
    let book_toml: BookToml = toml::from_str(&source)
        .with_context(|| format!("Could not parse {}", book_toml_path.display()))?;

    // By default, the source directory is found in the directory named `src`
    // directly under the root folder.
    let markdown_src = PathBuf::from(book_root_dir_path.as_ref())
        .join(book_toml.book.and_then(|bk| bk.src).unwrap_or("src".into()));

    // By default, the build directory is `book` in the book's root directory.
    let book_build_dir_path = PathBuf::from(book_root_dir_path.as_ref()).join(
        book_toml
            .build
            .and_then(|bld| bld.build_dir)
            .unwrap_or("book".into()),
    );

    let mut html_output = None;
    let mut markdown_output = None;

    // mdBook places its output directly in the book directory if there is only
    // one backend. If there is more than one backend, then each backend is
    // placed in a separate directory underneath `build-dir` (e.g.,
    // `book/html` and `book/markdown`). https://rust-lang.github.io/mdBook/format/configuration/renderers.html
    if let Some(output) = book_toml.output {
        let num_backends = output.extra.len();
        let renderers: Vec<_> = output.extra.keys().map(String::as_str).collect();
        debug!(renderer_count = num_backends, renderers = ?renderers, "mdBook renderers configured");

        if num_backends > 1 {
            if output.extra.contains_key("html") {
                html_output = Some(book_build_dir_path.join("html"));
            }
            if output.extra.contains_key("markdown") {
                markdown_output = Some(book_build_dir_path.join("markdown"));
            }
        } else if num_backends == 1 {
            if output.extra.contains_key("markdown") {
                markdown_output = Some(book_build_dir_path.clone());
            } else if output.extra.contains_key("html") {
                html_output = Some(book_build_dir_path.clone());
            }
        } else {
            // No renderer tables are configured, so mdBook enables HTML by
            // default.
            html_output = Some(book_build_dir_path.clone());
        }
    } else {
        // With no output tables, mdBook enables only HTML by default.
        html_output = Some(book_build_dir_path.clone());
    }
    debug!(
        markdown_src = %markdown_src.display(),
        html_output = ?html_output,
        markdown_output = ?markdown_output,
        "resolved mdBook paths"
    );

    Ok(Some(BookTomlPaths {
        markdown_src,
        html_output,
        markdown_output,
    }))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_try_parse_book_toml_default() -> Result<()> {
        let dir = tempdir()?;
        let book_toml_path = dir.path().join("book.toml");
        fs::write(book_toml_path, "[book]\ntitle = \"test\"")?;

        let paths = try_parse_book_toml(dir.path())?.expect("book.toml was created");
        assert_eq!(paths.markdown_src, dir.path().join("src"));
        assert_eq!(paths.html_output, Some(dir.path().join("book")));
        assert_eq!(paths.markdown_output, None);
        Ok(())
    }

    #[test]
    fn test_try_parse_book_toml_custom_dirs() -> Result<()> {
        let dir = tempdir()?;
        let book_toml_path = dir.path().join("book.toml");
        fs::write(
            book_toml_path,
            r#"[book]
src = "my_src"
[build]
build-dir = "my_book"
"#,
        )?;

        let paths = try_parse_book_toml(dir.path())?.expect("book.toml was created");
        assert_eq!(paths.markdown_src, dir.path().join("my_src"));
        assert_eq!(paths.html_output, Some(dir.path().join("my_book")));
        assert_eq!(paths.markdown_output, None);
        Ok(())
    }

    /// Verifies a sole explicitly configured HTML renderer writes to the build
    /// root.
    #[test]
    fn test_try_parse_book_toml_only_html_output() -> Result<()> {
        let dir = tempdir()?;
        fs::write(dir.path().join("book.toml"), "[output.html]\n")?;

        let paths = try_parse_book_toml(dir.path())?.expect("book.toml was created");
        assert_eq!(paths.html_output, Some(dir.path().join("book")));
        assert_eq!(paths.markdown_output, None);
        Ok(())
    }

    /// Verifies that multiple renderers receive renderer-specific
    /// subdirectories.
    #[test]
    fn test_try_parse_book_toml_multiple_outputs() -> Result<()> {
        let dir = tempdir()?;
        let book_toml_path = dir.path().join("book.toml");
        fs::write(
            book_toml_path,
            r#"[output.html]
[output.markdown]
"#,
        )?;

        let paths = try_parse_book_toml(dir.path())?.expect("book.toml was created");
        assert_eq!(
            paths.html_output,
            Some(dir.path().join("book").join("html"))
        );
        assert_eq!(
            paths.markdown_output,
            Some(dir.path().join("book").join("markdown"))
        );
        Ok(())
    }

    #[test]
    fn test_try_parse_book_toml_only_markdown_output() -> Result<()> {
        let dir = tempdir()?;
        let book_toml_path = dir.path().join("book.toml");
        fs::write(
            book_toml_path,
            r#"[output.markdown]
"#,
        )?;

        let paths = try_parse_book_toml(dir.path())?.expect("book.toml was created");
        assert_eq!(paths.html_output, None);
        assert_eq!(paths.markdown_output, Some(dir.path().join("book")));
        Ok(())
    }

    #[test]
    fn test_try_parse_book_toml_only_other_output() -> Result<()> {
        let dir = tempdir()?;
        let book_toml_path = dir.path().join("book.toml");
        fs::write(
            book_toml_path,
            r#"[output.pdf]
"#,
        )?;

        let paths = try_parse_book_toml(dir.path())?.expect("book.toml was created");
        assert_eq!(paths.html_output, None);
        assert_eq!(paths.markdown_output, None);
        Ok(())
    }

    /// Verifies that an empty output table retains mdBook's default HTML
    /// renderer.
    #[test]
    fn test_try_parse_book_toml_empty_output_uses_default_html_renderer() -> Result<()> {
        let dir = tempdir()?;
        fs::write(dir.path().join("book.toml"), "[output]\n")?;

        let paths = try_parse_book_toml(dir.path())?.expect("book.toml was created");
        assert_eq!(paths.html_output, Some(dir.path().join("book")));
        assert_eq!(paths.markdown_output, None);
        Ok(())
    }

    /// Verifies a missing optional book configuration does not produce an
    /// error.
    #[test]
    fn test_try_parse_book_toml_missing_is_optional() -> Result<()> {
        let dir = tempdir()?;
        assert_eq!(try_parse_book_toml(dir.path())?, None);
        Ok(())
    }

    /// Verifies malformed TOML errors identify the configuration file.
    #[test]
    fn test_try_parse_book_toml_malformed_reports_configuration_path() -> Result<()> {
        let dir = tempdir()?;
        fs::write(dir.path().join("book.toml"), "[build\nbuild-dir = 7")?;

        let error = try_parse_book_toml(dir.path()).expect_err("invalid TOML must fail");
        assert!(format!("{error:#}").contains("book.toml"));
        Ok(())
    }
}
