//! Command-line interface.
//!
//! TODO Reviewed ??-??-2026.

use std::env;

use anyhow::Result;
use tracing::debug;
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;
use crate::cli::Command;

mod cli;

/// Entrypoint of the command-line interface.
fn main() -> Result<()> {
    // Load environment variables from a `.env` file in the current directory,
    // if it exists. If variables with the same names already
    // exist in the environment, their values are preserved.
    let dotenv = dotenvy::from_filename(".env");

    match dotenv {
        Ok(pb) => {
            debug!("`.env` file loaded: {pb:?}");
        }
        Err(e) => {
            debug!("`.env` file not found or not readable: {e}");
        }
    }

    let Cli {
        command: cmd,
        global_opts,
    } = cli::parse_arguments();

    // Set log level and initialize logging
    let env_filter = if let Some(ref log_level) = global_opts.log {
        EnvFilter::new(log_level)
    } else if let Ok(env_log) = env::var("RUST_LOG") {
        EnvFilter::new(env_log)
    } else {
        EnvFilter::new("info")
    };
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    // Retrieves default configuration (from `book.toml`, env. vars,
    // or hard-coded defaults); also stores global_opts.
    let config = cli::config::init(global_opts)?;
    debug!("CLI configuration initialized");

    match cmd {
        Command::RefDefs(subcmd) => {
            cli::refdefs_commands::run(subcmd, config)?;
        }
        Command::Links(subcmd) => {
            cli::links_commands::run(subcmd, config)?;
        }
        Command::Markdown(subcmd) => {
            cli::markdown_commands::run(subcmd, config)?;
        }
        Command::SiteMap(args) => {
            cli::utility_commands::run_sitemap(args, config)?;
        }
        Command::Debug(args) => {
            cli::utility_commands::run_debug(args, config)?;
        } /* Add more subcommands here: Some(args::Commands::...) => { ... }
           * _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
