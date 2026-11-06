# Verify: where-an-agent-opens-is-given-by-kind-and-by-stage

Read on 2026-11-06 at `29c4ea7`, the head of the branch
`change/where-an-agent-opens-is-given-by-kind-and-by-stage`. The check
covers the change's proposal, design, nine delta specs and tasks. The
branch holds flume's main at `78ea62e`, its merge base. Main holds
nothing the branch lacks, and origin's main is the same revision.

## Verdict

**Holds; archived.**

- Every task of groups 1 to 10 is ticked.
- Every requirement of the nine delta specs is built, and each scenario
  is held by a test that asserts its THEN (below).
- `openspec validate where-an-agent-opens-is-given-by-kind-and-by-stage
  --strict --no-interactive` passes.
- A trial archive passes 43 of 43 specs. It ran on the head's
  `openspec/`, copied to a scratch directory, and applied 7
  requirements added and 18 modified. The specs it wrote are the same,
  file for file, as those the archive at `51ad435` wrote.
- `sluice exec main` at `29c4ea7` passes all 23 of the check tasks the
  head affects, flume:tools 101 of 101 among them (below).

## Since the verify at `8d4b6b8`

That verify found the work holding, and `51ad435` archived it. The
change came out of the archive for task 10.1, which marks record
0049's amendment accepted and cites the statement where it now stands.
Between `8d4b6b8` and `29c4ea7` these commits changed the tree:

- `78d2d7b` adds the earlier report.
- `51ad435` archives the change and writes its deltas to the specs.
- `55e397e` moves the change out of the archive and leaves
  `openspec/specs/` as main holds it. No file of the change was
  changed in the move.
- `29c4ea7` delivers task 10.1. It changes record 0049, the proposal,
  the design and the tasks, and nothing else.

No code, test, delta, site page or rule changed, so the earlier
report's reading of each scenario still holds at this head. Each test
named below was found again at `29c4ea7`.

### Task 10.1

- **Record 0049's amendment** is accepted. Its status says the user's
  Pending You card of 2026-11-06 approved it, and that its statement
  amendment is on agentplot/blueprints main at `12866a8`. Its sources
  and its consequences cite `12866a8` too.
- **agentplot/blueprints `12866a8`** was read on origin's main, where it
  is the head. Its parent is `fa40c94`. Its tree is `9e08d54`'s tree,
  `db7dde3`. It changes only `design/flume/requirements.md`, amending
  201 and C.5's runs. Its message marks record 0049's amendment
  accepted on the user's card of 2026-11-06.
- **The proposal, the design and task 8.1** cite `12866a8` on main
  where they cited `9e08d54` on a branch. The proposal says record
  0049's amendment is accepted on the user's card.
- **No file of the record, the proposal or the design** names
  `9e08d54` or says unpushed. The tasks name `9e08d54` only in 10.1's
  own words, as the tree `12866a8` holds. No other file of the tree but
  this report names it or the branch
  `design/flume-runs-reads-several-parts-at-once`.
- **Record 0048** cites no revision of 0049's amendment, and core.52's
  details cite "record 0049's amendment" with no revision. Neither
  needed a change.

## Tasks

All 24 tasks are ticked, `[x]`, and none is open.

- **Record 0048** and its amendment are `accepted`. They cite the
  statement on agentplot/blueprints main at `f676e8f` and `fa40c94`
  (1.1). core.44 and core.48 cite it.
- **Record 0049's amendment** is `accepted`, its statement on main at
  `12866a8` (8.1, 10.1).
- **The seams** (2.1, 2.2):
  - `Stream::kinds`, `KindSessions`, `StageSession`,
    `SessionFrom::StreamKind` and `StreamStage`, `NamedSession`,
    `NamedFor` and `TableFault`.
  - The fake host scripts a stream's `kinds`, which
    `engine/streams.rs`'s `the_hosts_sessions_for_its_kinds_and_stages`
    reads.
