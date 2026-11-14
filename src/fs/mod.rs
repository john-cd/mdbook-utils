//! Functions that use the filesystem: create or check existence of
//! directories, read multiple files from a directory, etc.
//!
//! TODO Reviewed ??-??-2026.

/// Directory validation and creation.
mod dir;

/// Deterministic discovery of Markdown source files.
mod find_markdown_files;

/// Path containment checks for existing and planned files.
mod path_validation;

/// Combined Markdown input and mapping of parser offsets back to source files.
mod read_files;

/// Reports for files omitted from a book or unused by examples.
pub(crate) mod unused;

/// Temporary sibling writes and destination replacement.
mod write_atomically;

pub(crate) use dir::*;
pub(crate) use find_markdown_files::*;
pub(crate) use path_validation::*;
pub(crate) use read_files::*;
pub(crate) use write_atomically::*;
