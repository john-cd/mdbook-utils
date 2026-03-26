//! Rules to create a reference label and/or a badge URL
//! from a link URL
#![allow(clippy::vec_init_then_push)]

mod regexes;
mod structs;

pub(crate) use regexes::*;
#[allow(unused_imports)]
pub(crate) use structs::*;
