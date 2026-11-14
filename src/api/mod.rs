//! Reusable public library operations for processing mdBook sources and
//! outputs.
//!
//! The binary's command-line modules handle argument parsing, configuration,
//! and output files. Report renderers return owned text so each caller can
//! choose its output destination and writing policy. File-writing convenience
//! operations remain available for callers that prefer that boundary.
//!
//! TODO Reviewed ??-??-2026.

/// Implements categories behavior for the api component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod categories;
/// Implements debug behavior for the api component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod debug;
/// Implements links behavior for the api component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod links;
/// Implements markdown behavior for the api component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod markdown;
/// Implements refdefs behavior for the api component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod refdefs;
/// Implements sitemap behavior for the api component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod sitemap;

pub use categories::*;
pub use debug::*;
pub use links::*;
pub use markdown::*;
pub use refdefs::*;
pub use sitemap::*;
