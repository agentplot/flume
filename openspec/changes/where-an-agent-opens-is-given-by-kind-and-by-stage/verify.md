# Verify: where-an-agent-opens-is-given-by-kind-and-by-stage

Read on 2026-11-06 at `65fa551`, the head of the branch
`change/where-an-agent-opens-is-given-by-kind-and-by-stage`. The check
covers the change's proposal, design, nine delta specs and tasks. The
branch holds flume's main at `b5dcf96`, its merge base. Origin's main
has since moved to `880f1d0`, which lands #123 (see "Noted, not
short").

## Verdict

**Holds; archived.**

- Every task of groups 1 to 11 is ticked: 25 of 25.
- Every requirement of the nine delta specs is built, and each scenario
  is held by a test that asserts its THEN. The deltas are unchanged
  since the verify at `29c4ea7`.
- Task 11.1's scenario fails without its mend, verify begun, and passes
  with it (below).
- `openspec validate where-an-agent-opens-is-given-by-kind-and-by-stage
  --strict --no-interactive` passes.
- A trial archive passes 43 of 43 specs. It ran on the head's
  `openspec/`, copied to a scratch directory, and applied 7
  requirements added and 18 modified. The specs it wrote are the same,
  file for file, as those the archive at `97efe33` wrote.
- `sluice exec main` at `65fa551` passes all 23 of the check tasks the
  head affects, flume:tools 101 of 101 among them, and sluice recorded
  the pass under its key (below).

## Since the verify at `29c4ea7`

That verify found the work holding, and `97efe33` archived it. #121
then failed on main: sluice read flume:tools flaky, since the verify's
run at `8d4b6b8` had failed one walkthrough at the inputs #121 passed
at. The change came out of the archive for task 11.1. Between
`29c4ea7` and `65fa551` these commits changed the tree:

- `7020278` adds the earlier report.
- `97efe33` archives the change and writes its deltas to the specs.
- `b82c2b8` moves the change out of the archive and leaves
  `openspec/specs/` as main holds it. No file of the change was
  changed in the move.
- `a775f45` merges main at `b5dcf96` (#120), which changes records 0047
  and 0050 and nothing else. Nothing was contested.
- `65fa551` delivers task 11.1. It changes `crates/flume-core`'s
  settle, the fake store, `engine/running_through.rs`, the proposal
  and the tasks.

Against the archived change at `97efe33`, only the proposal and the
tasks differ, by 11.1's lines. No delta, site page, record or rule of
the change changed, so the earlier report's reading of each scenario
still holds at this head.

### Task 11.1

- **The race.** One settle reads construct delivered; another runs
  whole beside it, starting code, which is refused, `held.current`
  moving to code. The first's `advance` then read code as the ended
  stage and started verify, past the refused code.
- **The mend.** `advance` in `crates/flume-core/src/settle.rs` reads
  the current stage again inside its update and starts nothing unless
  that stage still stands `Delivered`, `Finished` or `Repeating`.
  `advance`'s one caller, `step`, calls it only from those three
  states, so the guard changes nothing but the raced read. The doc
  comment cites 75.
- **The hook.** `FakeStore::after_read_of` runs an act once, just after
  the first read that hands out a held state its test holds of. It is
  taken out before it runs, so the act's own reads do not fire it
  again.
- **The scenario.** `running_through.rs`'s
  `a_refused_start_read_late`, citing 40, 46, 74 and 75, runs the second
  settle just after the first reads construct delivered, code refused
  by its stream's table for code naming a key it does not take. It
  checks that the second settle ran, that code is `Refused` and verify
  `NotReached`, that one agent opened, that status's refusal names
  `streams.<stream>.kinds.change.stages.code.pool`, and that the place
  waits on a retry of code, `Refused`.
- **Fails without the mend.** In a scratch worktree at `65fa551` with
  `settle.rs` as `a775f45` holds it, `cargo test -p flume-conformance
  --test engine` failed 1 of 495 engine scenarios, this one alone:
  `verify: Some(Begun) where Some(NotReached) was due`. With the mend
  it passes, in the gate's `conformance:test`.
- **No requirement changes.** The mend makes the settle meet clause 75
  as the specs already say it, so no delta carries it. The proposal's
  impact names it.

## Tasks

All 25 tasks are ticked, `[x]`, and none is open.

- **Record 0048** and its amendment are `accepted`. They cite the
  statement on agentplot/blueprints main at `f676e8f` and `fa40c94`
  (1.1). core.44 and core.48 cite it.
- **Record 0049's amendment** is `accepted`, its statement on main at
  `12866a8` (8.1, 10.1). No file of the record, the proposal or the
  design names `9e08d54` or says unpushed; the tasks name it only in
  10.1's own words.
- **The seams** (2.1, 2.2): `Stream::kinds`, `KindSessions`,
  `StageSession`, `SessionFrom::StreamKind` and `StreamStage`,
  `NamedSession`, `NamedFor` and `TableFault`; the fake host scripts a
  stream's `kinds`.
- **The host's configuration** (3.1): `kinds_sessions` in
  `crates/flume/src/host/config.rs`, both keys in `LOCAL_KEYS`.
- **Where an agent opens** (4.1): `StreamRoom::session_for`,
  `Queue::session_of` taking the kind and the stage, and
  `Engine::claim` refusing on either refusal.
- **Status** (5.1), **the list's place told** (5.2), **the open** (6.1,
  6.2), **the clause list** (7.1, `REVISION = "fa40c94"`), **the site**
  (7.2, 8.4), **runs over several parts and the list's sessions** (8.2,
  8.3), **the deltas within OpenSpec's length** (9.1) and **record
  0049's amendment accepted** (10.1): as the verify at `29c4ea7` read
  them, each test found again at `65fa551`.
