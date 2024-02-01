use anyhow::Result;
use clap::Subcommand;
use dialoguer::Confirm;

use super::args::*;
use super::config::Configuration;

/// Command-line subcommands to manipulate Markdown
#[derive(Subcommand, Debug)]
pub(crate) enum MarkdownSubCommand {
    /// Copy Rust code examples from the Markdown into .rs files.
    ExtractCodeExamples(MarkdownSrcDirAndDestDirArgs),

    /// Replace Rust code examples from the Markdown by
    /// {{#include ...}} statements
    ReplaceCodeExamplesByIncludes(MarkdownDirArgs),

    /// Replace {{#include file.md}} by the file contents
    ReplaceIncludesByContents(MarkdownDirArgs),

    /// Generate a listing of crates.io dependencies
    /// and write to a Markdown file
    GenerateCategories(DestFileArgs),

    /// Generate a crate index and write to a Markdown file
    GenerateCrates(MarkdownSrcDirAndDestFileArgs),
    // TODO autoreplace autolinks / inline links by ref links
}

pub(crate) fn run(subcmd: MarkdownSubCommand, config: Configuration) -> Result<()> {
    match subcmd {
        MarkdownSubCommand::ExtractCodeExamples(args) => {
            let markdown_drafts_dir_path = config.markdown_dir_path(args.src, "./drafts/")?;
            let code_dest_dir_path = config.dest_dir_path(args.dest);
            println!(
                "Parsing Markdown files found in {} and copying found Rust code blocks to {}",
                markdown_drafts_dir_path.display(),
                code_dest_dir_path.display()
            );
            mdbook_utils::markdown::extract_code_from_all_markdown_files_in(
                markdown_drafts_dir_path,
                code_dest_dir_path,
            )?;
            println!("Done.");
        }
        MarkdownSubCommand::ReplaceCodeExamplesByIncludes(args) => {
            let markdown_src_dir_path = config.markdown_dir_path(args, "./drafts/")?;
            println!(
                "About to remove Rust code examples from Markdown files in {}, replacing them with {{#include ... }} statements...",
                markdown_src_dir_path.display()
            );
            let confirmation = Confirm::new()
                .with_prompt("Do you want to continue?")
                .default(false)
                .interact()?;
            if confirmation {
                mdbook_utils::markdown::remove_code_from_all_markdown_files_in(
                    markdown_src_dir_path,
                )?;
                println!("Done.");
            } else {
                println!("Cancelled.");
            }
        }
        MarkdownSubCommand::ReplaceIncludesByContents(args) => {
            let markdown_src_dir_path = config.markdown_dir_path(args, "./drafts/")?;
            println!(
                "About to parse Markdown files found in {} and replace any {{#include <file>.md}} statements by the corresponding file contents (excluding includes of *refs.md files)...",
                markdown_src_dir_path.display()
            );
            let confirmation = Confirm::new()
                .with_prompt("Do you want to continue?")
                .default(false)
                .interact()?;
            if confirmation {
                mdbook_utils::markdown::include_in_all_markdown_files_in(markdown_src_dir_path)?;
                println!("Done.");
            } else {
                println!("Cancelled.");
            }
        }
        MarkdownSubCommand::GenerateCategories(args) => {
            let categories_dest_path = config.dest_file_path(args, "categories.md");
            println!(
                "Writing crates.io categories to {}...",
                categories_dest_path.display()
            );
            // TODO
            println!("NOT IMPLEMENTED");
            println!("Done.");
        }
        MarkdownSubCommand::GenerateCrates(args) => {
            let crates_dest_path = config.dest_file_path(args.dest, "crates.md");
            println!("Writing crate index to {}...", crates_dest_path.display());
            // TODO
            println!("NOT IMPLEMENTED");
            println!("Done.");
        } /* _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
