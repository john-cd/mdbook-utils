<!-- TODO Reviewed 2026-11-18. -->

# Task tracking

<!-- recommended-issue-order:start -->

## Ready and blocked issues

Snapshot: November 18, 2026, from live GitHub issues. No open unblocked issues remain.
Recheck the live issue before starting.

### Blocked

None.

<!-- recommended-issue-order:end -->

Open work is tracked in [GitHub Issues](https://github.com/john-cd/mdbook-utils/issues).
Keep one topic per issue, except epics, with a problem, task list, and acceptance criteria.
Record status and validation evidence in the issue rather than duplicating checklists here.

## Review evidence

At `6a41e01` (implementation `f840e39`), `just cargo test --all-targets --locked --offline`
passed 208 tests. `just cargo test --doc --locked --offline` passed with zero doctests, and
`just cargo clippy --all-targets --locked --offline -- -D warnings` passed. `just format-check`,
`dprint check`, and `git diff --check` passed. The #278 CLI regression compiles extracted Rust
with `rustc` and verifies mdBook 0.5.4 rendering for hidden lines and visible escaped hashes.

For #273, PR [#279](https://github.com/john-cd/mdbook-utils/pull/279) adds a hosted build of the
pinned `ci` image. Run [#44366470540](https://github.com/john-cd/mdbook-utils/actions/runs/44366470540)
passed Ubuntu, Windows, macOS, coverage, MSRV, and the hosted container build on November 18, 2026.
Local BuildKit checks also passed without warnings; actionlint, dprint, and `git diff --check` passed.
