<!-- TODO Reviewed ??-??-2026 -->
## Links

`mdbook-utils links` currently offers several subcommands:

```txt
Manage links

Usage: mdbook-utils links [OPTIONS] <COMMAND>

Commands:
  write-all        Write all existing links to a Markdown file
  write-inline     Write all existing inline / autolinks (i.e., not written as reference-style links) to a Markdown file
  duplicate-links  Identify links that repeat the same label, URL and title (inline links: the same URL) and write them to a Markdown file
  broken-links     Identify broken links (i.e. without reference definition) and write to a Markdown file
  help             Print this message or the help of the given subcommand(s)

Options:
  -y, --yes        Automatically answer `yes` to any user confirmation request
      --log <LOG>  Set the logging level (error, warn, info, debug, trace) [env: RUST_LOG=]
  -h, --help       Print help
```

`links write-inline` writes only inline links and autolinks that point to
external `http` / `https` sites; relative paths, fragments, other schemes, and
links inside code are excluded. Use `--link-format <reference|inline|autolink>`
(default `reference`) to choose the output syntax. The `autolink` format drops
link text and titles.

Link extraction follows CommonMark link syntax as implemented by
`pulldown-cmark`, including inline links, reference links, images, and URI
autolinks written in angle brackets. Plain URL text is not treated as an
autolink. Link-like text in fenced or indented code blocks and code spans is
preserved, as are mdBook helpers such as `{{# include path/to/file.rs}}`.

The parser enables the existing extensions for tables, footnotes, and
strikethrough; it does not enable GitHub's bare-URL autolinks or unrelated
Markdown dialects. See the [CommonMark specification][commonmark-spec] for
destination escaping and link syntax.

Generic website rules classify absolute `http` and `https` destinations. They
separate query strings and fragments from the host and final path segment,
preserve Unicode and percent-encoded text for matching, and reject destinations
without a host or with raw whitespace. This is a parsing and classification
boundary: mdBook-utils does not check whether a destination is reachable or
validate every URI component. A destination accepted by CommonMark syntax may
still contain a malformed percent escape and remains its original destination.

{{#include ../refs.md}}
