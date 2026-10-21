# mdbook-utils instructions

## Branches

`develop` is the default integration branch and the primary checkout branch. `main` starts with an empty root commit and has independent history. Keep the existing `main` build-cache directory identity across branch renames to reuse compiled artifacts.

## Build outputs

Cargo target artifacts are stored outside the source checkout in `D:\target\mdbook-utils\<worktree name>`. Cargo intermediate build files are stored in `C:\code_build\mdbook-utils\<worktree name>`.

The root `justfile` exports `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR`. Set `MDBOOK_UTILS_WORKTREE_NAME` before running `just` in another worktree; the default is `main`. Direct Cargo invocations use the `main` paths from `.cargo\config.toml` unless `CARGO_TARGET_DIR` or the corresponding Cargo build environment variable overrides them.

## Build, lint, and test

Run commands from this repository root:

- `just fmt` formats the workspace with nightly rustfmt.
- `just check` checks all targets with the locked dependency graph.
- `just build` builds the Rust targets and both mdBooks.
- `just clippy` runs Clippy with warnings denied.
- `just test` runs Rust tests and both mdBook test suites.
- `just prep` runs the complete pre-push sequence.

The `user_guide\` and `test_book\` directories are independent mdBook projects and are built with the repository's `mdbook` commands.
