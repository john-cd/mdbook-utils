<!-- TODO Reviewed ??-??-2026 -->
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

## Release binaries

Pushing a tag such as `v0.2.0` runs `.github/workflows/release.yml`. It builds `--release --locked` binaries for Linux (x86_64), Windows (x86_64), and macOS (Apple silicon and Intel), packs each with `LICENSE` and `README.md` into `mdbook-utils-<target>-v<version>.tar.gz` (`.zip` on Windows) with a `.sha256` file, runs `--help` on the unpacked binary, and attaches the archives to a GitHub release. The archive names match `[package.metadata.binstall]` in `Cargo.toml`, so users can run `cargo binstall mdbook-utils`; `cargo binstall` falls back to building from source when no archive exists.

A manual run of the workflow builds and smoke-tests the archives without creating a release, which is the way to validate packaging without any credentials.

### Release checklist and rollback

1. Update `RELEASES.md` and the version in `Cargo.toml`, merge to `develop`, and wait for CI.
2. Publish to crates.io with the *Build, test, optionally deploy to Pages & publish* workflow (or the manual method above).
3. Push the tag `vX.Y.Z` to create the GitHub release.
4. Rollback: `cargo yank --version X.Y.Z mdbook-utils` hides a bad crates.io release without deleting it; delete the GitHub release and tag if the archives are bad, fix forward, and publish a new patch version.

## User guide hosting

The `pages.yml` workflow builds and tests the guide on every push to `develop`, writes `sitemap.xml` into the rendered book with `mdbook-utils sitemap --base-url https://john-cd.com/mdbook-utils/`, and deploys it to GitHub Pages. The site has no analytics: the guide is documentation for a command-line tool, so no visitor data is collected and no cookie or privacy notice is needed. This can be revisited if usage data becomes useful.

## Docker image for CI

CI installs the pinned Rust toolchain and tools directly on the runners (with `Swatinem/rust-cache`), so there is no published CI image. A prebuilt image would save little time compared to the cache, add a registry to maintain, and need rebuilding for every toolchain update. The `.devcontainer` image remains for local development and the manual publish method.

{{#include ../refs.md}}
