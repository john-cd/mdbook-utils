# Worktrees

Updated 2026-11-03 (America/Los_Angeles). This file describes the managed layout;
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
intermediates use `CODE_BUILD_ROOT/mdbook-utils/<slot>` (`D:/code_build`).
The intermediate volume may use NTFS or ReFS with Rust 1.100.0 or newer.
Unix defaults use XDG cache or `~/.cache` under `cargo-target` and
`cargo-build`. The runner enables sccache with incremental compilation disabled
and computes base directories from registered allowed worktrees.

Run `just configure-cargo-host` once per host and after changing roots. It keeps shared compiler defaults in `CARGO_HOME/config.toml` and removes global target/build output overrides. Run build-setup once per checkout. Task-runner Cargo commands pass absolute `D:/target/<repo>/<slot>` and `D:/code_build/<repo>/<slot>` paths; use the checkout links or explicit Cargo environment paths for direct Cargo and rust-analyzer as required by the host filesystem. See [README](README.md).

When this repository is checked out inside `rust_howto`, the runner uses
`rust_howto-mdbook-utils` as its cache namespace to keep its slots separate
from a standalone `mdbook-utils` checkout.

## Current state

Reviewed 2026-11-01.
TODO Reviewed ??-??-2026.

Refresh this snapshot from live Git state before moving any checkout. This
inventory update is on an unpublished local issue branch.

| Checkout        | Branch or revision                                      | Status                                                                    |
| --------------- | ------------------------------------------------------- | ------------------------------------------------------------------------- |
| Repository root | `develop` at `e2914ad`                                  | Clean; matches `origin/develop`.                                          |
| `.worktrees/a`  | `codex/issue-268-269-ci-cache-fixtures` at current HEAD | Clean; local issue commits are unpublished and pending final integration. |
| `.worktrees/b`  | detached at `bc98697`                                   | Clean; retains patch-unique `1a0bec1`. Keep this checkout and cache.      |
| `.worktrees/c`  | detached at `f833761`                                   | Clean; retains patch-unique `8926394`. Keep this checkout and cache.      |

Other unmerged refs retained for review:

- Local `codex/cargo-update` points to patch-unique `4a975c2` (Cargo lockfile).
- Remote `codex/wsl-build-policy-f6bf7fd` points to patch-unique `f6bf7fd` (build/cache policy).
- The local and remote `backup/tooling-20261031/*` refs remain archival snapshots.
  `develop-before` and `working-state` retain unique patch ids `9b413c3`,
  `934a522`, `a2c3f28`, `d36909d`, and `0ce88b2`; the worktree snapshots retain
  `8926394`. The nested remote backups retain `2fd3a04`, `e3c148a`, `86b4ba0`,
  `ea8a3ee`, `9731954`, and `8c3fc8f`.
- `backup/develop-before-sync-20261031-5d9eaa67` has no patch-unique commits
  compared with `develop`; retain it as a recovery snapshot.

`main` remains the independent empty root history. No checkout, branch, or
build cache was removed or repurposed. Detached worktrees `b` and `c` retain
their unique commits in place. Refresh this inventory from
`git worktree list --porcelain`, per-checkout `git status --short`, and
`git cherry develop <ref>` before any checkout or branch retirement.
