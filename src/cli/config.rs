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

/// Stores environment variables into a Configuration struct.
/// Defaults apply if not present.
pub(crate) fn init() -> Result<Configuration> {
    // Serialize environment variables into the Configuration struct
    let c = envy::from_env::<Configuration>()?;
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
    /// Directory where mdbook outputs the book's HTML and JS,
    /// typically ./book/ or ./book/html/
    book_html_build_dir_path: Option<PathBuf>,
    /// CARGO_TOML_DIR_PATH environment variable:
    /// Directory where Cargo.toml may be found,
    /// typically '.'
    cargo_toml_dir_path: Option<PathBuf>,
    /// DEFAULT_DEST_DIR_PATH environment variable:
    /// Default destination directory for mdbook-utils outputs.
    default_dest_dir_path: Option<PathBuf>,
    /// BASE_URL environment variable:
    /// Base url of the website where the book will be deployed
    /// (used to build sitemaps) e.g. https://example.com/mybook/
    base_url: String,
}

/// Defaults if the environment variables are not set
impl Default for Configuration {
    fn default() -> Self {
        Self {
            book_root_dir_path: PathBuf::from("."),
            markdown_dir_path: None,
            book_html_build_dir_path: None,
            cargo_toml_dir_path: None,
            default_dest_dir_path: None,
            base_url: String::from("http://example.com/mybook/"),
        }
    }
}

impl Configuration {
    /// Returns the Markdown source directory provided by the
    /// command-line argument (if set);
    /// the MARKDOWN_DIR_PATH environment variable (if set);
    /// the "src" field in `book.toml` (if set);
    /// otherwise the default value passed as function argument
    /// (./src/ or ./drafts/ typically).
    ///
    /// `book.toml` is looked up in BOOK_ROOT_DIR_PATH, if set,
    /// or the current working directory.
    pub(crate) fn markdown_dir_path<S: AsRef<OsStr>>(
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
    /// Swallows errors, since having a `book.toml` is optional.
    fn get_markdown_dir_path_from_book_toml(&self) -> Option<PathBuf> {
        match super::book_toml::try_parse_book_toml(self.book_root_dir_path.clone()) {
            // `book.toml` exists, is parseable, and book.src is defined
            Ok((Some(src), _)) => Some(src),
            Ok((None, _)) => {
                debug!("`book.src` is not defined in `book.toml`");
                None
            }
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
        let p = args
            .cargo_toml_dir_path
            .unwrap_or(if let Some(ref ctdp) = self.cargo_toml_dir_path { ctdp.clone() } else { self.book_root_dir_path.clone() });
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
            } else if let Ok((_, Some(html_output_dir))) =
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
}
