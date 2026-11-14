//! Reusable public library operations for processing mdBook sources and
//! outputs.
//!
//! The binary's command-line modules handle argument parsing, configuration,
//! and output files. Report renderers return owned text so each caller can
//! choose its output destination and writing policy. File-writing convenience
//! operations remain available for callers that prefer that boundary.
//!
//! TODO Reviewed ??-??-2026.

/// Indexes of linked crates.io categories.
mod categories;

/// Markdown parser event reports.
mod debug;

/// Link inventories, duplicate detection, and unresolved reference reports.
mod links;

/// Public exports for Markdown scans and transformations.
mod markdown;

/// Reference definition generation, merging, and output.
mod refdefs;

/// Sitemap generation from a book's summary and rendered pages.
mod sitemap;

pub use categories::*;
pub use debug::*;
pub use links::*;
pub use markdown::*;
pub use refdefs::*;
pub use sitemap::*;
