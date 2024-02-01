//! Functions that use the filesystem: create or check existence of
//! directories, read multiple files from a directory, etc
mod dir;
mod find_markdown_files;
mod read_files;

pub(crate) use dir::*;
pub(crate) use find_markdown_files::*;
pub(crate) use read_files::*;
