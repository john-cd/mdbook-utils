## Repo structure

`mdbook-utils` is written in [Rust][rust-lang] and follows the typical [cargo package layout][cargo-layout].

- The source code is in the `src` folder. The main executable is in `main.rs` and the `cli` module. It calls the API in `lib.rs`.
- A simple test `mdbook` book is found in `test_book`.
- The user guide' sources are in `user_guide`.
- The Dev Container and Docker (Compose) configuration files are found in `.devcontainer`.
  - `devcontainer.json` uses Docker Compose (configured in `compose.yaml` and `compose.override.yaml`), which in turn runs a container from `Dockerfile`.
- `.github` contains the continuous integration (GitHub Actions) workflow.

{{#include ../refs.md}}
