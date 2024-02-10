//! Command-line subcommands to handle links
#![allow(dead_code)]

use anyhow::Context;
use anyhow::Result;
use clap::Subcommand;
use console::style;

use super::args::*;
use super::config::Configuration;

/// Command-line subcommands to handle links
#[derive(Subcommand, Debug)]
pub(crate) enum LinksSubCommand {
    /// Write all existing links to a Markdown file
    WriteAll(MarkdownSrcDirAndDestFileArgs),

    /// Write all existing inline / autolinks (i.e., not
    /// written as reference-style links) to a Markdown file
    WriteInline(MarkdownSrcDirAndDestFileArgs),

    /// Identify duplicate links / labels and write to a Markdown file
    #[command(skip)]
    DuplicateLinks(MarkdownSrcDirAndDestFileArgs),

    /// Identify broken links (i.e. without reference definition) and
    /// write to a Markdown file
    #[command(skip)]
    BrokenLinks(MarkdownSrcDirAndDestFileArgs),
}

/// Process "links" subcommands of the command-line interface
pub(crate) fn run(subcmd: LinksSubCommand, config: Configuration) -> Result<()> {
    match subcmd {
        LinksSubCommand::WriteAll(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "all_links.md");
            println!(
                "Parsing markdown files in {} and writing existing links to {}...",
                style(markdown_src_dir_path.display()).cyan(),
                style(links_dest_path.display()).cyan()
            );
            mdbook_utils::write_all_links(markdown_src_dir_path, links_dest_path)
                .context("[run] Failed to write links to a file.")?;
            println!("{}", style("Done.").green());
        }
        LinksSubCommand::WriteInline(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "inline_links.md");
            println!(
                "Parsing markdown files in {} and writing inline / auto links to {}...",
                style(markdown_src_dir_path.display()).cyan(),
                style(links_dest_path.display()).cyan()
            );
            mdbook_utils::write_inline_links(markdown_src_dir_path, links_dest_path)
                .context("[run] Failed to write inline links to a file.")?;
            println!("{}", style("Done.").green());
        }
        LinksSubCommand::DuplicateLinks(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "duplicate_links.md");
            println!(
                "Parsing markdown files in {} and writing duplicates links to {}...",
                style(markdown_src_dir_path.display()).cyan(),
                style(links_dest_path.display()).cyan()
            );
            mdbook_utils::write_duplicate_links(markdown_src_dir_path, links_dest_path)
                .context("[run] Failed to write duplicate links to a file.")?;
            println!("{}", style("Done.").green());
        }
        LinksSubCommand::BrokenLinks(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "broken_links.md");
            println!(
                "Parsing markdown files in {} and writing broken links to {}...",
                style(markdown_src_dir_path.display()).cyan(),
                style(links_dest_path.display()).cyan()
            );
            mdbook_utils::write_broken_links(markdown_src_dir_path, links_dest_path)
                .context("[run] Failed to write broken links to a file.")?;
            println!("{}", style("Done.").green());
        } /* _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
