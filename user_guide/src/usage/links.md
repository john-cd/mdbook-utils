## Links

`mdbook-utils links` currently offers several subcommands:

```txt
Manage links

Usage: mdbook-utils links [OPTIONS] <COMMAND>

Commands:
  write-all        Write all existing links to a Markdown file
  write-inline     Write all existing inline / autolinks (i.e., not written as reference-style links) to a Markdown file
  duplicate-links  Identify duplicate links / labels and write to a Markdown file
  broken-links     Identify broken links (i.e. without reference definition) and write to a Markdown file
  help             Print this message or the help of the given subcommand(s)

Options:
  -y, --yes   Automatically answer `yes` to any user confirmation request
  -h, --help  Print help
```

{{#include ../refs.md}}
