# Verify: where-an-agent-opens-is-given-by-kind-and-by-stage

Read on 2026-11-06 at `f25044a`, the head of the branch
`change/where-an-agent-opens-is-given-by-kind-and-by-stage`. The check
covers the change's proposal, design, nine delta specs and tasks. The
branch holds flume's main at `880f1d0`, which is origin's main after a
fetch on 2026-11-06.

## Verdict

**Holds; archived.**

- Every task of groups 1 to 12 is ticked: 26 of 26.
- Every requirement of the nine delta specs is built, and each scenario
  is held by a test that asserts its THEN.
- The merge of `880f1d0` (task 12.1) keeps both sides in each of the
  five files that conflicted (below).
- `openspec validate where-an-agent-opens-is-given-by-kind-and-by-stage
  --strict --no-interactive` passes.
- A trial archive passes 43 of 43 specs. It ran on the head's
  `openspec/`, copied to a scratch directory, and applied 7
  requirements added and 18 modified. Seven of the specs it wrote are
  the same, file for file, as those the archive at `01a8e35` wrote.
  command-line and the-list differ from that archive only by #123's
  requirements and by the refitted requirement of task 12.1.
- `sluice exec main` at `f25044a` passes all 23 of the check tasks the
  head affects, flume:tools 101 of 101 among them, and sluice recorded
  the pass under its key (below).

## Since the verify at `65fa551`

That verify found the work holding, and `01a8e35` archived it. Main's
#123, `fix/flume-list-leaves-finished-places-out`, had landed at
`880f1d0`, and the merge queue found the branch conflicting with it.
The change came out of the archive for task 12.1. Between `65fa551`
and `f25044a` these commits changed the tree:

- `f46506d` adds the earlier report.
- `01a8e35` archives the change and writes its deltas to the specs.
- `8b2a916` merges main at `880f1d0`, resolving the five files that
  conflicted.
- `f25044a` moves the change out of the archive and leaves
  `openspec/specs/` as main holds it at `880f1d0`. It refits the-list
  delta's "The list reads only what flume keeps" and ticks task 12.1.

Against the archived change at `01a8e35`, the proposal, the design and
eight of the nine deltas are unchanged. The tasks gain 12.1, and
the-list delta changes in one requirement. `crates/flume-core/src/settle.rs`
and `crates/flume-fakes` are unchanged since `65fa551`, so 11.1's mend
and its scenario stand as the earlier report read them.

### Task 12.1

- **Record 0049.** The record keeps both amendments. Its status line
  names the runs amendment, accepted on the user's card, as "the first
  section below". It names the finished-places amendment, accepted on
  the user's yes, as "the last section below". Both sections follow,
  in that order.
- **core.52.** The rule keeps main's clause on leaving out places that
  landed or were dropped. Its details keep both "Rules out" items: a
  terminal asked once for each session, citing record 0049's first
  amendment, and a landed or dropped place listed without `--all`,
  citing its second. The source line names 63 and 199 and "record 0049
  and its two amendments".
- **`reference/commands.md`.** The stream line keeps this change's
  sessions after the own. It ends with main's "A host with no places to
  list prints `no places` before `streams`."
- **`crates/flume-conformance/src/engine/list.rs`.** The module's doc
  gives both: a landed or dropped place only where every place is
  asked, and each session a stream names. It cites 63 and 199.
- **The code.** `crates/flume-core/src/list.rs` merged without a
  conflict. `Engine::list(every)` leaves out `landed_or_dropped` places
  unless every place is asked, and `stream_listed` reads a stream's own
  session and its named sessions in one runs. `flume list` takes
  `--all`, and `flume open` takes a session's name
  (`crates/flume/src/commands/mod.rs`).
- **The refit.** The-list delta's "The list reads only what flume
  keeps" is main's requirement at `880f1d0` with two additions. It
  reads each stream's terminal once "for all its sessions together",
  and it cites 199. Its body is 489 characters, within OpenSpec
  1.14.3's 500. Its scenario "Fifty places in three repositories" is
  main's word for word. `s108_the_list_of_fifty_places` asserts the
  scenario's THEN: 48 places, the two landed left out, three runs, and
  fifty places when every place is asked.
- **The trial archive.** The-list's spec holds both "The list leaves out
  a place that landed or was dropped unless every place is asked" and
  "A stream's line gives each session it names for a kind or a stage".
  command-line's holds main's `--all` and its scenario "A dropped place
  listed only with --all".

## Tasks

All 26 tasks are ticked, `[x]`, and none is open.

- **Record 0048** and its amendment are `accepted`. They cite the
  statement on agentplot/blueprints main at `f676e8f` and `fa40c94`
  (1.1). core.44 and core.48 cite it.
- **Record 0049's first amendment** is `accepted`, its statement on
  main at `12866a8` (8.1, 10.1). No file of the record, the proposal or
  the design names `9e08d54` or says unpushed. The tasks name it only
  in 10.1's own words.
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
  0049's amendment accepted** (10.1): as the verifies at `29c4ea7` and
  `65fa551` read them, each test found again at `f25044a`.
- **Two settles at once** (11.1): as the verify at `65fa551` read it.
  The mend, the fake store's hook and the scenario are unchanged.
- **Main's list beside the list's sessions** (12.1): above.

## Requirements and scenarios

For every scenario of the nine deltas, a test was found and read
against its scenario's THEN. Eight deltas are unchanged since the
verify at `8d4b6b8`, which first read them.

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
  `s108_the_list_of_fifty_places`, read again against the refitted
  scenario, `a_streams_sessions_for_its_kinds_and_stages`, which asserts
  the line and the one runs `runs work work-design`, and the round
  trip's `sessions`.
- **clause-coverage** (2 modified, 3 scenarios): `clauses.rs` at
  `fa40c94`, `coverage.rs`'s `a_clause_left_out_is_named`, and
  `conformance:coverage` in the gate.

## The gate

`sluice exec main` ran on 2026-11-06 at `f25044a`, over the checks the
head affects against main at `880f1d0`. It exited 0 after 5 minutes 7
seconds. Of the 24 tasks it completed, all 23 checks passed, and sluice
recorded the result under its key for `f25044a`:

- **root:** format, adrs (44 records) and changes (2 of 2 valid).
- **seams, fakes, core, conformance, flume:** format and lint.
- **tests:**
  - seams 68, core 27 and fakes 49;
  - conformance's suites, `the_engine_passes_its_scenarios` among them;
  - flume:test's suites 167, 2, 47, 19, 148 and 2;
  - plugin-claude.
- **conformance:coverage** passes at `fa40c94`, every clause cited by
  the suite's 598 scenarios except those it names for other changes.
- **site:** install, prose and build, every internal link valid.
- **flume:tools** passes 101 of 101, among them
  `s107_a_design_session_through_the_binary`,
  `runs_of_several_sessions_reads_them_in_one_listing` and
  `a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start`.

`root:specs` is not among the tasks the head affects, since the branch
leaves `openspec/specs/` as main holds it. Its command passes 43 of 43
over the specs the trial archive writes, as the verdict says.

## Noted, not short

- `openspec archive` leaves each spec's Purpose as main holds it.
  stream-views' Purpose still says `flume open [<stream>]`, where its
  requirements now take a session's name. No requirement or scenario
  depends on it. A later change that touches stream-views is where it
  is reworded.
