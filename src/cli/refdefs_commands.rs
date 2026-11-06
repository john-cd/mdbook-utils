//! Command-line subcommands to handle reference definitions
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use clap::Subcommand;
use console::style;
use mdbook_utils::api::BadgeStyle;

use super::args::DependenciesDirAndDestFileArgs;
use super::args::MarkdownSrcDirAndDestFileArgs;
use super::config::Configuration;
use super::run_logged;

/// Command-line subcommands to handle reference definitions
#[derive(Subcommand, Debug)]
pub(crate) enum RefDefsSubCommand {
    /// Write existing reference definitions to a file
    Write(MarkdownSrcDirAndDestFileArgs),

    /// Generate reference definitions
    /// (e.g. badges for Github links or from the dependencies of the code
    /// examples)
    #[command(name = "generate")]
    Generate(GenerateArgs),
}

/// Shields.io badge style, as accepted on the command line.
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub(crate) enum BadgeStyleArg {
    /// The standard flat style used by Shields.io by default.
    #[default]
    Flat,
    /// A flat style with square corners.
    #[value(name = "flat-square")]
    FlatSquare,
    /// A style with a glossy appearance.
    Plastic,
    /// A wider style with uppercase text.
    #[value(name = "for-the-badge")]
    ForTheBadge,
    /// The social style with a larger logo.
    Social,
}

impl From<BadgeStyleArg> for BadgeStyle {
    fn from(style: BadgeStyleArg) -> Self {
        match style {
            BadgeStyleArg::Flat => Self::Flat,
            BadgeStyleArg::FlatSquare => Self::FlatSquare,
            BadgeStyleArg::Plastic => Self::Plastic,
            BadgeStyleArg::ForTheBadge => Self::ForTheBadge,
            BadgeStyleArg::Social => Self::Social,
        }
    }
}

/// Arguments for generating reference definitions
#[derive(Debug, clap::Args)]
pub(crate) struct GenerateArgs {
    /// Generate badges (reference definitions) for e.g. Github links
    #[arg(long)]
    pub(crate) badges: bool,

    /// Select a Shields.io style and generate GitHub badges.
    #[arg(long = "badge-style", value_enum, value_name = "STYLE")]
    pub(crate) badge_style: Option<BadgeStyleArg>,

    /// Generate reference definitions from the dependencies of the code
    /// examples
    #[arg(long = "from-dependencies")]
    pub(crate) from_dependencies: bool,

    /// Source, manifest, destination, and log-file paths.
    #[command(flatten)]
    pub(crate) args: DependenciesDirAndDestFileArgs,
}

