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

## Reference Definitions

`mdbook-utils refdefs` offers the following subcommands:

```txt
Manage reference definitions

Usage: mdbook-utils refdefs [OPTIONS] <COMMAND>

Commands:
  write   Write existing reference definitions to a file
  badges  Generate badges (reference definitions) for e.g. Github links
  help    Print this message or the help of the given subcommand(s)

Options:
  -y, --yes   Automatically answer `yes` to any user confirmation request
  -h, --help  Print help
```

## Links

`mdbook-utils links` currently offers two main subcommands:

```txt
Manage links

Usage: mdbook-utils links [OPTIONS] <COMMAND>

Commands:
  write-all     Write all existing links to a Markdown file
  write-inline  Write all existing inline / autolinks (i.e., not written as reference-style links) to a Markdown file
  help          Print this message or the help of the given subcommand(s)

Options:
  -y, --yes   Automatically answer `yes` to any user confirmation request
  -h, --help  Print help
```

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

`mdbook-utils sitemap` and `mdbook-utils debug` do not have subcommands.

## Command-line options

Command-line options vary by subcommand and include `-o` to set the path of the output file; `-m` to set the path of the source Markdown directory (`./src` or `./drafts` by default, depending on the subcommand); `-c` to set the path to the directory containing the `Cargo.toml` that declares the dependencies (Rust crates) used in your book; and `-t` to set the path to the destination directory. `-y` is a global option that skips confirmation dialogs and is useful when calling `mdbook-utils` from a script.

Use `mdbook-utils <command> <subcommand> --help` or `help <command> <subcommand>` for more details. The following illustrates options for `mdbook-utils sitemap`:

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

{{#include ./refs.md}}
