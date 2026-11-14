//! Functions that generate reference definitions
//! e.g. from code dependencies and links.
//!
//! TODO Reviewed ??-??-2026.

/// Entry points for generating crates.io indexes from book links.
pub(crate) mod crates;

/// Index name extraction and Markdown rendering.
mod index;

/// Reference definition merging with precedence for existing labels.
mod merge_refdefs;

/// Reference labels and URLs derived from Cargo dependencies.
mod refdefs_from_dependencies;

pub(crate) use merge_refdefs::*;
pub(crate) use refdefs_from_dependencies::*;
