# Purpose: Build, validate, and maintain this Rust workspace with its configured tooling environment.
# Usage: Run just --list from this directory to inspect the available recipes.

# TODO Reviewed ??-??-2026.
# Uniform task names (short aliases; old names kept as aliases).
alias f := format
alias fmt := format
alias fa := format-all
alias fc := format-check
alias fmt-check := format-check
alias l := lint
alias lf := lint
alias lc := lint-check
alias clippy := lint-check
alias clippy-all := lint-check
alias lint-fix := lint
alias la := lint-all
alias c := check
alias ck := check
alias cka := check
alias ca := check-all
alias b := build
alias ba := build
alias br := release
alias build-release := release
alias t := test
alias ta := test
alias test-all := test
alias r := run
alias d := doc
alias docall := doc-serve
alias s := serve
alias cl := clean
alias de := deny
alias u := update
alias p := prep
alias hi := hooks-install
alias hr := hooks-run

# Storage-root overrides (existing environment values win over platform defaults):
# CODE_REPOS_ROOT names the canonical checkout root for tools; CODE_TEMP_ROOT holds temporary files.
# CODE_PACKAGES_ROOT holds package-manager data (including Cargo, npm, and NuGet caches);
# CODE_TOOLS_ROOT holds portable tool installations.
# CODE_TARGET_ROOT stores compiled Cargo target artifacts; CODE_BUILD_ROOT stores intermediate
# Cargo build files (the Windows helper requires this root to be on NTFS).
# CODE_SCCACHE_ROOT selects the compiler cache; SCCACHE_DIR is accepted as a legacy fallback.
set shell := ["bash", "-uc"]
# Recipes run in Bash on every OS: Git Bash on Windows.
set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

rustfmt_toolchain_default := trim(replace_regex(read(join(source_directory(), "rustfmt-toolchain.toml")), '(?s).*channel\s*=\s*"([^"]+)".*', "$1"))
rustfmt_toolchain_override := env("RUSTFMT_TOOLCHAIN", "")
rustfmt_toolchain := if rustfmt_toolchain_override == "" { rustfmt_toolchain_default } else { rustfmt_toolchain_override }
checkout_name := file_name(source_directory())
worktree_name := if file_name(parent_directory(repo_root)) == ".worktrees" { checkout_name } else { "main" }
cache_home := env("XDG_CACHE_HOME", join(home_directory(), ".cache"))
code_repos_root := env("CODE_REPOS_ROOT", if os_family() == "windows" { "D:/code" } else { join(home_directory(), "code") })
code_temp_root := env("CODE_TEMP_ROOT", if os_family() == "windows" { if path_exists("D:/temp") == "true" { "D:/temp" } else { "C:/temp" } } else { env("TMPDIR", "/tmp") })
code_packages_root := env("CODE_PACKAGES_ROOT", if os_family() == "windows" { "D:/packages" } else { join(home_directory(), ".local/share") })
code_tools_root := env("CODE_TOOLS_ROOT", if os_family() == "windows" { "C:/tools" } else { join(home_directory(), ".local/tools") })
code_target_root := env("CODE_TARGET_ROOT", if os_family() == "windows" { "D:/target" } else { join(cache_home, "cargo-target") })
code_build_root := env("CODE_BUILD_ROOT", if os_family() == "windows" { "C:/code_build" } else { join(cache_home, "cargo-build") })
code_sccache_root := env("CODE_SCCACHE_ROOT", env("SCCACHE_DIR", if os_family() == "windows" { "D:/sccache" } else { join(cache_home, "sccache") }))
repo_root := source_directory()
target_root := join(code_target_root, repo_cache_name)
build_root := join(code_build_root, repo_cache_name)
target_dir := join(code_target_root, repo_cache_name, tooling_slot)
build_dir := join(code_build_root, repo_cache_name, tooling_slot)
# Native Just paths; validate slots before exporting build directories.
tooling_slot := if worktree_name =~ '^(main|[a-j])$' { worktree_name } else { error("Invalid worktree slot: " + worktree_name) }
tooling_main := if file_name(parent_directory(repo_root)) == ".worktrees" { parent_directory(parent_directory(repo_root)) } else { repo_root }
sccache_basedirs := trim(shell('''
set -euo pipefail
root=$1
printf '%s' "$root"
git -C "$root" -c core.fsmonitor=false worktree list --porcelain | while IFS= read -r line; do
    case "$line" in
        "worktree $root/.worktrees/"*)
            slot=${line#"worktree $root/.worktrees/"}
            if [[ $slot =~ ^[a-j]$ ]]; then
                printf '%s%s/.worktrees/%s' "$2" "$root" "$slot"
            fi
            ;;
    esac
done
''', replace(tooling_main, "\\", "/"), if os() == "windows" { ";" } else { ":" }))

