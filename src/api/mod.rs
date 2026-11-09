//! Reusable public library operations for processing mdBook sources and
//! outputs.
//!
//! The binary's command-line modules handle argument parsing, configuration,
//! and output files. Report renderers return owned text so each caller can
//! choose its output destination and writing policy. File-writing convenience
//! operations remain available for callers that prefer that boundary.
//!
//! TODO Reviewed ??-??-2026.

mod categories;
mod debug;
mod links;
mod markdown;
mod refdefs;
mod sitemap;

pub use categories::*;
pub use debug::*;
pub use links::*;
pub use markdown::*;
pub use refdefs::*;
pub use sitemap::*;
