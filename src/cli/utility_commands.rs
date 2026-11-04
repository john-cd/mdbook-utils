//! Top-level CLI commands for sitemap generation and parser diagnostics.
//!
//! TODO Reviewed ??-??-2026

use anyhow::Result;
use console::style;

use super::args::MarkdownSrcDirAndDestFileArgs;
use super::args::MarkdownSrcDirUrlAndDestFileArgs;
use super::config::Configuration;
use super::run_logged;

/// Generate a sitemap from the book's Markdown source and configured output.
pub(crate) fn run_sitemap(
    args: MarkdownSrcDirUrlAndDestFileArgs,
    config: Configuration,
) -> Result<()> {
    let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
    let base_url = config.base_url(args.base)?;
    let sitemap_dest_file_path = config.sitemap_file_path(args.dest)?;
    let map_index = config.sitemap_map_index(args.map_index);
    let paths = [
        ("markdown_source", markdown_src_dir_path.as_path()),
        ("destination", sitemap_dest_file_path.as_path()),
    ];
    run_logged(
        "sitemap",
        &paths,
        &format!(
            "Generating {} from the list of Markdown files in {}...",
            style(sitemap_dest_file_path.display()).cyan(),
            style(markdown_src_dir_path.display()).cyan(),
        ),
        "[main] Failed to generate the sitemap.",
        || {
            mdbook_utils::api::generate_sitemap(
                &markdown_src_dir_path,
                base_url,
                &sitemap_dest_file_path,
                map_index,
            )
        },
    )
}

/// Write the Markdown parser events to a diagnostic file.
pub(crate) fn run_debug(args: MarkdownSrcDirAndDestFileArgs, config: Configuration) -> Result<()> {
    let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
    let log_dest_path = config.dest_file_path(args.dest, "debug.log");
    let paths = [
        ("markdown_source", markdown_src_dir_path.as_path()),
        ("destination", log_dest_path.as_path()),
    ];
    run_logged(
        "debug",
        &paths,
        &format!(
            "Parsing Markdown files in {} and writing raw events to {}...",
            style(markdown_src_dir_path.display()).cyan(),
            style(log_dest_path.display()).cyan()
        ),
        "[main] Failed to generate the debug log.",
        || mdbook_utils::api::debug_parse_to(&markdown_src_dir_path, &log_dest_path),
    )
}
