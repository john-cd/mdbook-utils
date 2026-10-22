//! Public operations that generate reference definitions and GitHub badges.
//!
//! TODO Reviewed ??-??-2026

use std::path::Path;

use anyhow::Result;

use crate::dependencies;
use crate::generate;
use crate::helper;
use crate::link;
use crate::parser;
use crate::write_from_parser;

/// Visual style for generated GitHub badges.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum BadgeStyle {
    /// The standard flat style used by Shields.io by default.
    #[default]
    Flat,
    /// A flat style with square corners.
    #[value(name = "flat-square")]
    FlatSquare,
    /// A style with a glossy appearance.
    Plastic,
    /// A wider style with uppercase text.
    #[value(name = "for-the-badge")]
    ForTheBadge,
    /// The social style with a larger logo.
    Social,
}

impl BadgeStyle {
    /// Return the query value for non-default Shields.io styles.
    pub(crate) fn shields_query_value(self) -> Option<&'static str> {
        match self {
            Self::Flat => None,
            Self::FlatSquare => Some("flat-square"),
            Self::Plastic => Some("plastic"),
            Self::ForTheBadge => Some("for-the-badge"),
            Self::Social => Some("social"),
        }
    }
}

// REFERENCE DEFINITIONS

/// Parse Markdown from all .md files in a given source directory
/// and write reference definitions found therein to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the destination
/// file cannot be created or written.
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
/// create default flat-style Shields.io badge URLs for these links,
/// and write to a file.
///
/// src_dir_path: path to the source directory.
///
/// dest_file_path: path to the file to create and write into.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the destination
/// file cannot be created or written.
pub fn generate_badges<P1, P2>(src_dir_path: P1, dest_file_path: P2) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    generate_badges_with_style(src_dir_path, dest_file_path, BadgeStyle::Flat)
}

/// Generate GitHub badge reference definitions using a Shields.io style.
///
/// `src_dir_path` is the directory containing Markdown source files.
/// `dest_file_path` is the output file to create or replace.
/// `style` is the visual style applied to generated GitHub badge image URLs.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the destination
/// file cannot be created or written.
pub fn generate_badges_with_style<P1, P2>(
    src_dir_path: P1,
    dest_file_path: P2,
    style: BadgeStyle,
) -> Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    helper(src_dir_path, dest_file_path, |parser, writer| {
        write_from_parser::write_github_repo_badge_refdefs(parser, writer, style)
    })?;
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
///
/// # Errors
///
/// Returns an error if Cargo cannot read the manifest or run `cargo tree`, the
/// Markdown source cannot be read, or the destination cannot be written.
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
    tracing::debug!("loading dependencies from Cargo.toml");
    // Generate ref defs from dependencies
    let deps = dependencies::get_dependencies(&cargo_toml_dir_path, log_file_path)?;
    let mut new_links = generate::generate_refdefs_from(deps);

    // Reads all markdown files in the source directory, then merges existing
    // definitions with newly generated dependency definitions.
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
