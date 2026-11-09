//! Command-line subcommands to handle links
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use clap::Args;
use clap::Subcommand;
use console::style;
use mdbook_utils::api::LinkFormat;

use super::args::MarkdownSrcDirAndDestFileArgs;
use super::config::Configuration;
use super::run_logged;

/// Link syntax, as accepted on the command line.
#[derive(clap::ValueEnum, Debug, Clone, Copy, Default)]
pub(crate) enum LinkFormatArg {
    /// Reference-style links with their reference definitions.
    #[default]
    Reference,
    /// Inline links: `[text](url "title")`.
    Inline,
    /// Autolinks: `<url>`. Link text and title are dropped.
    Autolink,
}

impl From<LinkFormatArg> for LinkFormat {
    fn from(format: LinkFormatArg) -> Self {
        match format {
            LinkFormatArg::Reference => Self::Reference,
            LinkFormatArg::Inline => Self::Inline,
            LinkFormatArg::Autolink => Self::Autolink,
        }
    }
}

/// Arguments of `links write-inline`
#[derive(Args, Debug)]
#[command(flatten_help = true)]
pub(crate) struct WriteInlineArgs {
    /// Source and destination
    #[command(flatten)]
    pub(crate) paths: MarkdownSrcDirAndDestFileArgs,

    /// Syntax used to write the external links found
    #[arg(long = "link-format", value_enum, default_value_t = LinkFormatArg::Reference)]
    pub(crate) link_format: LinkFormatArg,
}

/// Command-line subcommands to handle links
#[derive(Subcommand, Debug)]
pub(crate) enum LinksSubCommand {
    /// Write all existing links to a Markdown file
    WriteAll(MarkdownSrcDirAndDestFileArgs),

    /// Write all existing inline / autolinks (i.e., not
    /// written as reference-style links) to a Markdown file
    WriteInline(WriteInlineArgs),

    /// Identify links that repeat the same label, URL and title (inline links:
    /// the same URL) and write them to a Markdown file
    DuplicateLinks(MarkdownSrcDirAndDestFileArgs),

    /// Identify broken links (i.e. without reference definition) and
    /// write to a Markdown file
    BrokenLinks(MarkdownSrcDirAndDestFileArgs),
}

