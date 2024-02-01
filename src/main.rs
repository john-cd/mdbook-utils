use std::env;

use anyhow::Result;
use cli::*;

mod cli;

fn main() -> Result<()> {
    let key = "RUST_LOG";
    if env::var(key).is_err() {
        env::set_var(key, "info");
    }

    tracing_subscriber::fmt::init();

    let config = config::retrieve_env_vars()?;

    let Cli { command: cmd } = cli::parse_arguments();

    match cmd {
        Command::RefDefs(subcmd) => {
            refdefs_commands::run(subcmd, config)?;
        }
        Command::Links(subcmd) => {
            links_commands::run(subcmd, config)?;
        }
        Command::Markdown(subcmd) => {
            markdown_commands::run(subcmd, config)?;
        }
        Command::SiteMap(args) => {
            let markdown_src_dir_path = config.markdown_dir_path(args.src, "./src/")?;
            let base_url = config.base_url(args.base)?;
            let sitemap_dest_file_path = config.sitemap_file_path(args.dest);

            println!(
                "Generating {} from the list of Markdown files in {}...",
                sitemap_dest_file_path.display(),
                markdown_src_dir_path.display(),
            );
            mdbook_utils::generate_sitemap(
                markdown_src_dir_path,
                base_url,
                sitemap_dest_file_path,
            )?;
            println!("Done.");
        }
        Command::Debug(args) => {
            let markdown_src_dir_path = config.markdown_dir_path(args.src, "./src/")?;
            let log_dest_path = config.dest_file_path(args.dest, "debug.log");
            println!(
                "Parsing Markdown files found in {} and writing raw events to {}...",
                markdown_src_dir_path.display(),
                log_dest_path.display()
            );
            mdbook_utils::debug_parse_to(markdown_src_dir_path, log_dest_path)?;
            println!("Done.");
        }
        Command::Test => {
            mdbook_utils::test()?;
            println!("Done.");
        } /* Add more subcommands here: Some(args::Commands::...) => { ... }
           * _ => {
           *     println!("NOT IMPLEMENTED");
           * } */
    }
    Ok(())
}
