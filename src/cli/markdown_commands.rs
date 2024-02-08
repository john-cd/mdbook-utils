//! Command-line subcommands to manipulate Markdown
use anyhow::Context;
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
    ReplaceCodeExamplesByIncludes(MarkdownSrcDirAndDestDirArgs),

    /// Replace {{#include file.md}} by the file contents
    ReplaceIncludesByContents(MarkdownDirArgs),

    /// Remove {{#include }} statements
    /// (and replace them by a hard-coded string)
    RemoveIncludes(MarkdownDirArgs),

    /// Generate a listing of crates.io dependencies
    /// and write to a Markdown file
    #[allow(dead_code)]
    #[command(skip)]
    GenerateCategories(DestFileArgs),

    /// Generate a crate index and write to a Markdown file
    #[allow(dead_code)]
    #[command(skip)]
    GenerateCrates(MarkdownSrcDirAndDestFileArgs),
    // TODO autoreplace autolinks / inline links by ref links
}

/// Process "markdown" subcommands of the command-line interface
pub(crate) fn run(subcmd: MarkdownSubCommand, config: Configuration) -> Result<()> {
    match subcmd {
        MarkdownSubCommand::ExtractCodeExamples(args) => {
            let markdown_drafts_dir_path = config.markdown_src_dir_path(args.src, "./drafts/")?;
            let code_dest_dir_path = config.dest_dir_path(args.dest);
            println!(
                "Parsing Markdown files in {} and copying found Rust code blocks to {}...",
                markdown_drafts_dir_path.display(),
                code_dest_dir_path.display()
            );
            mdbook_utils::markdown::extract_code_from_all_markdown_files_in(
                markdown_drafts_dir_path,
                code_dest_dir_path,
            )
            .context("[run] Failed to extract code examples.")?;
            println!("Done.");
        }
        MarkdownSubCommand::ReplaceCodeExamplesByIncludes(args) => {
            let markdown_drafts_dir_path = config.markdown_src_dir_path(args.src, "./drafts/")?;
            let code_dir_path = config.dest_dir_path(args.dest);
            println!(
                "About to remove Rust code examples from Markdown files in {}, replacing them with {{#include ... }} statements pointing to code files in {}...",
                markdown_drafts_dir_path.display(),
                code_dir_path.display()
            );
            let confirmation = config.skip_confirm()
                || Confirm::new()
                    .with_prompt(
                        "This command will modify your Markdown files. Do you want to continue?",
                    )
                    .default(false)
                    .interact()
                    .context("Failed to obtain user confirmation.")?;
            if confirmation {
                mdbook_utils::markdown::remove_code_from_all_markdown_files_in(
                    markdown_drafts_dir_path,
                    code_dir_path,
                )
                .context("[run] Failed to remove code from Markdown files.")?;
                println!("Done.");
            } else {
                println!("Cancelled.");
            }
        }
        MarkdownSubCommand::ReplaceIncludesByContents(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args, "./drafts/")?;
            println!(
                "About to parse Markdown files in {} and replace any {{#include <file>.md}} statements by the corresponding file contents (excluding includes of *refs.md files)...",
                markdown_src_dir_path.display()
            );
            let confirmation = config.skip_confirm()
                || Confirm::new()
                    .with_prompt(
                        "This command will modify your Markdown files. Do you want to continue?",
                    )
                    .default(false)
                    .interact()
                    .context("Failed to obtain user confirmation.")?;
            if confirmation {
                mdbook_utils::markdown::include_in_all_markdown_files_in(markdown_src_dir_path)
                    .context("[run] Failed to replace {{#include ...}} statements by contents.")?;
                println!("Done.");
            } else {
                println!("Cancelled.");
            }
        }
        MarkdownSubCommand::RemoveIncludes(args) => {
            let book_markdown_build_dir_path =
                config.book_markdown_build_dir_path(args, "./book/markdown")?;
            println!(
                "About to parse Markdown files in {} and remove any left-over {{#include ...}} statements...",
                book_markdown_build_dir_path.display()
            );
            let confirmation = config.skip_confirm()
                || Confirm::new()
                    .with_prompt(
                        "This command will modify your Markdown files. Do you want to continue?",
                    )
                    .default(false)
                    .interact()
                    .context("Failed to obtain user confirmation.")?;
            if confirmation {
                let contents_to_insert = "// MISSING INCLUDE FILE\nfn main() {}";
                let modified_files =
                    mdbook_utils::markdown::remove_includes_in_all_markdown_files_in(
                        book_markdown_build_dir_path,
                        contents_to_insert,
                    )
                    .context("[run] Failed to remove {{#include ...}} statements.")?;
                for f in modified_files.iter() {
                    println!("Modified: {}", f.display())
                }
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
