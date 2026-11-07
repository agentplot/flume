# Design

## Context

flume's own kinds are data the kinds profile reads: each version a
declaration of its own under `crates/flume/src/profiles/kinds/weir/own/<name>@<version>/`,
`kind.toml` with its prompts beside it under `prompts/`, built into the
binary by the `OWN` table in `crates/flume/src/profiles/kinds/weir/mod.rs`
and read through weir-kinds' package reader at its identity,
`flume/<name>@<version>`, with its content hash (156, 157; core.4,
core.34). Main holds change@1 (its code stage one agent), change@2 (its
code stage repeating per task group, record 0019), fix@1 and chore@1. A
launch naming no version takes, among flume's own, the newest a lock
records, else the newest flume holds: `own.last()` in
`crates/flume-core/src/launch.rs` over the versions as the profile lists
them (154, 155). A place keeps the kind its launch read (21).

Record 0054 (accepted by the user 2026-11-06, "yes to 54") adds change@3:
each task checked by its own narrowest check, one agent coding the whole
change, and the repository's whole checks run once, at verify. It amends
record 0019 in part, and record 0022 where it names change@2 the newest;
both are marked so. The statement carries it at agentplot/blueprints
49b9d53, accepted at 9371cda: 22 (flume's own kinds and what change@3's
stages deliver), 154 and 157 (change@1, change@2 and change@3 side by
side), C.2's list and its kinds table (change@3 the newest), and S119.

