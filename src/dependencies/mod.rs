//! Get the book's examples' dependencies
//!
//! TODO Reviewed ??-??-2026.

/// Implements get dependencies behavior for the dependencies component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod get_dependencies;

pub(crate) use get_dependencies::*;
