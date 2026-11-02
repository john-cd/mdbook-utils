<!-- TODO Reviewed ??-??-2026 -->

# mdbook-utils instructions

## Working agreement and task tracking

- Keep answers concise, use American spelling, use Unix-style line endings.
- Read this file, the root `README.md`, `TODO.md` when present, the relevant `justfile` and Cargo manifests, and `git status` before editing. Follow the owning component's documentation and existing architecture.
- Keep updates concise and actionable, with links to relevant files. Whenever you change the active branch or worktree, immediately state the current branch name and worktree path in your next update. Include the current issue when ownership changes.
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

Run ad hoc Cargo commands through the root justfile as just cargo <subcommand> [args...]. Every checkout selects its own central output slot automatically. Cargo-make tasks also receive centralized paths where configured.

On Windows, target artifacts go to CODE_TARGET_ROOT/mdbook-utils/<slot> and intermediate build artifacts go to CODE_BUILD_ROOT/mdbook-utils/slot. On Linux/macOS, use the XDG cache folder (or ~/.cache) under cargo-target/mdbook-utils/slot and cargo-build/mdbook-utils/slot.

These repositories use sccache with incremental compilation disabled. The shared cache is limited to 35G. The Just runner sets `SCCACHE_BASEDIRS` to the main checkout and registered allowed `.worktrees/<letter>` paths only, discarding inherited paths. Use its Cargo command to select external per-worktree build paths.

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
- Use `CODE_TEMP_ROOT` for Windows temporary files where an override is needed. Keep temporary files separate from persistent target, intermediate-build, Cargo, and sccache caches.
- Finish with `git diff --check`. Report unavailable checks and pre-existing failures without attributing them to the current change.
## Central build outputs and worktrees

- Keep auxiliary Git worktrees directly under `.worktrees/` in slots `a`, `b`, or `c` only; keep the primary checkout separate.
- Keep the main worktree on `develop`. Keep topic branches in auxiliary worktrees.
- Windows: target outputs go to CODE_TARGET_ROOT/mdbook-utils/<slot> and intermediate build outputs go to CODE_BUILD_ROOT/mdbook-utils/slot. Keep CODE_BUILD_ROOT on an NTFS volume; do not use a ReFS volume for builds.
- Linux/macOS: target outputs go to the XDG cache folder (or ~/.cache) under cargo-target/mdbook-utils/<slot>; intermediate outputs go under cargo-build/mdbook-utils/<slot>.
- Run ad hoc commands as just cargo <subcommand> [args...]; existing cargo-make tasks also receive the configured paths.
- sccache is enabled with incremental compilation disabled. Cache size is 35G; the Windows cache directory is CODE_SCCACHE_ROOT and the Linux/macOS cache directory is under the user's XDG cache folder.
- The Just runner sets `SCCACHE_BASEDIRS` to the main checkout and registered allowed letter worktrees only. Large-build repositories allow `main`, `a`, and `b`; inherited paths are discarded. Use its Cargo command to select external per-worktree build paths.

## Machine storage roots

