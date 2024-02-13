# Usage

Run the tool without arguments to display the the list of commands:

```txt
Tools to manage links, reference definitions, and code examples in Markdown files, especially `mdbook` source directories.

Usage: mdbook-utils [OPTIONS] <COMMAND>

Commands:
  refdefs   Manage reference definitions
  links     Manage links
  markdown  Manage code blocks (embedded examples) and includes
  sitemap   Generate a sitemap.xml file from the list of Markdown files in a source directory
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
