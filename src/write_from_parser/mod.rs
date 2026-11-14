//! Functions that take a Markdown parser
//! and write (parts of) its contents to a file.
//!
//! TODO Reviewed ??-??-2026.

/// Implements github behavior for the write from parser component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod github;
/// Implements write raw to behavior for the write from parser component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod write_raw_to;
/// Implements write refdefs behavior for the write from parser component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod write_refdefs;

pub(crate) use github::*;
pub(crate) use write_raw_to::*;
pub(crate) use write_refdefs::*;