- On Windows, use `CODE_TEMP_ROOT` for temporary and staging files, setting `TEMP`, `TMP`, and `TMPDIR` as needed. Use `C:\temp` only when `CODE_TEMP_ROOT` is unavailable.
- Keep package-manager data, library caches, and package installations under `CODE_PACKAGES_ROOT`, including Cargo, npm, NuGet, pip/uv, vcpkg, and similar data. Do not store standalone tool distributions there.
- Keep portable developer tools and SDKs under `CODE_TOOLS_ROOT`, in a named subdirectory per tool (for example NASM, Oracle Instant Client, Cap'n Proto, Flutter, Pandoc, Tectonic, and repository tool bundles). Update PATH and installer/config references when relocating a tool. Leave package-manager-managed tools in their managed roots unless replacing them with a portable installation.
- In WSL, keep temporary files on the distribution's configured temp paths. Keep package data in `CODE_PACKAGES_ROOT` (default `~/.local/share`) and portable tools in `CODE_TOOLS_ROOT` (default `~/.local/tools`); avoid Windows drive mounts for these roots unless a tool requires them.

## Portable storage-root configuration

The current Windows defaults below can be overridden per machine or shell with environment variables. The per-repository Rust Justfile or Cargo Make helper, when present, reads these variables. Non-Rust repositories do not need a Cargo build runner; use the environment roots only for tools or scripts that consume them.

- CODE_REPOS_ROOT: D:/code on Windows; ~/code by default on Unix.
- CODE_BUILD_ROOT: C:/code_build on Windows; XDG cache or ~/.cache/cargo-build by default on Unix. Keep Windows build output on NTFS.
- CODE_TARGET_ROOT: D:/target on Windows; XDG cache or ~/.cache/cargo-target by default on Unix.
- CODE_SCCACHE_ROOT: D:/sccache on Windows; XDG cache or ~/.cache/sccache by default on Unix.
- CODE_TEMP_ROOT: D:/temp on Windows, with C:/temp as a fallback where a script needs one; Unix uses the existing TMPDIR or /tmp.
- CODE_PACKAGES_ROOT: D:/packages on Windows; ~/.local/share by default on Unix.
- CODE_TOOLS_ROOT: C:/tools on Windows; ~/.local/tools by default on Unix.

Windows drive paths elsewhere in this file are defaults. Set a root variable to relocate the corresponding directory. WSL must keep its distribution temp behavior and must not redirect temp variables to a Windows drive mount.

## Git Hooks

- Use prek with this repository's prek.toml; install Git hook shims with hooks-install (or prek install directly).
- Use `prek` with the repository's `prek.toml` for Git hooks.

## Required repository storage and Git policy

- Keep the primary checkout on `develop`. Keep `main` as an independent empty root commit; do not merge development history into it.
- Auxiliary worktrees must be direct children of `.worktrees/` and use slots `a`, `b`, or `c` only. Never use task-named or external worktree paths.
- Preserve unique commits on named branches before moving a detached checkout. Keep history linear, squash unpublished related changes, and never rewrite published history without explicit authorization.
- Use `CODE_TEMP_ROOT` for Windows temp files: `D:/temp` when available, otherwise `C:/temp`. Unix uses its native `TMPDIR` or `/tmp`.
- Use `CODE_PACKAGES_ROOT` for package data (Windows `D:/packages`) and `CODE_TOOLS_ROOT` for portable tools (Windows `C:/tools`). Existing tool-specific environment overrides take precedence; changing a root does not move installed packages.
- Run Cargo through `just cargo`. The runner must export central per-repo/per-slot target and build paths, sccache with incremental disabled, and only registered allowed sccache base directories.
- Windows intermediate builds must use NTFS. The runner validates `CODE_BUILD_ROOT` before Cargo starts; a ReFS override is an error. Defaults are `C:/code_build/<repo>/<slot>` and `D:/target/<repo>/<slot>`.
- Run `just configure-cargo-host` once per host and after changing roots to configure direct Cargo's central fallback. Run the build-setup recipe per checkout. Direct Cargo and rust-analyzer then use that checkout's target/build links; the host config remains a fallback for repositories without local output settings.

## Configuration consistency

Follow [CONFIGURATION.md](CONFIGURATION.md) when changing repository tooling. Use American
spelling and LF line endings; keep README, contributor instructions, and configuration comments
synchronized. Preserve documented platform, domain, and teaching-example exceptions.

## Required worktree workflow

- Keep the primary checkout (`main` worktree) on `develop`. Do task work in the primary checkout or in direct
  children of `.worktrees/` named `a`, `b`, or `c`. Never use task-named directories, other slots, or
  external locations. Reuse a clean, inactive permitted slot whenever possible.
- Before reusing a slot, inspect `git worktree list`, branch, status, ownership, and active build
  processes. Never reset, overwrite, move, or remove a dirty or active worktree. If no permitted slot is
  available, stop and ask; do not create another slot or custom path.

- For each issue or task, implement in its worktree, lint the changes, format the changed files, and
  make a focused commit. Keep the worktree clean; make a WIP commit if needed. Wait until the task is
  complete to integrate into `develop` linearly. Then lint and format the whole repository and commit
  integration fixes. Do not leave dirty worktrees.
- Detach a clean, unused worktree and retain its slot and build directory. Delete topic branches only
  after confirming they are fully merged; never delete `develop`.
- Always keep the primary checkout's build directory. Preserve build directories for slots `a`, `b`, and
  `c` when disk space allows. When disk space is low, inspect active worktrees and build processes; remove
  the oldest obsolete build directories first. Then clean task-owned temporary directories and obsolete
  `build`/`target` directories. Do not remove directories used by active checkouts or processes.
- `cargo-deny` findings are not a blocker gate. Fix them, add a justified exception, or create a follow-up
  issue or task and record the outcome. Other OS or Docker validation is not a blocker gate either;
  create a follow-up issue or task when that validation is unavailable.
