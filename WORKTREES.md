# Worktrees

Updated 2026-11-01 (America/Los_Angeles). This file describes the managed layout;
use `git worktree list --porcelain` and `git status --short` for live state.

- The primary checkout stays on `develop` and uses cache slot `main`.
- `main` is an independent empty root commit, with no development history.
- Auxiliary checkouts must be direct children of `.worktrees/`, named with one
  lowercase letter: a through j only. Reuse clean slots and preserve their caches.
- Before moving a detached checkout, preserve its unique commits on a named
  branch and push that branch. Dirty work must be committed before switching.
- Squash related unpublished commits, rebase or fast-forward to synchronize
  `develop`, and never introduce synchronization merge commits.
- Keep backup branches until their unique commits are integrated or deliberately
  retired. Delete ordinary topic branches only after verifying they are merged.

## Storage roots

Override the `CODE_*_ROOT` environment variables before invoking tooling.
Windows defaults are `CODE_REPOS_ROOT=D:/code`,
`CODE_TEMP_ROOT=D:/temp` (`C:/temp` fallback),
`CODE_PACKAGES_ROOT=D:/packages`, and `CODE_TOOLS_ROOT=C:/tools`.
Unix uses native home/cache/temp directories. Root overrides do not move existing
installations or rewrite independent package-manager settings.
## Rust

Use `just build-env` to inspect the effective environment and
`just cargo <arguments>` to run Cargo. Windows targets use
`CODE_TARGET_ROOT/mdbook-utils/<slot>` (default root `D:/target`);
intermediates use `CODE_BUILD_ROOT/mdbook-utils/<slot>` (`C:/code_build`).
The intermediate volume must be NTFS; the runner rejects ReFS.
Unix defaults use XDG cache or `~/.cache` under `cargo-target` and
`cargo-build`. The runner enables sccache with incremental compilation disabled
and computes base directories from registered allowed worktrees.

Run `just configure-cargo-host` once per host and after changing roots.
It configures the host fallback in `CARGO_HOME/config.toml`. Run the repository's
`build-setup` recipe per checkout; local Cargo output settings take precedence and
rust-analyzer follows the same target/build links. See [README](README.md).

When this repository is checked out inside `rust_howto`, the runner uses
`rust_howto-mdbook-utils` as its cache namespace to keep its slots separate
from a standalone `mdbook-utils` checkout.

## Current state

The 2026-11-01 configuration alignment was integrated into develop; no dedicated alignment worktree remains. Use git worktree list --porcelain and git status --short for live ownership and cleanliness.