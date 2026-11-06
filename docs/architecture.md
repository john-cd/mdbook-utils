# Architecture

<!-- TODO Reviewed ??-??-2026 -->

`mdbook-utils` is one crate with a library (`src/lib.rs`) and a thin CLI
(`src/main.rs`, `src/cli/`).

## Layers

- `cli/`: argument parsing (`clap`), configuration precedence (CLI, environment,
  `.env`, `book.toml`), and one `*_commands.rs` module per command group. It
  only calls the library.
- `api/`: the public library surface (`links`, `refdefs`, `markdown`,
  `categories`, `sitemap`, `debug`). Each function takes a source directory and
  a destination file.
- `lib.rs::helper`: the shared pipeline. It checks the source directory, reads
  all Markdown files, builds one `pulldown-cmark` parser, runs a writer closure
  into an in-memory buffer, and writes the destination atomically, so the
  destination is untouched on failure.
- `parser/`: parser construction and link extraction, including broken
  reference handling.
- `link/`: the `Link` model, builders, formatting, and URL classification rules.
- `write_from_parser/`, `generate/`: writers that turn parsed links into
  reference definitions, badges, indexes, and crate lists.
- `fs/`: directory checks, Markdown discovery, atomic writes.
- `sitemap/`, `dependencies/`: sitemap generation and `cargo tree` based
  dependency lookup.

## Data flow

source dir -> `fs` (read all `.md`) -> `parser` (events) -> writer
(`link`, `write_from_parser`, `generate`) -> buffer -> atomic write.

## Decisions

- Library functions perform file I/O themselves; moving to pure
  `String`/`Vec` returns is tracked in issue 259.
- Writing is atomic (temp file then rename) to avoid half-written outputs.
- No unsafe code (`unsafe_code = "deny"`).

## Examples and benchmarks

- `cargo run --example generate_indexes`
- `cargo run --example link_report`
- `cargo bench` (std-only timing in `benches/parse_links.rs`)
