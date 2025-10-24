alias f := fmt
alias fa := fmt
alias b := build
alias ba := build
alias ca := clippy
alias ck := check
alias cka := check
alias t := test
alias ta := test
alias r := run
alias d := doc
alias s := serve
alias p := prep

set shell := ["bash", "-uc"]
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

bin_dir := clean(join(source_directory(), "..", "bin"))
target_dir := clean(join(source_directory(), "..", "target", "mdbook-utils"))

set quiet

[no-exit-message]
_default:
  @just --list --unsorted

# Clean Cargo's `target` and mdbook's `book` folders
clean:
  cargo clean
  mdbook clean ./user_guide/
  mdbook clean ./test_book/

# Format all code
fmt:
  cargo +nightly fmt --all
  echo "DONE"

# Check whether the code can compile
check:
  cargo check --all-targets --locked
  echo "DONE"
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
  cargo clippy --all-targets --locked -- --deny warnings

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
doc: _builddoc
  cd "{{join(target_dir, "doc")}}" && python3 -m http.server 9000

# [windows]
# doc:
#   echo Not implemented.

_builddoc:
  cargo clean --doc
  cargo doc --no-deps --locked # --document-private-items

# Display the user guide
serve:
  mdbook serve ./user_guide/

# Run all the steps required before pushing code to GitHub
prep: fmt clean build clippy test _builddoc

@release:
  echo "Build the tools in $(pwd) in release mode and copy to ../bin"
  cargo +nightly build --bins --locked --all-features --release -Z unstable-options --artifact-dir "{{bin_dir}}"

## Utilities --------------------------------------

# Update Cargo.lock dependencies
[confirm]
update:
  cargo update
