use std::path::Path;

use anyhow::Result;

use crate::generate;

// MARKDOWN GENERATION

/// Generate a listing of crates.io categories
/// and write to a Markdown file.
#[tracing::instrument(skip_all)]
pub fn generate_categories<P1: AsRef<Path>, P2: AsRef<Path>>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()> {
    generate::crates::generate_categories(src_dir_path, dest_file_path)
}

/// Generate a crate index and write to a Markdown file.
#[tracing::instrument(skip_all)]
pub fn generate_crates<P1: AsRef<Path>, P2: AsRef<Path>>(
    src_dir_path: P1,
    dest_file_path: P2,
) -> Result<()> {
    generate::crates::generate_crates(src_dir_path, dest_file_path)
}
