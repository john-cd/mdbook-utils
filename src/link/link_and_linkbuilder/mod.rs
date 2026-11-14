//! Internal models for a Markdown [Link] and [`LinkBuilder`]
//!
//! TODO Reviewed ??-??-2026.

/// Incremental construction from Markdown parser events.
mod builder;

/// Link data, comparison, and Markdown representations.
mod link;

pub(crate) use builder::*;
pub(crate) use link::*;
