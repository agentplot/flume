# Verify: a-conflicting-base-place-gets-its-fix-launched

Verified on `change/a-conflicting-base-place-gets-its-fix-launched` at
b7392d0f, cut from `bolt/flume-launches-a-conflicting-bolts-fix`. The
branch holds origin's `bolt/flume-launches-a-conflicting-bolts-fix` at
c3e1af20.

## Outcome

**The whole checks pass and the work holds: archived.**

- `sluice exec main` passed 23 of 23 tasks.
- All 22 tasks, 1.1 to 7.1 with 2.5, are ticked done.
- Every scenario of the six delta specs is held by a scenario of the
  suite or a test.
- A trial archive on a copy of `openspec/` applies +19, ~3, -0 and →0.
  After it, `openspec validate --specs --strict` passes 46 of 46.

## The whole checks

Run once, `sluice exec main` at b7392d0f. It runs `#check-fast` and
`#check-tools`, as `.config/sluice.toml`'s `[branches.main]` requires.
All 23 tasks passed in 6m 40s, and sluice recorded the 23 results under
their keys for b7392d0. 24 tasks completed with `site:install`.

- **Tests:**
  - `flume:tools` (check-tools): 133 of 133, the walkthroughs and the
    pinned sluice's profile tests among them,
    `a_base_places_branch_forecast_over_a_scratch_repository` with
    them.
  - `flume:test`: 526.
  - `seams:test`: 103, the round trips among them.
  - `fakes:test`: 65.
  - `core:test`: 45.
  - `plugin-claude:test`: 10.
  - `conformance:test`: 25, over every binary. `engine` runs the
    engine's scenarios. `no_tool_is_named_outside_a_profile` passes:
    task 2.5 took `sluice` out of the forecast's comments in
    `crates/flume-seams/src/merge_queue.rs` (core.1). `seams` holds
    `the_fake_merge_queue_passes_the_base_place_scenarios`, with "Clear,
    then in conflict" among them, and `suite` passes too.
- **Format and lint:** `seams`, `core`, `fakes`, `conformance` and
  `flume` each passed `format` and `lint`.
- **The rest:** `conformance:coverage`, `root:adrs`, `root:changes`,
  `root:format`, `site:prose` and `site:build` passed.

## The work against the change

Read against the proposal, the design and the six delta specs.

### Tasks

Every task in `tasks.md` is ticked, 22 of 22, and none is open.

### Specs

- **base-places.** Each scenario is in
  `crates/flume-conformance/src/engine/base_places.rs`, which the
  engine's scenarios ran:
  - The fix kind at launch: `a_base_place_whose_kind_names_mend`,
    `a_declaration_changed_after_the_launch`,
    `a_fix_kind_that_resolves_to_nothing` and
    `a_fix_kind_with_no_stages`.
  - When a settle forecasts: `no_forecast_while_a_change_works`,
    `no_forecast_while_submitted` and
    `no_forecast_for_a_kind_naming_no_fix`.
  - What a forecast holds: `forecast_while_it_waits_on_its_caller` and
    `a_forecast_that_could_not_be_read`.
  - What launches a fix: `a_failed_submission_launches_mend`, whose
    failure goes once to `ops` and opens no agent in the base place;
    `a_conflicting_submission_launches_mend`; and
    `a_conflict_with_an_entry_queued_ahead`.
  - The fix as its base place's: `a_forecast_conflict_launches_mend`,
    in `east` at priority 4 with recipients from its base place, and
    S120's engine half,
    `s120_a_base_place_forecast_in_conflict_launches_its_fix`.
  - The brief: `the_brief_of_a_forecast_conflicts_fix`.
  - One at a time, once each: `the_fix_lands_and_the_forecast_is_clear`,
    `two_settles_at_once` and
    `a_launch_that_died_between_its_writing_and_its_making`.
  - A drop: `a_fix_dropped`, whose second fix waits until main moves.
  - The bound: `the_bound_spent`, whose stall goes once to `ops` after
    six minutes; `the_bound_left_out`; and `a_refused_launch`.
  - The modified requirement: `s112_a_failed_base_place_takes_a_fix`
    and `a_conflicting_submission`, over a `train` naming no fix.
- **place-status.** The JSON scenarios are in
  `crates/flume-conformance/src/engine/place_status.rs`:
  `a_base_place_whose_first_fix_is_live`,
  `the_fix_names_its_base_place`, and the fields left out of a change
  and of a base place naming no fix. Their fields are listed in
  `answers.rs`'s `FIELDS`. "A fix landed, in the person's form" is
  `commands::print::tests::a_fix_landed_in_the_persons_form`, its JSON
  half included.
- **seam-contracts.** The round trips are in
  `crates/flume-seams/src/round_trip.rs`:
  `a_forecast_clear_in_conflict_or_unread`,
  `a_kind_that_names_its_fix`, and a held state with fixes. A held
  state written before this change reads with no fix kind, forecast or
  fixes. "Clear, then in conflict" is `base_place_scenarios` in
  `crates/flume-conformance/src/seams/merge_queue.rs`. It passed against
  the fake in `conformance:test` and against sluice in `flume:tools`.
- **seam-fakes.** `the_fake_merge_queue_scripts_a_forecast` passed in
  `fakes:test`.
- **merge-queue-profile.** The sluice profile reads four kinds of
  answer: `a_clear_forecast`, `a_conflicts_words`,
  `a_conflict_with_entries_ahead` and `a_forecast_refused`, all in
  `crates/flume/src/profiles/merge_queue/sluice/mod.rs`. They passed in
  `flume:test`.
  `a_base_places_branch_forecast_over_a_scratch_repository` passed
  against the pinned sluice in `flume:tools`.
- **clause-coverage.** `clauses.rs` reads agentplot/blueprints at
  `REVISION` 31f9f1c, which holds 210 in Part B and S120, and no 209.
  `RENAME` is out of the unbuilt list. `conformance:coverage` passed, so
  this change's scenarios cite 210 and S120.

### The specs as archived

The trial archive ran on a copy of `openspec/` with the devenv's
openspec 1.14.3. It writes +12 and ~1 into `base-places` and ~2 into
`clause-coverage`. It writes +2 each into `merge-queue-profile`,
`place-status` and `seam-contracts`, and +1 into `seam-fakes`. Afterwards
`openspec validate --specs --strict` passes 46 of 46. `root:specs`
runs only once the archive changes `openspec/specs/`. It checks the
archive's commit when the branch is next gated.

### Left to the named follow-ups

As the proposal and design name them, and as task 7.1 holds:

- weir's unit a-kind-with-no-stages-names-its-fix;
- then flume's a-base-places-fix-kind-is-read-from-weirs-format, which
  maps weir's two fields in the kinds profile.

Until they land, a kind in weir's format names no fix, and its base
place waits on its caller as before. The merge-queue guide says so.
