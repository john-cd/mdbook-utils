# Contributing

Pull requests, comments, and issue submissions are actively encouraged!

## Repo structure

`mdbook-utils` is written in [Rust][rust-lang] and follows the typical [cargo package layout][cargo-layout].

- The source code is in the `src` folder. The main executable is in `main.rs` and the `cli` module. It calls the API in `lib.rs`.
- A simple test `mdbook` book is found in `test_book`.
- The user guide' sources are in `docs`.
- The Dev Container and Docker (Compose) configuration files are found in `.devcontainer`.
  - `devcontainer.json` uses Docker Compose (configured in `compose.yaml` and `compose.override.yaml`), which in turn runs a container from `Dockerfile`.
- `.github` contains the continuous integration (GitHub Actions) workflow.

## Development Setup

### Using VS Code

Clone the repo and open the folder in [VS Code][vs-code]. Edit `.devcontainer/.env` if needed. VS Code should prompt you to open the code in a Docker container, which installs Rust tooling automatically. Make sure you have previously installed the following:

- [Dev Container extension][dev-container-extension]
- [Docker Desktop][docker-desktop] (or at least the Docker engine).

Note that opening the code folder in VS Code using Dev Containers may take a little while the first time around.

### Other

If you are not using VS Code, install the [Dev Container CLI][dev-container-CLI], use `docker compose` directly (see below), or simply install the required tools on your local machine.

The following works with Ubuntu and WSL:

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

## Build and test the code

The [`just`][just] command runner is configured to simplify compilation and testing.

Type `just` at a shell prompt for a list of commands:

```sh
just clean  # Clean Cargo's `target` and mdbook's `book` folders

just fmt    # Format all code

just check  # Check whether the code can compile

just build  # Build all code and books

just clippy # Scan all code for common mistakes

just test   # Test all code and books

just run <command>  # Run the tool

just doc    # Build and display the `cargo doc` documentation

just serve  # Display the user guide

just prep   # Run all the steps required before pushing code to GitHub

just update # Update Cargo.lock dependencies
```

## Docker Compose

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

[cargo-layout]: https://doc.rust-lang.org/cargo/guide/project-layout.html
[dev-container-CLI]: https://github.com/devcontainers/cli
[dev-container-extension]: https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers
[docker-desktop]: https://www.docker.com/products/docker-desktop/
[just]: https://just.systems/
[rust-lang]: https://www.rust-lang.org/
[vs-code]: https://code.visualstudio.com/
