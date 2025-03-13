## Markdown

`mdbook-utils markdown` deals with fenced code blocks and includes:

```txt
Manage code blocks (embedded examples) and includes

Usage: mdbook-utils markdown [OPTIONS] <COMMAND>

Commands:
  extract-code-examples              Copy Rust code examples from the Markdown into .rs files
  replace-code-examples-by-includes  Replace Rust code examples from the Markdown by #include statements
  replace-includes-by-contents       Replace #include statements by the file contents
  remove-includes                    Remove #include statements (and replace them by a hard-coded string)
  help                               Print this message or the help of the given subcommand(s)

Options:
  -y, --yes   Automatically answer `yes` to any user confirmation request
  -h, --help  Print help
```

{{#include ../refs.md}}
