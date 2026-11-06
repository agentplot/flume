# Verify: flume-list-shows-each-place-and-stream-on-one-line

Read on 2026-11-06 at `bbab749`, the head of the branch
`change/flume-list-shows-each-place-and-stream-on-one-line`. The check
covers the change's proposal, design, six delta specs and tasks. The
branch holds main at `24551f1`, which is origin's main after a fetch
on 2026-11-06, and `git merge-tree` merges the two with no conflict.

## Verdict

**Holds; archived.**

- Every task of groups 1 to 6 is ticked, and
  `openspec validate flume-list-shows-each-place-and-stream-on-one-line
  --strict` passes.
- Every requirement of the six delta specs is built, and each of their
  29 scenarios is held by a test that asserts its THEN.
- `sluice exec main` at `bbab749` against main at `24551f1` names no
  file contested and passes all 23 tasks the head affects, flume:tools
  99 of 99 among them. sluice recorded the 23 results for `bbab749`.
- A trial archive over the merged tree, in a scratch worktree, applies
  all six deltas, 16 requirements added and 3 modified, and leaves
  `openspec validate --specs --strict` passing 43 of 43. The new
  `openspec/specs/the-list/spec.md` carries the delta's own Purpose.

## Tasks

All 21 tasks are ticked, `[x]`, and none is open.

- **Record 0049** is `accepted`, on the user's yes of 2026-11-05. It
  quotes their words of that day and cites the statement's amendment
  on agentplot/blueprints main at `7bd1891` (1.1).
- **core.52** is drawn after core.15 in `docs/architecture/core.md`,
  citing record 0049, and `site/.../architecture/rules.md` mirrors it
  (1.2).
- **The terminal seam's runs** is `Terminal::runs` with no default,
  answering `SessionRuns`, re-exported. Every terminal implements it:
  the fake, the printed command, herdr and each test's stand-in (2.1).
- **The core's list** is `crates/flume-core/src/list.rs`. It reads
  `store.read_every()` and each live start's asking mark, and asks each
  declared stream's terminal for its runs once. It calls none of
  `bound`, `facts`, `read_agent`, `place_clues` or `host_status`, nor
  status's side-by-side reads that main's #117 added. It takes no lock
  and writes no document (3.1). `line.rs`'s `stage_at` and
  `state_words` are `pub(crate)` and give the list its stage and state.
- **Task 3.2:** `where-an-agent-opens-is-given-by-kind-and-by-stage`
  has not landed on main at `24551f1`. Leaving its sessions to that
  change's build stands, as design 9 has it. See "Beside the change in
  flight" below.
- **The clause list** reads blueprints at `7bd1891`. 200 is in Part A,
  201 is in Part B, and S108 is among the scenarios. Each is cited by
  this change's scenarios (5.2).
- **Main merged (6.1).** The three files main at `24551f1` contested
  are resolved as the tasks ask:
  - `crates/flume-conformance/src/clauses.rs` reads `7bd1891` with
    `SKIPPED`, S93, S94, S98, S99 and S104 to S107. `SCENARIO_GAP` and
    `SOURCES` are gone, and 191, 192 and S95 to S97 are among the
    built. The unbuilt list keeps the entries for records 0041 (S92),
    0043 (195, 196, S100 to S102) and 0044 (197, S103), each by its
    change. `the_clauses_listed` holds 1 to 201 but 100, 150, 193, 194,
    198 and 199. `bin/coverage.rs` names the skipped scenarios where it
    named `SCENARIO_GAP`.
  - `docs/architecture/core.md` holds main's amended core.15 details
    and this change's core.52, each once.
  - The commands reference's "Settling first" names both `flume list`
    and `flume kinds lock`.
- **The Purpose (6.2).** `specs/the-list/spec.md` opens with a
  `## Purpose` of nine lines citing 56, 58, 62, 200 and 201.
- **Each group's closing task** (1.3, 2.4, 3.4, 4.4, 5.3, 6.3) names the
  checks it ran through sluice. The gate run below covers them all at
  the branch's head.

## Requirements and scenarios

Each of the six specs' 19 requirements is built, and each of their 29
scenarios is held by a test that asserts its THEN. Every test named
below was found by name at `bbab749`.

### the-list (7 requirements, 11 scenarios)

All 11 scenarios are in `crates/flume-conformance/src/engine/list.rs`,
over the fakes:

| Scenario | Held by |
|---|---|
| A working fix and a construct on hold | `a_working_fix_and_a_construct_on_hold` |
| A repeating stage and a place being made | `a_repeating_stage_and_a_place_being_made` |
| Places of two repositories | `places_of_two_repositories` |
| The caller, the host and the merge queue | `the_caller_the_host_and_the_merge_queue` |
| A teardown that failed | `a_teardown_that_failed` |
| Holds, a live agent and a tidied document | `holds_a_live_agent_and_a_tidied_document` |
| Three streams over the fake terminal | `three_streams_over_the_fake_terminal` |
| A multiplexer flume carries no profile for | `a_multiplexer_flume_carries_no_profile_for` |
| Fifty places in three repositories | `s108_the_list_of_fifty_places`, citing S108 |
| Every long word left to status | `every_long_word_left_to_status` |
| The list round-trips and holds its fields | `the_list_round_trips_and_holds_its_fields`, with `engine/answers.rs`'s field list |

Where the spec's words name sluice's step, `sluice release`, the
engine's scenarios name it `release`. core.1 rules out a tool's name
outside a profile, and the tasks' preamble asks it of these scenarios.
The step is the teardown's own name either way, so the scenario holds
the same THEN. S108's scenario checks four things after the list
reads:

