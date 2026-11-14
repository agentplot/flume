# Verify

The verify stage read `change/a-stage-delivers-what-an-earlier-start-committed`
at `9d9fac79` on 2026-11-14, against its base
`bolt/flumes-places-need-no-hand` at `f7a8837`. The whole checks pass,
every task is ticked, and the work meets every requirement and scenario
of the three delta specs. Archived, the specs pass `openspec validate
--specs --strict`. The change is archived with this report.

## The whole checks

`sluice exec bolt/flumes-places-need-no-hand` passed: 23 tasks completed
and 22 check results passed, recorded under their keys for `9d9fac7`, in
44s. The tasks the head affects ran: `seams`, `core` and `conformance`
format, lint and test; `fakes` lint and test; `flume` lint and test;
`conformance:coverage`; `plugin-claude:test`; `nix:typecheck` and
`nix:test`; `site:install`, `site:prose` and `site:build`; and
`root:format`, `root:adrs` and `root:changes`. `conformance:test`'s
`the_engine_passes_its_scenarios` passed, the new scenarios among it.
`site:prose` gave 0 errors and 0 warnings.

The run went through the host's sluice, 0.93.2, not the tree's devenv
sluice, 0.71.0. Run first, `devenv shell -- sluice exec
bolt/flumes-places-need-no-hand` exited 3 before running a check: it
cannot read the host's `~/.config/sluice/config.toml`, whose `[forge]
credential` key it does not know ("unknown field `credential`, expected
one of `app_id`, `key`, `discovery`"). The policy's `toolchain`, `devenv
print-dev-env`, runs every check in the tree's devenv environment either
way, so the tools the tree pins ran each one (core.22). The refusal is
the host's configuration and the tree's sluice pin standing apart, not
this change's work.

## The specs archived

`root:specs` reads `openspec/specs/`, which the branch leaves alone until
the archive, so the gate does not run it. In a scratch copy of
`openspec/`, the pinned OpenSpec, 1.14.3, found the change valid under
`openspec validate a-stage-delivers-what-an-earlier-start-committed
--strict`. `openspec archive a-stage-delivers-what-an-earlier-start-committed
--yes` then applied 3 added and 2 modified requirements, and `openspec
validate --specs --strict --no-interactive` passed 49 of 49.

Each modified requirement keeps every scenario `openspec/specs/` holds
for it. "A stage ends at its deliverable, read at one head" and "A
repetition delivers its group and the stage delivers every task" stand
as `openspec/specs/` holds them. Their "since the stage started" and
"since it started" are read through the added requirements: the first of
the stage's starts since it last delivered, and the first start given a
repetition's group.

## Tasks

Every task of `tasks.md`, 1.1 to 5.1, is ticked, and each is met on the
branch:

- 1.1: `flume_seams::Counted`, `StageHeld::counts`, `StartHeld::counts`
  and `StartHeld::counted()` in `crates/flume-seams/src/held.rs`, each
  count left out when none and read as none in a document written
  before; `a_start_written_before_counts_from_itself` in
  `crates/flume-seams/src/lib.rs`; the round trip carries both counts.
- 2.1: `carried_count` at the start's claim in
  `crates/flume-core/src/start.rs` begins the stage's count where it has
  none, carries the last start's for a repetition given the same group
  by number and title, and the stage's otherwise; `ended` lets the
  stage's count go on `Delivered` and `Finished` alone; `delivered`,
  `made_since` and `unmade` read from `counted()` in
  `crates/flume-core/src/reading.rs`.
- 2.2: `fix_round` lets go of its target stage's count, for a failed
  submission and a finding alike, and leaves every later stage's
  (`crates/flume-core/src/submit.rs`).
- 2.3, 3.1 and 3.2: the stage's end carries its lines through
  `lines_from` and `stage_made`, read before a delivery lets the count
  go; no start writes `StageHeld::made_from`, and a document written
  before has its `made_from` read and let go. Each scenario the tasks
  name is in the suite.
- 4.1: core.9's details rule out and allow what 25 now says, citing 25,
  49, 112, 140, 146, S52, S131 and record 0065.
- 4.2: the guide to kinds says when a stage started for its deliverable,
  naming no tool or word of ours as the reader's (core.35).
- 5.1: stage-runs' count stands as two added requirements and
  repeating-stages' as one, each with the scenarios that walk it;
  `root:changes` passed, and the archived specs validate as above.

## Requirements and scenarios

| Spec | Requirement | Scenario | Conformance scenario |
|---|---|---|---|
| stage-runs | A stage counts from the first of its starts since it last delivered | An answered stop-short delivers what its start committed | end: an answered stop-short delivers what its start committed, and the place is submitted (S131) |
| stage-runs | A stage counts from the first of its starts since it last delivered | The start after a spent account counts the spent agent's commit | end: the start after a spent account counts the spent agent's commit |
| stage-runs | A stage counts from the first of its starts since it last delivered | A report written by an earlier start | end: a report an earlier start wrote delivers its stage at the retry |
| stage-runs | A stage counts from the first of its starts since it last delivered | A going-back past a stage leaves its count | end: a retry of construct leaves code's count, and code's next agent delivers on its first start's commit |
| stage-runs | A fix round and a delivered stage's retry start the stage afresh | A fix round counts only what is made after it | end: S131, a fix round whose agent makes no commit is not delivered, and the caller's retry of the submission submits the same head with no agent started |
| stage-runs | A fix round and a delivered stage's retry start the stage afresh | A retry of a stage that has delivered starts afresh | end: a retry of a stage that has delivered starts afresh, and is not delivered with no commit of its own |
| stage-runs | A fix round and a delivered stage's retry start the stage afresh | A stage written before counts from its next start | end: a stage written before it kept its count counts from its next start, as flume did |
| made-things | A declared thing with no line is not yet made | A line an earlier start wrote | made things: a line an earlier start wrote counts for its retry, and the stage's end carries it |
| repeating-stages | A repetition counts from the first start given its group | The last group ticked before a stop-short | repeating stages: a retry after the last group was ticked and the repetition stopped short starts with no group, and the stage delivers on the commit before it |
| repeating-stages | A repetition counts from the first start given its group | A repetition given its group again | repeating stages: a repetition given its group again counts the line the first start given that group wrote |

Each conformance scenario asserts what its spec scenario's THEN says:
the stage's state, the head it delivered at, the start its count came
from, the stop check's words, the made lines its end carries, and the
merge queue's submissions. The scenarios the modified requirements keep,
"Commits and no line", "The line written" and "A retry after a
stop-short", passed as before. The new scenarios cite the clauses the
suite's list holds and name S131 in their words, as design 8 says, and
`conformance:coverage` passed.

The added sentence of "A retry starts the unticked group again", a retry
given its last start's group counting from the first start given it, is
met by the "repetition given its group again" scenario, whose retry
counts from start 2. The added requirement's "a start with no group
SHALL deliver as its stage does" is `carried_count`'s carrying of the
stage's count to a start given no group, which "the last group ticked
before a stop-short" walks.

## Outside this change

The tree's sluice, 0.71.0, cannot read the host's
`~/.config/sluice/config.toml` because of its `[forge] credential` key,
so `devenv shell -- sluice exec <base>` runs no check on this host. The
checks section gives the run that stood in for it.