- **Two settles at once** (11.1): above.

## Requirements and scenarios

For every scenario of the nine deltas, a test was found at `8d4b6b8`
and read against its scenario's THEN; the deltas and those tests are
unchanged since.

- **stream-sessions** (8 requirements, 27 scenarios):
  `crates/flume-conformance/src/engine/sessions.rs`, with
  `s82_each_stream_in_its_own_session` and `config.rs`'s
  `a_session_that_is_no_word`.
- **host-configuration** (1 modified, 8 scenarios): `config.rs`, among
  them `a_kinds_session_and_a_stages_set_locally`,
  `a_kinds_table_key_the_local_layer_cannot_set` and
  `a_kinds_session_pinned`.
- **stream-views** (4 requirements, 18 scenarios): `engine/open.rs`,
  the refusals leaving the terminal and the window logging nothing.
- **command-line** (1 modified, 7 scenarios):
  `s107_a_design_session_through_the_binary` under the pinned herdr,
  and `tests/commands.rs`'s `a_name_no_streams_word_gives`.
- **seam-contracts, seam-fakes, terminal-profiles**: `runs_reads_each_part`,
  `several_parts_in_one_runs`,
  `runs_of_several_sessions_reads_them_in_one_listing` and
  `runs_under_the_printed_command_holds_no_part_and_runs_nothing`.
- **the-list** (2 modified, 1 added, 3 scenarios): `engine/list.rs`'s
  `a_streams_sessions_for_its_kinds_and_stages`,
  `s108_the_list_of_fifty_places` and the round trip's `sessions`.
- **clause-coverage** (2 modified, 3 scenarios): `clauses.rs` at
  `fa40c94`, `coverage.rs`'s `a_clause_left_out_is_named`, and
  `conformance:coverage` in the gate.

## The gate

`sluice exec main` ran on 2026-11-06 at `65fa551`, over the checks the
head affects against origin's main at `880f1d0`. It exited 0 after 5
minutes 36 seconds, with 24 tasks completed and all 23 checks among
them passed, and recorded the result under its key for `65fa551`:

- **root:** format, adrs and changes. Changes found 2 of 2 valid.
- **seams, fakes, core, conformance, flume:** format and lint.
- **tests:**
  - seams 68, core 26 and fakes 49;
  - conformance's suites, the engine's 495 scenarios among them with
    11.1's;
  - flume:test's suites 167, 2, 46, 19, 148 and 2;
  - plugin-claude.
- **conformance:coverage** passes at `fa40c94`.
- **site:** install, prose and build.
- **flume:tools** passes 101 of 101, among them
  `s107_a_design_session_through_the_binary`,
  `runs_of_several_sessions_reads_them_in_one_listing` and
  `a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start`,
  the walkthrough the race failed.

`root:specs` is not among the tasks the head affects, since the branch
leaves `openspec/specs/` as main holds it. Its command passes 43 of 43
over the specs the trial archive writes, as the verdict says.

## Noted, not short

- **Main has moved.** Origin's main is `880f1d0`, which lands #123,
  `fix/flume-list-leaves-finished-places-out`. The branch does not hold
  it. A trial merge of `880f1d0` into the head conflicts in four files,
  each where both sides reworded one paragraph:
  `crates/flume-conformance/src/engine/list.rs`, record 0049,
  `docs/architecture/core.md`'s core.52 details and
  `site/src/content/docs/reference/commands.md`'s `flume list`. Merged
  into a scratch commit of the archive, it conflicts in
  `openspec/specs/the-list/spec.md` too. This is the merge queue's to
  read as a conflicting submission, which starts the kind's stage for
  it with sluice's words (core.27); nothing of this change's own
  requirements turns on it.
- `openspec archive` leaves each spec's Purpose as main holds it.
  stream-views' Purpose still says `flume open [<stream>]`, where its
  requirements now take a session's name. No requirement or scenario
  depends on it; a later change that touches stream-views is where it
  is reworded.
