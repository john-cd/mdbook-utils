## Publish to crates.io

1. Manual method

- Go to `crates.io`, sign in, and create an API token in `Account Settings` > `API Tokens`.
- Use `cargo login <token>` to save the token in `$CARGO_HOME/credentials.toml`.
- `just build; just clippy; just run; just doc; cargo package --locked`
- Review the packaging output in `target/mdbook-utils/package` or use `cargo package --list`.
- When ready, `cargo publish --locked --dry-run; cargo publish --locked`

2. Docker Compose method

- Pass the `publish.sh` script (and required argument `-y`) as a `command` to `docker compose run`.
- Pass the `CRATES_TOKEN` env. variable (which is used by `publish.sh`) to Docker Compose using [`--env`][docker-compose-env-vars].

```bash
export CRATES_TOKEN="<token from crates.io>"
docker compose -f .devcontainer/compose.yaml -f .devcontainer/compose-ci.yaml run \
       --rm --env CRATES_TOKEN mdbook-utils .devcontainer/publish.sh -y
```

{{#include ../refs.md}}
