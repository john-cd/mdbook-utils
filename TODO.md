<!-- TODO Reviewed ??-??-2026. -->

# Task tracking

<!-- recommended-issue-order:start -->

## Ready and blocked issues

Snapshot: November 17, 2026, from live GitHub issues. Ready issues can start now, in priority order.
Blocked issues are grouped by blocker; each group has one tracking issue labelled `tracking`.
Epics are listed where they wait on a blocker. Recheck the live issue before starting.

### Ready

- **P2 — [#278](https://github.com/john-cd/mdbook-utils/issues/278):** Match mdBook hidden-line
  semantics during Rust extraction. A bare `#` survives extraction and makes the generated
  Rust fail to compile. The issue records the reproduction and regression criteria.

### Blocked

#### Container build validation — tracker [#273](https://github.com/john-cd/mdbook-utils/issues/273)

- Only the tracker itself (its body lists the deferred checks).

<!-- recommended-issue-order:end -->

Open work is tracked in [GitHub Issues](https://github.com/john-cd/mdbook-utils/issues).
Keep one topic per issue, except epics, with a problem, task list, and acceptance criteria.
Record status and validation evidence in the issue rather than duplicating checklists here.

## Review evidence

At `a6e36f9`, `just cargo test --workspace --locked --offline` passed 206 tests, including
doctests. `just cargo clippy --workspace --all-targets --locked --offline -- -D warnings`
also passed. The extraction edge case in #278 is outside the existing tests.

[Rust CI](https://github.com/john-cd/mdbook-utils/actions/runs/44245484471) and
[guide deployment](https://github.com/john-cd/mdbook-utils/actions/runs/44245484627) passed
at that revision. Neither establishes the container-build criterion in #273. No container
build was run during this review.
