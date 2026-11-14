//! Generates category and crate indexes from a small Markdown source tree.
//!
//! Run with `cargo run --example generate_indexes`.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;

use mdbook_utils::api::generate_categories;
use mdbook_utils::api::generate_crates;

fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let src = dir.path().join("src");
    fs::create_dir(&src)?;
    fs::write(
        src.join("links.md"),
        "[serde](https://crates.io/crates/serde)\n\
         [parsing](https://crates.io/categories/parsing)\n",
    )?;

    let categories = dir.path().join("categories.md");
    let crates = dir.path().join("crates.md");
    generate_categories(&src, &categories)?;
    generate_crates(&src, &crates)?;

    println!("{}", fs::read_to_string(categories)?);
    println!("{}", fs::read_to_string(crates)?);
    Ok(())
}
