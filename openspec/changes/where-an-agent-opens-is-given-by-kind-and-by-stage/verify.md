# Verify: where-an-agent-opens-is-given-by-kind-and-by-stage

Read on 2026-11-06 at `8d4b6b8`, the head of the branch
`change/where-an-agent-opens-is-given-by-kind-and-by-stage`. The check
covers the change's proposal, design, nine delta specs and tasks. The
branch holds flume's main at `78ea62e`, its merge base. Main holds
nothing the branch lacks, and origin's main is the same revision.

## Verdict

**Holds; archived.**

- Every task of groups 1 to 9 is ticked.
- Every requirement of the nine delta specs is built, and each scenario
  is held by a test that asserts its THEN (below).
- A trial archive passes 43 of 43 specs. It ran on the head's
  `openspec/`, copied to a scratch directory, and applied 7 requirements
  added and 18 modified. `openspec validate --specs --strict
  --no-interactive`, the command `root:specs` runs, passes 43 of 43
  both before and after it.
- `sluice exec main` at `8d4b6b8` passes 23 of the 24 tasks the head
  affects. flume:tools failed one walkthrough on a race in main's
  settle, which this change does not touch. Run again, flume:tools
  passes 101 of 101 (below).

## Since the verify at `4abb16a`

The verify at `4abb16a` found the code holding every scenario. Its
trial archive left `root:specs` failing 4 of 43 specs: six modified
requirements were past OpenSpec 1.14.3's 500 characters. Task 9.1 was
added for them, and `8d4b6b8` delivers it. Between `4abb16a` and
`8d4b6b8` only the change's own files changed, in these commits:

- `1335d89` adds the earlier report.
- `8d4b6b8` shortens four deltas and adds task 9.1.

No code, test, site page or record changed, so the earlier report's
reading of the code still holds at this head. It was read again here
for each scenario, and is summarized below.

The six requirements now read within the limit:

- **seam-contracts:**
  - "The host's facts give sessions, labels and the layers" and "The
    types of a session, a label and the layers" are said in fewer
    words. Each keeps every fact, type and origin it named.
- **stream-sessions:**
  - "A stream's agents open in its own session" is said in fewer words.
    It keeps the order: the stream's word for the kind's stage, then the
    kind, then the stream's own.
- **stream-views:**
  - "A person opens a stream's session in its view" gives the stream's
    own session.
  - A new requirement, "An open shows a session a stream names for a
    kind or a stage", gives the others: a named session is shown in its
    stream's view under the session's name, and one that is also the
    stream's own under the stream's title. Its four scenarios moved
    with it, word for word.
- **the-list:**
  - "The list gives a line for each stream the host declares" is no
    longer modified, and stays as main holds it.
  - A new requirement, "A stream's line gives each session it names for
    a kind or a stage", gives the sessions after the stream's own,
    read in one runs. Where the multiplexer is uncarried, the sessions
    come with no runs part, before the one `no profile for
    <multiplexer>`. Its scenario moved with it.
  - "The list's JSON form" is said in fewer words.

Each new or shortened text was read against the code it describes:

- `engine/open.rs`'s `the_design_session_by_its_name` gives the title
  `work-design` from `session`.
- `a_streams_own_session_named_for_a_kind` gives the title `Work
  Agents` from `stream`, the session from `stream`.
- `flume-core/src/list.rs`'s unit tests give `work · session work ·
  session work-design · no profile for rex` for an uncarried stream.

## Tasks

All 23 tasks are ticked, `[x]`, and none is open.

- **Record 0048** and its amendment are `accepted`. They cite the
  statement on agentplot/blueprints main at `f676e8f` and `fa40c94`
  (1.1). core.44 and core.48 cite it.
- **Record 0049's amendment** is `proposed`, so 8.1 holds as it asks.
  Its amendment of 201 and C.5's runs is on the blueprints branch
  `design/flume-runs-reads-several-parts-at-once` at `9e08d54`, cut
  from `fa40c94` and committed for the user's card. Task 8.1 asks for
  the branch unpushed. The amendment still waits on the user's card,
  and the orchestrator pushes it on their yes.
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
  "fa40c94"`, with 199, S106 and S107 among the built.
- **The site** (7.2, 8.4): the pages design 14 names. None names a
  host, team or word of ours (core.35).
- **runs over several parts and the list's sessions** (8.2, 8.3): see
  below.
- **The deltas within OpenSpec's length** (9.1): the trial archive
  above.

## Requirements and scenarios

For every scenario of the nine deltas, a test was found at `8d4b6b8`.
Each test was read against its scenario's THEN.

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

`sluice exec main` ran on 2026-11-06 at `8d4b6b8`, over the checks the
head affects. Of 24 tasks, 23 passed and one failed, in 5 minutes 19
seconds:

- **root:** format, adrs and changes. Changes found 2 of 2 valid.
- **seams, fakes, core, conformance, flume:** format and lint.
- **tests:**
  - seams 68, core 26 and fakes 49;
  - conformance's suites;
  - flume:test's suites 167, 2, 46, 19, 148 and 2;
  - plugin-claude.
- **conformance:coverage** passes at `fa40c94`.
- **site:** install, prose and build.
- **flume:tools** failed: 100 of 101 passed.

The test that failed is
`walkthroughs::a_pool_with_no_account_of_a_stages_program_refuses_it_at_its_launch_and_at_a_later_start`.
It waits for code to read `refused` and then asserts that the place
waits on a retry of code. It found the place waiting on a retry of
verify, with both code and verify `refused` for want of an account of
`claude`.

`sluice exec main -- moon run flume:tools`, run again at `8d4b6b8`,
passes 101 of 101 in 5 minutes 9 seconds. That test passes there, as do
`s107_a_design_session_through_the_binary` and
`runs_of_several_sessions_reads_them_in_one_listing`. The verify at
`4abb16a` also passed 101 of 101 over the same code.

The failure is a race in main's code, and this change does not reach
it:

- The cause:
  - `Engine::step` in `crates/flume-core/src/settle.rs` reads the
    current stage `delivered` and calls `advance`.
  - `advance`'s update reads `held.current` again and starts the stage
    after it. It checks only that the current stage has no live start
    and is not repeating, not that it is still `delivered` or
    `finished`.
  - `settle_place` holds no lock, and the walk settles every 300
    milliseconds beside the stand-in agent's own settle.
- How it fails:
  - One settle reads construct `delivered`.
  - Another starts code, and the refusal path of `Engine::claim` writes
    code `refused` and `held.current` at code.
  - The first settle's `advance` then takes verify as the stage after
    code, and refuses it too.
- Why it is main's:
  - The branch leaves `settle.rs` as main holds it.
  - In `start.rs` it changes only the start's session, which now
    refuses in the same path. The pool refusal and its `held.current`
    are main's.

The fix belongs to a fix place on main, outside this change: `advance`
should start the next stage only while the current stage is
`delivered` or `finished` within its update. Holding this change for
it would send a stage back to code for work this change does not own.
So the work is taken as holding, and the race is named here and to the
caller.

`root:specs` is not among the tasks the head affects, since the branch
leaves `openspec/specs/` as main has it. Its command passes 43 of 43
over the specs the trial archive writes, as the verdict says.

## Noted, not short

`openspec archive` leaves each spec's Purpose as main holds it.
stream-views' Purpose still says `flume open [<stream>]`, where its
requirements now take a session's name. The OpenSpec deltas carry no
Purpose, and no spec file is edited outside a change, so a later change
that touches stream-views is where it is reworded. No requirement or
scenario depends on it.
