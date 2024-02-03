//! Handles CLI configuration, environment variables, and defaults

use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

use super::args::CargoTomlDirArgs;
use super::args::DestDirArgs;
use super::args::DestFileArgs;
use super::args::MarkdownDirArgs;
use super::args::UrlArgs;

// CONFIGURATION FROM ENVIRONMENT VARIABLES
// ----------------------------------------

/// Stores environment variables into a Configuration struct.
/// Defaults apply if not present.
pub(crate) fn retrieve_env_vars() -> Result<Configuration> {
    // Serialize environment variables into the Configuration struct
    let c = envy::from_env::<Configuration>()?;
    Ok(c)
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub(crate) struct Configuration {
    /// Markdown source directory
    /// e.g. ./src/
    markdown_dir_path: Option<PathBuf>,
    /// Directory where mdbook outputs the book's HTML and JS
    /// e.g. ./book/
    book_dir_path: PathBuf,
    /// Directory where Cargo.toml may be found
    /// e.g. '.'
    cargo_toml_dir_path: PathBuf,
    /// Default destination directory for mdbook-utils outputs.
    /// If the environment variable is not set,
    /// it will use ${book_dir_path}/temp/
    default_dest_dir_path: Option<PathBuf>,
    /// Base url of the website where the book will be deployed
    /// (used to build sitemaps) e.g. https://example.com/mybook/
    base_url: String,
}

/// Defaults if the environment variables are not set
impl Default for Configuration {
    fn default() -> Self {
        Self {
            markdown_dir_path: None,
            book_dir_path: PathBuf::from("./book/"),
            cargo_toml_dir_path: PathBuf::from("."),
            default_dest_dir_path: None, /* getter method will return
                                          * ${book_dir_path}/temp/ */
            base_url: String::from("http://example.com/mybook/"),
        }
    }
}

impl Configuration {
    /// Returns the Markdown source directory provided by the
    /// command-line argument (if set), the MARKDOWN_DIR_PATH environment
    /// variable (if set), otherwise the default value passed as function
    /// argument (./src/ or ./drafts/ typically)
    pub(crate) fn markdown_dir_path<S: AsRef<OsStr>>(
        &self,
        args: MarkdownDirArgs,
        default_dir_path: S,
    ) -> Result<PathBuf> {
        Ok(args
            .markdown_dir_path
            .unwrap_or(if let Some(ref p) = self.markdown_dir_path {
                p.clone()
            } else {
                PathBuf::from(default_dir_path.as_ref())
            })
            .canonicalize()?)
    }

    /// Returns the default destination directory where to store mdbook-utils
    /// outputs, as provided by the DEFAULT_DEST_DIR_PATH environment variable
    /// (if set), otherwise ${book_dir_path}/temp/
    fn default_dest_dir_path(&self) -> PathBuf {
        if let Some(ref pb) = self.default_dest_dir_path {
            pb.into()
        } else {
            self.book_dir_path.join("temp/")
        }
    }

    /// Returns the destination directory where to store mdbook-utils
    /// outputs, as provided by the command-line argument (if set),
    /// the DEFAULT_DEST_DIR_PATH environment variable (if set),
    /// or ${book_dir_path}/temp/ otherwise.
    pub(crate) fn dest_dir_path(&self, args: DestDirArgs) -> PathBuf {
        args.dir_path.unwrap_or(self.default_dest_dir_path())
    }

    /// Returns the destination file path, as provided by
    /// the command-line argument (if set) or the default destination path
    /// otherwise (see `fn default_dest_dir_path(&self) -> PathBuf`).
    pub(crate) fn dest_file_path(&self, args: DestFileArgs, filename: &str) -> PathBuf {
        args.file_path
            .unwrap_or_else(|| self.default_dest_dir_path().join(filename))
    }

    /// Returns the directory where Cargo.toml may be found, as
    /// provided by the command-line argument (if set), the
    /// CARGO_TOML_DIR_PATH environment variable (if set),
    /// or the default value ('.') otherwise.
    pub(crate) fn cargo_toml_dir_path(&self, args: CargoTomlDirArgs) -> Result<PathBuf> {
        Ok(args
            .cargo_toml_dir_path
            .unwrap_or(self.cargo_toml_dir_path.clone())
            .canonicalize()?)
    }

    /// Returns the base url of the website where the book will be deployed
    /// (used to build sitemaps), as provided by the BASE_URL environment
    /// variable (if set), otherwise the default value.
    pub(crate) fn base_url(&self, args: UrlArgs) -> Result<url::Url> {
        Ok(args.url.unwrap_or(url::Url::parse(&self.base_url)?))
    }

    /// Sitemap output file path
    pub(crate) fn sitemap_file_path(&self, args: DestFileArgs) -> PathBuf {
        args.file_path
            .unwrap_or(self.book_dir_path.join("sitemap.xml"))
    }
}
