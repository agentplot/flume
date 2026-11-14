# Verify

The verify stage read `change/a-stage-delivers-what-an-earlier-start-committed`
at `1bb4d2c8` on 2026-11-14, against its base
`bolt/flumes-places-need-no-hand`. The work holds: the whole checks
pass, every task is ticked, and every requirement and scenario of the
three delta specs is met.

## The whole checks

`sluice exec bolt/flumes-places-need-no-hand` passed: 23 tasks completed
and 22 check results passed, recorded under their keys for `1bb4d2c`, in
1m 55s. The tasks the head affects ran: `seams`, `core`, `fakes`,
`conformance` and `flume` format, lint and test; `conformance:coverage`;
`plugin-claude:test`; `nix:typecheck` and `nix:test`; `site:install`,
`site:prose` and `site:build`; and `root:format`, `root:adrs` and
`root:changes`. `conformance:test`'s `the_engine_passes_its_scenarios`
passed, the new scenarios among it.

The run went through the host's sluice, 0.93.2, not the tree's devenv
sluice, 0.71.0. The devenv sluice refused before running a check: it
cannot read the host's `~/.config/sluice/config.toml`, whose `[forge]
credential` key, written on 2026-11-14 for the host's sluice, it does not
know. The policy's `toolchain`, `devenv print-dev-env`, runs every check
in the tree's devenv environment either way, so the tools the tree pins
ran each one (core.22). The devenv sluice's refusal is the host's
configuration and the tree's sluice pin standing apart, not this change's
work.

## Tasks

Every task of `tasks.md`, 1.1 to 4.2, is ticked, and each is met on the
branch:

- 1.1: `flume_seams::Counted`, `StageHeld::counts`, `StartHeld::counts`
  and `StartHeld::counted()` in `crates/flume-seams/src/held.rs`, each
  count left out when none; `a_start_written_before_counts_from_itself`
  in `crates/flume-seams/src/lib.rs`; the round trip carries both counts.
- 2.1: `carried_count` at the start's claim in
  `crates/flume-core/src/start.rs`; `ended` lets the stage's count go on
  `Delivered` and `Finished` alone; `delivered`, `made_since` and
  `unmade` read from `counted()` in `crates/flume-core/src/reading.rs`.
- 2.2: `fix_round` lets go of its target stage's count and leaves every
  later stage's (`crates/flume-core/src/submit.rs`).
- 2.3, 3.1 and 3.2: the stage's end carries its lines through
  `lines_from` and `stage_made`, no start writes `StageHeld::made_from`,
  and a document written before has its `made_from` read and let go; each
  scenario the tasks name is in the suite.
- 4.1: core.9's details rule out and allow what 25 now says, citing 25,
  49, 112, 140, 146, S52, S131 and record 0065.
- 4.2: the guide to kinds says when a stage started for its deliverable,
  naming no tool or word of ours as the reader's (core.35).

## Requirements and scenarios

Each delta requirement names one in `openspec/specs/`, and each keeps
every scenario the spec holds and adds its own.

| Spec | Scenario | Conformance scenario |
|---|---|---|
| stage-runs | An answered stop-short delivers what its start committed | end: an answered stop-short delivers what its start committed, and the place is submitted (S131) |
| stage-runs | A fix round counts only what is made after it | end: S131, a fix round whose agent makes no commit is not delivered, and the caller's retry of the submission submits the same head with no agent started |
| stage-runs | A retry of a stage that has delivered starts afresh | end: a retry of a stage that has delivered starts afresh, and is not delivered with no commit of its own |
| stage-runs | The start after a spent account counts the spent agent's commit | end: the start after a spent account counts the spent agent's commit |
| stage-runs | A report written by an earlier start | end: a report an earlier start wrote delivers its stage at the retry |
| stage-runs | A going-back past a stage leaves its count | end: a retry of construct leaves code's count, and code's next agent delivers on its first start's commit |
| stage-runs | A stage written before counts from its next start | end: a stage written before it kept its count counts from its next start, as flume did |
| made-things | A line an earlier start wrote | made things: a line an earlier start wrote counts for its retry, and the stage's end carries it |
| repeating-stages | The last group ticked before a stop-short | repeating stages: a retry after the last group was ticked and the repetition stopped short starts with no group, and the stage delivers on the commit before it |
| repeating-stages | A repetition given its group again | repeating stages: a repetition given its group again counts the line the first start given that group wrote |

Each conformance scenario asserts what its spec scenario's THEN says:
the stage's state, the head it delivered at, the start its count came
from, the stop check's words, the made lines its end carries, and the
merge queue's submissions. The scenarios the delta keeps unchanged
passed as before. The new scenarios cite the clauses the suite's list
holds and name S131 in their words, as design 8 says, and
`conformance:coverage` passed.

The added sentence of "A retry starts the unticked group again", a retry
given its last start's group counting from the first start given it, is
met by the "repetition given its group again" scenario, whose retry
counts from start 2.

## Findings

None for the code stage. The host's sluice configuration and the tree's
sluice pin stand apart, as the checks section says; that is outside this
change.
