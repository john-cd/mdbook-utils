alias f := fmt
alias b := build
alias c := clippy
alias t := test
alias r := run
alias d := doc
alias s := serve
alias p := prep

set windows-shell := ["cmd.exe", "/c"]

default:
  @just --list --unsorted
# or: @just --choose

# Clean Cargo's `target` and mdbook's `book` folders
clean:
  cargo clean
  mdbook clean ./user_guide/
  mdbook clean ./test_book/

# Format all code
fmt:
  cargo +nightly fmt --all

# Check whether the code can compile
check:
  cargo check --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Build all code and books
build:
  cargo build --all-targets --locked
  mdbook build ./user_guide/
  mdbook build ./test_book/
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
# optional: --timings

# Scan all code for common mistakes
clippy:
  cargo clippy --all-targets --locked

# Test all code and books
test:
  cargo test --all-targets --locked
  mdbook test ./user_guide/
  mdbook test ./test_book/
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

help := 'help'
empty := ''

# Run the tool
run cmd=help subcmd=empty:
  cargo run --locked -- {{cmd}} {{subcmd}}

# Build and display the `cargo doc` documentation
[unix]
doc: _buildoc
  cd /cargo-target-mdbook-utils/target/doc/ ; python3 -m http.server 9000

_buildoc:
  cargo clean --doc
  cargo doc --no-deps --locked # --document-private-items

# Display the user guide
serve:
  mdbook serve ./user_guide/

# Run all the steps required before pushing code to GitHub
prep: fmt clean build clippy test _buildoc

## Utilities --------------------------------------

# Update Cargo.lock dependencies
[confirm]
update:
  cargo update
