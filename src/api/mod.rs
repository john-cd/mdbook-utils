//! Reusable public library operations for processing mdBook sources and
//! outputs.
//!
//! The binary's command-line modules handle argument parsing and configuration;
//! callers that need the underlying work can use these functions directly.
//!
//! TODO Reviewed ??-??-2026

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
