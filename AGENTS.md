<!-- TODO Reviewed ??-??-2026 -->

# mdbook-utils instructions

## Working agreement and task tracking

- Read this file, the root `README.md`, `TODO.md` when present, the relevant `justfile` and Cargo manifests, and `git status` before editing. Follow the owning component's documentation and existing architecture.
- Keep updates concise and actionable, with links to relevant files. State the current issue, branch, and checkout when ownership changes.
- Preserve unrelated staged, unstaged, and untracked work. Keep edits and commits scoped to the requested task.
- Track implementation work in GitHub Issues. Link acceptance criteria to source and validation evidence; record blockers and concrete remaining work on the issue. Use priorities `P0` through `P4` where the tracker supports them.
- Reconcile actionable `TODO` and `LATER` entries with issues instead of maintaining duplicate backlogs. Do not delete unresolved comments or rename a `TODO` to a `NOTE` to hide unfinished work.
- Preserve human review comments and `TODO Reviewed` markers; those markers are reserved for human review.
- Close an issue only after its acceptance criteria, required checks, and focused commits are complete. A merged PR alone is insufficient evidence.

## Git workflow

- Keep the primary checkout on `develop`; use short-lived topic branches in clean, inactive worktree slots for implementation. Prefer `codex/issue-<number>-<description>` for issue work.
- Keep history linear. Rebase topic branches onto current `develop` before integration; use fast-forward integration or squash-merge PRs according to the requested workflow. Do not create merge commits to synchronize `develop`.
- Inspect local and remote history before integrating. Fast-forward when only behind; rebase unpublished local commits when histories diverge. Never rewrite shared history; use `--force-with-lease` only for an authorized update to an unshared rebased topic branch.
- Preserve the repository's existing independent `main` history. Do not push directly to `main`.
- Use focused commits with `type(scope): imperative summary` (`feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `build`, `ci`, `chore`, or `style`). Include related GitHub issue or PR links and short bullets describing the changes.
- Compare content and ancestry before retiring topic branches. Keep reusable worktree directories and their caches; do not discard unique work.

## Branches and worktrees

`develop` is the default integration branch and the primary checkout branch. `main` starts with an empty root commit and has independent history. Keep the existing `main` build-cache directory identity across branch renames to reuse compiled artifacts.

- Keep topic worktrees under the repository's `.worktrees/` directory in reusable slots `a`, `b`, `c`, and so on. Inspect `git worktree list`, the branch, status, and ownership of each slot before reuse; never reset, switch, move, or remove a dirty or active checkout.
- Keep `develop` in the primary checkout and use only topic branches in linked worktrees. Reuse clean inactive slots rather than creating task-named directories.
- Preserve stable `main` and slot cache identities across branch changes. Do not set one checkout's target/build paths globally, clean shared caches routinely, or duplicate a task to bypass a build lock.

## Build outputs

The committed `.cargo/config.toml` contains no machine-specific paths, so the repository builds on Windows, Linux, macOS and CI. On the maintainer's Windows machine, artifacts live in `D:\target\mdbook-utils\<worktree name>` and intermediate files in `C:\code_build\mdbook-utils\<worktree name>`.

The root `justfile` exports `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR` (Windows defaults above; `$XDG_CACHE_HOME/mdbook-utils/...` elsewhere; override with `MDBOOK_UTILS_TARGET_ROOT` and `MDBOOK_UTILS_BUILD_ROOT`). Set `MDBOOK_UTILS_WORKTREE_NAME` before running `just` in another worktree; the default is `main`. `just cargo <args>` runs any Cargo command with those same settings (for example `just cargo tree`). For direct `cargo` invocations, set the paths in your user-level `%CARGO_HOME%\config.toml` (or `~/.cargo/config.toml`) or via `CARGO_TARGET_DIR` / `CARGO_BUILD_BUILD_DIR`; without them Cargo uses `./target`. Do not put them back in the repository config.

`just` and CI use `sccache` 0.18 or newer as the Rust compiler wrapper (`RUSTC_WRAPPER`); plain `cargo` builds work without it. Local Windows builds default to `D:\sccache`; `just` defaults to `${XDG_CACHE_HOME:-$HOME/.cache}/sccache` on Linux. An existing `SCCACHE_DIR` environment variable takes precedence. CI and the dev container set their own cache locations.

## Build, lint, and test

Run commands from this repository root:

- `just fmt` formats the workspace with nightly rustfmt.
- `just check` checks all targets with the locked dependency graph.
- `just build` builds the Rust targets and both mdBooks.
- `just clippy` runs Clippy with warnings denied.
- `just test` runs Rust tests and both mdBook test suites.
- `just prep` runs the complete pre-push sequence, including `clean`. Use the individual format, build, lint, test, and documentation recipes for routine validation that preserves warm build artifacts; reserve `just prep` for a deliberately clean run.

The `user_guide\` and `test_book\` directories are independent mdBook projects and are built with the repository's `mdbook` commands.

## Rust design and code organization

- Prefer direct, readable control and data flow, one owner for each rule or state, and acyclic crate dependencies. Lower-level libraries must not depend on CLI composition roots.
- Keep `main.rs` thin and reusable behavior in libraries or focused modules. Avoid pass-through wrappers, speculative traits, stateless namespace structs, and abstractions without a demonstrated boundary.
- Name modules after their owning type, behavior, or domain; avoid generic names such as `types.rs`, `models.rs`, or `implementation.rs`. Keep one primary type or closely related group of functions per file.
- Use folder modules with `mod.rs` when modules have children. Keep `lib.rs` and `mod.rs` primarily declarative; put substantial implementations in focused child modules. Keep nested modules private and re-export only required items owned by the crate.
- Do not use `#[path = "..."]` to arrange modules. Update callers when moving or renaming code; remove superseded implementations and orphan source files in the same change.
- Order module documentation and attributes before declarations, then imports, constants/statics, types and implementations/functions, with inline tests last. Sort sibling module declarations alphabetically and let Rustfmt order imports.
- Prefer `use` imports over long qualified paths unless qualification prevents ambiguity. Import external items from their defining crate instead of re-exporting them.
- Use domain types for identities and values with invariants. Keep behavioral state private; expose constructors and narrow accessors or mutation methods. Passive configuration, event, result, and serialization records may have public fields.
- Load application configuration and environment variables at the entry point and pass typed configuration into reusable code.
- In actual Cargo workspaces, declare shared dependencies in `[workspace.dependencies]` and inherit them with `workspace = true`. Keep standalone package dependencies in their own manifest. Preserve declared Rust versions and feature choices unless the task requires a change.

## Errors, IO, and observability

- Propagate fallible operations with `Result` and `?`, adding actionable context. Do not swallow errors, ignore failed writes, or substitute default values for failed computations.
- Do not use `unwrap()` in maintained Rust code, including tests and benchmarks, or `panic!` in production code. Use `expect()` only for a documented invariant, or with a descriptive message in tests and benchmarks.
- Prefer specific error types with `thiserror` in libraries and `anyhow` at CLI/application boundaries. Preserve the owning crate's established public error contracts when making a focused change.
- Use `tracing` for diagnostics and instrument meaningful orchestration boundaries; avoid unconditional spans or allocation in tight loops.
- Write replacements through temporary sibling files and atomic rename where supported. Preserve source data and report partial failures accurately.
- Prefer existing maintained crates and measured improvements. Preallocate known sizes and reuse hot-loop buffers when useful; do not add speculative performance machinery.

## Documentation, tests, and formatting

- Document every module and public API, including fields and variants. Explain ownership, invariants, side effects, and concrete error conditions; document non-obvious private helpers and algorithms.
- Add a specific doc comment to every function and method, including private helpers and tests, except standard trait implementations whose contracts are already defined by the trait.
- Keep Rustdoc specific to the item. Put doc comments before attributes; avoid generic placeholder text and document fallible APIs with meaningful `# Errors` sections.
- Add focused behavior tests for new code and bug fixes, integration tests for CLI/file effects, and benchmarks when performance-sensitive behavior warrants them.
- Keep unit tests near their code in an inline `#[cfg(test)] mod tests` at the bottom of the file. Do not wrap integration-test files in `#[cfg(test)]`.
- Give tests descriptive names and a concise doc comment stating the behavior they verify. Construct concrete fixtures and meaningful identifiers; review snapshots before accepting changes.
- Keep README files, user guides, command examples, and configuration documentation synchronized with changed behavior.
- Use American spelling, LF line endings, and CommonMark. Leave blank lines after headings and specify a language on fenced code blocks.
- Prefer ASCII in tooling code and agent instructions; preserve intentional Unicode in book content, examples, and fixtures.
- Format Rust with the repository's nightly Rustfmt recipe. Use `dprint` for Markdown/JSON when configured, and sort changed Cargo manifests with `cargo sort -w`.
- Fix lint causes instead of hiding them. Use `#[expect(lint, reason = "...")]` for intentional, documented exceptions rather than unexplained suppression.
- Prefer Rust for new permanent processing tools. Retain established shell entrypoints and platform/build setup scripts; do not introduce permanent Python or Bash programs for application logic.
- Prefer `cargo binstall` for supported development tools and pin CI tool versions.

## Validation and resource use

- Select checks by impact: documentation-only changes need applicable Markdown/link checks; Rust behavior changes need focused crate and behavior tests; dependency, feature, or shared-interface changes need affected consumers and configurations.
- Use the owning repository's `just` recipes. Do not assume another repository's `cargo make`, UI checks, or release gates exist here.
- Use focused checks in the inner loop; run the relevant complete gates before integration or delivery. Include doctests, examples, and required feature variants when they cover affected behavior.
- Do not run `cargo check` immediately before equivalent Clippy checks. Reuse passing evidence while the relevant inputs and configuration remain unchanged.
- Record commands, configurations, results, and remaining failures. Use read-only validation; apply fixes deliberately, inspect their diff, and preserve unrelated work.
- Run builds for a checkout sequentially. Independent checkouts may build concurrently when CPU, memory, and disk capacity permit; avoid changing inputs during a build.
- Inspect running Cargo/rustc processes before starting expensive builds or cleaning artifacts. Do not stop another task's processes or restart the shared sccache server while builds are active.
- Use `D:\temp` for Windows temporary files where an override is needed. Keep temporary files separate from persistent target, intermediate-build, Cargo, and sccache caches.
- Finish with `git diff --check`. Report unavailable checks and pre-existing failures without attributing them to the current change.
