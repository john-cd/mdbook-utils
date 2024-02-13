## Command-line options

Command-line options vary by subcommand and include `-o` to set the path of the output file; `-m` to set the path of the source Markdown directory (`./src` or `./drafts` by default, depending on the subcommand); `-c` to set the path to the directory containing the `Cargo.toml` that declares the dependencies (Rust crates) used in your book; and `-t` to set the path to the destination directory.

`-y` is a global option that skips confirmation dialogs and is useful when calling `mdbook-utils` from a script.

Use `mdbook-utils <command> <subcommand> --help` or `help <command> <subcommand>` for more details.

The following illustrates options for `mdbook-utils sitemap`:

```txt
Generate a sitemap.xml file from the list of Markdown files in a source directory

Usage: mdbook-utils sitemap [OPTIONS]

Options:
  -m, --markdown-dir <DIR>  Source directory containing the source Markdown files
  -b, --base-url <URL>
  -o, --output <FILE>       Path of the file to create
  -y, --yes                 Automatically answer `yes` to any user confirmation request
  -h, --help                Print help
```

{{#include ../refs.md}}
