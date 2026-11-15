//! Utilities and a command-line application for working with mdBook books.
//!
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

/// Public entry points for scanning and transforming mdBook projects.
pub mod api;

/// Cargo metadata loading and dependency selection.
mod dependencies;

/// Source discovery, path validation, and atomic file replacement.
mod fs;

/// Generators for indexes and reference definitions.
mod generate;

/// Link representation, URL classification, and Markdown rendering.
mod link;

/// Markdown manipulation modules
pub mod markdown;

/// Markdown event parsing and unresolved reference collection.
mod parser;

/// Published page discovery and sitemap generation.
mod sitemap;

/// Writers that consume parsed links and Markdown events.
mod write_from_parser;

/// Flat re-exports of the grouped operations in the [`api`] module.
pub use api::*;
