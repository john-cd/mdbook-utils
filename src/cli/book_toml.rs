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
    src: Option<String>, /* TODO consider std::ffi::OsString - need a custom deserializer?
                          * We don't care about the rest. */
}

/// [build] table
#[derive(Deserialize, Debug)]
pub(crate) struct Build {
    // Directory where the output is placed
    // [build]
    // build-dir = "book"
    #[serde(rename = "build-dir")]
    build_dir: Option<String>,
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
) -> Result<(PathBuf, PathBuf, Option<PathBuf>)> {
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

    let mut book_html_build_dir_path = book_build_dir_path.clone();
    let mut book_markdown_build_dir_path = None;

    // If there is only one [output.*] backend in `book.toml`, `mdbook` places
    // its output directly in the book directory (see `build.build-dir`).
    // If there is more than one backend, then each backend is
    // placed in a separate directory underneath `build-dir`
    // - for example, directories `book/html` and `book/markdown`.
    // https://rust-lang.github.io/mdBook/format/configuration/renderers.html
    debug!("{:?}", book_toml.output);

    if let Some(output) = book_toml.output {
        if output.extra.len() >= 2 {
            book_html_build_dir_path = book_html_build_dir_path.join("html");
        }
        // [output.markdown] is defined in `book.toml`.
        if output.extra.contains_key("markdown") {
            book_markdown_build_dir_path = Some(book_build_dir_path.join("markdown"));
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
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
