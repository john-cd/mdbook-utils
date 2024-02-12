# TODO

- fix env var CRATES_TOKEN not passed to Docker Compose?
- test publish.yml
- publish 0.1.3

- README - add better usage explanation of env. vars, book.toml parsing, and command line options - WIP
- split the user guide into more pages (reenable includes?)

- fix TODOs
- write_inline_links: remove internal links

- add interactivity & prompt for destination paths, etc

- finish generation of refdefs from dependencies - WIP
- duplicate links - WIP
- broken links - WIP
- locate all autolink / inline references to external sites - WIP
- suggest label names based on URL type - WIP

- sitemap and GA for user guide

- config file in TOML format?

- add unit tests - WIP
- use test_book in automated (integration) tests

- make more functions fully public

New commands:

- autoreplace autolinks / inline links by ref links

Links commands:

- add "link format" option to output links in reference, inline, autolink... formats

Markdown commands

- generate categories.md - WIP
- generate crates.md - WIP
- identify .md files not in SUMMARY.md
- identify .rs examples not used

Later

- review <https://docs.rs/markdown-gen/1.2.1/markdown_gen/markdown/index.html>
- review <https://crates.io/crates/parse-hyperlinks>
- align with CommonMark spec?
