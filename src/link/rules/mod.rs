//! Rules to create a reference label and/or a badge URL
//! from a link URL
//!
//! TODO Reviewed ??-??-2026.

/// URL classification patterns and their compiled forms.
mod regexes;

pub(crate) use regexes::*;
