# Introduction

## What is `mdbook-utils`? What is it used for?

The `mdbook-utils` command-line tool manages _links_, _reference definitions_, and _code blocks_ in large collections of Markdown files, especially [mdbook][mdbook] source directories. It is the companion tool for the [Rust How-to][rust-howto] book ([github][rust-howto-github]).

`mdbook-utils` is useful for the following:

- centralize all reference definitions in one file to make Markdown files more readable and ease link maintenance,
- replace simple Markdown links by badges,
- identify duplicate or broken links,
- generate a sitemap file for your book or website,
- extract fenced code bocks embedded into the Markdown to separate files for easier formatting, debugging and testing,
- replace code examples by mdbook [`#include`][mdbook-include] statements,
- conversely replace includes by the file contents.

`mdbook-utils`' underlying library also exposes a [public API](https://docs.rs/mdbook-utils/latest/mdbook_utils/) that may be used from your code.

## Key Links

`mdbook-utils` [(GitHub repo)][mdbook-utils-github]  [(docs.rs)][mdbook-utils-docs-rs]  [(crates.io)][mdbook-utils-crates-io]  [(user guide - this book)][mdbook-utils-user-guide]

{{#include ./refs.md}}