- the fake change, merge queue, history and place seam log no call;
- the terminal logs three `runs` and nothing else;
- the store logs one `read_every`;
- every document's version is unchanged.

### The modified and added capabilities

- **command-line** (4 requirements, 7 scenarios):
  - *The same answer…* and *A refusal* pass unchanged;
  - `tests/commands.rs` holds the five new scenarios:
    `the_list_through_the_command_line_and_the_core`,
    `a_fix_under_the_printed_command_is_listed_outside_any_repository_running_no_tool`
    (`PATH` an empty folder, the state's files unchanged),
    `fifty_places_are_listed_within_a_second`,
    `the_list_for_a_person_under_the_printed_command` and
    `the_lists_help_names_status_for_a_places_every_fact`;
  - `print.rs`'s unit tests
    `the_list_by_repository_then_the_streams` and
    `a_host_with_no_places_lists_none_then_its_streams` hold the
    person's form.
- **seam-contracts** (2, 2):
  - `seams/terminal.rs`'s `runs_reads_each_part`, over the fake
    harness's `runs_parts`, holds *Runs over the fake*;
  - `round_trip.rs`'s `each_answer_of_runs` holds *Each answer
    round-trips*.
- **seam-fakes** (1, 1): `tests/fakes.rs`'s `a_scripted_runs`.
- **terminal-profiles** (3, 5):
  - `tests/profiles/tools/terminal_herdr.rs` holds herdr's four:
    `runs_of_a_session_not_listed_reads_not_running_and_leaves_it_so`,
    `runs_reads_a_running_session_and_the_own_session_by_its_name`,
    `runs_of_a_herdr_that_does_not_answer_is_unread_within_two_seconds`
    (a stand-in herdr that sleeps ten seconds) and
    `runs_of_an_own_server_held_by_its_socket_reads_the_session_listed_there`;
  - `tests/profiles/terminal_printed.rs`'s
    `runs_under_the_printed_command_holds_no_part_and_runs_nothing`
    holds the printed command's.
- **clause-coverage** (2, 3):
  - `clauses.rs`'s `the_clauses_listed` holds *The clauses listed* at
    `7bd1891`;
  - `coverage.rs` holds *A clause left out*;
  - `conformance:coverage` in the gate holds *The suite as this change
    leaves it*.

## Code against the design

- **`Engine::list`** follows designs 1 to 4:
  - the places are ordered by repository, then by `<kind>--<name>`,
    with a stable sort;
  - the streams are ordered by priority, then by name;
  - `waits_on` comes from `waiting`, a stage `WaitingForRoom` and
    `submitted`, in that order;
  - `kept_words` follows design 2's table, `kept holding work and a
    lock` among it;
  - a stream's line leaves out the session part on `NoPart`;
  - a stream whose multiplexer has no profile takes `terminals.rs`'s
    `uncarried` words, and no terminal is asked.
- **herdr's runs** (design 6) reads one `herdr session list --json`
  through `run`, bounded by `RUNS_BOUND` at one second. It finds the
  part by `name`, the own session by the profile's own name, and a
  socket-held own server by `socket_path`. It calls neither
  `socket_of` nor `start_session`. The module's header names it beside
  attach.
- **The printed command's runs** answers `NoPart` in attach's words,
  sharing one constant with attach.
- **`flume list`** is `Command::List`. It calls `build()?.list()` with
  no settle, and its help says what 4.3 asks.

## The gate

`sluice exec main`, run on 2026-11-06 at `bbab749` against main at
`24551f1`, names no file contested and runs the checks the head
affects: 24 tasks completed, 23 passed, none failed, and it exits 0.
sluice recorded the 23 results under their keys for `bbab749`.

- **root:** format, adrs and changes;
- **seams, fakes, core, conformance, flume:** format and lint;
- **tests:** seams 67, core 25, fakes 47, conformance's suites,
  flume:test's suites 160, 2, 45, 19, 148 and 2, and plugin-claude;
- **conformance:coverage:** every clause of Parts A and B, I1 to I8 and
  S1 to S108 but the skipped is cited at `7bd1891` by the suite's 568
  scenarios;
- **flume:tools:** 99 of 99, herdr's four runs scenarios and the
  walkthroughs among them;
- **site:** prose, 0 errors and 0 warnings, and build.

`root:specs` is not among the tasks the head affects, since the branch
leaves `openspec/specs/` as main has it. The trial archive above ran
its command, `openspec validate --specs --strict`, over the specs the
archive writes, and it passes 43 of 43.

## Beside the change in flight

On 2026-11-06, while this verify ran, the place of
`where-an-agent-opens-is-given-by-kind-and-by-stage` (its task 5.2)
sent clause 200 as amended at blueprints `fa40c94`: "The list gives a
line for each stream the host declares, with each of its sessions, its
own (107) and each it names for a kind or a stage (199), and whether
each runs now (201). The sessions are what a person opens, each by its
name (186)."

This change does not fall short on it:

- Its clause list reads `7bd1891`, and 199 is not built on main.
- Both changes' designs give the sessions a stream names for a kind or
  a stage to whichever lands second. That place's word is that its
  change builds them where this one lands first. It puts them on the
  stream's line, read in the one runs reading of each stream's
  terminal that the list makes, with a scenario citing 199 and 200.
- Design 9 gives each such session a line of its own and names
  `flume open <stream> --session <name>`. Clause 200 and clause 186 as
  amended at `fa40c94` (record 0048's amendment) supersede both. No
  spec, code, help or site page of this change uses either form, and
  design 9 is archived as this change's record. The statement binds
  that change's build, so design 9 is left as written.
