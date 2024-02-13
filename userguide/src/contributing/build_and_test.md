## Build and test the code and user guide

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

Use the following commands to build and test the code and user guide using the Continuous Integration configuration:

```bash
docker compose -f .devcontainer/compose.yaml -f .devcontainer/compose-ci.yaml run \
--build --rm mdbook-utils
```

{{#include ../refs.md}}
