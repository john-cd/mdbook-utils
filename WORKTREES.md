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

Reviewed 2026-11-01.
TODO Reviewed ??-??-2026.

Refresh this snapshot from live Git state before moving any
checkout. The inventory itself is edited on the active local issue branch and is
not yet integrated or published.

| Checkout        | Branch or revision                                   | Status                                                                              |
| --------------- | ---------------------------------------------------- | ----------------------------------------------------------------------------------- |
| Repository root | `develop` at `e2914ad`                               | Clean; matches `origin/develop`.                                                    |
| `.worktrees/a`  | `codex/issue-268-269-ci-cache-fixtures` at `c6698c1` | Clean; four local issue commits, not published.                                     |
| `.worktrees/b`  | detached at `bc98697`                                | Clean; retains unique `1a0bec1` and `bc98697` commits. Do not move until preserved. |
| `.worktrees/c`  | detached at `f833761`                                | Clean; retains unique `f833761` commit. Do not move until preserved.                |

Update (2026-11-03): `.worktrees/a` has advanced to `01bf403` and contains
uncommitted #259 RuleKind work; the table's `.worktrees/a` row is superseded.
The other checkout rows remain current.

The remote `codex/wsl-build-policy-f6bf7fd` branch at `f6bf7fd` remains
unmerged and patch-unique; retain it for review. Historical backup refs are
preserved. No checkout or branch was removed during this inventory. Use
`git worktree list --porcelain`, `git status --short`, and `git log` to refresh
this snapshot before making changes.
