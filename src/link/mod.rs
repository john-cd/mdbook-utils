//! Markdown (inline or reference-style) [links][Link], and
//! Rules to create a reference label and/or a badge URL
//! from a link URL
mod link_and_linkbuilder;
mod rules;
mod write_to_file;

pub(crate) use link_and_linkbuilder::*;
pub(crate) use rules::*;
pub(crate) use write_to_file::*;
