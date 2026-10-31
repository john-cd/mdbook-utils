# TODO Reviewed ??-??-2026.
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
# Recipes run in Bash on every OS: Git Bash on Windows.
set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

rustfmt_toolchain_default := trim(replace_regex(read(join(source_directory(), "rustfmt-toolchain.toml")), '(?s).*channel\s*=\s*"([^"]+)".*', "$1"))
rustfmt_toolchain_override := env("RUSTFMT_TOOLCHAIN", "")
rustfmt_toolchain := if rustfmt_toolchain_override == "" { rustfmt_toolchain_default } else { rustfmt_toolchain_override }
checkout_name := file_name(source_directory())
worktree_name := if checkout_name == "mdbook-utils" { "main" } else { checkout_name }
cache_home := env("XDG_CACHE_HOME", join(home_directory(), ".cache"))
repo_root := source_directory()
sccache_base := if file_name(parent_directory(repo_root)) == ".worktrees" { parent_directory(parent_directory(repo_root)) } else { repo_root }
inherited_sccache_bases := env("SCCACHE_BASEDIRS", "")
sccache_base_separator := if os() == "windows" { ";" } else { ":" }
target_root := if os() == "windows" { "D:/target/mdbook-utils" } else { join(cache_home, "cargo-target/mdbook-utils") }
build_root := if os() == "windows" { "C:/code_build/mdbook-utils" } else { join(cache_home, "cargo-build/mdbook-utils") }
target_dir := target_root + "/" + worktree_name
build_dir := build_root + "/" + worktree_name
export CARGO_TARGET_DIR := target_dir
export CARGO_BUILD_TARGET_DIR := target_dir
export CARGO_BUILD_BUILD_DIR := build_dir
export RUSTC_WRAPPER := "sccache"
export CARGO_BUILD_RUSTC_WRAPPER := "sccache"
export CARGO_INCREMENTAL := "0"
export SCCACHE_DIR := if os() == "windows" { "D:/sccache" } else { join(cache_home, "sccache") }
export SCCACHE_CACHE_SIZE := "20G"
export SCCACHE_BASEDIRS := if inherited_sccache_bases == "" { sccache_base } else { inherited_sccache_bases + sccache_base_separator + sccache_base }


bin_dir := join(build_root, worktree_name, "artifacts")

set quiet

[no-exit-message]
_default:
  @just --list --unsorted

# Run any cargo command with this justfile's target/build dirs, e.g. `just cargo tree`
[positional-arguments]
cargo *args:
  cargo "$@"

# Clean Cargo's `target` and mdbook's `book` folders
clean:
  cargo clean
  mdbook clean ./user_guide/
  mdbook clean ./test_book/

# Format all code
fmt:
  cargo +{{rustfmt_toolchain}} fmt --all
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
  cd "{{join(target_dir, "doc")}}" && (python3 -m http.server 9000 || python -m http.server 9000)

_builddoc:
  cargo clean --doc
  cargo doc --no-deps --locked # --document-private-items

# Display the user guide
serve:
  mdbook serve ./user_guide/ -p 3001

# Run all the steps required before pushing code to GitHub
prep: fmt clean build clippy test _builddoc

@release:
  echo "Build the tools in $(pwd) in release mode and copy to ../bin"
  cargo +{{rustfmt_toolchain}} build --bins --locked --all-features --release -Z unstable-options --artifact-dir "{{bin_dir}}"

## Utilities --------------------------------------

# Update Cargo.lock dependencies
[confirm]
update:
  cargo update
