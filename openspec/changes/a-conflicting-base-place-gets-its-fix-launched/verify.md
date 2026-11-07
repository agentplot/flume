# Verify: a-conflicting-base-place-gets-its-fix-launched

Verified on `change/a-conflicting-base-place-gets-its-fix-launched` at
7fc1a197, cut from `bolt/flume-launches-a-conflicting-bolts-fix`.

## Outcome

**The whole checks fail. Nothing is archived.** The change goes back to
its code stage with the failure below.

## The whole checks

Run once, `sluice exec main`, which runs `#check-fast` and
`#check-tools` (`.config/sluice.toml`, `[branches.main]`): 23 tasks
passed and 1 failed, in 6m 55s.

- **Failed: `conformance:test`** (check-fast). The test
  `no_tool_is_named_outside_a_profile` (`crates/flume-conformance/tests/no_tool.rs`)
  fails on core.1, since a tool is named outside a profile:

  ```
  a tool is named outside a profile (core.1):
  crates/flume-seams/src/merge_queue.rs:241: sluice
  crates/flume-seams/src/merge_queue.rs:330: sluice
  ```

  Both are the citation `sluice 64, 65` in the doc comments of
  `Forecast` and `MergeQueue::forecast`, added in e6c97f51
  (`feat(seams): the merge queue seam's forecast…`). core.1 rules out
  the string `sluice` in any module outside a profile directory, and
  `flume-seams` is no profile. The fix drops the tool's name from the two
  comments, citing only flume's clauses (203, 210), or citing sluice's
  clauses only inside the sluice profile, where the profile already
  cites them.

  Because `cargo test -p flume-conformance` stops at the first failing
  test binary, the binaries after `no_tool` did not run in this gate:
  `tests/seams.rs`, which holds the seam scenarios (task 2.4's "Clear,
  then in conflict" on the fake among them), and `tests/suite.rs`. Their
  outcome is unknown until the gate runs again. `tests/engine.rs`, the
  engine's scenarios, ran before it and passed, and so did the library's
  own tests.

- **Passed:** `seams:test` (103), `seams:format`, `seams:lint`,
  `core:test`, `core:format`, `core:lint`, `fakes:test`, `fakes:format`,
  `fakes:lint`, `conformance:coverage`, `conformance:format`,
  `conformance:lint`, `flume:test`, `flume:format`, `flume:lint`,
  `flume:tools` (check-tools, 133 passed, the walkthroughs and the
  pinned sluice's profile tests, `merge_queue_sluice` among them),
  `plugin-claude:test`, `site:prose`, `site:install`, `site:build`,
  `root:adrs`, `root:changes`, `root:format`.

## The work against the change

Read against the proposal, the design and the six delta specs. Apart
from the failure above, the work holds.

### Tasks

All 21 tasks, 1.1 to 7.1, are ticked done.

### Specs

Each scenario of each delta spec has a scenario or test that checks it.

- **base-places**: every added requirement's scenarios are in
  `crates/flume-conformance/src/engine/base_places.rs`:
  - a base place naming `mend`
  - the two refused fix kinds
  - the three settles that ask no forecast
  - the forecast held, and one that could not be read
  - the failed and conflicting submissions launching `mend--spring-1`
  - the entry queued ahead
  - the forecast conflict launching the fix in `east` at priority 4
    with recipients from its base place
  - the forecast conflict's brief
  - S120 as `s120_a_base_place_forecast_in_conflict_launches_its_fix`
  - the fix landing, then a clear forecast
  - two settles at once
  - a launch that died between its writing and its making
  - a dropped fix
  - the bound spent, stall included
  - the bound left out
  - a refused launch
  - a declaration changed after the launch

  The modified requirement's two scenarios are moved to a `train` naming
  no fix, S112 among them. The engine's scenarios passed in the gate.
- **place-status**: the four JSON scenarios are in
  `crates/flume-conformance/src/engine/place_status.rs`, and their fields
  are listed in `answers.rs`'s `FIELDS`. "A fix landed, in the person's
  form" is `commands::print::tests::a_fix_landed_in_the_persons_form`,
  JSON half included.
- **seam-contracts**: the round trips are in
  `crates/flume-seams/src/round_trip.rs`, including a held state written
  before this change; they passed in `seams:test`. "Clear, then in
  conflict" is `base_place_scenarios` in
  `crates/flume-conformance/src/seams/merge_queue.rs`. Against sluice it
  passed in `flume:tools`. Against the fake it is in `tests/seams.rs`,
  which the gate did not reach.
- **seam-fakes**: `the_fake_merge_queue_scripts_a_forecast` passed in
  `fakes:test`.
- **merge-queue-profile**: the sluice profile's four readings of an
  answer (`a_clear_forecast`, `a_conflicts_words`,
  `a_conflict_with_entries_ahead`, `a_forecast_refused`) passed in
  `flume:test`. `a_base_places_branch_forecast_over_a_scratch_repository`
  passed against the pinned sluice in `flume:tools`.
- **clause-coverage**: `clauses.rs` reads agentplot/blueprints 31f9f1c.
  That revision of `design/flume/requirements.md` holds 210, in Part B,
  and S120, and no clause 209. `RENAME` is gone from the unbuilt list,
  and 210 and S120 are cited and named as built. `conformance:coverage`
  passed.

### Core behaviour checked by reading

- **The forecast** (`base.rs`, `forecast_base`) is asked only for a live
  base place that names a fix, with no live fix, no place on its branch
  and no open submission. It is held only where its outcome, head or
  base's revision changed. `Engine::update` skips a write that changes
  nothing, so a repeated forecast writes nothing.
- **A failure's fix** (`submit.rs`, `verdict_at`) is decided in the same
  compare-and-set write that holds the verdict.
- **A fix's end** is written onto its base place before its tidy
  removes anything (`tidy.rs`, `fixes.rs`'s `fix_ended`).
- **What `decide` checks**, in `fixes.rs`, in order:
  1. one fix per cause;
  2. one fix live at a time;
  3. the bound, else `Defaults::send_backs`.

## What goes back to the code stage

Take `sluice` out of the doc comments at
`crates/flume-seams/src/merge_queue.rs:241` and `:330` (core.1). Then run
the gate again, so that `conformance:test`'s `seams` and `suite`
binaries run.
