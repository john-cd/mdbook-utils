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

### Ready: priority order

- [#265: Epic: Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/mdbook-utils/issues/265) — Updater/action/cache work independent of owner decisions.
- [#264: Epic: Validate and finalize Cargo profiles, output paths, and linker policy](https://github.com/john-cd/mdbook-utils/issues/264) — Local profile/path measurements; external hosts still needed for platform gates.
- [#259: Epic: optional library, CLI, and coverage follow-ups](https://github.com/john-cd/mdbook-utils/issues/259) — Optional follow-ups after CI defects; choose one focused child at a time.

### Blocked or deferred

None currently open.

<!-- recommended-issue-order:end -->

Open work is tracked in [GitHub Issues](https://github.com/john-cd/mdbook-utils/issues).
Keep one topic per issue, except epics, with a problem, task list, and acceptance criteria.
Record status and validation evidence in the issue rather than duplicating checklists here.

## Blocker tracking

None currently open.

## Maintenance

- [Validate and finalize Cargo profiles, output paths, and linker policy](https://github.com/john-cd/mdbook-utils/issues/264).
- [Complete dependency-update coverage and consistent CI caching](https://github.com/john-cd/mdbook-utils/issues/265).