The rules this design relies on are core.4 (the format is weir's, and
flume defines none of it), core.10 (one fresh agent per start), core.31
(change@2's repeating prompt order, which change@3 leaves as it is),
core.33 (a prompt holds the unit's facts and then its run's), core.34 (a
kind's name and version resolve as 154 orders; flume's own run with no
lock), core.41 (simple kinds), core.19 (the statement first) and
core.35 (the site speaks to any host's operator). The one rules page is
`docs/architecture/core.md`; this change touches no rule and no area
another page would own. Where flume has no rule, the kit standard's code
profile binds: kit:specs.3 (each task names the check that proves it)
and kit:testing.5 (a change is proved by the repository's checks).

## Goals / Non-Goals

**Goals:**

- flume's own change@3, built in beside change@1 and change@2, the
  newest, so a bare `change` runs it where no lock records a version
  (22, 154, 157, C.2).
- change@3's construct gives each task the narrowest check that shows it
  done, its code stage is one agent for the whole change, and its verify
  runs the repository's whole checks once (22, C.2, S119).
- This repository's planning rules ask the same of every change written
  here (record 0054's consequences).

**Non-Goals:**

- Changing change@1 or change@2: a version keeps its content in every
  later release, so a change to what one holds is a new version (156).
- Removing `repeat` from the format or from flume: it stays weir's, for
  a repository's own kinds and for change@2 (140 to 142; core.4).
- Standing instructions for change@3's stages: flume's own kinds give
  none until record 0021's change builds 148, 149 and 152, which the
  clause list names unbuilt by it. change@3's prompts hold everything,
  as change@1's and change@2's do.
- Reading or enforcing the checks a change's tasks name: flume reads a
  change's tasks ticked and nothing of their text (86, C.3).
- Other repositories' planning rules (sluice's `openspec/config.yaml`
  among them): record 0054 asks them too, and each is that repository's
  own change.

## Decisions

### 1. change@3 is a declaration of its own beside change@1 and change@2

`own/change@3/kind.toml` declares `change` at version 3 with
`branch = "change"`, `starts_unready = false` and `lands = true`, and
three stages, each `agents = [{}]`: `construct` delivering `change` from
`prompts/change-construct.md`, `code` delivering `tasks` from
`prompts/change-code.md` with no `repeat`, and `verify` delivering
`archived` from `prompts/change-verify.md` with `on_fail = "code"`. Its
header comment is change@2's, naming change@3 and record 0054. The stage
names are change@1's and change@2's, so settings a repository or a host
gives `change`'s stages by name join change@3 as they join the others
(32, 158, C.2).

`OWN` gains `flume/change@3` with its declaration and its three prompts,
listed after change@2, so the versions of `change` stay in ascending
order and `own.last()` takes change@3 as the newest (154). The module's
doc comment names change@1, change@2 and change@3 side by side.

Path not taken: change@3 reusing change@1's code prompt and change@2's
construct and verify by reference. A package refers to nothing outside
itself (157), so each version carries its own.

### 2. change@3's prompts

Each prompt states the place's facts, then what the stage does, then the
way to stop short, and puts the run's words last, in a `{{#words}}`
section at the end, so a retry's prompt shares every character before
the words with the first start's (core.33). change@1's code prompt puts
its words first; change@3 does not copy that. No prompt holds the words
`submit`, `push`, `land the`, `wt ` or, outside a conflict's words,
`merge`, and each names a rebase only to rule it out (28, 71, 112;
core.5). The texts, each file ending in one newline:

`prompts/change-construct.md`:

```
You are the construct stage of the change {{name}}, working in the place {{place}} on the branch {{branch}}, which was cut from {{base}}.

{{brief}}

Write the change at {{change}} from the brief: its proposal, its specs, its design and its tasks, each task a checkbox line. Write no code of the change: a later stage builds it, one agent working through every task in order.

Give each task the narrowest check that shows that task done, named so it runs as written: one test by name, one package's tests, or a lint of the files the task touches. No task runs the repository's whole checks, and no task checks a task group or every part the change touches: the task groups are headings for reading, and the verify stage runs the repository's whole checks once, after every task is done.

Commit the change on {{branch}}. Your deliverable is the change written: the stage is done once a commit on {{branch}} holds the change with its planning complete and you are quiet.

If you cannot go on without an answer, stop short: {{needs}}
{{#words}}
You are started again with these words:

{{words}}
{{/words}}
```

`prompts/change-code.md`:

```
You are the code stage of the change {{name}}, working in the place {{place}} on the branch {{branch}}, which was cut from {{base}}. You build the whole change, every task of it.

Read the change at {{change}}, its proposal, design, specs and tasks. Work through its tasks in order, the task groups only headings: make each task true, run the check it names as it names it, and tick the task in the change's tasks file once its check passes. Commit as you go on {{branch}}. Run no check but a task's own: not the repository's whole checks, not a task group's and not every part the change touches. The verify stage runs the repository's whole checks once, after you. Leave every heading of the tasks file as it is.

Your deliverable is the tasks ticked: the stage is done once a commit on {{branch}} holds every task of the change ticked and you are quiet. Leave the branch where your last commit puts it; what happens to it after this stage is not yours to do.

If you cannot go on without an answer, stop short: {{needs}}
{{#words}}
This stage was started again with these words:

{{words}}

Where the words name files contested and the base's revision the merge queue tested against, take that revision in before anything else: run `git merge <the revision the words name>` in this place, which merges it into {{branch}}, resolve each of the files contested, and commit the merge. Never rebase, and never rewrite a commit already on {{branch}}.

Where the change has been archived on {{branch}}, move it back out of the archive to {{change}} first. Then fix what the words name, and record the fix as a task of the change naming its own check, ticked once that check passes.
{{/words}}
```

`prompts/change-verify.md`:

```
You are the verify stage of the change {{name}}, working in the place {{place}} on the branch {{branch}}, which was cut from {{base}}.

First run the repository's whole checks once, on {{branch}} as it stands, as the repository's instructions name them. Fix nothing yourself and run them no second time: what fails is the code stage's to fix.

Then check the work on {{branch}} against the change at {{change}}: every requirement and scenario of its specs met, and every task ticked done. Write what you find as a report, `verify.md` in the change's directory, {{change}}/verify.md, the whole checks' outcome among it, and commit it on {{branch}}.

Where the whole checks pass and the work holds, archive the change on {{branch}}, the report with it, and commit. Your deliverable is the change archived: the stage is done once a commit on {{branch}} holds the change archived and you are quiet.

Where the whole checks fail or the work falls short, archive nothing: stop short having found work, naming what fails, so the change goes back to its code stage with your words. {{needs}}
{{#words}}
You are started again with these words:

{{words}}
{{/words}}
```

The verify prompt names no tool and no command, since flume's own kinds
serve any host's repository: "the repository's whole checks, as the
repository's instructions name them" is `sluice exec main` here, by
`CLAUDE.md`, and another repository's own there. It runs them before
checking the specs, so a failure is in the report
whatever the specs' reading finds, and runs them once, since a second
run waits for room again (record 0054).

Paths not taken: verify running each task's check again, which the
whole checks hold; verify fixing what the whole checks find, which is
the fix round's, so the code stage gets verify's words (131); a code
prompt that tells the agent to compact or to hand over between groups,
which record 0054 leaves to the change's size ("Changes are written
small enough for one agent to code").

### 3. The newest, and the places already launched

Nothing in the core changes: `own.last()` is change@3 once `OWN` lists
it after change@2, and a repository whose lock records `flume/change@1`
or `flume/change@2` keeps launching that version with a bare `change`
(154, 155). A place launched at change@2 keeps its held kind, its
repeating code stage among it, when a flume holding change@3 settles it
(21). The kinds profile's test of this is change@1's
(`a_place_launched_at_change_1_keeps_it`) made for change@2: launched by
a flume holding change@1 and change@2, settled by one holding all three.

### 4. Content never changes: the hashes

`OWN_HASHES` in `crates/flume/tests/profiles/built_in.rs` gains
`flume/change@3` with the hash its content gives once its files are
written, and every other entry stays as it is, so the check fails were a
byte of change@1's or change@2's to change (156). change@3's hash is
recorded from the test's own failure message the first time it runs,
and the files are not changed after.

### 5. The tests that named change@2 the newest

The fake harness's `flumes_own()` in `crates/flume-conformance/src/fake_harness.rs`
gains change@3: change@1's stages at version 3, with a code prompt of
its own, after change@2. Every engine scenario that launches a bare
`change` and reads a repeating code stage, a group, `code 1/2` or a
repetition's notes then names `change@2` in its launch; one that reads
nothing of repetition keeps its bare `change`, now running change@3. The
scenarios of `engine/launching.rs` read the newest as change@3: no
version runs change@3, not repeating; `change@2` named runs change@2,
repeating; `change@4` is refused naming versions 1, 2 and 3. The kinds
seam's `flumes_own` scenario reads fix@1, chore@1, change@1, change@2 and
change@3, change@2's code alone repeating.

The walkthroughs in `crates/flume/tests/profiles/tools/walkthroughs.rs`
that read a repeating stage, `a_change_code_agents_line_names_its_group_its_run_and_its_tasks`,
`a_change_of_three_task_groups_takes_three_agents_in_turn` and
`a_failed_submission_sends_a_repeating_stage_back_for_one_repetition_with_no_group`,
launch `change@2`. `a_change_runs_construct_code_and_verify` keeps its
bare `change` and reads that the place runs `flume/change@3` with one
start of its code stage, so a change carried to its landing by the
`check-tools` layer is the newest (S2, S35, S119).

### 6. S119 in the engine

`engine/running_through.rs` gains a scenario citing 22, 154 and S119: a
bare `change` in a repository that locks nothing runs change@3, status
naming `flume/change@3`; the fake construct writes a change of four task
groups; the code stage starts once, its status `code` with no group
count and its line's stage `code`; the one agent ticks every task and
goes quiet; verify starts, delivers the change archived, and the place
is submitted, its code stage having had one start. The scenario reads
no prompt text: the prompts' content is the kinds profile's tests'
(decision 2), which render each prompt for a place and assert what it
asks.

### 7. The clause list

`crates/flume-conformance/src/clauses.rs` reads agentplot/blueprints
main at 9371cda, which holds record 0054's acceptance, or the later
revision of main the build reads. Since the list's c94c496 that
statement added 193, 202 to 206, 208 and 209, and S93, S94, S98, S99,
S109 to S112, S118 and S119: `SCENARIOS` becomes the last scenario the
revision holds, `SKIPPED` holds only the numbers that revision still
skips, each new clause goes into Part A or Part B as the revision
places it, and each new clause and scenario but S119 is named unbuilt
by the change that builds it, by its record where the change has no
name yet (record 0039's, record 0051's, record 0053's), as the list
names record 0021's. S119 is cited by decision 6's scenario and is not
unbuilt. The list's own tests are updated to the revision's counts.

### 8. This repository's planning rules

`openspec/config.yaml`'s `rules.tasks` replaces its first rule and its
last with:

- "Each task says what must be true and names the narrowest check that
  shows it done, written so it runs as named: one test by name,
  `devenv shell -- moon run <project>:test -- <test name>`, one
  package's tests, or the formatter or linter the gate runs given only
  the files the task touched (kit:specs.3; record 0054). No task is a
  step done by hand, a deploy or work after the landing."
- "No task runs the repository's whole checks, `sluice exec main` or a
  layer's tag, and no task checks a task group or every project the
  change touches: the task groups are headings for reading, a change's
  verify stage runs the whole checks once, and sluice runs them again
  at the merge (record 0054)."

The three rules between them stay as they are. `CLAUDE.md`'s paragraph
under "Checks" says `sluice exec main` runs the gate's checks, which a
change's verify stage runs once and no task of a change runs (record
0054), and that a task's own check runs as the task names it, beside
`cargo check` and one unit test, which are fine to run directly.

Path not taken: leaving `CLAUDE.md` as it stands, which tells every
agent to run `sluice exec main` once its work is committed, and would
have change@3's code agent run the whole checks its prompt forbids.

### 9. The site

`site/src/content/docs/guides/kinds.mdx` lists flume's own kinds as
`flume/fix@1`, `flume/chore@1`, `flume/change@1`, `flume/change@2` and
`flume/change@3`, says a bare `change` takes the newest, `change@3`,
unless a lock records another, and says in a sentence each how the three
versions code a change: change@1 with one agent, change@2 with a fresh
agent for each task group, and change@3 with one agent checking each
task by its own check and the repository's whole checks run once at
verify (core.35). The examples elsewhere that show a change@2 place's
line, `code 1/2`, stay: they show a repeating stage's line.

## Risks / Trade-offs

- [One agent may fill its conversation on a long change and compact,
  losing the detail record 0019 kept] → Changes are written small
  enough for one agent to code (record 0054); a long one can still be
  launched as `change@2`.
- [A task's narrow check misses what another task broke] → verify runs
  the whole checks once, and the merge queue's gate runs them again,
  so nothing lands unchecked; the cost is a later finding, sent back to
  code with verify's words (131).
- [A construct names a check that does not run as written] → the code
  agent stops short or fixes the task's check as an open detail; the
  config's apply guidance already says to tick a task only once its
  check passes.
- [The place that builds this change runs the version it was launched
  at, change@2 where its launch named none, whose verify runs the checks
  a change names] → this change names only narrow checks, so that verify
  runs those, and the merge queue's gate runs the whole checks.
