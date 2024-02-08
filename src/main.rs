//! Command-line interface
use std::env;

use anyhow::Context;
use anyhow::Result;
use tracing::debug;

use crate::cli::Cli;
use crate::cli::Command;

mod cli;

/// Entrypoint of the command-line interface
fn main() -> Result<()> {
    // Load environment variables from a `.env` file (in the current directory or
    // parents), if it exists. If variables with the same names already exist in
    // the environment, their values are preserved.
    let dotenv = dotenvy::dotenv();

    // Set RUST_LOG, if not present, and initialize logging
    let key = "RUST_LOG";
    if env::var(key).is_err() {
        env::set_var(key, "info"); // TODO
    }
    tracing_subscriber::fmt::init();

    match dotenv {
        Ok(pb) => {
            debug!("`.env` file loaded: {:?}", pb);
        }
        Err(e) => {
            debug!("`.env` file not found or not readable: {}", e);
        }
    }

    let Cli {
        command: cmd,
        global_opts,
    } = cli::parse_arguments();

    // Retrieves default configuration (from `book.toml``, env. vars,
    // or hard-coded defaults); also stores global_opts
    let config = cli::config::init(global_opts)?;
    debug!("{:?}", config);

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
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let base_url = config.base_url(args.base)?;
            let sitemap_dest_file_path = config.sitemap_file_path(args.dest);

            println!(
                "Generating {} from the list of Markdown files in {}...",
                sitemap_dest_file_path.display(),
                markdown_src_dir_path.display(),
            );
            mdbook_utils::generate_sitemap(markdown_src_dir_path, base_url, sitemap_dest_file_path)
                .context("[main] Failed to generate the sitemap.")?;
            println!("Done.");
        }
        Command::Debug(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let log_dest_path = config.dest_file_path(args.dest, "debug.log");
            println!(
                "Parsing Markdown files in {} and writing raw events to {}...",
                markdown_src_dir_path.display(),
                log_dest_path.display()
            );
            mdbook_utils::debug_parse_to(markdown_src_dir_path, log_dest_path)
                .context("[main] Failed to generate the debug log.")?;
            println!("Done.");
        }
        Command::Test => {
            mdbook_utils::test().context("[main] Failed to generate the test file.")?;
            println!("Done.");
        } /* Add more subcommands here: Some(args::Commands::...) => { ... }
           * _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
