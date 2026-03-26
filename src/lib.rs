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
mod build_book;
mod dependencies;
mod fs;
mod generate;
mod link;
/// Markdown manipulation modules
pub mod markdown;
mod parser;
mod sitemap;
/// Example Markdown for testing
pub mod test_markdown;
mod write_from_parser;

pub use api::*;

use std::fs::File;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use pulldown_cmark::Parser;

/// Helper function:
///
/// Checks if the source directory exists,
/// create the destination directory if it doesn't exist,
/// create the destination file,
/// parse all the Markdown files in the source directory,
/// and invoke a closure that uses the parser to write to the file.
fn helper<P1, P2, F>(src_dir_path: P1, dest_file_path: P2, func: F) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    F: for<'a, 'b> FnOnce(&'a mut Parser<'a>, &'b mut File) -> Result<()>,
{
    let src_dir_path = fs::check_is_dir(src_dir_path)?;

    fs::create_parent_dir_for(dest_file_path.as_ref())?;

    let mut f = File::create(dest_file_path.as_ref()).with_context(|| {
        format!(
            "[helper] Could not create file {}",
            dest_file_path.as_ref().display()
        )
    })?;

    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());

    func(&mut parser, &mut f)?;
    Ok(())
}
