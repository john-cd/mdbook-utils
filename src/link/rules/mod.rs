//! Rules to create a reference label and/or a badge URL
//! from a link URL
//!
//! TODO Reviewed ??-??-2026.

/// Implements regexes behavior for the rules component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod regexes;

pub(crate) use regexes::*;
