//! Functions that take a Markdown parser
//! and write (parts of) its contents to a file.
//!
//! TODO Reviewed ??-??-2026.

/// GitHub badge URLs derived from repository links.
mod github;

/// Human-readable dumps of Markdown parser events.
mod write_raw_to;

/// Markdown serialization of reference definitions.
mod write_refdefs;

pub(crate) use github::*;
pub(crate) use write_raw_to::*;
pub(crate) use write_refdefs::*;
