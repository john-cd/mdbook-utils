<!-- TODO Reviewed ??-??-2026 -->
# Usage

Run the tool without arguments to display the the list of commands:

```txt
Tools to manage links, reference definitions, and code examples in Markdown files, especially `mdbook` source directories.

Usage: mdbook-utils [OPTIONS] <COMMAND>

Commands:
  refdefs   Manage reference definitions
  links     Manage links
  markdown  Manage code blocks (embedded examples) and includes
  sitemap   Generate a sitemap.xml file from published HTML pages linked in SUMMARY.md
  debug     Parse the entire Markdown code as events and write them to a file
  help      Print this message or the help of the given subcommand(s)

Options:
  -y, --yes      Automatically answer `yes` to any user confirmation request
  -h, --help     Print help
  -V, --version  Print version
```

In turn, most commands offer a menu of subcommands.

{{#include ./usage/refdefs.md}}

{{#include ./usage/links.md}}

{{#include ./usage/markdown.md}}

`mdbook-utils sitemap` and `mdbook-utils debug` do not have subcommands.

{{#include ./usage/command_line_options.md}}

{{#include ./refs.md}}

## Which tool for what

`mdbook-utils` and [`book_tooling`](https://github.com/john-cd/book_tooling) overlap. The split is:

| Task | Tool |
| --- | --- |
| Sitemap, `book.toml` path resolution | `mdbook-utils` |
| Extract, replace, or remove `{{#include}}` and code examples; unused `.rs` examples; files not in `SUMMARY.md` | `mdbook-utils` |
| Crate and category indexes, dependency reference definitions, GitHub badges | `mdbook-utils` |
| Reports on links (`links write-all`, `duplicate-links`, `broken-links`) | `mdbook-utils` (read-only reports) |
| Rewriting inline links as reference links in place, reference-definition clean-up, HTTP link checking, search | `book_tooling` |

`mdbook-utils` keeps pulldown-cmark for its read-only scans and does not rewrite link syntax in place. Commands that overlap with `book_tooling` (`links write-inline`, `refdefs write`) stay as reports and are not extended further.
