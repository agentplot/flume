# Verify: flume-list-shows-each-place-and-stream-on-one-line

Read on 2026-11-06 at `459e56f`, the head of the branch
`change/flume-list-shows-each-place-and-stream-on-one-line`. The check
covers the change's proposal, design, six delta specs and tasks. The
branch was cut from main at `045361c` and has not taken main since.
origin's main is now `24551f1`, which landed #112 to #117 after the
cut.

## Verdict

**Falls short; not archived.**

- Every task is ticked.
- Every requirement of the six delta specs is built, and each of their
  29 scenarios is held by a test that asserts its THEN.
- `sluice exec main` passes all 37 of its tasks at `459e56f`,
  flume:tools 97 of 97, but over the branch alone, since it does not
  merge with main.
- **The archive breaks the gate.** `the-list` is a new capability, and
  its delta carries no `## Purpose`. A trial archive therefore leaves
  `openspec validate --specs --strict`, which `root:specs` runs in
  `check-fast` on every branch, failing 41 of 42 on the placeholder
  Purpose that `openspec archive` writes.
- **The branch conflicts with main at `24551f1`** in three files. Once
  merged, its clause list also names as unbuilt three clauses and
  three scenarios that main has now built.

## Tasks

All 18 tasks of groups 1 to 5 are ticked, `[x]`, and none is open.
`openspec validate flume-list-shows-each-place-and-stream-on-one-line
--strict` passes.

- **Record 0049** is `accepted`, on the user's yes of 2026-11-05. It
  quotes their words of that day and cites the
  statement's amendment on agentplot/blueprints main at `7bd1891`
  (1.1).
- **core.52** is drawn after core.15 in `docs/architecture/core.md`,
  citing record 0049, and `site/.../architecture/rules.md` mirrors it
  (1.2).
- **The terminal seam's runs** is `Terminal::runs` with no default,
  answering `SessionRuns`, re-exported. Every terminal implements it:
  the fake, the printed command, herdr and each test's stand-in (2.1).
- **The core's list** is `crates/flume-core/src/list.rs`. It reads
  `store.read_every()` and each live start's asking mark, and asks
  each declared stream's terminal for its runs once. It calls none of
  `bound`, `facts`, `read_agent`, `place_clues` or `host_status`. It
  takes no lock and writes no document (3.1). `line.rs`'s `stage_at`
  and `state_words` are `pub(crate)` and give the list its stage and
  state.
- **Task 3.2:** `where-an-agent-opens-is-given-by-kind-and-by-stage`
  has still not landed on main at `24551f1`. Leaving its sessions to
  that change's build stands, as design 9 has it.
- **The clause list** reads blueprints at `7bd1891`. 200 is in Part A,
  201 is in Part B, and S108 is among the scenarios. Each is cited by
  this change's scenarios (5.2).
- **Each group's closing task** (1.3, 2.4, 3.4, 4.4, 5.3) names the
  checks it ran through sluice. The gate run below covers them all at
  the branch's head.

## Requirements and scenarios

Each of the six specs' 19 requirements is built, and each of their 29
scenarios is held by a test that asserts its THEN. Every test named
below was found by name at `459e56f`.

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
    harness's `runs_parts`, holds *Runs over the fake*. It checks
    `work` running, `desk` unread with `no listing`, `office` and the
    own part not running as `own`, and a log of only the four `runs`
    lines;
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
  - `clauses.rs`'s tests hold *The clauses listed* at `7bd1891`;
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
  - `kept_words` follows design 2's table;
  - a stream's line leaves out the session part on `NoPart`;
  - a stream whose multiplexer has no profile takes `terminals.rs`'s
    `uncarried` words, and no terminal is asked.
- **herdr's runs** (design 6) reads one `herdr session list --json`
  through `run`, bounded by `RUNS_BOUND` at one second. It finds the
  part by `name`, the own session by the profile's own name, and a
  socket-held own server by `socket_path`. It calls neither
  `socket_of` nor `start_session`. The module's header names it
  beside attach.
- **The printed command's runs** answers `NoPart` in attach's words,
  sharing one constant with attach.
