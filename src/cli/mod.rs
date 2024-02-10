//! Command-line argument parser
//!
//! Useful links:
//! <https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html>
//!
//! <https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html>
//!
//! <https://github.com/clap-rs/clap/tree/master/examples>

mod args;
mod book_toml;
pub(crate) mod config;
// mod interact;
pub(crate) mod links_commands;
pub(crate) mod markdown_commands;
pub(crate) mod refdefs_commands;

use args::*;
use clap::Args;
use clap::Parser;
use clap::Subcommand;
use links_commands::LinksSubCommand;
use markdown_commands::MarkdownSubCommand;
use refdefs_commands::RefDefsSubCommand;

/// Parse command-line arguments
pub(crate) fn parse_arguments() -> Cli {
    Cli::parse()
}

#[derive(Parser, Debug)]
// Reads the following attributes from the package's `Cargo.toml`
#[command(author, version, about, long_about = None)]
// Displays the help, if no arguments are provided
// #[command(arg_required_else_help = true)]
/// Command-line interface: commands and global options
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
    // This structure allows the addition of global options, if needed
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

    /// Generate a sitemap.xml file from the list of Markdown files
    /// in a source directory
    #[command(name = "sitemap")]
    SiteMap(MarkdownSrcDirUrlAndDestFileArgs),

    /// Parse the entire Markdown code as events
    /// and write them to a file.
    Debug(MarkdownSrcDirAndDestFileArgs),

    /// Test Markdown parsing
    #[allow(dead_code)]
    #[command(skip)]
    Test,
}

/// Global options that apply to all (sub)commands
#[derive(Debug, Args, Default)]
pub(crate) struct GlobalOpts {
    /// Automatically answer `yes` to any user confirmation request.
    #[clap(long, short = 'y', global = true)]
    pub(crate) yes: bool,
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test() {
    // }
}
