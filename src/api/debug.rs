use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;

use crate::fs;
use crate::helper;
use crate::parser;
use crate::test_markdown;
use crate::write_from_parser;

// DEBUG

/// Parse Markdown from all .md files in a given source directory and
/// write all raw events to a file for debugging purposes.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
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

/// Test function that uses fake Markdown and writes events to
/// `./book/temp/test.log`.
pub fn test() -> Result<()> {
    fs::create_dir("./book/temp/")?;

    let dest_file_path = "./book/temp/test.log";
    let mut f = BufWriter::new(File::create(dest_file_path).context(
        "[test] Failed to create the destination file. Does the full directory path exist?",
    )?);

    let test_markdown = test_markdown::get_test_markdown();
    let mut parser = parser::get_parser(test_markdown.as_ref());
    write_from_parser::write_raw_to(&mut parser, &mut f)?;
    f.flush()
        .context("Not all bytes could be written due to I/O errors or EOF being reached.")?;
    Ok(())
}
