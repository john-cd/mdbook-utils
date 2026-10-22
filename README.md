<!-- TODO Reviewed ??-??-2026 -->
# mdBook Utilities

Project status and roadmap are actively maintained.

This is a **Work In Progress**. The command-line interface (CLI) and underlying API are subject to change. A summary of recently released changes is found in [RELEASES.md](./RELEASES.md). You may also consult [TODO.md](./TODO.md).

## What is `mdbook-utils`? What is it used for?

The `mdbook-utils` command-line tool manages _links_, _reference definitions_, and _code blocks_ in large collections of Markdown files, especially [`mdbook`][mdbook] source directories. It is the companion tool for the [Rust How-to][rust-howto] book ([github][rust-howto-github]).

`mdbook-utils` is useful for the following:

- centralize all reference definitions in one file to make Markdown files more readable and ease link maintenance,
- replace simple Markdown links by badges,
- identify duplicate or broken links,
- identify Markdown files not listed in `SUMMARY.md`,
- identify unused Rust code examples,
- generate a sitemap from pages listed in SUMMARY.md and present in the selected HTML renderer output,
- extract fenced code bocks embedded into the Markdown to separate files for easier formatting, debugging and testing,
- replace code examples by mdBook [`#include`][mdbook-include] statements,
- conversely replace mdBook includes by the file contents.

## Installation and Usage

Consult the [User Guide][mdbook-utils-user-guide] for installation and usage instructions.

### Command-line Subcommands

- `refdefs`: Manage reference definitions.
    - `write`: Write existing reference definitions to a file.
    - `generate`: Generate reference definitions: GitHub badges (`--badges`, `--badge-style <STYLE>`) or definitions from the `Cargo.toml` dependencies of the code examples (`--from-dependencies`).
- `links`: Manage links.
    - `write-all`: Write all existing links to a Markdown file.
    - `write-inline`: Write the external (`http`/`https`) inline links and autolinks to a Markdown file, without modifying the sources. `--link-format reference|inline|autolink` selects the output syntax (default `reference`). Labels default to the kebab-cased link text; when two links with the same text point to different URLs, the later label gets a numeric suffix (`docs`, `docs-2`). To rewrite sources in place, use `book_tooling`'s `inline-links`.
    - `duplicate-links`: Identify duplicate links/labels.
    - `broken-links`: Identify broken links.
- `markdown`: Manage code blocks and includes.
    - `extract-code-examples`: Extract Rust code examples to separate files.
    - `replace-code-examples-by-includes`: Replace code examples with `{{#include}}` statements.
    - `replace-includes-by-contents`: Resolve `{{#include}}` statements.
    - `remove-includes`: Remove `{{#include}}` statements.
    - `identify-files-not-in-summary`: Find `.md` files missing from `SUMMARY.md`.
    - `identify-unused-rs-examples`: Find `.rs` files not included in any `.md` file.
    - `generate-crates`: Generate an index of the crates linked from the book.
    - `generate-categories`: Generate an index of the crates.io categories linked from the book.
- `sitemap`: Generate a `sitemap.xml` file (`--base-url`, `--map-index FROM:TO`). `README.md` pages are listed as `index.html`, as mdBook renders them; use `--map-index` for any other renamed page.
- `debug`: Write the Markdown parser events to a file.

Run `mdbook-utils <command> --help` for the options of each command.

### Environment Variables

- `MDBOOK_UTILS_MARKDOWN_DIR_PATH`: Path to the Markdown source directory (default: `./src/`).
- `MDBOOK_UTILS_BOOK_ROOT_DIR_PATH`: Path to the book's root directory containing `book.toml` (default: `.`).
- `MDBOOK_UTILS_BOOK_HTML_BUILD_DIR_PATH`: Path where `mdbook` outputs HTML (usually `./book/` for one renderer or `./book/html/` for multiple renderers).
- `MDBOOK_UTILS_BOOK_MARKDOWN_BUILD_DIR_PATH`: Path where the Markdown renderer writes expanded Markdown.
- `MDBOOK_UTILS_CARGO_TOML_DIR_PATH`: Directory where a `Cargo.toml` can be found.
- `MDBOOK_UTILS_DEFAULT_DEST_DIR_PATH`: Default directory for generated `mdbook-utils` files.
- `MDBOOK_UTILS_BASE_URL`: Base URL for sitemap generation (defaults to `output.html.site-url` in `book.toml`; there is no placeholder default).
- `RUST_LOG`: Logging level (error, warn, info, debug, trace).

The unprefixed names used before (for example `BASE_URL`) still work when the prefixed variable is not set, with a deprecation warning.

### Configuration via `book.toml`

`mdbook-utils` uses mdBook's [`book.toml` configuration][mdbook-config] for the Markdown source directory (`book.src`) and renderer output directories (`build.build-dir` and `[output.*]`). Relative paths are resolved from the directory containing `book.toml`, selected by `MDBOOK_UTILS_BOOK_ROOT_DIR_PATH`.

For a path with a command-line option, the precedence is command-line option, matching environment variable, `book.toml`, then that command's default. A missing `book.toml` is optional; an unreadable or malformed file produces an error when resolution reaches it. mdBook enables HTML when no renderer tables are configured. One renderer writes directly to `build.build-dir`; with multiple renderers, each writes to a renderer-named subdirectory such as `html` or `markdown`.

`book.toml` is the only TOML configuration source. There is no separate `mdbook-utils` TOML file.

## Development and Testing

`just test` runs the unit, integration, and doc tests, and the mdBook tests of the user guide and `test_book`. For the Rust tests alone, run `cargo test --all-targets --locked`. Unit tests live next to the code in `src/`; the `tests/` directory holds integration tests that run the compiled binary and the public API against temporary directories and the fixtures in `tests/fixtures/`. See the user guide's contributing section for details.

## Public API

`mdbook-utils`' underlying library also exposes a [public API][mdbook-utils-docs-rs] that may be used from your code.

## Key Links

`mdbook-utils` [(github)][mdbook-utils-github]  [(docs.rs)][mdbook-utils-docs-rs]  [(crates.io)][mdbook-utils-crates-io]  [(user guide)][mdbook-utils-user-guide]

[mdbook]: https://rust-lang.github.io/mdBook/
[mdbook-config]: https://rust-lang.github.io/mdBook/format/configuration/general.html
[mdbook-include]: https://rust-lang.github.io/mdBook/format/mdbook.html#including-files
[mdbook-utils-github]: https://github.com/john-cd/mdbook-utils
[mdbook-utils-docs-rs]: https://docs.rs/mdbook-utils/latest/mdbook_utils/
[mdbook-utils-crates-io]: https://crates.io/crates/mdbook-utils
[mdbook-utils-user-guide]: https://john-cd.github.io/mdbook-utils
[rust-howto]: https://www.john-cd.com/rust_howto/
[rust-howto-github]: https://github.com/john-cd/rust_howto
