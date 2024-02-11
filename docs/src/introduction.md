# Introduction

## What is `mdbook-utils` for?

The `mdbook-utils` command-line tool manages _links_, _reference definitions_, and _code blocks_ in large collections of Markdown files, especially [mdbook](https://rust-lang.github.io/mdBook/) source directories. It is the companion tool for the ["Rust How-to"](https://www.john-cd.com/rust_howto/) book ([github](https://github.com/john-cd/rust_howto)).

`mdbook-utils` is useful for the following:

- centralize all reference definitions in one file to make Markdown files more readable and ease link maintenance,
- replace simple Markdown links by badges,
- identify duplicate or broken links,
- generate a sitemap file for your book or website,
- extract fenced code bocks embedded into the Markdown to separate files for easier formatting, debugging and testing,
- replace code examples by `{{#include  ... }}` statements,
- conversely replace includes by the file contents.

`mdbook-utils`' underlying library also exposes a [public API](https://docs.rs/mdbook-utils/latest/mdbook_utils/) that may be used from your code.

## Key Links

`mdbook-utils` [(GitHub repo)][mdbook-utils-github]  [(docs.rs)][mdbook-utils-docs-rs]  [(crates.io)][mdbook-utils-crates-io]  [(user guide - this book)][mdbook-utils-user-guide]

[mdbook-utils-github]: https://github.com/john-cd/mdbook-utils
[mdbook-utils-docs-rs]: https://docs.rs/mdbook-utils/latest/mdbook_utils/
[mdbook-utils-crates-io]: https://crates.io/crates/mdbook-utils
[mdbook-utils-user-guide]: https://john-cd.github.io/mdbook-utils
