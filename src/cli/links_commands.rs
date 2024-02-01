#![allow(dead_code)]

use anyhow::Result;
use clap::Subcommand;

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

pub(crate) fn run(subcmd: LinksSubCommand, config: Configuration) -> Result<()> {
    match subcmd {
        LinksSubCommand::WriteAll(args) => {
            let markdown_src_dir_path = config.markdown_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "all_links.md");
            println!(
                "Parsing markdown files found in {} and writing existing links to {}...",
                markdown_src_dir_path.display(),
                links_dest_path.display()
            );
            mdbook_utils::write_links(markdown_src_dir_path, links_dest_path)?;
            println!("Done.");
        }
        LinksSubCommand::WriteInline(args) => {
            let markdown_src_dir_path = config.markdown_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "inline_links.md");
            println!(
                "Parsing markdown files found in {} and writing found inline / auto links to {}",
                markdown_src_dir_path.display(),
                links_dest_path.display()
            );
            mdbook_utils::write_inline_links(markdown_src_dir_path, links_dest_path)?;
            println!("Done.");
        }
        LinksSubCommand::DuplicateLinks(args) => {
            let markdown_src_dir_path = config.markdown_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "duplicate_links.md");
            println!(
                "Parsing markdown files found in {} and writing duplicates links to {}...",
                markdown_src_dir_path.display(),
                links_dest_path.display()
            );
            mdbook_utils::write_duplicate_links(markdown_src_dir_path, links_dest_path)?;
            println!("Done.");
        }
        LinksSubCommand::BrokenLinks(args) => {
            let markdown_src_dir_path = config.markdown_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "broken_links.md");
            println!(
                "Parsing markdown files found in {} and writing broken links to {}...",
                markdown_src_dir_path.display(),
                links_dest_path.display()
            );
            mdbook_utils::write_broken_links(markdown_src_dir_path, links_dest_path)?;
            println!("Done.");
        } /* _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