- **The host's configuration** (3.1): `kinds_sessions` in
  `crates/flume/src/host/config.rs` reads the kind's and the stage's
  tables. Both keys are in `LOCAL_KEYS`.
- **Where an agent opens** (4.1): `StreamRoom::session_for` and
  `Queue::session_of` take the kind and the stage, and `Engine::claim`
  refuses on either refusal.
- **Status** (5.1): each stream's `sessions` and `faults`, with
  `print.rs`'s unit tests of the person's form.
- **The list's place told** (5.2): the list landed on main as #118.
- **The open** (6.1, 6.2): `Engine::open_session`, `flume open`'s
  session argument and help, and
  `tests/profiles/tools/open.rs`'s
  `s107_a_design_session_through_the_binary` under the pinned herdr.
- **The clause list** (7.1): `clauses.rs` reads `REVISION =
  "fa40c94"`, with 199, S106 and S107 among the built. `12866a8`
  rewords 201 and C.5's runs and adds no clause, so the list at
  `fa40c94` names the same clauses.
- **The site** (7.2, 8.4): the pages design 14 names. None names a
  host, team or word of ours (core.35).
- **runs over several parts and the list's sessions** (8.2, 8.3): see
  below.
- **The deltas within OpenSpec's length** (9.1): the trial archive
  above.
- **Record 0049's amendment accepted** (10.1): see above.

## Requirements and scenarios

For every scenario of the nine deltas, a test was found. Each test was
read against its scenario's THEN at `8d4b6b8`, and the code and tests
are unchanged since.

### stream-sessions (8 requirements, 27 scenarios)

`crates/flume-conformance/src/engine/sessions.rs` holds every scenario
but one, over the fakes:

- the kind's and the stage's session, and the stage's before the
  kind's;
- another stream's word;
- a kind's session changed between starts;
- the launch's session and a pane over the stream's word;
- a kind and a stage the place does not run;
- a table's other key, and a stage's table's other key;
- another stream's own session, and two streams naming one session;
- a stream's own session named for a kind;
- room and shares unchanged;
- the sessions a stream names, in status's JSON form.

`s82_each_stream_in_its_own_session` holds "Two streams, each in its
own session". `config.rs`'s
`a_session_that_is_no_word` holds "A session that is no word".

### host-configuration (1 modified, 8 scenarios)

`config.rs` holds every scenario of the requirement:

- the five already on main;
- `a_kinds_session_and_a_stages_set_locally`;
- `a_kinds_table_key_the_local_layer_cannot_set`;
- `a_kinds_session_pinned`.

### stream-views (4 requirements, 18 scenarios)

`engine/open.rs` holds every scenario. Among them are the four of the
added requirement, `the_design_session_by_its_name`,
`a_stages_session_by_its_name`,
`a_streams_own_session_named_for_a_kind` and
`a_design_session_in_the_streams_app`. It also holds the refusals:

- `a_name_no_streams_word_gives`;
- `a_streams_name_that_is_not_its_sessions`;
- `a_session_two_streams_words_give`.

Each refusal leaves the terminal and the window logging nothing.

### command-line (1 modified, 7 scenarios)

- *A design session through the binary* is
  `s107_a_design_session_through_the_binary`, under the pinned herdr.
- *A name no stream's word gives* is `tests/commands.rs`'s
  `a_name_no_streams_word_gives`, in JSON, in the person's form and in
  the help.
- *Two streams under two multiplexers through the binary* is
  `s91_a_stream_under_the_printed_command_beside_one_under_herdr`.
  `commands.rs`'s `open_under_the_printed_command` and
  `open_of_an_undeclared_stream`, and `open.rs`'s
  `s90_a_streams_view_through_the_binary` and
  `s90_the_printed_window_through_the_binary`, hold the other four
  scenarios already on main.

### seam-contracts, seam-fakes, terminal-profiles (runs over several parts)

