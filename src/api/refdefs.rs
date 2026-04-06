use std::path::Path;

use anyhow::Result;

use crate::dependencies;
use crate::generate;
use crate::helper;
use crate::link;
use crate::parser;
use crate::write_from_parser;

// REFERENCE DEFINITIONS

/// Parse Markdown from all .md files in a given source directory
/// and write reference definitions found therein to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
pub fn write_refdefs_to<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_refdefs_to,
    )?;
    Ok(())
}

/// Parse Markdown from all .md files in a given source directory,
/// extract existing reference definitions,
/// identify URLs that are GitHub repos,
/// create badge URLs for these links,
/// and write to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
pub fn generate_badges<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(
        src_dir_path,
        dest_file_path,
        write_from_parser::write_github_repo_badge_refdefs,
    )?;
    Ok(())
}

// GENERATE REF DEFS FROM DEPENDENCIES

/// Given a Cargo.toml path,
/// generate reference definitions from code dependencies
/// and write them to a file.
///
/// cargo_toml_dir_path: path to the directory containing `Cargo.toml`.
///
/// markdown_dir_path: path to the directory containing Markdown files.
///
/// refdef_dest_file_path: path to the file to create and
/// write into.
///
/// log_file_path: optional path to a log file where the output of `cargo tree`
/// will be written.
pub fn generate_refdefs_to<P1, P2, P3, P4>(
    cargo_toml_dir_path: P1,
    markdown_dir_path: P2,
    refdef_dest_file_path: P3,
    log_file_path: Option<P4>,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
    P4: AsRef<Path>,
{
    // Generate ref defs from dependencies
    let deps = dependencies::get_dependencies(&cargo_toml_dir_path, log_file_path)?;
    // for (_, d) in &deps {
    //     tracing::info!("{d:?}");
    // }
    let mut new_links = generate::generate_refdefs_from(deps);

    // TODO can we read just the *-refs.md files?
    helper(
        markdown_dir_path,
        refdef_dest_file_path,
        move |parser, f| {
            // Read existing ref defs
            let existing_links: Vec<link::Link<'_>> = parser::extract_links(parser);
            let existing_links_static: Vec<link::Link<'static>> =
                existing_links.into_iter().map(|l| l.to_static()).collect();

            let links = generate::merge_links(existing_links_static, &mut new_links);
            link::write_refdefs_to(links, f)?;
            Ok(())
        },
    )?;
    Ok(())
}
