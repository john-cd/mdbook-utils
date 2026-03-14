# TODO

## Features

- refdefs
  - [x] write existing reference definitions to a file
  - [x] badges (generate reference definitions for GitHub links)
  - [x] generation of refdefs from dependencies
- links
  - [x] write-all (write all existing links to a Markdown file)
  - [x] write-inline (write all existing inline / autolinks to a Markdown file)
    - TODO: add "link format" option to output links in reference, inline, autolink formats
  - [x] duplicate-links (identify duplicate links / labels)
  - [x] broken-links (identify broken links)
- markdown
  - [x] extract-code-examples (copy Rust code examples from the Markdown into .rs files)
  - [x] replace-code-examples-by-includes (replace Rust code examples from the Markdown by {{#include ...}} statements)
  - [x] replace-includes-by-contents (replace {{#include file.md}} by the file contents)
  - [x] remove-includes (remove {{#include }} statements and replace them by a hard-coded string)
  - [x] identify .md files not in SUMMARY.md
  - [x] identify .rs examples not used
  - [x] generate crates.md
  - [ ] generate categories.md (currently a stub)

- [x] sitemap (generate a sitemap.xml file)

- [x] debug (parse Markdown code as events and write to a file)

- TODO: locate all autolink / inline references to external sites
- TODO: suggest label names based on URL type
- TODO: autoreplace autolinks / inline links by ref links

## High Priority

- [ ] Add unit tests (WIP)
- [ ] Use `test_book` in automated (integration) tests
- [ ] Handle nested directories more accurately in `SUMMARY.md` link parsing
- [ ] Support other ways of including/using .rs files beyond `{{#include ...}}`
- [ ] Improve the logic for determining output directories in `book.toml` to fully match mdBook's behavior
- [ ] Improve generic URL regexes in `rules.rs`

## TODOs

- [x] improve CLI help messages

- [x] improve user guide - description of functionality
- [x] README - add better usage explanation of env. vars, book.toml parsing, and command line options - WIP

- move common functionality to separate library?
  - move cli to bin folder? or create a cargo workspace?
  - make this repo a submodule of rust_howto
  - share with tools

- change port used by mdbook serve ./user_guide/
- test publish.yml

- fix TODOs
- write_inline_links: remove internal links

- publish as a binary for use by cargo binstall

- add interactivity & prompt for destination paths, etc
- config file in TOML format?

- sitemap and GA for user guide

- add unit tests - WIP
- use test_book in automated (integration) tests

- Github Action: publish Docker image and use in cache_from? [publishing-docker-images][publishing-docker-images] [publishing-docker-images]: https://docs.github.com/en/actions/publishing-packages/publishing-docker-images

- make more functions fully public

Later

- review [markdown-gen][c-markdown_gen]
[c-markdown_gen]: https://docs.rs/markdown-gen/1.2.1/markdown_gen/markdown/index.html
- review [parse-hyperlinks][c-parse-hyperlinks-crates.io] [c-parse-hyperlinks-crates.io]: https://crates.io/crates/parse-hyperlinks
- align with CommonMark spec?