export CARGO_TARGET_DIR := target_dir
export CARGO_BUILD_TARGET_DIR := target_dir
host_build_volume := if os() == "windows" { trim(shell('pwsh.exe -NoLogo -NoProfile -File "$1" -BuildRoot "$2"', join(repo_root, "scripts/cargo-host.ps1"), code_build_root)) } else { "ok" }
export CARGO_BUILD_BUILD_DIR := if host_build_volume == "ok" { build_dir } else { error("Cargo build root must use NTFS") }
export RUSTC_WRAPPER := "sccache"
export CARGO_BUILD_RUSTC_WRAPPER := "sccache"
export CARGO_INCREMENTAL := "0"
export SCCACHE_DIR := code_sccache_root
export SCCACHE_CACHE_SIZE := "35G"
export SCCACHE_BASEDIRS := sccache_basedirs
export TEMP := if os() == "windows" { code_temp_root } else { env("TEMP", "/tmp") }
export TMP := if os() == "windows" { code_temp_root } else { env("TMP", "/tmp") }
export TMPDIR := if os() == "windows" { code_temp_root } else { env("TMPDIR", "/tmp") }


bin_dir := join(build_root, worktree_name, "artifacts")

set quiet

export CARGO_TERM_QUIET := env("CARGO_TERM_QUIET", "true")

# List the available recipes and task modules.
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
format:
  cargo +{{rustfmt_toolchain}} fmt --all
  echo "DONE"

# Format everything.
format-all: format

# Check Rust formatting with the pinned formatter without changing files.
format-check:
  cargo +{{rustfmt_toolchain}} fmt --all -- --check

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

# Scan all code for common mistakes (no changes)
lint-check:
  cargo clippy --all-targets --locked -- --deny warnings

# Lint with autofix: apply Clippy suggestions
lint:
  cargo clippy --all-targets --locked --fix --allow-dirty --allow-staged -- --deny warnings

# Lint with autofix, then lint Markdown.
lint-all: lint markdownlint

# Run all checks without modifying files: format, lint, check, test (deny is separate).
check-all: format-check lint-check check test

# Run the local CI gates: check-all plus dependency policy.
ci: check-all deny

# Check dependency advisories, licenses, duplicate versions, and registry sources.
deny:
  cargo deny --locked check

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

# Build the cargo doc documentation
doc: _builddoc

# Build and display the cargo doc documentation
doc-serve: doc
  cd "{{join(target_dir, "doc")}}" && (python3 -m http.server 9000 || python -m http.server 9000)

# Remove old API documentation and rebuild the project docs without dependency docs.
_builddoc:
  cargo clean --doc
  cargo doc --no-deps --locked # --document-private-items

# Display the user guide
serve:
  mdbook serve ./user_guide/ -p 3001

# Run all the steps required before pushing code to GitHub
prep: format-all lint check-all build _builddoc

# Build release binaries into the configured external artifact directory.
@release:
  echo "Build the tools in $(pwd) in release mode and copy to ../bin"
  cargo +{{rustfmt_toolchain}} build --bins --locked --all-features --release -Z unstable-options --artifact-dir "{{bin_dir}}"

## Utilities --------------------------------------

# Update Cargo.lock dependencies
[confirm]
update:
  cargo update

