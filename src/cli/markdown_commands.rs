// TODO Reviewed ??-??-2026
//! Command-line subcommands to manipulate Markdown
use anyhow::Context;
use anyhow::Result;
use clap::Subcommand;
use console::style;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;

use super::args::*;
use super::config::Configuration;
use super::log_command_cancelled;
use super::log_command_outcome;
use super::log_command_paths;

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
    GenerateCategories(MarkdownSrcDirAndDestFileArgs),

    /// Generate a crate index and write to a Markdown file
    GenerateCrates(MarkdownSrcDirAndDestFileArgs),
    /// Identify .md files not in SUMMARY.md
    IdentifyFilesNotInSummary(MarkdownDirArgs),

    /// Identify .rs examples not used in Markdown files
    IdentifyUnusedRsExamples(MarkdownSrcDirAndDestDirArgs),
}

/// Process "markdown" subcommands of the command-line interface
pub(crate) fn run(subcmd: MarkdownSubCommand, config: Configuration) -> Result<()> {
    match subcmd {
        MarkdownSubCommand::ExtractCodeExamples(args) => {
            let markdown_drafts_dir_path = config.markdown_src_dir_path(args.src, "./drafts/")?;
            let code_dest_dir_path = config.dest_dir_path(args.dest);
            let paths = [
                ("markdown_source", markdown_drafts_dir_path.as_path()),
                ("code_destination", code_dest_dir_path.as_path()),
            ];
            log_command_paths("markdown extract-code-examples", &paths);
            println!(
                "Parsing Markdown files in {} and copying found Rust code blocks to {}...",
                style(markdown_drafts_dir_path.display()).cyan(),
                style(code_dest_dir_path.display()).cyan(),
            );
            let result = mdbook_utils::markdown::extract_code_from_all_markdown_files_in(
                &markdown_drafts_dir_path,
                &code_dest_dir_path,
            )
            .context("[run] Failed to extract code examples.");
            log_command_outcome("markdown extract-code-examples", &paths, &result);
            result?;
            println!("{}", style("Done.").green());
        }
        MarkdownSubCommand::ReplaceCodeExamplesByIncludes(args) => {
            let markdown_drafts_dir_path = config.markdown_src_dir_path(args.src, "./drafts/")?;
            let code_dir_path = config.dest_dir_path(args.dest);
            let paths = [
                ("markdown_source", markdown_drafts_dir_path.as_path()),
                ("code_source", code_dir_path.as_path()),
            ];
            log_command_paths("markdown replace-code-examples-by-includes", &paths);
            println!(
                "About to remove Rust code examples from Markdown files in {}, replacing them with {{#include ... }} statements pointing to code files in {}...",
                style(markdown_drafts_dir_path.display()).cyan(),
                style(code_dir_path.display()).cyan()
            );
            let confirmation = config.skip_confirm()
                || Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "This command will modify your Markdown files. Do you want to continue?",
                    )
                    .default(false)
                    .interact()
                    .context("Failed to obtain user confirmation.")?;
            if confirmation {
                let result = mdbook_utils::markdown::remove_code_from_all_markdown_files_in(
                    &markdown_drafts_dir_path,
                    &code_dir_path,
                )
                .context("[run] Failed to remove code from Markdown files.");
                log_command_outcome(
                    "markdown replace-code-examples-by-includes",
                    &paths,
                    &result,
                );
                result?;
                println!("{}", style("Done.").green());
            } else {
                log_command_cancelled("markdown replace-code-examples-by-includes", &paths);
                println!("{}", style("Cancelled.").yellow());
            }
        }
        MarkdownSubCommand::ReplaceIncludesByContents(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args, "./drafts/")?;
            let paths = [("markdown_directory", markdown_src_dir_path.as_path())];
            log_command_paths("markdown replace-includes-by-contents", &paths);
            println!(
                "About to parse Markdown files in {} and replace any {{#include <file>.md}} statements by the corresponding file contents (excluding includes of *refs.md files)...",
                style(markdown_src_dir_path.display()).cyan()
            );
            let confirmation = config.skip_confirm()
                || Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "This command will modify your Markdown files. Do you want to continue?",
                    )
                    .default(false)
                    .interact()
                    .context("Failed to obtain user confirmation.")?;
            if confirmation {
                let result = mdbook_utils::markdown::include_in_all_markdown_files_in(
                    &markdown_src_dir_path,
                )
                .context("[run] Failed to replace {{#include ...}} statements by contents.");
                log_command_outcome("markdown replace-includes-by-contents", &paths, &result);
                result?;
                println!("{}", style("Done.").green());
            } else {
                log_command_cancelled("markdown replace-includes-by-contents", &paths);
                println!("{}", style("Cancelled.").yellow());
            }
        }
        MarkdownSubCommand::RemoveIncludes(args) => {
            let book_markdown_build_dir_path =
                config.book_markdown_build_dir_path(args, "./book/markdown")?;
            let paths = [("markdown_directory", book_markdown_build_dir_path.as_path())];
            log_command_paths("markdown remove-includes", &paths);
            println!(
                "About to parse Markdown files in {} and remove any left-over {{#include ...}} statements...",
                style(book_markdown_build_dir_path.display()).cyan()
            );
            let confirmation = config.skip_confirm()
                || Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        "This command will modify your Markdown files. Do you want to continue?",
                    )
                    .default(false)
                    .interact()
                    .context("Failed to obtain user confirmation.")?;
            if confirmation {
                let contents_to_insert = "// MISSING INCLUDE FILE\nfn main() {}";
                let result = mdbook_utils::markdown::remove_includes_in_all_markdown_files_in(
                    &book_markdown_build_dir_path,
                    contents_to_insert,
                )
                .context("[run] Failed to remove {{#include ...}} statements.");
                log_command_outcome("markdown remove-includes", &paths, &result);
                let modified_files = result?;
                for f in modified_files.iter() {
                    println!("Modified: {}", style(f.display()).cyan())
                }
                println!("{}", style("Done.").green());
            } else {
                log_command_cancelled("markdown remove-includes", &paths);
                println!("{}", style("Cancelled.").yellow());
            }
        }
        MarkdownSubCommand::GenerateCategories(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let categories_dest_path = config.dest_file_path(args.dest, "categories.md");
            let paths = [
                ("markdown_source", markdown_src_dir_path.as_path()),
                ("destination", categories_dest_path.as_path()),
            ];
            log_command_paths("markdown generate-categories", &paths);
            println!(
                "Writing crates.io categories to {} from Markdown sources in {}...",
                style(categories_dest_path.display()).cyan(),
                style(markdown_src_dir_path.display()).cyan(),
            );
            let result = mdbook_utils::api::generate_categories(
                &markdown_src_dir_path,
                &categories_dest_path,
            )
            .context("[run] Failed to generate categories.");
            log_command_outcome("markdown generate-categories", &paths, &result);
            result?;
            println!("{}", style("Done.").green());
        }
        MarkdownSubCommand::GenerateCrates(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let crates_dest_path = config.dest_file_path(args.dest, "crates.md");
            let paths = [
                ("markdown_source", markdown_src_dir_path.as_path()),
                ("destination", crates_dest_path.as_path()),
            ];
            log_command_paths("markdown generate-crates", &paths);
            println!(
                "Writing crate index to {} from Markdown sources in {}...",
                style(crates_dest_path.display()).cyan(),
                style(markdown_src_dir_path.display()).cyan()
            );
            let result =
                mdbook_utils::api::generate_crates(&markdown_src_dir_path, &crates_dest_path)
                    .context("[run] Failed to generate crate index.");
            log_command_outcome("markdown generate-crates", &paths, &result);
            result?;
            println!("{}", style("Done.").green());
        }
        MarkdownSubCommand::IdentifyFilesNotInSummary(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args, "./src/")?;
            let paths = [("markdown_source", markdown_src_dir_path.as_path())];
            log_command_paths("markdown identify-files-not-in-summary", &paths);
            println!(
                "Identifying Markdown files in {} not in SUMMARY.md...",
                style(markdown_src_dir_path.display()).cyan(),
            );
            let result = mdbook_utils::api::identify_files_not_in_summary(&markdown_src_dir_path)
                .context("[run] Failed to identify files not in SUMMARY.md.");
            log_command_outcome("markdown identify-files-not-in-summary", &paths, &result);
            let missing = result?;
            if missing.is_empty() {
                println!("{}", style("All files are in SUMMARY.md.").green());
            } else {
                println!("{}", style("Files not in SUMMARY.md:").yellow());
                for f in missing {
                    println!("{}", f.display());
                }
            }
        }
        MarkdownSubCommand::IdentifyUnusedRsExamples(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let code_dir_path = config.dest_dir_path(args.dest);
            let paths = [
                ("markdown_source", markdown_src_dir_path.as_path()),
                ("code_directory", code_dir_path.as_path()),
            ];
            log_command_paths("markdown identify-unused-rs-examples", &paths);
            println!(
                "Identifying .rs files in {} not used in Markdown files in {}...",
                style(code_dir_path.display()).cyan(),
                style(markdown_src_dir_path.display()).cyan(),
            );
            let result = mdbook_utils::api::identify_unused_rs_examples(
                &markdown_src_dir_path,
                &code_dir_path,
            )
            .context("[run] Failed to identify unused .rs examples.");
            log_command_outcome("markdown identify-unused-rs-examples", &paths, &result);
            let unused = result?;
            if unused.is_empty() {
                println!("{}", style("All .rs files are used.").green());
            } else {
                println!("{}", style("Unused .rs files:").yellow());
                for f in unused {
                    println!("{}", f.display());
                }
            }
        } /* _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