/// "refdefs" subcommands of the command-line interface
pub(crate) fn run(subcmd: RefDefsSubCommand, config: &Configuration) -> Result<()> {
    match subcmd {
        RefDefsSubCommand::Write(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let refdef_dest_path = config.dest_file_path(args.dest, "existing_refs.md");
            run_logged(
                "refdefs write",
                &[
                    ("markdown_source", markdown_src_dir_path.as_path()),
                    ("destination", refdef_dest_path.as_path()),
                ],
                &format!(
                    "Parsing markdown files in {} and writing existing reference definitions to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(refdef_dest_path.display()).cyan()
                ),
                "[run] Failed to write reference definitions to a file.",
                || mdbook_utils::api::write_refdefs_to(&markdown_src_dir_path, &refdef_dest_path),
            )?;
        }
        RefDefsSubCommand::Generate(args) => {
            let badge_style = args.badge_style.unwrap_or_default();
            let mut run_badges = args.badges || args.badge_style.is_some();
            let mut run_deps = args.from_dependencies;

            // If neither is specified, we default to running both
            if !run_badges && !run_deps {
                run_badges = true;
                run_deps = true;
            }

            if run_badges {
                // If we also run dependencies, the output paths might clash if
                // not explicitly set. We'll use "badge_refs.md"
                // as the default name.
                let markdown_src_dir_path =
                    config.markdown_src_dir_path(args.args.src.clone(), "./src/")?;
                let refdef_dest_path =
                    config.dest_file_path(args.args.dest.clone(), "badge_refs.md");
                run_logged(
                    "refdefs generate-badges",
                    &[
                        ("markdown_source", markdown_src_dir_path.as_path()),
                        ("destination", refdef_dest_path.as_path()),
                    ],
                    &format!(
                        "Parsing markdown files in {} and writing new (github badge) reference definitions to {}...",
                        style(markdown_src_dir_path.display()).cyan(),
                        style(refdef_dest_path.display()).cyan()
                    ),
                    "[run] Failed to generate badges.",
                    || {
                        mdbook_utils::api::generate_badges_with_style(
                            &markdown_src_dir_path,
                            &refdef_dest_path,
                            badge_style.into(),
                        )
                    },
                )?;
            }

            if run_deps {
                let markdown_src_dir_path =
                    config.markdown_src_dir_path(args.args.src, "./src/")?;
                let cargo_toml_dir_path = config.cargo_toml_dir_path(args.args.manifest)?;
                let refdef_dest_file_path =
                    config.dest_file_path(args.args.dest, "dependencies_refs.md");
                let mut paths = vec![
                    ("manifest_directory", cargo_toml_dir_path.as_path()),
                    ("markdown_source", markdown_src_dir_path.as_path()),
                    ("destination", refdef_dest_file_path.as_path()),
                ];
                if let Some(log_file_path) = args.args.log_file.as_deref() {
                    paths.push(("cargo_log", log_file_path));
                }
                run_logged(
                    "refdefs generate-from-dependencies",
                    &paths,
                    &format!(
                        "Creating reference definitions in {} from the manifest in {} and Markdown sources in {}...",
                        style(refdef_dest_file_path.display()).cyan(),
                        style(cargo_toml_dir_path.display()).cyan(),
                        style(markdown_src_dir_path.display()).cyan(),
                    ),
                    "[run] Failed to generate reference definitions from dependencies.",
                    || {
                        mdbook_utils::api::generate_refdefs_to(
                            &cargo_toml_dir_path,
                            &markdown_src_dir_path,
                            &refdef_dest_file_path,
                            args.args.log_file.as_ref(),
                        )
                    },
                )?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use tempfile::tempdir;

    use super::*;
    use crate::cli::args::DestFileArgs;
    use crate::cli::args::MarkdownDirArgs;

    /// `refdefs write` copies the existing reference definitions.
    #[test]
    fn test_refdefs_write() -> Result<()> {
        let dir = tempdir()?;
        let src = dir.path().join("src");
        fs::create_dir(&src)?;
        fs::write(
            src.join("page.md"),
            "See [ex].\n\n[ex]: https://example.com\n",
        )?;
        let dest = dir.path().join("refs.md");
        let args = MarkdownSrcDirAndDestFileArgs {
            src: MarkdownDirArgs {
                markdown_dir_path: Some(src),
            },
            dest: DestFileArgs {
                file_path: Some(dest.clone()),
            },
        };
        run(RefDefsSubCommand::Write(args), &Configuration::default())?;
        assert!(fs::read_to_string(dest)?.contains("https://example.com"));
        Ok(())
    }

    /// A missing source directory is reported and nothing is written.
    #[test]
    fn test_refdefs_write_missing_source_fails() -> Result<()> {
        let dir = tempdir()?;
        let dest = dir.path().join("refs.md");
        let args = MarkdownSrcDirAndDestFileArgs {
            src: MarkdownDirArgs {
                markdown_dir_path: Some(dir.path().join("missing")),
            },
            dest: DestFileArgs {
                file_path: Some(dest.clone()),
            },
        };
        assert!(run(RefDefsSubCommand::Write(args), &Configuration::default()).is_err());
        assert!(!dest.exists());
        Ok(())
    }

    /// Badge styles map onto the library's styles.
    #[test]
    fn test_badge_style_conversion() {
        assert!(matches!(
            BadgeStyle::from(BadgeStyleArg::ForTheBadge),
            BadgeStyle::ForTheBadge
        ));
        assert!(matches!(
            BadgeStyle::from(BadgeStyleArg::default()),
            BadgeStyle::Flat
        ));
    }
}
