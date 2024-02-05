# mdBook Utilities

This is a **Work In Progress**.

The `mdbook-utils` command-line tool manages links, reference definitions, and code examples in Markdown files, especially `mdbook` source directories.
It is the companion tool for the ["Rust How-to"](https://www.john-cd.com/rust_howto/) book ([github](https://github.com/john-cd/rust_howto)).

## Installation

```bash
cargo install mdbook-utils
```

## Usage

Run the tool without arguments to display the the list of commands:

```txt
  refdefs   Manage reference definitions
  links     Manage links
  markdown  Manage code blocks (embedded examples) and includes
  sitemap   Generate a sitemap.xml file from the list of Markdown files in a source directory
  debug     Parse the entire Markdown code as events and write them to a file
  help      Print this message or the help of the given subcommand(s)
```

In turn, each command offers a menu of subcommands. Try e.g. `mdbook-utils refdefs`.

Use `mdbook-utils <command> <subcommand> --help` or `help <command> <subcommand>` for more details.

### Command-line options and environment variables

Each subcommand uses defaults that are overwritten by values in `book.toml` (if present), by environment variables (if set), or command-line options (the latter trumps the former).

Command-line options vary by subcommand and include `-o` to set the path of the output file; `-m` to set the path of the source Markdown directory (`./src` or `./drafts` by default, depending on the subcommand); `-c` to set the path to the directory containing the `Cargo.toml` that declares the dependencies (Rust crates) used in the book; and `-t` to set the path to the destination directory.

You may export environment variables manually or store them in a `.env` file:

```bash
# Root directory of the book
# `book.toml` is looked up in BOOK_ROOT_DIR_PATH, if set,
# otherwise the current working directory.
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

## Development

The following is of interest only if you want to contribute to the project.

`mdbook-utils` is written in [Rust](https://www.rust-lang.org/).

### Repo structure

- The source code is in the `src` folder. The main executable is in `main.rs` and the `cli` module. It calls the API in `lib.rs`.
- A simple test `mdbook` book is found in `test_book`.
- The Dev Container and Docker (Compose) configuration files are found in `.devcontainer`.
  - `devcontainer.json` uses Docker Compose (configured in `compose.yaml` and `compose.override.yaml`), which in turn creates a container from `Dockerfile`.
- `.github` contains the CI GitHub Actions workflow.

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
