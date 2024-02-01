alias f := fmt
alias b := build
alias c := clippy
alias t := test
alias r := run
alias d := doc
alias p := prep

set windows-shell := ["cmd.exe", "/c"]

default:
  @just --list --unsorted
# or: @just --choose

# Clean Cargo's `target`
clean:
  cargo clean

# Format all code
fmt:
  cargo +nightly fmt --all

# Check all code
check:
  cargo check --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Build all code
build:
  cargo build --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
# optional: --timings

# Scan all code for common mistakes
clippy:
  cargo clippy --all-targets --locked

# Test all code
test:
  cargo test --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

help := 'help'
empty := ''

# Run
run cmd=help subcmd=empty:
  cargo run --locked -- {{cmd}} {{subcmd}}

# Build and display the `cargo doc` documentation
[unix]
doc:
  cargo clean --doc
  cargo doc --no-deps --locked
  cd /cargo-target-mdbook-utils/target/doc/ ; python3 -m http.server 9000

# Prepare for git push
prep: fmt clean build clippy test

## Utilities --------------------------------------

# Update Cargo.lock dependencies
[confirm]
update:
  cargo update
