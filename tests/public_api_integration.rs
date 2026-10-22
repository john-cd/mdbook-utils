// TODO Reviewed ??-??-2026
use std::fs;

use mdbook_utils::api::generate_categories;
use mdbook_utils::api::generate_crates;
use mdbook_utils::api::write_refdefs_to;

/// Generates category and crate indexes through the grouped public API.
#[test]
fn integration_generate_categories_and_crates() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir)?;
    fs::write(
        src_dir.join("links.md"),
        "\
[serde](https://crates.io/crates/serde)\n\
[rayon](https://crates.io/crates/rayon?sort=downloads)\n\
[parsing](https://crates.io/categories/parsing)\n\
[tools](https://crates.io/categories/development-tools/)\n",
    )?;

    let categories_dest = dir.path().join("categories.md");
    let crates_dest = dir.path().join("crates.md");

    generate_categories(&src_dir, &categories_dest)?;
    generate_crates(&src_dir, &crates_dest)?;

    let categories = fs::read_to_string(categories_dest)?;
    let crates = fs::read_to_string(crates_dest)?;

    assert!(
        categories
            .contains("- [development-tools](https://crates.io/categories/development-tools)")
    );
    assert!(categories.contains("- [parsing](https://crates.io/categories/parsing)"));
    assert!(crates.contains("- [rayon](https://crates.io/crates/rayon)"));
    assert!(crates.contains("- [serde](https://crates.io/crates/serde)"));
    Ok(())
}

/// Writes reference definitions from source Markdown through the grouped API.
#[test]
fn integration_write_refdefs() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let src_dir = dir.path().join("src");
    fs::create_dir(&src_dir)?;
    fs::write(
        src_dir.join("references.md"),
        "[mdbook]: https://rust-lang.github.io/mdBook/\n",
    )?;

    let destination = dir.path().join("reference-definitions.md");
    write_refdefs_to(&src_dir, &destination)?;

    let definitions = fs::read_to_string(destination)?;
    assert_eq!(
        definitions,
        "[mdbook]: https://rust-lang.github.io/mdBook/\n"
    );
    Ok(())
}
