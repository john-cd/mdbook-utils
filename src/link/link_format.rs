//! Output format for links written by the `links` commands.
//!
//! TODO Reviewed ??-??-2026

use clap::ValueEnum;

/// Markdown syntax used when writing links.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum LinkFormat {
    /// Reference-style links with their reference definitions:
    /// `[text][label]` and `[label]: url "title"`.
    #[default]
    Reference,
    /// Inline links: `[text](url "title")`.
    Inline,
    /// Autolinks: `<url>`. Link text and title are dropped.
    Autolink,
}
