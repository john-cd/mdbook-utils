//! Functions that use the filesystem: create or check existence of
//! directories, read multiple files from a directory, etc.
//!
//! TODO Reviewed ??-??-2026.

/// Implements dir behavior for the fs component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod dir;
/// Implements find markdown files behavior for the fs component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod find_markdown_files;
/// Implements path validation behavior for the fs component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod path_validation;
/// Implements read files behavior for the fs component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod read_files;
/// Implements unused behavior for the fs component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
pub(crate) mod unused;
/// Implements write atomically behavior for the fs component, keeping the related rules and data transformations at one boundary. The parent module calls these routines when it needs this component’s behavior.
mod write_atomically;

pub(crate) use dir::*;
pub(crate) use find_markdown_files::*;
pub(crate) use path_validation::*;
pub(crate) use read_files::*;
pub(crate) use write_atomically::*;
