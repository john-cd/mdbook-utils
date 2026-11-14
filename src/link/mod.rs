//! Markdown (inline or reference-style) [links][Link], and
//! Rules to create a reference label and/or a badge URL
//! from a link URL
//!
//! TODO Reviewed ??-??-2026.

/// Implements link and linkbuilder behavior for the link component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod link_and_linkbuilder;
/// Implements link format behavior for the link component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod link_format;
/// Implements rules behavior for the link component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod rules;
/// Implements write to file behavior for the link component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod write_to_file;

pub(crate) use link_and_linkbuilder::*;
pub use link_format::LinkFormat;
pub(crate) use rules::*;
pub(crate) use write_to_file::*;
