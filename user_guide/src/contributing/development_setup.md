## Development Setup

### Using VS Code

Clone the repo and open the folder in [VS Code][vs-code]. Edit `.devcontainer/.env` if needed. VS Code should prompt you to open the code in a Docker container, which installs Rust tooling automatically. Make sure you have previously installed the following:

- [Dev Container extension][dev-container-extension]
- [Docker Desktop][docker-desktop] (or at least the Docker engine).

Note that opening the code folder in VS Code using Dev Containers may take a little while the first time around.

### Other

If you are not using VS Code, install the [Dev Container CLI][dev-container-cli], use `docker compose` directly (see [below](./build_and_test.md)), or simply install the required tools on your local machine.

The following works with Ubuntu (including WSL):

```bash
sudo apt-get update
rustup update
rustup component add clippy
rustup toolchain install nightly
rustup component add rustfmt --toolchain nightly
cargo install just
cargo install mdbook
```

Review `.devcontainer/Dockerfile` for other optional dependencies.

{{#include ../refs.md}}
