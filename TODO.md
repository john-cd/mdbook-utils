# TODO

- change port used by mdbook serve ./user_guide/
- test publish.yml

- README - add better usage explanation of env. vars, book.toml parsing, and command line options - WIP

- fix TODOs
- write_inline_links: remove internal links

- publish as a binary for use by cargo binstall

- add interactivity & prompt for destination paths, etc
- config file in TOML format?

- finish generation of refdefs from dependencies - WIP
- duplicate links - WIP
- broken links - WIP
- locate all autolink / inline references to external sites - WIP
- suggest label names based on URL type - WIP

- sitemap and GA for user guide

- add unit tests - WIP
- use test_book in automated (integration) tests

- Github Action: publish Docker image and use in cache_from? [publishing-docker-images][publishing-docker-images] [publishing-docker-images]: https://docs.github.com/en/actions/publishing-packages/publishing-docker-images

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

- review [markdown-gen][c-markdown_gen]
[c-markdown_gen]: https://docs.rs/markdown-gen/1.2.1/markdown_gen/markdown/index.html
- review [parse-hyperlinks][c-parse-hyperlinks-crates.io] [c-parse-hyperlinks-crates.io]: https://crates.io/crates/parse-hyperlinks
- align with CommonMark spec?
