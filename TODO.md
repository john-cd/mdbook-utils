<!-- TODO Reviewed 2026-11-18. -->

# Task tracking

<!-- recommended-issue-order:start -->

## Ready and blocked issues

Snapshot: November 18, 2026, from live GitHub issues. No open unblocked issues remain.
Blocked issues are grouped by blocker; each group has one tracking issue labelled `tracking`.
Epics are listed where they wait on a blocker. Recheck the live issue before starting.

### Blocked

#### Container build validation — tracker [#273](https://github.com/john-cd/mdbook-utils/issues/273)

- Only the tracker itself (its body lists the deferred checks).

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

For the separate container-build criterion #273, [Rust CI](https://github.com/john-cd/mdbook-utils/actions/runs/44309557766)
and [guide deployment](https://github.com/john-cd/mdbook-utils/actions/runs/44309557764) passed
at `bac0ffb`; newer runs [44245484471](https://github.com/john-cd/mdbook-utils/actions/runs/44245484471)
and [44245484627](https://github.com/john-cd/mdbook-utils/actions/runs/44245484627) also passed.
The CI workflow now builds the pinned `ci` image as a separate job. Local BuildKit checks pass
without warnings; keep #273 open until the full hosted image build succeeds.
