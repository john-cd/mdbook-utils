# TODO

- README - add better usage explanation of env. vars, book.toml parsing, and command line options
- retest all commands / subcommands - WIP
- use the test_book in #[test]
- fix TODOs
- write_inline_links: remove internal links

- publish crate

- add unit tests

```rust
# [cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test() {
    }
}
```

New commands

- finish generation of refdefs from dependencies - WIP
- duplicate links - WIP
- broken links - WIP
- locate all autolink / inline references to external sites - WIP
- suggest label names based on URL type - WIP
- autoreplace autolinks / inline links by ref links

Links commands:

- add "link format" option to output links in reference, inline, autolink... formats

Markdown commands

- generate categories.md - WIP
- generate crates.md - WIP
- identify .md files not in SUMMARY.md
- identify .rs examples not used

Parse book.toml to extract `src`, etc...

Later

- review <https://docs.rs/markdown-gen/1.2.1/markdown_gen/markdown/index.html>
- review <https://crates.io/crates/parse-hyperlinks>
- align with CommonMark spec?
