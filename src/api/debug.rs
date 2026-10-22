// TODO Reviewed ??-??-2026
//! Public operation that writes raw Markdown parser events for debugging.
use std::path::Path;

use anyhow::Result;

use crate::helper;
use crate::write_from_parser;

// DEBUG

/// Parse Markdown from all .md files in a given source directory and
/// write all raw events to a file for debugging purposes.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the destination
/// file cannot be created or written.
pub fn debug_parse_to<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_raw_to,
    )?;
    Ok(())
}
