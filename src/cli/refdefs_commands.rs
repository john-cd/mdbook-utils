//! Command-line subcommands to handle reference definitions
use anyhow::Context;
use anyhow::Result;
use clap::Subcommand;
use console::style;

use super::args::*;
use super::config::Configuration;

/// Command-line subcommands to handle reference definitions
#[derive(Subcommand, Debug)]
pub(crate) enum RefDefsSubCommand {
    /// Write existing reference definitions to a file
    Write(MarkdownSrcDirAndDestFileArgs),

    /// Generate reference definitions
    /// (e.g. badges for Github links or from the dependencies of the code examples)
    #[command(name = "generate")]
    Generate(GenerateArgs),
}

/// Arguments for generating reference definitions
#[derive(Debug, clap::Args)]
pub(crate) struct GenerateArgs {
    /// Generate badges (reference definitions) for e.g. Github links
    #[arg(long)]
    pub(crate) badges: bool,

    /// Generate reference definitions from the dependencies of the code examples
    #[arg(long = "from-dependencies")]
    pub(crate) from_dependencies: bool,

    #[command(flatten)]
    pub(crate) args: DependenciesDirAndDestFileArgs,
}

/// "refdefs" subcommands of the command-line interface
pub(crate) fn run(subcmd: RefDefsSubCommand, config: Configuration) -> Result<()> {
    match subcmd {
        RefDefsSubCommand::Write(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let refdef_dest_path = config.dest_file_path(args.dest, "existing_refs.md");
            println!(
                "Parsing markdown files in {} and writing existing reference definitions to {}...",
                style(markdown_src_dir_path.display()).cyan(),
                style(refdef_dest_path.display()).cyan()
            );
            mdbook_utils::write_refdefs_to(markdown_src_dir_path, refdef_dest_path)
                .context("[run] Failed to write reference definitions to a file.")?;
            println!("{}", style("Done.").green());
        }
        RefDefsSubCommand::Generate(args) => {
            let mut run_badges = args.badges;
            let mut run_deps = args.from_dependencies;

            // If neither is specified, we default to running both
            if !run_badges && !run_deps {
                run_badges = true;
                run_deps = true;
            }

            if run_badges {
                // If we also run dependencies, the output paths might clash if not explicitly set.
                // We'll use "badge_refs.md" as the default name.
                let markdown_src_dir_path =
                    config.markdown_src_dir_path(args.args.src.clone(), "./src/")?;
                let refdef_dest_path =
                    config.dest_file_path(args.args.dest.clone(), "badge_refs.md");
                println!(
                    "Parsing markdown files in {} and writing new (github badge) reference definitions to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(refdef_dest_path.display()).cyan()
                );
                mdbook_utils::generate_badges(markdown_src_dir_path, refdef_dest_path)
                    .context("[run] Failed to generate badges.")?;
                println!("{}", style("Done.").green());
            }

            if run_deps {
                let markdown_src_dir_path =
                    config.markdown_src_dir_path(args.args.src, "./src/")?;
                let cargo_toml_dir_path = config.cargo_toml_dir_path(args.args.manifest)?;
                let refdef_dest_file_path =
                    config.dest_file_path(args.args.dest, "dependencies_refs.md");
                println!(
                    "Creating reference definitions in {} from the manifest in {} and Markdown sources in {}...",
                    style(refdef_dest_file_path.display()).cyan(),
                    style(cargo_toml_dir_path.display()).cyan(),
                    style(markdown_src_dir_path.display()).cyan(),
                );
                mdbook_utils::generate_refdefs_to(
                    cargo_toml_dir_path,
                    markdown_src_dir_path,
                    refdef_dest_file_path,
                )
                .context("[run] Failed to generate reference definitions from dependencies.")?;
                println!("{}", style("Done.").green());
            }
        }
    }
    Ok(())
}
