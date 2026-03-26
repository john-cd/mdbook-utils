//! Parse book.toml
//!
//! Any relative path specified in the configuration
//! will always be taken relative from the root of the book
//! where the configuration file is located.

use std::collections::HashMap;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;

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

/// Parse `book.toml`, the configuration file use by `mdbook`
///
/// book_root_dir_path: the path to the directory that contains `book.toml`
///
/// Returns, if found, `markdown_dir_path`, the directory storing the book's
/// markdown sources, `book_html_build_dir_path`, the directory storing
/// the book's HTML output, and/or `book_markdown_build_dir_path`,
/// the directory storing the book's intermediate, fully expanded Markdown.
///
/// Failure to open `book.toml` or to parse it returns an Error.
pub(crate) fn try_parse_book_toml<P: AsRef<Path>>(
    book_root_dir_path: P,
) -> Result<(PathBuf, Option<PathBuf>, Option<PathBuf>)> {
    let book_toml_path = book_root_dir_path.as_ref().join("book.toml");
    debug!(
        "try_parse_book_toml: book_toml_path: {}",
        book_toml_path.display()
    );

    // Deserialize book.toml, if possible
    let book_toml: BookToml = toml::from_str(&fs::read_to_string(book_toml_path)?)?;

    // By default, the source directory is found in the directory named `src`
    // directly under the root folder.
    let markdown_dir_path = PathBuf::from(book_root_dir_path.as_ref())
        .join(book_toml.book.and_then(|bk| bk.src).unwrap_or("src".into()));

    // By default, the build directory is `book` in the book's root directory.
    let book_build_dir_path = PathBuf::from(book_root_dir_path.as_ref()).join(
        book_toml
            .build
            .and_then(|bld| bld.build_dir)
            .unwrap_or("book".into()),
    );

    let mut book_html_build_dir_path = None;
    let mut book_markdown_build_dir_path = None;

    // Determine output directories to match mdBook's behavior.
    // If there is only one [output.*] backend in `book.toml`, `mdbook`
    // places its output directly in the book directory (see `build.build-dir`).
    // If there is more than one backend, then each backend is placed in a
    // separate directory underneath `build-dir` - for example,
    // directories `book/html` and `book/markdown`.
    // https://rust-lang.github.io/mdBook/format/configuration/renderers.html
    debug!("{:?}", book_toml.output);

    match book_toml.output {
        None => {
            book_html_build_dir_path = Some(book_build_dir_path);
        }
        Some(output) => {
            let extra_count = output.extra.len();

            let get_output_dir = |backend_name: &str| -> PathBuf {
                output
                    .extra
                    .get(backend_name)
                    .and_then(|v| v.get("output-dir"))
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                    .unwrap_or_else(|| {
                        if extra_count <= 1 {
                            PathBuf::new()
                        } else {
                            PathBuf::from(backend_name)
                        }
                    })
            };

            if output.extra.contains_key("html") || extra_count == 0 {
                book_html_build_dir_path = Some(book_build_dir_path.join(get_output_dir("html")));
            }
            if output.extra.contains_key("markdown") {
                book_markdown_build_dir_path =
                    Some(book_build_dir_path.join(get_output_dir("markdown")));
            }
        }
    }
    debug!(
        "try_parse_book_toml: markdown_dir_path: {markdown_dir_path:?}; book_build_dir_path: {book_html_build_dir_path:?}; book_markdown_build_dir_path: {book_markdown_build_dir_path:?}",
    );

    Ok((
        markdown_dir_path,
        book_html_build_dir_path,
        book_markdown_build_dir_path,
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_try_parse_book_toml_default() -> Result<()> {
        let dir = tempdir()?;
        let book_toml_path = dir.path().join("book.toml");
        fs::write(book_toml_path, "[book]\ntitle = \"test\"")?;

        let (src, html, markdown) = try_parse_book_toml(dir.path())?;
        assert_eq!(src, dir.path().join("src"));
        assert_eq!(html, Some(dir.path().join("book")));
        assert_eq!(markdown, None);
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

        let (src, html, markdown) = try_parse_book_toml(dir.path())?;
        assert_eq!(src, dir.path().join("my_src"));
        assert_eq!(html, Some(dir.path().join("my_book")));
        assert_eq!(markdown, None);
        Ok(())
    }

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

        let (_, html, markdown) = try_parse_book_toml(dir.path())?;
        assert_eq!(html, Some(dir.path().join("book").join("html")));
        assert_eq!(markdown, Some(dir.path().join("book").join("markdown")));
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

        let (_, html, markdown) = try_parse_book_toml(dir.path())?;
        assert_eq!(html, None);
        assert_eq!(markdown, Some(dir.path().join("book")));
        Ok(())
    }

    #[test]
    fn test_try_parse_book_toml_only_markdown_output_with_custom_dir() -> Result<()> {
        let dir = tempdir()?;
        let book_toml_path = dir.path().join("book.toml");
        fs::write(
            book_toml_path,
            r#"[output.markdown]
output-dir = "custom"
"#,
        )?;

        let (_, html, markdown) = try_parse_book_toml(dir.path())?;
        assert_eq!(html, None);
        assert_eq!(markdown, Some(dir.path().join("book").join("custom")));
        Ok(())
    }

    #[test]
    fn test_try_parse_book_toml_multiple_outputs_with_custom_dir() -> Result<()> {
        let dir = tempdir()?;
        let book_toml_path = dir.path().join("book.toml");
        fs::write(
            book_toml_path,
            r#"[output.html]
output-dir = "custom_html"
[output.markdown]
output-dir = "custom_markdown"
"#,
        )?;

        let (_, html, markdown) = try_parse_book_toml(dir.path())?;
        assert_eq!(html, Some(dir.path().join("book").join("custom_html")));
        assert_eq!(
            markdown,
            Some(dir.path().join("book").join("custom_markdown"))
        );
        Ok(())
    }

    #[test]
    fn test_try_parse_book_toml_multiple_outputs_including_other() -> Result<()> {
        let dir = tempdir()?;
        let book_toml_path = dir.path().join("book.toml");
        fs::write(
            book_toml_path,
            r#"[output.html]
[output.epub]
"#,
        )?;

        let (_, html, markdown) = try_parse_book_toml(dir.path())?;
        assert_eq!(html, Some(dir.path().join("book").join("html")));
        assert_eq!(markdown, None);
        Ok(())
    }
}
