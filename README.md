# mdBook Utilities

This is a **Work In Progress**. The command-line interface (CLI) and underlying API are subject to change. A summary of recently released changes is found in [RELEASES.md](./RELEASES.md). You may also consult [TODO.md](./TODO.md).

## What is `mdbook-utils`? What is it used for?

The `mdbook-utils` command-line tool manages _links_, _reference definitions_, and _code blocks_ in large collections of Markdown files, especially [`mdbook`][mdbook] source directories. It is the companion tool for the [Rust How-to][rust-howto] book ([github][rust-howto-github]).

`mdbook-utils` is useful for the following:

- centralize all reference definitions in one file to make Markdown files more readable and ease link maintenance,
- replace simple Markdown links by badges,
- identify duplicate or broken links,
- generate a sitemap file for your book or website,
- extract fenced code bocks embedded into the Markdown to separate files for easier formatting, debugging and testing,
- replace code examples by mdBook [`#include`][mdbook-include] statements,
- conversely replace mdBook includes by the file contents.

## Installation and Usage

Consult the [User Guide][mdbook-utils-user-guide] for installation and usage instructions.

## Public API

`mdbook-utils`' underlying library also exposes a [public API][mdbook-utils-docs-rs] that may be used from your code.

## Key Links

`mdbook-utils` [(github)][mdbook-utils-github]  [(docs.rs)][mdbook-utils-docs-rs]  [(crates.io)][mdbook-utils-crates-io]  [(user guide)][mdbook-utils-user-guide]

[mdbook]: https://rust-lang.github.io/mdBook/
[mdbook-include]: https://rust-lang.github.io/mdBook/format/mdbook.html#including-files
[mdbook-utils-github]: https://github.com/john-cd/mdbook-utils
[mdbook-utils-docs-rs]: https://docs.rs/mdbook-utils/latest/mdbook_utils/
[mdbook-utils-crates-io]: https://crates.io/crates/mdbook-utils
[mdbook-utils-user-guide]: https://john-cd.github.io/mdbook-utils
[rust-howto]: https://www.john-cd.com/rust_howto/
[rust-howto-github]: https://github.com/john-cd/rust_howto
