//! Command-line argument parser
//!
//! Useful links:
//! <https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html>
//!
//! <https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html>
//!
//! <https://github.com/clap-rs/clap/tree/master/examples>
//!
//! TODO Reviewed ??-??-2026

mod args;
mod book_toml;
pub(crate) mod config;
pub(crate) mod links_commands;
pub(crate) mod markdown_commands;
pub(crate) mod refdefs_commands;

use args::MarkdownSrcDirAndDestFileArgs;
use args::MarkdownSrcDirUrlAndDestFileArgs;
use clap::Args;
use clap::Parser;
use clap::Subcommand;
use links_commands::LinksSubCommand;
use markdown_commands::MarkdownSubCommand;
use refdefs_commands::RefDefsSubCommand;

/// Log the paths selected for a CLI command without logging file contents.
pub(crate) fn log_command_paths(command: &'static str, paths: &[(&'static str, &std::path::Path)]) {
    tracing::debug!(
        command,
        paths = ?format_command_paths(paths),
        "selected command paths"
    );
}

/// Log whether a CLI operation succeeded without exposing its error contents.
pub(crate) fn log_command_outcome<T>(
    command: &'static str,
    paths: &[(&'static str, &std::path::Path)],
    result: &anyhow::Result<T>,
) {
    let paths = format_command_paths(paths);
    match result {
        Ok(_) => {
            tracing::info!(command, paths = ?paths, outcome = "success", "CLI command completed");
        }
        Err(_) => {
            tracing::error!(command, paths = ?paths, outcome = "failure", "CLI command failed");
        }
    }
}

/// Run one CLI operation: log the selected paths, print `progress`, run `op`,
/// log the outcome, then print `Done.`.
///
/// `failure` is the context attached to an error from `op`.
pub(crate) fn run_logged(
    command: &'static str,
    paths: &[(&'static str, &std::path::Path)],
    progress: &str,
    failure: &'static str,
    op: impl FnOnce() -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    use anyhow::Context as _;

    log_command_paths(command, paths);
    println!("{progress}");
    let result = op().context(failure);
    log_command_outcome(command, paths, &result);
    result?;
    println!("{}", console::style("Done.").green());
    Ok(())
}

/// Ask the user to confirm a command that modifies files.
///
/// `--yes` answers yes without asking. Without `--yes`, a terminal is
/// required: when standard input is not a terminal the command fails instead
/// of guessing, so a script never modifies files by accident.
///
/// # Errors
///
/// Returns an error when no terminal is available or the prompt fails.
pub(crate) fn confirm(config: &config::Configuration, prompt: &str) -> anyhow::Result<bool> {
    use std::io::IsTerminal as _;

    use anyhow::Context as _;

    if config.skip_confirm() {
        return Ok(true);
    }
    anyhow::ensure!(
        std::io::stdin().is_terminal(),
        "Confirmation is required but standard input is not a terminal; rerun with --yes to proceed."
    );
    dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(prompt)
        .default(false)
        .interact()
        .context("Failed to obtain user confirmation.")
}

/// Log that a user declined a command after its paths were selected.
pub(crate) fn log_command_cancelled(
    command: &'static str,
    paths: &[(&'static str, &std::path::Path)],
) {
    tracing::info!(
        command,
        paths = ?format_command_paths(paths),
        outcome = "cancelled",
        "CLI command cancelled"
    );
}

/// Format named paths for structured tracing fields.
fn format_command_paths(paths: &[(&'static str, &std::path::Path)]) -> Vec<String> {
    paths
        .iter()
        .map(|(name, path)| format!("{name}={}", path.display()))
        .collect()
}

/// Parse command-line arguments
pub(crate) fn parse_arguments() -> Cli {
    Cli::parse()
}

/// Command-line interface for the available commands and global options.
// Reads the following attributes from the package's `Cargo.toml`.
// Displays help when no arguments are provided.
// #[command(arg_required_else_help = true)]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Selected top-level command.
    #[command(subcommand)]
    pub(crate) command: Command,
    // This structure allows the addition of global options, if needed
    /// Options shared by every command.
    #[clap(flatten)]
    pub(crate) global_opts: GlobalOpts,
}

/// Command-line commands
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Manage reference definitions
    #[command(subcommand, name = "refdefs")]
    RefDefs(RefDefsSubCommand),

    /// Manage links
    #[command(subcommand)]
    Links(LinksSubCommand),

    /// Manage code blocks (embedded examples) and includes
    #[command(subcommand)]
    Markdown(MarkdownSubCommand),

    /// Generate a sitemap.xml file from published HTML pages in SUMMARY.md
    #[command(name = "sitemap")]
    SiteMap(MarkdownSrcDirUrlAndDestFileArgs),

    /// Parse the entire Markdown code as events
    /// and write them to a file.
    Debug(MarkdownSrcDirAndDestFileArgs),
}

/// Global options that apply to all (sub)commands
#[derive(Debug, Args, Default)]
pub(crate) struct GlobalOpts {
    /// Automatically answer `yes` to any user confirmation request.
    #[clap(long, short = 'y', global = true)]
    pub(crate) yes: bool,

    /// Set the logging level (error, warn, info, debug, trace).
    #[clap(long, global = true, env = "RUST_LOG")]
    pub(crate) log: Option<String>,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// Lets clap check the command definitions for mistakes.
    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
