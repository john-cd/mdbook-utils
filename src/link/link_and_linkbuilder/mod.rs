//! Internal models for a Markdown [Link] and [`LinkBuilder`]
//!
//! TODO Reviewed ??-??-2026.

/// Implements builder behavior for the link and linkbuilder component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod builder;
/// Implements link behavior for the link and linkbuilder component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod link;

pub(crate) use builder::*;
pub(crate) use link::*;
