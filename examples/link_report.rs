//! Writes link reports (all, duplicate, broken) for a small Markdown source tree.
//!
//! Run with `cargo run --example link_report`.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::path::Path;

use mdbook_utils::api::write_all_links;
use mdbook_utils::api::write_broken_links;
use mdbook_utils::api::write_duplicate_links;

/// A link-writing library function.
type Writer = fn(&Path, &Path) -> anyhow::Result<()>;

fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let src = dir.path().join("src");
    fs::create_dir(&src)?;
    fs::write(
        src.join("a.md"),
        "[one](https://example.com/a) and [again](https://example.com/a)\n\
         [missing][nowhere]\n",
    )?;

    let writers: [(&str, Writer); 3] = [
        ("all.md", |s, d| write_all_links(s, d)),
        ("duplicates.md", |s, d| write_duplicate_links(s, d)),
        ("broken.md", |s, d| write_broken_links(s, d)),
    ];
    for (name, write) in writers {
        let dest = dir.path().join(name);
        write(&src, &dest)?;
        println!("== {name} ==\n{}", fs::read_to_string(dest)?);
    }
    Ok(())
}
