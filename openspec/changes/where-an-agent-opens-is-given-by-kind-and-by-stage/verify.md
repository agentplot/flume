# Verify: where-an-agent-opens-is-given-by-kind-and-by-stage

Read on 2026-11-06 at `4abb16a`, the head of the branch
`change/where-an-agent-opens-is-given-by-kind-and-by-stage`. The check
covers the change's proposal, design, nine delta specs and tasks. The
branch holds flume's main at `78ea62e`, its merge base, and main holds
nothing the branch lacks.

## Verdict

**Falls short; not archived.** The code holds every scenario, but the
archive would fail the gate.

- **The archive fails `root:specs`.** A trial archive, in a scratch
  worktree at `4abb16a`, applies all nine deltas: 5 requirements added
  and 19 modified. It then leaves `openspec validate --specs --strict
  --no-interactive`, the command `root:specs` runs, failing 4 of 43
  specs. Before the archive it passes 43 of 43. Each failure is a
  requirement this change modifies whose text passes OpenSpec 1.14.3's
  limit of 500 characters, which `--strict` reads as an error:

  | Spec | Requirement | Characters |
  |---|---|---|
  | seam-contracts | The host's facts give sessions, labels and the layers | 519 |
  | seam-contracts | The types of a session, a label and the layers | 505 |
  | stream-sessions | A stream's agents open in its own session | 531 |
  | stream-views | A person opens a stream's session in its view | 732 |
  | the-list | The list gives a line for each stream the host declares | 653 |
  | the-list | The list's JSON form | 513 |

  `openspec validate where-an-agent-opens-is-given-by-kind-and-by-stage
  --strict` passes, since it does not apply the length check to a
  delta's text. The branch's own `root:specs` passes too, because it
  validates the specs as main holds them. The commit that archives the
  change is where the failure shows, and main would carry it. The
  deltas need these six requirements shortened, or split with their
  examples moved into scenarios, before the archive.
- Every task of groups 1 to 8 is ticked.
- Every requirement of the nine delta specs is built, and each scenario
  is held by a test that asserts its THEN (below).
- `sluice exec main` at `4abb16a` passes all 23 tasks the head
  affects, flume:tools 101 of 101 among them.

## Tasks

All 22 tasks are ticked, `[x]`, and none is open.

- **Record 0048** and its amendment are `accepted`. They cite the
  statement on agentplot/blueprints main at `f676e8f` and `fa40c94`,
  which is blueprints main's head on GitHub on 2026-11-06 (1.1). core.44
  and core.48 cite it.
- **Record 0049's amendment** is `proposed`. Its statement amendment
  of 201 and C.5's runs is on the blueprints branch
  `design/flume-runs-reads-several-parts-at-once` at `9e08d54`, cut
  from `fa40c94`. That commit is local to the host and not on GitHub.
  Task 8.1 asks for it "committed and unpushed for the user's card",
  so the task holds. The amendment still waits on the user's card, and
  the orchestrator pushes it once they approve.
- **The seams** (2.1): `Stream::kinds`, `KindSessions`,
  `StageSession`, `SessionFrom::StreamKind` and `StreamStage`,
  `NamedSession`, `NamedFor` and `TableFault`. Each reads with none
  where written before and writes only where set. `round_trip.rs`'s
  `a_starts_session_from_its_streams_word_for_its_stage` checks the
  JSON form.
- **The host's configuration** (3.1): `kinds_sessions` in
  `crates/flume/src/host/config.rs` reads the kind's and the stage's
  tables. The two keys are in `LOCAL_KEYS`.
- **Where an agent opens** (4.1): `StreamRoom::session_for` and
  `Queue::session_of` take the kind and the stage. `Engine::claim`
  refuses on either refusal, and a stage waiting for room is refused at
  the next settle
  (`a_waiting_stage_whose_table_comes_to_hold_another_key`).
- **Status** (5.1): each stream's `sessions` and `faults`. `print.rs`'s
  `each_agents_session` and
  `a_streams_named_sessions_and_its_tables_at_fault` check the
  person's form.
- **The list's place told** (5.2): the list landed on main as #118.
  Its archived verify report quotes clause 200 as this place sent it.
- **The open** (6.1): `Engine::open_session`. `flume open`'s help
  says "A session's name … one a stream names for a kind or a stage".
- **Under the pinned herdr** (6.2):
  `tests/profiles/tools/open.rs`'s
  `s107_a_design_session_through_the_binary`.
- **The clause list** (7.1): `clauses.rs` reads `REVISION =
  "fa40c94"`, with 199, S106 and S107 among the built.
- **The site** (7.2, 8.4): six pages under `site/src/content/docs`.
  None names a host, team or word of ours (core.35).
- **runs over several parts** (8.2) and **the list's sessions** (8.3)
  are covered below.

## Requirements and scenarios

Every scenario below was found by name at `4abb16a`, and its THEN read
against its assertions.

### stream-sessions (8 requirements, 27 scenarios)

`crates/flume-conformance/src/engine/sessions.rs` holds every scenario,
over the fakes. The new ones are:

- `a_kinds_session`, `a_stages_session` and
  `a_stages_session_before_its_kinds`. Each checks where each stage's
  agent opened and the `SessionHeld` its start records.
- `another_streams_word`.
- `a_kinds_session_changed_between_starts`: the running agent stays
  where it opened, and one `open` is logged before its delivery.
- `a_launchs_session_over_the_streams_word_for_a_kind` and
  `..._for_a_stage`.
