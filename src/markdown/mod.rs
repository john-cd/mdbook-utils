//! Operations on Markdown that are not link- or reference-related
//!
//! TODO Reviewed ??-??-2026.

/// Extraction of fenced Rust examples and conversion to mdBook includes.
pub mod extract_code;

/// Removal of mdBook include directives.
pub mod remove_includes;

/// Expansion of mdBook includes, line ranges, and named anchors.
pub mod replace_include;

/// Rust fence discovery and source byte ranges.
mod rust_blocks;

#[doc(inline)]
pub use extract_code::*;
#[doc(inline)]
pub use remove_includes::*;
#[doc(inline)]
pub use replace_include::*;
