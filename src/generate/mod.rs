//! Functions that generate reference definitions
//! e.g. from code dependencies and links.
//!
//! TODO Reviewed ??-??-2026.

/// Implements crates behavior for the generate component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
pub(crate) mod crates;
/// Implements index behavior for the generate component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod index;
/// Implements merge refdefs behavior for the generate component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod merge_refdefs;
/// Implements refdefs from dependencies behavior for the generate component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod refdefs_from_dependencies;

pub(crate) use merge_refdefs::*;
pub(crate) use refdefs_from_dependencies::*;
