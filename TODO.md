<!-- TODO Reviewed ??-??-2026 -->

# Task tracking

<!-- recommended-issue-order:start -->

## Recommended issue order

Snapshot: November 3, 2026. Read each list from top to bottom: quick wins first,
then the remaining ready work in priority/dependency order. Quick wins are bounded
candidates, not time estimates. Recheck the live issue and current source before
starting; this index does not mark acceptance complete. Existing notes below are
context and may reference issues that have since closed.

Ready means a concrete engineering slice can start without a new owner decision or
external account/host. Where an issue mixes local work and external acceptance, only
the named local slice is ready. Blocked entries state what releases the block;
fixture preparation and investigation can continue independently.

### Ready: quick wins

- [#268: P1: Attach rust-cache inputs to rust-cache instead of install-action](https://github.com/john-cd/mdbook-utils/issues/268) — P1 move cache inputs to the correct action.
- [#269: P1: Restore byte-exact generated-index fixtures after review-marker insertion](https://github.com/john-cd/mdbook-utils/issues/269) — P1 restore exact fixture expectations while preserving human markers.

### Ready: priority order

- [#263: Epic: Finish shared formatting, lint, and hook policy validation](https://github.com/john-cd/mdbook-utils/issues/263) — Local formatting/lint baseline.
- [#265: Epic: Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/mdbook-utils/issues/265) — Updater/action/cache work independent of owner decisions.
- [#264: Epic: Validate and finalize Cargo profiles, output paths, and linker policy](https://github.com/john-cd/mdbook-utils/issues/264) — Local profile/path measurements; external hosts still needed for platform gates.
- [#197: Finish external-link conversion, labels, and output formats](https://github.com/john-cd/mdbook-utils/issues/197) — Finish local conversion behavior without moving features between repositories.
- [#259: Epic: optional library, CLI, and coverage follow-ups](https://github.com/john-cd/mdbook-utils/issues/259) — Optional follow-ups after CI defects; choose one focused child at a time.
- [#266: Review and reconcile repository branches and worktrees](https://github.com/john-cd/mdbook-utils/issues/266) — Inspect current candidate history and ownership before cleanup.

### Blocked or deferred

- [#270: P2: Verify the corrected Rust CI matrix on the current develop revision](https://github.com/john-cd/mdbook-utils/issues/270) — Needs #268/#269 plus a fresh hosted Linux/Windows/macOS run.
- [#271: P2: Validate the publish workflow with both publishing inputs disabled](https://github.com/john-cd/mdbook-utils/issues/271) — Needs corrected workflows and dispatch access; both publishing inputs must remain false.
- [#223: mdbook-utils / book_tooling: Decide which tool owns each overlapping feature](https://github.com/john-cd/mdbook-utils/issues/223) — Maintainer decision on overlapping feature ownership; coordinate book_tooling#67.
- [#267: Epic: human decisions, reviews, and host-dependent validation](https://github.com/john-cd/mdbook-utils/issues/267) — Tracks owner decisions and hosted acceptance, not independent code fixes.

<!-- recommended-issue-order:end -->

Open work is tracked in [GitHub Issues](https://github.com/john-cd/mdbook-utils/issues).
Keep one topic per issue, except epics, with a problem, task list, and acceptance criteria.
Record status and validation evidence in the issue rather than duplicating checklists here.

## Blocker tracking

- [Human decisions, reviews, and host prerequisites](https://github.com/john-cd/mdbook-utils/issues/267).

## Maintenance

- [Finish shared formatting, lint, and hook policy validation](https://github.com/john-cd/mdbook-utils/issues/263).
- [Validate and finalize Cargo profiles, output paths, and linker policy](https://github.com/john-cd/mdbook-utils/issues/264).
- [Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/mdbook-utils/issues/265).
- [Review and reconcile repository branches and worktrees](https://github.com/john-cd/mdbook-utils/issues/266).