- **`flume list`** is `Command::List`. It calls `build()?.list()` with
  no settle, and its help says what 4.3 asks.

## The gate

`sluice exec main`, run on 2026-11-06 at `459e56f` against main at
`24551f1`, first reports "would conflict:
crates/flume-conformance/src/clauses.rs, docs/architecture/core.md,
site/src/content/docs/reference/commands.md with main". It then runs
the checks the head affects over the branch alone: 38 tasks completed,
37 passed, none failed, and it exits 0. flume:tools passes 97 of 97,
herdr's four runs scenarios among them. flume:test's suites pass 37,
13, 140 and 2. site:prose reports 0 errors and 0 warnings. sluice
recorded nothing, because this report was in the worktree beyond the
head while the checks ran.

So the checks the tasks name pass at the branch's head. None of them
has run over the branch merged with main, which is what a submission
lands.

## What falls short

### 1. The archive leaves `root:specs` failing

`the-list` is a new capability. `openspec archive` writes a new
capability's `## Purpose` from the delta's own, and where the delta
has none it writes a placeholder. A trial archive at `459e56f`, in a
scratch worktree, applies all six deltas (+16 requirements, ~3
modified). Then `openspec validate --specs --strict` fails 41 of 42:

```text
✗ spec/the-list
  ⚠ [WARNING] overview: Purpose section is still a placeholder rather than
  a Purpose anyone wrote (the sentence `openspec archive` writes for a new
  capability, or a `TBD`/`TODO` marker left in its place).
```

`root:specs` runs that command in `check-fast`, which every branch's
gate requires, so archiving as the branch stands breaks the gate. This
is the shortfall `a-host-declares-where-its-kinds-come-from`'s first
verify found in `kind-sources` (`984665e`). That change fixed it in
`1be48f0` by giving the delta its `## Purpose`.

**The fix:** open `specs/the-list/spec.md` with a `## Purpose` of more
than 50 characters, citing 200 and 201: what the list is for, a line
for each place and each stream read from what flume keeps. Then run a
trial archive and `openspec validate --specs --strict` again.

### 2. main at `24551f1` conflicts, and its clause list moved

`git merge-tree --write-tree HEAD main` conflicts in three files, and
`sluice exec main` names the same three:

- **`crates/flume-conformance/src/clauses.rs`.** main's list reads
  blueprints at `ff616eb`. It has 191 in Part A and 192 in Part B,
  scenarios up to S97, and a `SCENARIO_GAP` of S92 to S94. The branch
  reads at `7bd1891`, with `SKIPPED` in place of `SCENARIO_GAP`.
  #114, `a-host-declares-where-its-kinds-come-from`, has landed. So
  191, 192 and S95 to S97, which the branch names as unbuilt by
  `SOURCES`, are now cited by main's scenarios. The coverage check
  fails naming any clause the list calls unbuilt that a scenario
  cites. A resolution that keeps the branch's unbuilt list therefore
  fails `conformance:coverage`. The merge should:
  - keep `7bd1891` and its `SKIPPED`;
  - take main's scenarios as built;
  - drop `SOURCES` and its entries;
  - keep the entries for records 0043 and 0044, whose changes have not
    landed.
- **`docs/architecture/core.md`.** main's #117 amended core.15's
  details, and the branch adds core.52 after core.15. Keep both.
- **`site/src/content/docs/reference/commands.md`.** "Settling first":
  main adds `flume kinds lock`, and the branch adds `flume list`. Keep
  both.

The other files both sides touched merge without conflict. Two need
the gate run again over the merged tree:

- main's #116 removed `could_not_run`, the exit 77, from the tools
  tests. The branch's new herdr tests do not call it, and they build
  on `SessionHome`, which #116 changed.
- main's #117 changed how status reads the places, side by side. The
  list calls none of status's reads.

Merged before the next verify, the archive then syncs over main's
specs as they stand. main's archive of
`a-host-declares-where-its-kinds-come-from` synced `seam-contracts`,
`seam-fakes` and `clause-coverage`, which this change's deltas also
touch.

**The fix:** merge main into the branch, resolve as above, and run the
gate over the merged tree.
