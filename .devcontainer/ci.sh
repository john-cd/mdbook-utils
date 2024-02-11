#!/usr/bin/env bash
set -eux
set -o pipefail

echo "----------"

## Checks the Rust code formatting
## Fails if not formatted properly
cargo +nightly fmt --all --check

## Check dependencies
# cargo deny check \
#     && cargo outdated --exit-code 1 \
#     && cargo udeps \
#     && rm -rf ~/.cargo/advisory-db \
#     && cargo audit \
#     && cargo pants

## Fetch the dependencies
cargo fetch

## Compile
## - We prefer `cargo build ...` to `cargo check --all-targets --locked --profile ci`
## Some diagnostics and errors are only emitted during code generation, so they inherently won’t be reported with cargo check.
## - `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
## - see .cargo/config.toml for `ci` profile config.
cargo build --all-targets --locked --profile ci

## Scan the code for common errors
## - Elevate clippy warnings to errors, which will in turn fail the build.
cargo clippy --all-targets --locked --profile ci -- --deny warnings

## Test the code
cargo test --all-targets --locked --profile ci -- --show-output

## Generate docs.rs documentation
cargo doc --no-deps --locked

## Build and test the user guide
mdbook build ./userguide/
mdbook test ./userguide/

echo "----------"

## Do not remove.
## This is what will cause the dockerfile CMD to run.
exec "$@"
