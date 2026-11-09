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
//! - Integrate it in your project, for example call it from a `build.rs` build script,
//! - Extend its capabilities,
//! - ...
//!
//! TODO Reviewed ??-??-2026.

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

/// Flat re-exports of the grouped operations in the [`api`] module.
pub use api::*;
