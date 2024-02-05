//! Handles CLI configuration, environment variables, and defaults

use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use tracing::debug;

use super::args::CargoTomlDirArgs;
use super::args::DestDirArgs;
use super::args::DestFileArgs;
use super::args::MarkdownDirArgs;
use super::args::UrlArgs;
use super::GlobalOpts;

/// Stores environment variables into a Configuration struct.
/// Defaults apply if not present.
pub(crate) fn init(global_opts: GlobalOpts) -> Result<Configuration> {
    // Serialize environment variables into the Configuration struct
    let mut c = envy::from_env::<Configuration>()?;
    c.global_opts = global_opts;
    Ok(c)
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub(crate) struct Configuration {
    /// BOOK_ROOT_DIR_PATH environment variable:
    /// the book's root directory (which contains `book.toml`),
    /// typically '.'
    book_root_dir_path: PathBuf,
    /// MARKDOWN_DIR_PATH environment variable:
    /// Markdown source directory,
    /// typically ./src/
    markdown_dir_path: Option<PathBuf>,
    /// BOOK_HTML_BUILD_DIR_PATH environment variable:
    /// Directory where `mdbook` outputs the book's HTML and JS,
    /// typically ./book/ or ./book/html/
    book_html_build_dir_path: Option<PathBuf>,
    /// BOOK_MARKDOWN_BUILD_DIR_PATH environment variable:
    /// Directory where `mdbook` outputs the book's fully expanded Markdown,
    /// i.e. with all includes resolved. It is typically ./book/markdown/
    /// The directory is created only if `[output.markdown]` is added to
    /// `book.toml`.
    book_markdown_build_dir_path: Option<PathBuf>,
    /// CARGO_TOML_DIR_PATH environment variable:
    /// Directory where `Cargo.toml` may be found,
    /// typically '.'
    cargo_toml_dir_path: Option<PathBuf>,
    /// DEFAULT_DEST_DIR_PATH environment variable:
    /// Default destination directory for `mdbook-utils` outputs.
    default_dest_dir_path: Option<PathBuf>,
    /// BASE_URL environment variable:
    /// Base url of the website where the book will be deployed
    /// e.g. https://example.com/mybook/
    /// It is used to build sitemaps.
    base_url: String,

    /// Global options that apply to all (sub)commands.
    #[serde(skip)]
    global_opts: GlobalOpts,
}

/// Defaults if the environment variables are not set
impl Default for Configuration {
    fn default() -> Self {
        Self {
            book_root_dir_path: PathBuf::from("."),
            markdown_dir_path: None,
            book_html_build_dir_path: None,
            book_markdown_build_dir_path: None,
            cargo_toml_dir_path: None,
            default_dest_dir_path: None,
            base_url: String::from("http://example.com/mybook/"),
            global_opts: GlobalOpts::default(),
        }
    }
}

impl Configuration {
    /// Returns the Markdown source directory provided by the
    /// command-line argument (if set);
    /// the MARKDOWN_DIR_PATH environment variable (if set);
    /// the "book.src" field (which defaults to {book_root_dir_path}/src) in
    /// `book.toml` (if `book.toml` is found); otherwise the default value
    /// passed as function argument (./src/ or ./drafts/ typically).
    ///
    /// `book.toml` is looked up in BOOK_ROOT_DIR_PATH, if set,
    /// or the current working directory.
    pub(crate) fn markdown_src_dir_path<S: AsRef<OsStr>>(
        &self,
        args: MarkdownDirArgs,
        default_dir_path: S,
    ) -> Result<PathBuf> {
        let p = args
            .markdown_dir_path
            .unwrap_or(if let Some(ref mdp) = self.markdown_dir_path {
                debug!("MARKDOWN_DIR_PATH set: {}", mdp.display());
                mdp.clone()
            } else if let Some(p) = self.get_markdown_dir_path_from_book_toml() {
                debug!("markdown_dir_path set from `book.toml`: {}", p.display());
                p
            } else {
                debug!(
                    "markdown_dir_path set to default: {:?}",
                    default_dir_path.as_ref()
                );
                PathBuf::from(default_dir_path.as_ref())
            });

        let p = p.canonicalize()
            .with_context(|| format!("[markdown_dir_path] The Markdown source directory {} does not exist or cannot be resolved.", p.display()))?;

        Ok(p)
    }

    /// Return markdown_dir_path if retrievable from `book.toml`,
    /// None otherwise.
    ///
    /// Does not propagate errors, since having a `book.toml` is optional.
    fn get_markdown_dir_path_from_book_toml(&self) -> Option<PathBuf> {
        match super::book_toml::try_parse_book_toml(self.book_root_dir_path.clone()) {
            // `book.toml` exists and is parseable
            Ok((src, _, _)) => Some(src),
            Err(e) => {
                debug!(
                    "`book.toml` does not exist in {} or is not parseable. Error: {}",
                    self.book_root_dir_path.display(),
                    e
                );
                None
            }
        }
    }

    /// Returns the directory where `mdbook` outputs the book's fully expanded
    /// Markdown, i.e. with all includes resolved, if `[output.markdown]` is
    /// added to `book.toml`
    ///
    /// The return value is provided by the command-line argument (if set);
    /// the BOOK_MARKDOWN_BUILD_DIR_PATH environment variable (if set);
    /// the "build.build-dir" field in `book.toml` (which defaults to
    /// `{book_root_dir_path}/book`) followed by `markdown` (if `book.toml`
    /// is found); otherwise the default value passed as function argument
    /// (`./book/markdown` typically).
    ///
    /// `book.toml` is looked up in BOOK_ROOT_DIR_PATH, if set,
    /// or the current working directory.
    pub(crate) fn book_markdown_build_dir_path<S: AsRef<OsStr>>(
        &self,
        args: MarkdownDirArgs,
        default_dir_path: S,
    ) -> Result<PathBuf> {
        let p = args.markdown_dir_path.unwrap_or(
            if let Some(ref mdp) = self.book_markdown_build_dir_path {
                debug!("BOOK_MARKDOWN_BUILD_DIR_PATH set: {}", mdp.display());
                mdp.clone()
            } else if let Ok((_, _, Some(p))) =
                super::book_toml::try_parse_book_toml(self.book_root_dir_path.clone())
            {
                debug!(
                    "book_markdown_build_dir_path set from `book.toml`: {}",
                    p.display()
                );
                p
            } else {
                debug!(
                    "book_markdown_build_dir_path set to default: {:?}",
                    default_dir_path.as_ref()
                );
                PathBuf::from(default_dir_path.as_ref())
            },
        );

        let p = p.canonicalize()
            .with_context(|| format!("[book_markdown_build_dir_path] The Markdown output (build) directory {} does not exist or cannot be resolved. Try `mdbook build`.", p.display()))?;

        Ok(p)
    }

    /// Returns the default destination directory where to store mdbook-utils
    /// outputs, as provided by the DEFAULT_DEST_DIR_PATH environment variable
    /// (if set), otherwise the book root directory (which defaults to '.').
    fn default_dest_dir_path(&self) -> PathBuf {
        if let Some(ref pb) = self.default_dest_dir_path {
            pb.into()
        } else {
            self.book_root_dir_path.clone()
        }
    }

    /// Returns the destination directory where to store mdbook-utils
    /// outputs, as provided by the command-line argument (if set),
    /// the DEFAULT_DEST_DIR_PATH environment variable (if set),
    /// or the current working directory otherwise.
    pub(crate) fn dest_dir_path(&self, args: DestDirArgs) -> PathBuf {
        args.dir_path.unwrap_or(self.default_dest_dir_path())
    }

    /// Returns the destination file path, as provided by
    /// the command-line argument (if set) or the default destination path and
    /// default filename otherwise (see `default_dest_dir_path`).
    pub(crate) fn dest_file_path(&self, args: DestFileArgs, filename: &str) -> PathBuf {
        args.file_path
            .unwrap_or_else(|| self.default_dest_dir_path().join(filename))
    }

    /// Returns the directory where `Cargo.toml` may be found,
    /// as provided by the command-line argument (if set),
    /// the CARGO_TOML_DIR_PATH environment variable (if set),
    /// or BOOK_ROOT_DIR_PATH (which defaults to '.') otherwise.
    pub(crate) fn cargo_toml_dir_path(&self, args: CargoTomlDirArgs) -> Result<PathBuf> {
        let p =
            args.cargo_toml_dir_path
                .unwrap_or(if let Some(ref ctdp) = self.cargo_toml_dir_path {
                    ctdp.clone()
                } else {
                    self.book_root_dir_path.clone()
                });
        let p = p.canonicalize().with_context(|| format!("[cargo_toml_dir_path] The directory {} where `Cargo.toml` may be found does not exist or cannot be resolved.", p.display()))?;
        Ok(p)
    }

    /// Returns the base url of the website where the book will be deployed
    /// (used to build sitemaps), as provided by the BASE_URL environment
    /// variable (if set), otherwise the default value.
    pub(crate) fn base_url(&self, args: UrlArgs) -> Result<url::Url> {
        Ok(args.url.unwrap_or(
            url::Url::parse(&self.base_url)
                .context("[base_url] Could not parse the base url provided.")?,
        ))
    }

    /// Returns the sitemap output file path, as provided by
    /// the command-line argument (if set); or {path}/sitemap.xml,
    /// where the HTML output path is retrieved from `book.toml`, if possible,
    /// or the default (`./book`) otherwise.
    pub(crate) fn sitemap_file_path(&self, args: DestFileArgs) -> PathBuf {
        if let Some(file_path) = args.file_path {
            file_path
        } else {
            let dir: PathBuf = if let Some(ref d) = self.book_html_build_dir_path {
                d.clone()
            } else if let Ok((_, html_output_dir, _)) =
                super::book_toml::try_parse_book_toml(self.book_root_dir_path.clone())
            {
                // `book.toml`` exists, is parseable and build.build-dir is defined
                html_output_dir
            } else {
                "./book".into()
            };
            dir.join("sitemap.xml")
        }
    }

    /// if true, skip confirmation prompts
    pub(crate) fn skip_confirm(&self) -> bool {
        self.global_opts.yes
    }
}