# Print the active worktree paths and compiler-cache configuration.
build-env:
    @printf 'WORKTREE_DIR=%s\n' {{quote(repo_root)}}
    @printf 'WORKTREE_SLOT=%s\n' {{quote(tooling_slot)}}
    @printf 'REPO_ROOT=%s\n' {{quote(tooling_main)}}
    @printf 'CODE_TARGET_ROOT=%s\n' {{quote(code_target_root)}}
    @printf 'CODE_BUILD_ROOT=%s\n' {{quote(code_build_root)}}
    @printf 'CODE_SCCACHE_ROOT=%s\n' {{quote(code_sccache_root)}}
    @printf 'CODE_TEMP_ROOT=%s\n' {{quote(code_temp_root)}}
    @printf 'BRANCH=%s\n' "$(git -C {{quote(repo_root)}} -c core.fsmonitor=false branch --show-current)"
    @if test -z "$(git -C {{quote(repo_root)}} -c core.fsmonitor=false status --porcelain)"; then echo STATUS=clean; else echo STATUS=dirty; fi
    @printf 'CARGO_TARGET_DIR=%s\n' {{quote(CARGO_TARGET_DIR)}}
    @printf 'CARGO_BUILD_TARGET_DIR=%s\n' {{quote(CARGO_BUILD_TARGET_DIR)}}
    @printf 'CARGO_BUILD_BUILD_DIR=%s\n' {{quote(CARGO_BUILD_BUILD_DIR)}}
    @printf 'RUSTC_WRAPPER=%s\n' {{quote(RUSTC_WRAPPER)}}
    @printf 'CARGO_BUILD_RUSTC_WRAPPER=%s\n' {{quote(CARGO_BUILD_RUSTC_WRAPPER)}}
    @printf 'CARGO_INCREMENTAL=%s\n' {{quote(CARGO_INCREMENTAL)}}
    @printf 'CARGO_BUILD_JOBS=%s\n' {{quote(env("CARGO_BUILD_JOBS", ""))}}
    @printf 'SCCACHE_DIR=%s\n' {{quote(SCCACHE_DIR)}}
    @printf 'SCCACHE_CACHE_SIZE=%s\n' {{quote(SCCACHE_CACHE_SIZE)}}
    @printf 'SCCACHE_BASEDIRS=%s\n' {{quote(SCCACHE_BASEDIRS)}}
    @printf 'TEMP=%s\n' {{quote(TEMP)}}
    @printf 'TMP=%s\n' {{quote(TMP)}}
    @printf 'TMPDIR=%s\n' {{quote(TMPDIR)}}

# Install the repository Git hook shim using prek.
hooks-install:
    prek install

# Run prek against the staged files.
hooks-run:
    prek run

export CODE_REPOS_ROOT := code_repos_root
export CODE_PACKAGES_ROOT := code_packages_root
export CODE_TOOLS_ROOT := code_tools_root
export CODE_TARGET_ROOT := code_target_root
export CODE_BUILD_ROOT := code_build_root
export CODE_TEMP_ROOT := code_temp_root
export CARGO_HOME := env("CARGO_HOME", if os() == "windows" { join(code_packages_root, "cargo") } else { join(home_directory(), ".cargo") })
export NUGET_PACKAGES := env("NUGET_PACKAGES", join(code_packages_root, "nuget"))
export npm_config_cache := env("npm_config_cache", join(code_packages_root, "npm"))

# Configure shared Cargo compiler defaults; checkout output slots remain separate.
configure-cargo-host:
    pwsh -NoLogo -NoProfile -File {{quote(join(repo_root, "scripts/cargo-host.ps1"))}} -BuildRoot {{quote(code_build_root)}} -ConfigureHost

repo_cache_name := if file_name(parent_directory(tooling_main)) == "rust_howto" { "rust_howto-mdbook-utils" } else { "mdbook-utils" }


# Link Cargo and rust-analyzer outputs without replacing existing directories.
build-setup:
    pwsh -NoLogo -NoProfile -File {{quote(join(source_directory(), "scripts/cargo-host.ps1"))}} -BuildRoot {{quote(code_build_root)}} -LinkOutputs

# Lint repository Markdown with the pinned markdownlint-cli2.
markdownlint *args='':
    npx --yes markdownlint-cli2@0.24.0 {{ args }}
