<!-- TODO Reviewed ??-??-2026 -->
# Configuration

Path selection follows this order when a command supports the corresponding option: command-line option, matching environment variable, mdBook's `book.toml`, then the command's built-in default. The Markdown source default varies by command. `MDBOOK_UTILS_BOOK_ROOT_DIR_PATH` selects the directory containing `book.toml`; it defaults to the current working directory.

`mdbook-utils` reads the mdBook [`book.src` and `build.build-dir` settings][mdbook-config], and uses the configured [`[output.*]` renderers][mdbook-backend] to resolve renderer output directories. Relative paths in `book.toml` are relative to the book root. If there are no output tables, mdBook enables HTML by default. One renderer writes into `build.build-dir`; with multiple renderers, each writes into a subdirectory named after the renderer, such as `html` or `markdown`.

The mdBook `book.toml` is the only TOML configuration source; no separate `mdbook-utils` TOML file is needed. A missing `book.toml` is optional. If path resolution reaches an unreadable or malformed `book.toml`, the command reports the error instead of silently using defaults. When a valid configuration disables the HTML renderer, `sitemap` requires an explicit destination through `--output` or `MDBOOK_UTILS_BOOK_HTML_BUILD_DIR_PATH`.

| Priority | Source | Example |
| --- | --- | --- |
| 1 | Command-line option | `--markdown-dir ./src/` |
| 2 | Environment variable (or `.env` file in the current directory) | `MDBOOK_UTILS_MARKDOWN_DIR_PATH` |
| 3 | mdBook's `book.toml` | `book.src`, `build.build-dir`, `output.html.site-url` |
| 4 | The command's default | `./src/` |

All environment variables start with `MDBOOK_UTILS_`. The older unprefixed names (for example `BASE_URL` or `MARKDOWN_DIR_PATH`) are still read when the prefixed variable is not set, but print a deprecation warning and will be removed in a later release. Only a `.env` file in the current directory is loaded; parent directories are not searched.

You may export environment variables manually or store them in a `.env` file, which will be read automatically:

```bash
# Root directory of the book
# `book.toml` is looked up in MDBOOK_UTILS_BOOK_ROOT_DIR_PATH, if set,
# in the current working directory otherwise.
export MDBOOK_UTILS_BOOK_ROOT_DIR_PATH=./test_book/

# Markdown source directory. This overrides `book.src`.
export MDBOOK_UTILS_MARKDOWN_DIR_PATH=./test_book/src/

# Directory where mdBook outputs HTML and JS. This overrides `build.build-dir`
# and renderer-based directory selection.
export MDBOOK_UTILS_BOOK_HTML_BUILD_DIR_PATH=./test_book/book/

# Directory where `mdbook` outputs the book's fully expanded Markdown,
# i.e. with all includes resolved, when `[output.markdown]` is added to `book.toml`.
# It is typically ./book/markdown/.
export MDBOOK_UTILS_BOOK_MARKDOWN_BUILD_DIR_PATH=./test_book/book/markdown/

# Directory where `Cargo.toml` may be found
export MDBOOK_UTILS_CARGO_TOML_DIR_PATH=./test_book/book/code/

# Default destination directory for mdbook-utils outputs.
export MDBOOK_UTILS_DEFAULT_DEST_DIR_PATH=./test_book/temp/

# Base url of the website where the book will be deployed
# (used to build sitemaps). Optional if `book.toml` sets
# `output.html.site-url`; `sitemap` fails when no base URL is configured.
export MDBOOK_UTILS_BASE_URL=http://myexample.com/some_book/
```

For example, when `book.toml` contains `[output.html]` and `[output.markdown]` and sets `build-dir = "dist"`, mdBook writes HTML to `dist/html` and expanded Markdown to `dist/markdown`, relative to the book root. A sole `[output.html]` writes directly to `dist`.

You may also set the [`RUST_LOG`][rust-log] environment variable to display the logs.

The CLI options `--markdown-dir`, `--output`, `--target-dir`, `--cargo-toml-dir`, and `--base-url` override the matching settings for the current command. See the [mdBook general configuration][mdbook-config] and [renderer configuration][mdbook-backend] for the underlying format.

Note: `mdbook-utils` is not a mdbook [mdbook-preprocessor] or [mdbook-backend] at this point.

{{#include ./refs.md}}
