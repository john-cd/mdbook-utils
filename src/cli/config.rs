//! Handles CLI configuration, environment variables, and defaults
//!
//! TODO Reviewed ??-??-2026

use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use tracing::debug;

use super::GlobalOpts;
use super::args::CargoTomlDirArgs;
use super::args::DestDirArgs;
use super::args::DestFileArgs;
use super::args::MarkdownDirArgs;
use super::args::UrlArgs;

/// Stores environment variables into a Configuration struct.
/// Defaults apply if not present.
pub(crate) fn init(global_opts: GlobalOpts) -> Result<Configuration> {
    // Serialize environment variables into the Configuration struct
    let mut c = envy::from_env::<Configuration>()?;
    c.global_opts = global_opts;
    Ok(c)
}

/// Application configuration and environment variables
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
    base_url: Option<String>,
    /// SITEMAP_MAP_INDEX environment variable:
    /// Map a filename to another (e.g., 'intro.md' to 'index.md')
    sitemap_map_index: Option<String>,
    /// MDBOOK_PATH environment variable:
    /// Path to the `mdbook` binary
    /// typically `mdbook`
    mdbook_path: Option<PathBuf>,

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
            base_url: None,
            sitemap_map_index: Some("intro.md:index.md".to_string()),
            mdbook_path: None,
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
        let (p, source) = if let Some(path) = args.markdown_dir_path {
            (path, "command-line")
        } else if let Some(path) = &self.markdown_dir_path {
            (path.clone(), "environment")
        } else if let Some(path) = self.get_markdown_dir_path_from_book_toml()? {
            (path, "book.toml")
        } else {
            (PathBuf::from(default_dir_path.as_ref()), "default")
        };
        let p = p.canonicalize()
            .with_context(|| format!("[markdown_dir_path] The Markdown source directory {} does not exist or cannot be resolved.", p.display()))?;
        debug!(
            source,
            selected_path = %p.display(),
            "selected Markdown source directory"
        );

        Ok(p)
    }

    /// Return `book.src` resolved from `book.toml`, if that file exists.
    ///
    /// File access and parse errors are propagated so explicit configuration
    /// cannot silently fall back to a different source directory.
    fn get_markdown_dir_path_from_book_toml(&self) -> Result<Option<PathBuf>> {
        Ok(
            super::book_toml::try_parse_book_toml(&self.book_root_dir_path)?
                .map(|paths| paths.markdown_src),
        )
    }

    /// Returns the directory where `mdbook` outputs the book's fully expanded
    /// Markdown, i.e. with all includes resolved, if `[output.markdown]` is
    /// added to `book.toml`
    ///
    /// The return value is provided by the command-line argument (if set);
    /// the BOOK_MARKDOWN_BUILD_DIR_PATH environment variable (if set);
    /// the "build.build-dir" field in `book.toml` (which defaults to
    /// `{book_root_dir_path}/book`). The Markdown renderer writes directly to
    /// that directory when it is the only renderer, or to its `markdown`
    /// subdirectory when there are multiple renderers. If no `book.toml` is
    /// found, the default value passed as function argument is used
    /// (`./book/markdown` typically).
    ///
    /// `book.toml` is looked up in BOOK_ROOT_DIR_PATH, if set,
    /// or the current working directory.
    pub(crate) fn book_markdown_build_dir_path<S: AsRef<OsStr>>(
        &self,
        args: MarkdownDirArgs,
        default_dir_path: S,
    ) -> Result<PathBuf> {
        let (p, source) = if let Some(path) = args.markdown_dir_path {
            (path, "command-line")
        } else if let Some(path) = &self.book_markdown_build_dir_path {
            (path.clone(), "environment")
        } else {
            match super::book_toml::try_parse_book_toml(&self.book_root_dir_path)? {
                Some(paths) => match paths.markdown_output {
                    Some(path) => (path, "book.toml"),
                    None => (PathBuf::from(default_dir_path.as_ref()), "default"),
                },
                None => (PathBuf::from(default_dir_path.as_ref()), "default"),
            }
        };

        let p = p.canonicalize()
            .with_context(|| format!("[book_markdown_build_dir_path] The Markdown output (build) directory {} does not exist or cannot be resolved. Try `mdbook build`.", p.display()))?;
        debug!(
            source,
            selected_path = %p.display(),
            "selected expanded Markdown directory"
        );

        Ok(p)
    }

    /// Returns the destination directory where to store mdbook-utils
    /// outputs, as provided by the command-line argument (if set),
    /// the DEFAULT_DEST_DIR_PATH environment variable (if set),
    /// or the current working directory otherwise.
    pub(crate) fn dest_dir_path(&self, args: DestDirArgs) -> PathBuf {
        let (path, source) = if let Some(path) = args.dir_path {
            (path, "command-line")
        } else if let Some(path) = &self.default_dest_dir_path {
            (path.clone(), "environment")
        } else {
            (self.book_root_dir_path.clone(), "book root")
        };
        debug!(source, selected_path = %path.display(), "selected destination directory");
        path
    }

    /// Returns the destination file path, as provided by
    /// the command-line argument (if set) or the default destination path and
    /// default filename otherwise (see `default_dest_dir_path`).
    pub(crate) fn dest_file_path(&self, args: DestFileArgs, filename: &str) -> PathBuf {
        let (path, source) = if let Some(path) = args.file_path {
            (path, "command-line")
        } else if let Some(directory) = &self.default_dest_dir_path {
            (directory.join(filename), "environment")
        } else {
            (self.book_root_dir_path.join(filename), "book root")
        };
        debug!(source, selected_path = %path.display(), "selected destination file");
        path
    }

    /// Returns the directory where `Cargo.toml` may be found,
    /// as provided by the command-line argument (if set),
    /// the CARGO_TOML_DIR_PATH environment variable (if set),
    /// or BOOK_ROOT_DIR_PATH (which defaults to '.') otherwise.
    pub(crate) fn cargo_toml_dir_path(&self, args: CargoTomlDirArgs) -> Result<PathBuf> {
        let (p, source) = if let Some(path) = args.cargo_toml_dir_path {
            (path, "command-line")
        } else if let Some(path) = &self.cargo_toml_dir_path {
            (path.clone(), "environment")
        } else {
            (self.book_root_dir_path.clone(), "book root")
        };
        let p = p.canonicalize().with_context(|| format!("[cargo_toml_dir_path] The directory {} where `Cargo.toml` may be found does not exist or cannot be resolved.", p.display()))?;
        debug!(
            source,
            selected_path = %p.display(),
            "selected Cargo manifest directory"
        );
        Ok(p)
    }

    /// Returns the base url of the website where the book will be deployed
    /// (used to build sitemaps): the command-line option, otherwise the BASE_URL
    /// environment variable, otherwise `output.html.site-url` in `book.toml`.
    ///
    /// # Errors
    ///
    /// Returns an error if none of them is set, or the URL cannot be parsed.
    pub(crate) fn base_url(&self, args: UrlArgs) -> Result<url::Url> {
        if let Some(url) = args.url {
            return Ok(url);
        }
        let configured = match &self.base_url {
            Some(url) => Some(url.clone()),
            None => super::book_toml::try_parse_book_toml(&self.book_root_dir_path)?
                .and_then(|paths| paths.site_url),
        };
        let configured = configured.context(
            "[base_url] No base URL: pass --base-url, set BASE_URL, or set output.html.site-url in book.toml.",
        )?;
        url::Url::parse(&configured).context("[base_url] Could not parse the base url provided.")
    }

    pub(crate) fn sitemap_map_index(&self, map_index: Option<String>) -> Option<(String, String)> {
        map_index
            .or_else(|| self.sitemap_map_index.clone())
            .and_then(|s| {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
    }

    /// Resolve the sitemap output file from the command-line argument,
    /// environment, mdBook HTML output, or the default `./book` directory.
    ///
    /// # Errors
    ///
    /// Returns an error when the explicit `book.toml` is unreadable or
    /// malformed, or when it disables the HTML renderer without an explicit
    /// destination override.
    pub(crate) fn sitemap_file_path(&self, args: DestFileArgs) -> Result<PathBuf> {
        let (path, source) = if let Some(file_path) = args.file_path {
            (file_path, "command-line")
        } else {
            let (dir, source) = if let Some(dir) = &self.book_html_build_dir_path {
                (dir.clone(), "environment")
            } else {
                match super::book_toml::try_parse_book_toml(&self.book_root_dir_path)? {
                    Some(paths) => match paths.html_output {
                        Some(html_output_dir) => (html_output_dir, "book.toml"),
                        None => anyhow::bail!(
                            "The HTML renderer is not configured in {}/book.toml; pass --output or set BOOK_HTML_BUILD_DIR_PATH to choose a sitemap destination.",
                            self.book_root_dir_path.display()
                        ),
                    },
                    None => ("./book".into(), "default"),
                }
            };
            (dir.join("sitemap.xml"), source)
        };
        debug!(source, selected_path = %path.display(), "selected sitemap file");
        Ok(path)
    }

    /// if true, skip confirmation prompts
    pub(crate) fn skip_confirm(&self) -> bool {
        self.global_opts.yes
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_configuration_default() {
        let config = Configuration::default();
        assert_eq!(config.book_root_dir_path, PathBuf::from("."));
        assert_eq!(config.base_url, None);
        assert_eq!(
            config.sitemap_map_index,
            Some("intro.md:index.md".to_string())
        );
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn test_markdown_src_dir_path() -> Result<()> {
        let dir = tempdir()?;
        let src_dir = dir.path().join("src");
        fs::create_dir(&src_dir)?;

        let mut config = Configuration::default();
        config.book_root_dir_path = dir.path().to_path_buf();

        // 1. Default case
        let args = MarkdownDirArgs {
            markdown_dir_path: None,
        };
        let path = config.markdown_src_dir_path(args, src_dir.to_str().unwrap())?;
        assert_eq!(path, src_dir.canonicalize()?);

        // 2. Environment variable case
        let env_src = dir.path().join("env_src");
        fs::create_dir(&env_src)?;
        config.markdown_dir_path = Some(env_src.clone());
        let args = MarkdownDirArgs {
            markdown_dir_path: None,
        };
        let path = config.markdown_src_dir_path(args, "src")?;
        assert_eq!(path, env_src.canonicalize()?);

        // 3. Argument case
        let arg_src = dir.path().join("arg_src");
        fs::create_dir(&arg_src)?;
        let args = MarkdownDirArgs {
            markdown_dir_path: Some(arg_src.clone()),
        };
        let path = config.markdown_src_dir_path(args, "src")?;
        assert_eq!(path, arg_src.canonicalize()?);

        // 4. book.toml case
        config.markdown_dir_path = None;
        let toml_src = dir.path().join("toml_src");
        fs::create_dir(&toml_src)?;
        fs::write(
            dir.path().join("book.toml"),
            r#"[book]
src = "toml_src"
"#,
        )?;
        let args = MarkdownDirArgs {
            markdown_dir_path: None,
        };
        let path = config.markdown_src_dir_path(args, "src")?;
        assert_eq!(path, toml_src.canonicalize()?);

        Ok(())
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn test_book_markdown_build_dir_path() -> Result<()> {
        let dir = tempdir()?;
        let build_dir = dir.path().join("book").join("markdown");
        fs::create_dir_all(&build_dir)?;

        let mut config = Configuration::default();
        config.book_root_dir_path = dir.path().to_path_buf();

        // 1. Default case
        let args = MarkdownDirArgs {
            markdown_dir_path: None,
        };
        let path = config.book_markdown_build_dir_path(args, build_dir.to_str().unwrap())?;
        assert_eq!(path, build_dir.canonicalize()?);

        // 2. Environment variable case
        let env_build = dir.path().join("env_build");
        fs::create_dir(&env_build)?;
        config.book_markdown_build_dir_path = Some(env_build.clone());
        let args = MarkdownDirArgs {
            markdown_dir_path: None,
        };
        let path = config.book_markdown_build_dir_path(args, "book/markdown")?;
        assert_eq!(path, env_build.canonicalize()?);

        // 3. Argument case
        let arg_build = dir.path().join("arg_build");
        fs::create_dir(&arg_build)?;
        let args = MarkdownDirArgs {
            markdown_dir_path: Some(arg_build.clone()),
        };
        let path = config.book_markdown_build_dir_path(args, "book/markdown")?;
        assert_eq!(path, arg_build.canonicalize()?);

        // 4. book.toml case
        config.book_markdown_build_dir_path = None;
        let toml_build = dir.path().join("book");
        fs::create_dir_all(&toml_build)?;
        fs::write(
            dir.path().join("book.toml"),
            r#"[output.markdown]
"#,
        )?;
        let args = MarkdownDirArgs {
            markdown_dir_path: None,
        };
        let path = config.book_markdown_build_dir_path(args, "book/markdown")?;
        assert_eq!(path, toml_build.canonicalize()?);

        Ok(())
    }

    #[test]
    fn test_dest_dir_path() {
        let mut config = Configuration::default();
        let root = if cfg!(windows) { r"C:\root" } else { "/root" };
        let env_dest = if cfg!(windows) {
            r"C:\env_dest"
        } else {
            "/env_dest"
        };
        let arg_dest = if cfg!(windows) {
            r"C:\arg_dest"
        } else {
            "/arg_dest"
        };

        config.book_root_dir_path = PathBuf::from(root);

        // 1. Default
        let args = DestDirArgs { dir_path: None };
        assert_eq!(config.dest_dir_path(args), PathBuf::from(root));

        // 2. Environment
        config.default_dest_dir_path = Some(PathBuf::from(env_dest));
        let args = DestDirArgs { dir_path: None };
        assert_eq!(config.dest_dir_path(args), PathBuf::from(env_dest));

        // 3. Argument
        let args = DestDirArgs {
            dir_path: Some(PathBuf::from(arg_dest)),
        };
        assert_eq!(config.dest_dir_path(args), PathBuf::from(arg_dest));
    }

    #[test]
    fn test_dest_file_path() {
        let mut config = Configuration::default();
        let root = if cfg!(windows) { r"C:\root" } else { "/root" };
        let arg_file = if cfg!(windows) {
            r"C:\arg\file.txt"
        } else {
            "/arg/file.txt"
        };

        config.book_root_dir_path = PathBuf::from(root);

        // 1. Default
        let args = DestFileArgs { file_path: None };
        assert_eq!(
            config.dest_file_path(args, "test.txt"),
            PathBuf::from(root).join("test.txt")
        );

        // 2. Argument
        let args = DestFileArgs {
            file_path: Some(PathBuf::from(arg_file)),
        };
        assert_eq!(
            config.dest_file_path(args, "test.txt"),
            PathBuf::from(arg_file)
        );
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn test_cargo_toml_dir_path() -> Result<()> {
        let dir = tempdir()?;
        let root_dir = dir.path().to_path_buf();
        let mut config = Configuration::default();
        config.book_root_dir_path = root_dir.clone();

        // 1. Default
        let args = CargoTomlDirArgs {
            cargo_toml_dir_path: None,
        };
        assert_eq!(config.cargo_toml_dir_path(args)?, root_dir.canonicalize()?);

        // 2. Environment
        let env_dir = dir.path().join("env");
        fs::create_dir(&env_dir)?;
        config.cargo_toml_dir_path = Some(env_dir.clone());
        let args = CargoTomlDirArgs {
            cargo_toml_dir_path: None,
        };
        assert_eq!(config.cargo_toml_dir_path(args)?, env_dir.canonicalize()?);

        // 3. Argument
        let arg_dir = dir.path().join("arg");
        fs::create_dir(&arg_dir)?;
        let args = CargoTomlDirArgs {
            cargo_toml_dir_path: Some(arg_dir.clone()),
        };
        assert_eq!(config.cargo_toml_dir_path(args)?, arg_dir.canonicalize()?);

        Ok(())
    }

    #[test]
    fn test_base_url() -> Result<()> {
        let config = Configuration::default();

        // 1. Nothing configured: an error instead of a placeholder URL
        let args = UrlArgs { url: None };
        assert!(config.base_url(args).is_err());

        // 1b. Environment value
        let configured = Configuration {
            base_url: Some("https://env.example/book/".into()),
            ..Configuration::default()
        };
        assert_eq!(
            configured.base_url(UrlArgs { url: None })?.as_str(),
            "https://env.example/book/"
        );

        // 2. Argument
        let arg_url = url::Url::parse("https://test.com/")?;
        let args = UrlArgs {
            url: Some(arg_url.clone()),
        };
        assert_eq!(config.base_url(args)?, arg_url);

        Ok(())
    }

    #[test]
    fn test_sitemap_map_index() {
        let mut config = Configuration::default();

        // 1. Default
        assert_eq!(
            config.sitemap_map_index(None),
            Some(("intro.md".into(), "index.md".into()))
        );

        // 2. Argument
        assert_eq!(
            config.sitemap_map_index(Some("a.md:b.md".into())),
            Some(("a.md".into(), "b.md".into()))
        );

        // 3. Invalid format
        assert_eq!(config.sitemap_map_index(Some("invalid".into())), None);

        // 4. Configuration override
        config.sitemap_map_index = Some("conf.md:idx.md".into());
        assert_eq!(
            config.sitemap_map_index(None),
            Some(("conf.md".into(), "idx.md".into()))
        );
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn test_sitemap_file_path() -> Result<()> {
        let dir = tempdir()?;
        let mut config = Configuration::default();
        config.book_root_dir_path = dir.path().to_path_buf();

        // 1. Argument
        let args = DestFileArgs {
            file_path: Some(PathBuf::from("arg_sitemap.xml")),
        };
        assert_eq!(
            config.sitemap_file_path(args)?,
            PathBuf::from("arg_sitemap.xml")
        );

        // 2. Environment (book_html_build_dir_path)
        let env_html = PathBuf::from("env_html");
        config.book_html_build_dir_path = Some(env_html.clone());
        let args = DestFileArgs { file_path: None };
        assert_eq!(
            config.sitemap_file_path(args)?,
            env_html.join("sitemap.xml")
        );

        // 3. book.toml
        config.book_html_build_dir_path = None;
        fs::write(
            dir.path().join("book.toml"),
            r#"[build]
build-dir = "toml_book"
"#,
        )?;
        let args = DestFileArgs { file_path: None };
        assert_eq!(
            config.sitemap_file_path(args)?,
            dir.path().join("toml_book").join("sitemap.xml")
        );

        // 4. Default
        fs::remove_file(dir.path().join("book.toml"))?;
        let args = DestFileArgs { file_path: None };
        // A missing book.toml leaves the command default in effect.
        assert_eq!(
            config.sitemap_file_path(args)?,
            PathBuf::from("./book").join("sitemap.xml")
        );

        Ok(())
    }

    /// Verifies malformed configuration fails unless the sitemap destination is
    /// explicit.
    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn test_sitemap_file_path_reports_malformed_book_toml_unless_overridden() -> Result<()> {
        let dir = tempdir()?;
        fs::write(dir.path().join("book.toml"), "[build\nbuild-dir = 7")?;

        let mut config = Configuration::default();
        config.book_root_dir_path = dir.path().to_path_buf();
        let args = DestFileArgs { file_path: None };
        let error = config
            .sitemap_file_path(args)
            .expect_err("malformed explicit configuration must fail");
        assert!(format!("{error:#}").contains("book.toml"));

        let overridden_args = DestFileArgs {
            file_path: Some(PathBuf::from("chosen.xml")),
        };
        assert_eq!(
            config.sitemap_file_path(overridden_args)?,
            PathBuf::from("chosen.xml")
        );
        Ok(())
    }

    /// Verifies the sitemap requires an HTML renderer when no destination is
    /// overridden.
    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn test_sitemap_file_path_requires_html_when_config_disables_it() -> Result<()> {
        let dir = tempdir()?;
        fs::write(dir.path().join("book.toml"), "[output.markdown]\n")?;
        let mut config = Configuration::default();
        config.book_root_dir_path = dir.path().to_path_buf();

        let error = config
            .sitemap_file_path(DestFileArgs { file_path: None })
            .expect_err("sitemap needs an HTML destination when HTML is disabled");
        assert!(format!("{error:#}").contains("HTML renderer is not configured"));
        Ok(())
    }

    #[test]
    fn test_skip_confirm() {
        let mut config = Configuration::default();
        assert!(!config.skip_confirm());
        config.global_opts.yes = true;
        assert!(config.skip_confirm());
    }
}
