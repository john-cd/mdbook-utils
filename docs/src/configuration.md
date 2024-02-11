# Configuration

Each subcommand uses defaults that are overwritten by values in `book.toml` (if present), by environment variables (if set), or command-line options (the latter trumps the former).

You may export environment variables manually or store them in a `.env` file, which will be read automatically:

```bash
# Root directory of the book
# `book.toml` is looked up in BOOK_ROOT_DIR_PATH, if set,
# in the current working directory otherwise.
export BOOK_ROOT_DIR_PATH=./test_book/

# Markdown source directory
export MARKDOWN_DIR_PATH=./test_book/src/

# Directory where mdbook outputs the book's HTML and JS;
# typically ./book/ or ./book/html/
export BOOK_HTML_BUILD_DIR_PATH=./test_book/book/

# Directory where `mdbook` outputs the book's fully expanded Markdown,
# i.e. with all includes resolved, when `[output.markdown]` is added to `book.toml`.
# It is typically ./book/markdown/.
export BOOK_MARKDOWN_BUILD_DIR_PATH=./test_book/book/markdown/

# Directory where `Cargo.toml` may be found
export CARGO_TOML_DIR_PATH=./test_book/book/code/

# Default destination directory for mdbook-utils outputs.
export DEFAULT_DEST_DIR_PATH=./test_book/temp/

# Base url of the website where the book will be deployed
# (used to build sitemaps)
export BASE_URL=http://myexample.com/some_book/
```

You may also set the [`RUST_LOG`][rust-log] environment variable to display the logs.

See `cli/config.rs` in the [GitHub repo][mdbook-utils-github] for more details.

Note: `mdbook-utils` is not a mdbook [preprocessor] or [backend] at this point.

[backend]: https://rust-lang.github.io/mdBook/format/configuration/renderers.html
[mdbook-utils-github]: https://github.com/john-cd/mdbook-utils
[preprocessor]: https://rust-lang.github.io/mdBook/for_developers/preprocessors.html
[rust-log]: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html
