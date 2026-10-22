// TODO Reviewed ??-??-2026
//! # mdbook-utils
//!
//! For installation and usage instructions for the `mdbook-utils` command-line
//! tool, consult the [User Guide](https://john-cd.com/mdbook-utils/).
//!
//! A list of available commands is displayed when entering `mdbook-utils` at a
//! shell prompt.
//!
//! The following (<https://docs.rs/mdbook-utils/>) contains the **library API** doc.
//! Please consult the [Public API](https://john-cd.com/mdbook-utils/public_api.html) page as well.
//!
//! You will want to use the Public API (over the CLI) to:
//!
//! - Integrate it in your project, for example call it from a `build.rs` build
//!   script,
//! - Extend its capabilities,
//! - ...

#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
// #![doc(html_favicon_url = "https://example.com/favicon.ico")]
// #![doc(html_logo_url = "https://example.com/logo.jpg")]

pub mod api;
mod dependencies;
mod fs;
mod generate;
mod link;
/// Markdown manipulation modules
pub mod markdown;
mod parser;
mod sitemap;
mod write_from_parser;

use std::path::Path;

use anyhow::Result;
use pulldown_cmark::Parser;

/// Helper function:
///
/// Checks if the source directory exists,
/// parses all the Markdown files in the source directory,
/// invokes a closure that uses the parser to write to an in-memory buffer,
/// and atomically writes the buffer to the destination file (creating its
/// parent directories). The destination is untouched if any step fails.
fn helper<P1, P2, F>(src_dir_path: P1, dest_file_path: P2, func: F) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    F: for<'a, 'b> FnOnce(&'a mut Parser<'a>, &'b mut Vec<u8>) -> Result<()>,
{
    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());

    let mut buffer = Vec::new();
    func(&mut parser, &mut buffer)?;
    fs::write_atomically(dest_file_path.as_ref(), &buffer)
}

/// Flat re-exports of the grouped operations in the [`api`] module.
pub use api::*;
