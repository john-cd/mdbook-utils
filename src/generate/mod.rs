//! Functions that generate reference definitions
//! e.g. from code dependencies and links.
pub mod crates;
mod merge_refdefs;
mod refdefs_from_dependencies;

pub use crates::*;
pub(crate) use merge_refdefs::*;
pub(crate) use refdefs_from_dependencies::*;
