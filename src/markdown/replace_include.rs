//! Replace {{#include file.md}} by the file contents
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::info;

/// Regex to find {{#include \<file\>.md}}.
static INSERT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{\{#include (?<filepath>\S+?\.md)\}\}").unwrap());

///  Within each mdBook-style Markdown file in a source directory,
/// replace {{#include file.md}} statements by the contents of the
/// included file.
///
/// See the [mdBook documentation](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files).
///
/// Note: {{#include *-refs.md}} are excluded.
///
/// markdown_src_dir_path: path to the source directory containing the
/// Markdown files.
pub fn include_in_all_markdown_files_in<P>(markdown_src_dir_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    // Locate the Markdown files with the src directory
    let paths = crate::fs::find_markdown_files_in(markdown_src_dir_path.as_ref())?;

    // Process each .md file
    for p in paths {
        info!("Looking into {p:?}");
        let parent_dir = p.parent().unwrap().to_string_lossy();
        let buf = fs::read_to_string(p.as_path())?;
        if INSERT_REGEX.is_match(&buf) {
            let mut new_txt = String::with_capacity(buf.len());
            let mut last_match = 0;
            let mut modified = false;

            for cap in INSERT_REGEX.captures_iter(&buf) {
                let m = cap.get(0).unwrap();
                new_txt.push_str(&buf[last_match..m.start()]);

                let rel_file_path = cap.name("filepath").unwrap().as_str();
                // debug!("relative file path: {rel_file_path:?}");
                if !rel_file_path.ends_with("refs.md") {
                    let path_file_to_insert = Path::new(parent_dir.as_ref()).join(rel_file_path);
                    info!("Insert {path_file_to_insert:?}");
                    let contents_to_insert = fs::read_to_string(path_file_to_insert)?;
                    // debug!("\n{}", contents_to_insert);
                    // debug!("{}", m.as_str());
                    new_txt.push_str(&contents_to_insert);
                    modified = true;
                } else {
                    info!("Ignored");
                    new_txt.push_str(m.as_str());
                }
                last_match = m.end();
            }
            new_txt.push_str(&buf[last_match..]);

            if modified {
                // debug!("{}",  new_txt);
                File::create(p)?.write_all(new_txt.as_bytes())?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_include_in_all_markdown_files_in() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let main_md_path = src_dir.join("main.md");
        let include1_md_path = src_dir.join("include1.md");
        let refs_md_path = src_dir.join("some-refs.md");
        let sub_dir = src_dir.join("sub");
        fs::create_dir(&sub_dir)?;
        let include2_md_path = sub_dir.join("include2.md");

        fs::write(&include1_md_path, "Content of include1")?;
        fs::write(&refs_md_path, "Content of refs")?;
        fs::write(&include2_md_path, "Content of include2")?;

        let main_content = r#"
# Main
{{#include include1.md}}
{{#include some-refs.md}}
{{#include sub/include2.md}}
"#;
        fs::write(&main_md_path, main_content)?;

        include_in_all_markdown_files_in(&src_dir)?;

        let updated_content = fs::read_to_string(&main_md_path)?;
        assert!(updated_content.contains("Content of include1"));
        assert!(updated_content.contains("{{#include some-refs.md}}"));
        assert!(!updated_content.contains("Content of refs"));
        assert!(updated_content.contains("Content of include2"));
        assert!(!updated_content.contains("{{#include include1.md}}"));
        assert!(!updated_content.contains("{{#include sub/include2.md}}"));

        Ok(())
    }
}
