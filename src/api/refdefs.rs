//! Public operations that generate reference definitions and GitHub badges.
//!
//! TODO Reviewed ??-??-2026.

use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;

use crate::dependencies;
use crate::fs;
use crate::generate;
use crate::link;
use crate::parser;
use crate::write_from_parser;

/// Visual style for generated GitHub badges.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BadgeStyle {
    /// The standard flat style used by Shields.io by default.
    #[default]
    Flat,
    /// A flat style with square corners.
    FlatSquare,
    /// A style with a glossy appearance.
    Plastic,
    /// A wider style with uppercase text.
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

/// Render the reference definitions found in all Markdown files in a source directory.
///
/// The caller chooses where and how to write the returned Markdown.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the Markdown
/// output cannot be rendered.
pub fn render_refdefs<P: AsRef<Path>>(src_dir_path: P) -> Result<String> {
    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let mut output = Vec::new();
    write_from_parser::write_refdefs_to(&mut parser, &mut output)?;
    String::from_utf8(output).context("rendered reference definitions were not valid UTF-8")
}

/// Parse Markdown sources and atomically write reference definitions to a file.
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
    let output = render_refdefs(src_dir_path)?;
    fs::write_atomically(dest_file_path.as_ref(), output.as_bytes())
}

/// Render GitHub badge reference definitions found in all Markdown sources.
///
/// The caller chooses where and how to write the returned Markdown.
///
/// # Errors
///
/// Returns an error if the source directory cannot be read or the Markdown
/// output cannot be rendered.
pub fn render_badges<P: AsRef<Path>>(src_dir_path: P, style: BadgeStyle) -> Result<String> {
    let src_dir_path = fs::check_is_dir(src_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(src_dir_path)?;
    let mut parser = parser::get_parser(all_markdown.as_ref());
    let mut output = Vec::new();
    write_from_parser::write_github_repo_badge_refdefs(&mut parser, &mut output, style)?;
    String::from_utf8(output).context("rendered badge reference definitions were not valid UTF-8")
}

/// Parse Markdown sources and atomically write default-style GitHub badge
/// reference definitions to a file.
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

/// Parse Markdown sources and atomically write GitHub badge reference
/// definitions using a Shields.io style.
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
    let output = render_badges(src_dir_path, style)?;
    fs::write_atomically(dest_file_path.as_ref(), output.as_bytes())
}

// GENERATE REF DEFS FROM DEPENDENCIES

/// Render reference definitions from Cargo dependencies and existing sources.
///
/// The caller chooses where and how to write the returned Markdown. An
/// optional `log_file_path` receives the `cargo tree` output.
///
/// # Errors
///
/// Returns an error if Cargo cannot read the manifest or run `cargo tree`, the
/// Markdown source cannot be read, the requested log cannot be written, or the
/// Markdown output cannot be rendered.
pub fn render_dependency_refdefs<P1, P2, P3>(
    cargo_toml_dir_path: P1,
    markdown_dir_path: P2,
    log_file_path: Option<P3>,
) -> Result<String>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
    P3: AsRef<Path>,
{
    tracing::debug!("loading dependencies from Cargo.toml");
    // Generate ref defs from dependencies
    let deps = dependencies::get_dependencies(&cargo_toml_dir_path, log_file_path)?;
    let mut new_links = generate::generate_refdefs_from(deps);

    // Reads all markdown files in the source directory, then merges existing
    // definitions with newly generated dependency definitions.
    let markdown_dir_path = fs::check_is_dir(markdown_dir_path)?;
    let all_markdown = fs::read_to_string_all_markdown_files_in(markdown_dir_path)?;
    let parser = parser::get_parser(all_markdown.as_ref());
    let existing_links = parser::extract_reference_definitions(&parser);
    let links = generate::merge_links(&existing_links, &mut new_links);
    let mut output = Vec::new();
    link::write_refdefs_to(&links, &mut output)?;
    String::from_utf8(output).context("rendered dependency references were not valid UTF-8")
}

/// Render dependency reference definitions and atomically write them to a file.
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
    let output = render_dependency_refdefs(cargo_toml_dir_path, markdown_dir_path, log_file_path)?;
    fs::write_atomically(refdef_dest_file_path.as_ref(), output.as_bytes())
}
