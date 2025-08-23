# TODO

## Features

- refdefs
  - write   Write existing reference definitions to a file
  - badges  Generate badges (reference definitions) for e.g. Github links
  - ?
    - TODO finish generation of refdefs from dependencies - WIP
- links
  - write-all     Write all existing links to a Markdown file
  - write-inline  Write all existing inline / autolinks (i.e., not written as reference-style links) to a Markdown file
    - TODO add "link format" option to output links in reference, inline, autolink... formats
  - ?
- markdown
  - extract-code-examples              Copy Rust code examples from the Markdown into .rs files
  - replace-code-examples-by-includes  Replace Rust code examples from the Markdown by {{#include ...}} statements
  - replace-includes-by-contents       Replace {{#include file.md}} by the file contents
  - remove-includes                    Remove {{#include }} statements (and replace them by a hard-coded string)

- sitemap

- debug

- generate categories.md - WIP
- generate crates.md - WIP
- TODO identify .md files not in SUMMARY.md
- TODO identify .rs examples not used
- TODO duplicate links - WIP
- TODO broken links - WIP
- TODO locate all autolink / inline references to external sites - WIP
- TODO suggest label names based on URL type - WIP
- TODO autoreplace autolinks / inline links by ref links - TODO

## TODOs

- improve CLI help messages

- improve user guide - description of functionality
- README - add better usage explanation of env. vars, book.toml parsing, and command line options - WIP

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
