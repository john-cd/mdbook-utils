//! Wall-clock benchmark of the parse and link-check hot path.
//!
//! Uses only `std` (no benchmark framework). Run with `cargo bench`.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::hint::black_box;
use std::path::Path;
use std::time::Instant;

use mdbook_utils::api::write_all_links;
use mdbook_utils::api::write_broken_links;

/// A link-writing library function.
type Writer = fn(&Path, &Path) -> anyhow::Result<()>;

/// Number of Markdown files generated.
const FILES: usize = 200;
/// Links per generated file.
const LINKS_PER_FILE: usize = 50;
/// Timed repetitions per function.
const ITERATIONS: u32 = 10;

fn main() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let src = dir.path().join("src");
    fs::create_dir(&src)?;
    for f in 0..FILES {
        let mut text = String::new();
        for l in 0..LINKS_PER_FILE {
            text.push_str(&format!(
                "[l{f}-{l}](https://example.com/{f}/{l}) [r{l}][ref{l}]\n"
            ));
        }
        fs::write(src.join(format!("f{f}.md")), text)?;
    }
    let dest = dir.path().join("out.md");

    let runs: [(&str, Writer); 2] = [
        ("write_all_links", |s, d| write_all_links(s, d)),
        ("write_broken_links", |s, d| write_broken_links(s, d)),
    ];
    for (name, run) in runs {
        let start = Instant::now();
        for _ in 0..ITERATIONS {
            black_box(run(&src, &dest)?);
        }
        println!(
            "{name}: {:?}/iter ({FILES} files x {LINKS_PER_FILE} links)",
            start.elapsed() / ITERATIONS
        );
    }
    Ok(())
}