- **seam-contracts:**
  - `seams/terminal.rs`'s `runs_reads_each_part` asks every part in one
    call;
  - `engine/streams.rs`'s `the_hosts_sessions_labels_and_layers` and
    `the_hosts_sessions_for_its_kinds_and_stages`;
  - `round_trip.rs`'s `a_held_state_with_a_session_and_a_label` and
    `a_starts_session_from_its_streams_word_for_its_stage`.
- **seam-fakes:** `tests/fakes.rs`'s `a_scripted_runs` and
  `several_parts_in_one_runs`.
- **terminal-profiles:**
  - `tools/terminal_herdr.rs`'s
    `runs_of_a_session_not_listed_reads_not_running_and_leaves_it_so`,
    `runs_reads_a_running_session_and_the_own_session_by_its_name` and
    `runs_of_several_sessions_reads_them_in_one_listing`;
  - `terminal_printed.rs`'s
    `runs_under_the_printed_command_holds_no_part_and_runs_nothing`,
    with three parts each holding no part, in order.

### the-list (2 modified, 1 added, 3 scenarios)

- `engine/list.rs`'s `a_streams_sessions_for_its_kinds_and_stages`
  checks `work · session work · runs · session work-design · not
  running`. Its `sessions` holds `work-design` alone, and its JSON
  gives `work-design` not running. The terminal logs `runs work
  work-design`.
- `s108_the_list_of_fifty_places` holds "Fifty places in three
  repositories".
- `the_list_round_trips_and_holds_its_fields` and `answers.rs`'s field
  list hold `sessions`.

### clause-coverage (2 modified, 3 scenarios)

- `clauses.rs`'s tests hold `fa40c94`.
- `coverage.rs`'s `a_clause_left_out_is_named` holds "A clause left
  out".
- `conformance:coverage` in the gate holds "The suite as this change
  leaves it".

## The gate

`sluice exec main` ran on 2026-11-06 at `29c4ea7`, over the checks the
head affects against main at `78ea62e`. It exited 0 after 5 minutes 29
seconds, with 24 tasks completed and all 23 checks among them passed:

- **root:** format, adrs and changes. Changes found 2 of 2 valid.
- **seams, fakes, core, conformance, flume:** format and lint.
- **tests:**
  - seams 68, core 26 and fakes 49;
  - conformance's suites;
  - flume:test's suites 167, 2, 46, 19, 148 and 2;
  - plugin-claude.
- **conformance:coverage** passes at `fa40c94`.
- **site:** install, prose and build.
- **flume:tools** passes 101 of 101, among them
  `s107_a_design_session_through_the_binary`,
  `runs_of_several_sessions_reads_them_in_one_listing` and the
  walkthrough the verify at `8d4b6b8` saw fail once on main's race.

sluice did not record the pass, because this report was being written
in the worktree during the run, beyond its head. The run's checks read
the head's code, and this file is the only one that differed.

`root:specs` is not among the tasks the head affects, since the branch
leaves `openspec/specs/` as main holds it. Its command,
`openspec validate --specs --strict --no-interactive`, passes 43 of 43
over the specs the trial archive writes, as the verdict says.

## Noted, not short

- `openspec archive` leaves each spec's Purpose as main holds it.
  stream-views' Purpose still says `flume open [<stream>]`, where its
  requirements now take a session's name. The OpenSpec deltas carry no
  Purpose, and no spec file is edited outside a change, so a later
  change that touches stream-views is where it is reworded. No
  requirement or scenario depends on it.
- The verify at `8d4b6b8` named a race in main's settle that failed one
  walkthrough of flume:tools once: `advance` in
  `crates/flume-core/src/settle.rs` starts the stage after the current
  one without reading again, within its update, that the current stage
  is still `delivered` or `finished`. This change leaves `settle.rs` as
  main holds it. The fix belongs to a fix place on main.
