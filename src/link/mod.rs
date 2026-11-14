//! Markdown (inline or reference-style) [links][crate::link::Link], and
//! Rules to create a reference label and/or a badge URL
//! from a link URL
//!
//! TODO Reviewed ??-??-2026.

/// Parsed link records and their incremental construction.
mod link_and_linkbuilder;

/// Output syntax choices for link reports.
mod link_format;

/// URL patterns for inferred labels and badge metadata.
mod rules;

/// Markdown serialization of links, badges, and reference definitions.
mod write_to_file;

pub(crate) use link_and_linkbuilder::*;
pub use link_format::LinkFormat;
pub(crate) use rules::*;
pub(crate) use write_to_file::*;
