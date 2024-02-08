//! Functions that take a Markdown parser
//! and write (parts of) its contents to a file
mod github;
mod write_raw_to;
mod write_refdefs;
mod write_whole;

pub(crate) use github::*;
pub(crate) use write_raw_to::*;
pub(crate) use write_refdefs::*;
#[allow(unused_imports)]
pub(crate) use write_whole::*;