- `a_pane_before_the_streams_word_for_a_kind`.
- `a_kind_no_place_runs` and `a_stage_the_kind_does_not_hold`.
- `a_tables_other_key`: refused naming
  `streams.work.kinds.research.label`, while the change on `work` and
  the research on `personal` start; a retry once the table is mended
  starts the stage in `work-design`.
- `a_stages_tables_other_key`.
- `another_streams_own_session` and
  `two_streams_naming_one_session_for_a_kind`, both naming both streams
  and the session.
- `a_streams_own_session_named_for_a_kind`, from `stream-kind`.
- `room_and_shares_unchanged`: `work`'s running, bound and room are
  `(2, Some(2), Some(0))`, each account comes from `work`'s pool, and
  the chore is `WaitingForRoom`.
- `the_sessions_a_stream_names_for_its_kinds_and_stages`, in the JSON
  form, with `print.rs` holding the person's form.

`config.rs`'s `a_session_that_is_no_word` holds "A session that is no
word".

### host-configuration (1 modified, 3 new scenarios)

`config.rs` holds `a_kinds_session_and_a_stages_set_locally`,
`a_kinds_table_key_the_local_layer_cannot_set` and
`a_kinds_session_pinned`. The last passes over both keys under the pin
`streams.work.kinds` and takes `work`'s three agents.

### stream-views (3 modified, 18 scenarios)

`engine/open.rs` holds the new ones:

- `the_design_session_by_its_name`, titled `work-design` from
  `session`;
- `a_stages_session_by_its_name`;
- `a_streams_own_session_named_for_a_kind`, under `Work Agents` as the
  stream's own;
- `a_design_session_in_the_streams_app`, where the window logs nothing;
- `a_name_no_streams_word_gives`;
- `a_streams_name_that_is_not_its_sessions`;
- `a_session_two_streams_words_give`.

Each refusal leaves the terminal and the window logging nothing.

### command-line (1 modified, 2 new scenarios)

- *A design session through the binary* is
  `s107_a_design_session_through_the_binary`, under the pinned herdr.
  It checks:
  - the servers `w-design` and `w` running, with a tab on each;
  - status giving `stream-kind` and `stream-name`;
  - the open's JSON, the stream, session, `named` and title;
  - the stand-in Ghostty's two windows, each running `session attach`
    under its title;
  - that no server keeps a tab or workspace of flume's after the tidies.
- *A name no stream's word gives* is `tests/commands.rs`'s
  `a_name_no_streams_word_gives`, in JSON, in the person's form and in
  the help.

### seam-contracts, seam-fakes, terminal-profiles (runs over several parts)

- **seam-contracts:**
  - `seams/terminal.rs`'s `runs_reads_each_part` asks every part in one
    call, and checks the one logged `runs <part> <part>…`;
  - `engine/streams.rs`'s `the_hosts_sessions_for_its_kinds_and_stages`;
  - `round_trip.rs` holds the start's session from `stream-stage`.
- **seam-fakes:** `tests/fakes.rs`'s `several_parts_in_one_runs` and
  `a_streams_sessions_for_its_kinds_and_stages_are_scripted`.
- **terminal-profiles:**
  - `tools/terminal_herdr.rs`'s
    `runs_of_several_sessions_reads_them_in_one_listing`, through a
    logging herdr: one `session list --json`, and `x` still unlisted;
  - `terminal_printed.rs`'s several parts, each holding no part, in
    order.

### the-list (3 modified, 5 scenarios)

- `engine/list.rs`'s `a_streams_sessions_for_its_kinds_and_stages`
  checks the line `work · session work · runs · session work-design ·
  not running`. Its `sessions` holds `work-design` alone, so `work`,
  named for `fix`, is not repeated. The JSON is checked, and the
  terminal logs `runs work work-design` and two runs in all.
- `answers.rs`'s field list holds `list.streams[].sessions` and its
  fields.
- `list.rs`'s unit test
  `a_streams_line_gives_each_session_it_names_after_its_own` covers:
  - a stream that runs, with a named session not running;
  - an uncarried multiplexer, its sessions before `no profile for rex`;
  - a terminal that holds no part, `work · no session`.

### clause-coverage (2 modified, 3 scenarios)

`clauses.rs`'s tests hold `fa40c94`, with 199, S106 and S107 built.
`conformance:coverage` in the gate holds "The suite as this change
leaves it".

## The gate

`sluice exec main`, run on 2026-11-06 at `4abb16a`, ran the checks the
head affects: 24 tasks completed, 23 passed, none failed, and it exits
0. It took 5 minutes 16 seconds. sluice did not record the results for
`4abb16a`, because this report was written into the worktree while the
run went on.

- **root:** format, adrs and changes, the last with 2 of 2 changes
  valid;
- **seams, fakes, core, conformance, flume:** format and lint;
- **tests:** seams 68, core 26, fakes 49, conformance's suites,
  flume:test's suites 167, 2, 46, 19, 148 and 2, and plugin-claude;
- **conformance:coverage** passes at `fa40c94`;
- **flume:tools:** 101 of 101, with
  `s107_a_design_session_through_the_binary` and
  `runs_of_several_sessions_reads_them_in_one_listing`;
- **site:** install, prose and build.

`root:specs` is not among the tasks the head affects, since the branch
leaves `openspec/specs/` as main has it. Its command, run over the
specs the trial archive writes, fails 4 of 43, as the verdict says.

## What goes back to the code stage

Shorten the six requirements in the table above, in the change's
deltas, until each requirement's text is 500 characters or fewer. Move
examples and edge cases into scenarios, or split a requirement into
ones that each state one behaviour. Then a trial `openspec archive`
must leave `openspec validate --specs --strict --no-interactive`
passing 43 of 43. Nothing else falls short.