/// Process "links" subcommands of the command-line interface
pub(crate) fn run(subcmd: LinksSubCommand, config: &Configuration) -> Result<()> {
    match subcmd {
        LinksSubCommand::WriteAll(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "all_links.md");
            run_logged(
                "links write-all",
                &[
                    ("source", markdown_src_dir_path.as_path()),
                    ("destination", links_dest_path.as_path()),
                ],
                &format!(
                    "Parsing markdown files in {} and writing existing links to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(links_dest_path.display()).cyan()
                ),
                "[run] Failed to write links to a file.",
                || {
                    let report = mdbook_utils::api::render_all_links(&markdown_src_dir_path)?;
                    super::write_output::write_text_atomically(&links_dest_path, &report)
                },
            )?;
        }
        LinksSubCommand::WriteInline(args) => {
            let link_format = args.link_format;
            let args = args.paths;
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "inline_links.md");
            run_logged(
                "links write-inline",
                &[
                    ("source", markdown_src_dir_path.as_path()),
                    ("destination", links_dest_path.as_path()),
                ],
                &format!(
                    "Parsing markdown files in {} and writing inline / auto links to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(links_dest_path.display()).cyan()
                ),
                "[run] Failed to write inline links to a file.",
                || {
                    let report = mdbook_utils::api::render_inline_links(
                        &markdown_src_dir_path,
                        link_format.into(),
                    )?;
                    super::write_output::write_text_atomically(&links_dest_path, &report)
                },
            )?;
        }
        LinksSubCommand::DuplicateLinks(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "duplicate_links.md");
            run_logged(
                "links duplicate-links",
                &[
                    ("source", markdown_src_dir_path.as_path()),
                    ("destination", links_dest_path.as_path()),
                ],
                &format!(
                    "Parsing markdown files in {} and writing duplicates links to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(links_dest_path.display()).cyan()
                ),
                "[run] Failed to write duplicate links to a file.",
                || {
                    let report = mdbook_utils::api::render_duplicate_links(&markdown_src_dir_path)?;
                    super::write_output::write_text_atomically(&links_dest_path, &report)
                },
            )?;
        }
        LinksSubCommand::BrokenLinks(args) => {
            let markdown_src_dir_path = config.markdown_src_dir_path(args.src, "./src/")?;
            let links_dest_path = config.dest_file_path(args.dest, "broken_links.md");
            run_logged(
                "links broken-links",
                &[
                    ("source", markdown_src_dir_path.as_path()),
                    ("destination", links_dest_path.as_path()),
                ],
                &format!(
                    "Parsing markdown files in {} and writing broken links to {}...",
                    style(markdown_src_dir_path.display()).cyan(),
                    style(links_dest_path.display()).cyan()
                ),
                "[run] Failed to write broken links to a file.",
                || {
                    let report = mdbook_utils::api::render_broken_links(&markdown_src_dir_path)?;
                    super::write_output::write_text_atomically(&links_dest_path, &report)
                },
            )?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use anyhow::Result;
    use tempfile::tempdir;

    use super::*;
    use crate::cli::args::DestFileArgs;
    use crate::cli::args::MarkdownDirArgs;

    /// Constructor of a `links` subcommand taking source and destination.
    type Make = fn(MarkdownSrcDirAndDestFileArgs) -> LinksSubCommand;

    /// Builds the source/destination arguments for a test.
    fn paths(src: &Path, dest: &Path) -> MarkdownSrcDirAndDestFileArgs {
        MarkdownSrcDirAndDestFileArgs {
            src: MarkdownDirArgs {
                markdown_dir_path: Some(src.to_path_buf()),
            },
            dest: DestFileArgs {
                file_path: Some(dest.to_path_buf()),
            },
        }
    }

    /// Creates a source directory with one page and returns its path.
    fn source_dir(root: &Path) -> Result<std::path::PathBuf> {
        let src = root.join("src");
        fs::create_dir(&src)?;
        fs::write(
            src.join("page.md"),
            "See [Example][ex] and [Missing][nope].\n\n[ex]: https://example.com\n",
        )?;
        Ok(src)
    }

    /// Each `links` arm writes its destination file.
    #[test]
    fn test_links_arms_write_destination() -> Result<()> {
        let dir = tempdir()?;
        let src = source_dir(dir.path())?;
        let arms: [(&str, Make); 3] = [
            ("all.md", LinksSubCommand::WriteAll),
            ("dup.md", LinksSubCommand::DuplicateLinks),
            ("broken.md", LinksSubCommand::BrokenLinks),
        ];
        for (name, make) in arms {
            let dest = dir.path().join(name);
            run(make(paths(&src, &dest)), &Configuration::default())?;
            assert!(dest.is_file(), "{name} was not written");
        }
        Ok(())
    }

    /// `write-inline` writes its destination file.
    #[test]
    fn test_links_write_inline() -> Result<()> {
        let dir = tempdir()?;
        let src = source_dir(dir.path())?;
        let dest = dir.path().join("inline.md");
        let args = WriteInlineArgs {
            paths: paths(&src, &dest),
            link_format: LinkFormatArg::Inline,
        };
        run(
            LinksSubCommand::WriteInline(args),
            &Configuration::default(),
        )?;
        assert!(dest.is_file());
        Ok(())
    }

    /// A missing source directory is reported as an error.
    #[test]
    fn test_links_missing_source_fails() -> Result<()> {
        let dir = tempdir()?;
        let missing = dir.path().join("nope");
        let dest = dir.path().join("out.md");
        let result = run(
            LinksSubCommand::WriteAll(paths(&missing, &dest)),
            &Configuration::default(),
        );
        assert!(result.is_err());
        assert!(!dest.exists());
        Ok(())
    }
}
