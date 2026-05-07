use std::fs;

use mdbook_utils::generate_categories;
use mdbook_utils::generate_crates;

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
