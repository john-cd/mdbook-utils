# mdBook Utilities

This is a **Work In Progress**. The command-line interface (CLI) and underlying API are subject to change. A summary of recently released changes is found in [RELEASES.md](./RELEASES.md). You may also consult [TODO.md](./TODO.md).

## What is `mdbook-utils` for?

The `mdbook-utils` command-line tool manages _links_, _reference definitions_, and _code blocks_ in large collections of Markdown files, especially [mdbook](https://rust-lang.github.io/mdBook/) source directories. It is the companion tool for the ["Rust How-to"](https://www.john-cd.com/rust_howto/) book ([github](https://github.com/john-cd/rust_howto)).

`mdbook-utils` is useful for the following:

- centralize all reference definitions in one file to make Markdown files more readable and ease link maintenance,
- replace simple Markdown links by badges,
- identify duplicate or broken links,
- generate a sitemap file for your book or website,
- extract fenced code bocks embedded into the Markdown to separate files for easier formatting, debugging and testing,
- replace code examples by `{{#include  ... }}` statements,
- conversely replace includes by the file contents.

`mdbook-utils`' underlying library also exposes a [public API](https://docs.rs/mdbook-utils/latest/mdbook_utils/) that may be used from your code.

## Installation

Install the command-line tool using [`cargo`](https://doc.rust-lang.org/cargo/index.html):

```bash
cargo install mdbook-utils
```

then invoke `mdbook-utils` at a shell prompt.

For the bleeding edge development version, use:

```bash
cargo install --git https://github.com/john-cd/mdbook-utils
```

To uninstall the tool, enter the following in a shell:

```bash
cargo uninstall mdbook-utils
```

## Definitions

[`mdbook`](https://rust-lang.github.io/mdBook/) is a command-line tool to create books with [Markdown](https://en.wikipedia.org/wiki/Markdown). It is commonly used for Rust user guides, such as the [Rust book](https://doc.rust-lang.org/book/) and the [Rust How-to](https://www.john-cd.com/rust_howto/) book.

_Markdown_ is a lightweight, readable markup language for writing structured documents.

A Markdown _link_ can be an _autolink_, e.g. `<https://example.com>`, an _inline link_ like `[Example](https://example.com)`, or a _reference-style link_: `[The user will see this][thisisthelabel]`.

A reference-style link requires a _reference definition_ with a matching _label_:

~~~markdown
thisisthelabel: https://example.com/
~~~

_Images_ can be inserted using `![Image alternative text](link/to/image.png)` or, reference-style, `![Image][1]` followed by a _reference definition_ `[1]: <http://url/b.jpg>`.

More details may be found in the [CommonMark](https://commonmark.org/) spec.

A status _badge_ is a small image that provides at-a-glance information, for example the build status of a code repository. Badges are commonly displayed on GitHub READMEs and inserted in `mdbook` documentation as links to a crate's [docs.rs](https://docs.rs/) documentation, GitHub repo, or [crates.io](https://crates.io/) page. There is no "badge" concept in the Markdown spec; badges are simply clickable images e.g. `[ ![image-alt-text](link-to-image) ](link-to-webpage)`. More information about badges may be found in the [awesome-badges](https://github.com/badges/awesome-badges) repo and in the [shields.io](https://shields.io/) documentation.

Markdown _fenced code blocks_ (we will call them _code examples_ as well) are inserted between two  _code fences_ (e.g. sets of triple backticks), with an optional _info string_ (a.k.a. _attributes_ ) after the first backtick group:

~~~markdown
```rust
fn main() {}
```
~~~

`mdbook` allows [including files](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files) into your book via an _include statement_ written as `{{#include file.md}}`. mdBook interprets included files as Markdown. Since the include syntax is usually used for inserting code snippets and examples, it is often wrapped with ``` to display the file contents without interpreting them:

~~~markdown
```
{{#include file.rs}}
```
~~~

## Usage

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

In turn, most commands offer a menu of subcommands. `mdbook-utils refdefs` offers the following subcommands:

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

`mdbook-utils markdown` deals with fenced code blocks and includes:

```txt
Manage code blocks (embedded examples) and includes

Usage: mdbook-utils markdown [OPTIONS] <COMMAND>

Commands:
  extract-code-examples              Copy Rust code examples from the Markdown into .rs files
  replace-code-examples-by-includes  Replace Rust code examples from the Markdown by {{#include ...}} statements
  replace-includes-by-contents       Replace {{#include file.md}} by the file contents
  remove-includes                    Remove {{#include }} statements (and replace them by a hard-coded string)
  help                               Print this message or the help of the given subcommand(s)

Options:
  -y, --yes   Automatically answer `yes` to any user confirmation request
  -h, --help  Print help
```

`mdbook-utils sitemap` and `mdbook-utils debug` do not have subcommands.

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

### Defaults and environment variables

Each subcommand uses defaults that are overwritten by values in `book.toml` (if present), by environment variables (if set), or command-line options (the latter trumps the former).

You may export environment variables manually or store them in a `.env` file, which will be read automatically:

```bash
# Root directory of the book
# `book.toml` is looked up in BOOK_ROOT_DIR_PATH, if set,
# in the current working directory otherwise.
export BOOK_ROOT_DIR_PATH=./test_book/

# Markdown source directory
export MARKDOWN_DIR_PATH=./test_book/src/

# Directory where mdbook outputs the book's HTML and JS;
# typically ./book/ or ./book/html/
export BOOK_HTML_BUILD_DIR_PATH=./test_book/book/

# Directory where `mdbook` outputs the book's fully expanded Markdown,
# i.e. with all includes resolved, when `[output.markdown]` is added to `book.toml`.
# It is typically ./book/markdown/.
export BOOK_MARKDOWN_BUILD_DIR_PATH=./test_book/book/markdown/

# Directory where `Cargo.toml` may be found
export CARGO_TOML_DIR_PATH=./test_book/book/code/

# Default destination directory for mdbook-utils outputs.
export DEFAULT_DEST_DIR_PATH=./test_book/temp/

# Base url of the website where the book will be deployed
# (used to build sitemaps)
export BASE_URL=http://myexample.com/some_book/
```

You may also set the [`RUST_LOG`][rust-log] environment variable to display the logs.

See `cli/config.rs` for more details.

Note: `mdbook-utils` is not a mdbook [preprocessor](https://rust-lang.github.io/mdBook/for_developers/preprocessors.html) or [backend](https://rust-lang.github.io/mdBook/format/configuration/renderers.html) at this point.

## Library usage

To use the library in your code, add the crate to  your `Cargo.toml` as usual:

```bash
cargo add mdbook-utils
```

and peruse its [documentation](https://docs.rs/mdbook-utils/latest/mdbook_utils/).

Note that `cargo` changes the dash into an underscore, thus insert `use mdbook_utils::*;` or similar into your code.

## Contributing

Pull requests, comments, and issue submissions are actively encouraged!

### Repo structure

`mdbook-utils` is written in [Rust](https://www.rust-lang.org/) and follows the typical [cargo package layout](https://doc.rust-lang.org/cargo/guide/project-layout.html).

- The source code is in the `src` folder. The main executable is in `main.rs` and the `cli` module. It calls the API in `lib.rs`.
- A simple test `mdbook` book is found in `test_book`.
- The Dev Container and Docker (Compose) configuration files are found in `.devcontainer`.
  - `devcontainer.json` uses Docker Compose (configured in `compose.yaml` and `compose.override.yaml`), which in turn runs a container from `Dockerfile`.
- `.github` contains the continuous integration (GitHub Actions) workflow.

### Development Setup

#### Using VS Code

Clone the repo and open the folder in [VS Code][vs-code]. Edit `.devcontainer/.env` if needed. VS Code should prompt you to open the code in a `docker` container, which installs `rust` tooling automatically. Make sure you have previously installed

- [Dev Container extension][dev-container-extension]
- [Docker Desktop][docker-desktop] (or at least the Docker engine).

Note that opening the code folder in `VS Code` may take a little while the first time around.

#### Other

If you are not using `VS Code`, install the [Dev Container CLI][dev-container-CLI], use `Docker Compose` directly (see below), or simply install the required tools on your local machine.

The following works with `Ubuntu` and `Windows` `WSL`:

```bash
sudo apt-get update
rustup update
rustup component add clippy

rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
cargo install just
# Optional
cargo install mdbook
```

Review `.devcontainer/Dockerfile` for other optional dependencies.

### Build and test the code

The [`just`](https://just.systems/) command runner is configured to simplify compilation and testing.

Type `just` at a shell prompt for a list of commands:

```sh
just clean  # Clean the `target` folder

just fmt    # Format the code

just check  # Check whether the code can compile

just build  # Build the code

just clippy # Scan the code for common mistakes

just test   # Test the code

just run <command>  # Run the tool

just doc    # Generate the documentation

just prep   # Run all the steps required before pushing code to GitHub

just update # Update Cargo.lock
```

### Docker Compose

Test the `Docker Compose` setup used during developement (which `Dev Containers` runs) with:

```bash
cd ./.devcontainer
docker compose build   # uses compose.yaml and compose.override.yaml by default
docker compose up -d
# or simply
docker compose up --build -d
```

## Publish to crates.io

- Go to `crates.io`, sign in, and create an API token in `Account Settings` > `API Tokens`.
- Use `cargo login` to save the token in `$CARGO_HOME/credentials.toml`.
- `just build; just clippy; just run; just doc; cargo package`
- Review the packaging output in `/cargo-target-mdbook-utils/target/package`.
- When ready, `cargo publish --dry-run; cargo publish`

## Links

`mdbook-utils`' [GitHub repo][mdbook-utils-github]

[dev-container-CLI]: https://github.com/devcontainers/cli
[dev-container-extension]: https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers
[docker-desktop]: https://www.docker.com/products/docker-desktop/
[mdbook-utils-github]: https://github.com/john-cd/mdbook-utils
[rust-log]: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html
[vs-code]: https://code.visualstudio.com/
