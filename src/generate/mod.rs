//! Functions that generate reference definitions
//! e.g. from code dependencies and links.
mod merge_refdefs;
mod refdefs_from_dependencies;

pub(crate) use merge_refdefs::*;
pub(crate) use refdefs_from_dependencies::*;
