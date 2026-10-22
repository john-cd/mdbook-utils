//! Command-line subcommands to handle links
//!
//! TODO Reviewed ??-??-2026

use anyhow::Result;
use clap::Args;
use clap::Subcommand;
use console::style;
use mdbook_utils::api::LinkFormat;

use super::args::MarkdownSrcDirAndDestFileArgs;
use super::config::Configuration;
use super::run_logged;

/// Link syntax, as accepted on the command line.
#[derive(clap::ValueEnum, Debug, Clone, Copy, Default)]
pub(crate) enum LinkFormatArg {
    /// Reference-style links with their reference definitions.
    #[default]
    Reference,
    /// Inline links: `[text](url "title")`.
    Inline,
    /// Autolinks: `<url>`. Link text and title are dropped.
    Autolink,
}

impl From<LinkFormatArg> for LinkFormat {
    fn from(format: LinkFormatArg) -> Self {
        match format {
            LinkFormatArg::Reference => Self::Reference,
            LinkFormatArg::Inline => Self::Inline,
            LinkFormatArg::Autolink => Self::Autolink,
        }
    }
}

/// Arguments of `links write-inline`
#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub(crate) struct WriteInlineArgs {
    /// Source and destination
    #[command(flatten)]
    pub(crate) paths: MarkdownSrcDirAndDestFileArgs,

    /// Syntax used to write the external links found
    #[arg(long = "link-format", value_enum, default_value_t = LinkFormatArg::Reference)]
    pub(crate) link_format: LinkFormatArg,
}

/// Command-line subcommands to handle links
#[derive(Subcommand, Debug)]
pub(crate) enum LinksSubCommand {
    /// Write all existing links to a Markdown file
    WriteAll(MarkdownSrcDirAndDestFileArgs),

    /// Write all existing inline / autolinks (i.e., not
    /// written as reference-style links) to a Markdown file
    WriteInline(WriteInlineArgs),

    /// Identify links that repeat the same label, URL and title (inline links:
    /// the same URL) and write them to a Markdown file
    DuplicateLinks(MarkdownSrcDirAndDestFileArgs),

    /// Identify broken links (i.e. without reference definition) and
    /// write to a Markdown file
    BrokenLinks(MarkdownSrcDirAndDestFileArgs),
}

/// Process "links" subcommands of the command-line interface
pub(crate) fn run(subcmd: LinksSubCommand, config: Configuration) -> Result<()> {
    match subcmd {
        LinksSubCommand::WriteAll(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "all_links.md");
            run_logged(
                "links write-all",
                &[
                    ("source", markdown_src_dir_path.as_path()),
                    ("destination", links_dest_path.as_path()),
                ],
                &format!(
                    "Parsing markdown files in {} and writing existing links to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(links_dest_path.display()).cyan()
                ),
                "[run] Failed to write links to a file.",
                || mdbook_utils::api::write_all_links(&markdown_src_dir_path, &links_dest_path),
            )?;
        }
        LinksSubCommand::WriteInline(args) => {
            let link_format = args.link_format;
            let args = args.paths;
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "inline_links.md");
            run_logged(
                "links write-inline",
                &[
                    ("source", markdown_src_dir_path.as_path()),
                    ("destination", links_dest_path.as_path()),
                ],
                &format!(
                    "Parsing markdown files in {} and writing inline / auto links to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(links_dest_path.display()).cyan()
                ),
                "[run] Failed to write inline links to a file.",
                || {
                    mdbook_utils::api::write_inline_links_with_format(
                        &markdown_src_dir_path,
                        &links_dest_path,
                        link_format.into(),
                    )
                },
            )?;
        }
        LinksSubCommand::DuplicateLinks(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "duplicate_links.md");
            run_logged(
                "links duplicate-links",
                &[
                    ("source", markdown_src_dir_path.as_path()),
                    ("destination", links_dest_path.as_path()),
                ],
                &format!(
                    "Parsing markdown files in {} and writing duplicates links to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(links_dest_path.display()).cyan()
                ),
                "[run] Failed to write duplicate links to a file.",
                || {
                    mdbook_utils::api::write_duplicate_links(
                        &markdown_src_dir_path,
                        &links_dest_path,
                    )
                },
            )?;
        }
        LinksSubCommand::BrokenLinks(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "broken_links.md");
            run_logged(
                "links broken-links",
                &[
                    ("source", markdown_src_dir_path.as_path()),
                    ("destination", links_dest_path.as_path()),
                ],
                &format!(
                    "Parsing markdown files in {} and writing broken links to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(links_dest_path.display()).cyan()
                ),
                "[run] Failed to write broken links to a file.",
                || mdbook_utils::api::write_broken_links(&markdown_src_dir_path, &links_dest_path),
            )?;
        }
    }
    Ok(())
}
