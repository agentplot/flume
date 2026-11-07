# Verify: a-found-at-its-kinds-own-fix-round-waits-for-an-answer

The verify stage's report on
`change/a-found-at-its-kinds-own-fix-round-waits-for-an-answer` at
dc584c4, against the change's proposal, design, delta specs and tasks.

## Verdict

The work falls short, and the change is not archived. The code, the
tests, the record, the rule and the site hold to the change, every
task is ticked, and the change's three delta specs are each met by a
test. But the change amends one requirement of the specs and leaves a
second, which says the opposite, as it stands: `stage-runs`'s "A
stop-short ends the stage short" still says found work from the kind's
last stage starts the kind's fix round, with no word of where the fix
round stands. Archived, `openspec/specs` would hold that requirement
beside `running-through`'s amended one, and the two disagree for every
fix and chore. The change goes back to its code stage to carry
`stage-runs` in its deltas.

## The base and the statement

- The branch is cut from `bolt/a-fix-round-that-finds-work-asks` at
  376c17c, which a fetch of origin reads unmoved. Its eleven commits
  are the change's own.
- The statement on agentplot/blueprints at 734d60c, read for this
  report, says what the change builds: 41 calls a found work a stage
  before the agent's own must do again; 131 starts the fix round on a
  found from the kind's last stage only where the fix round is a stage
  before the one that found, and takes found work from any other stage,
  and from a last stage that is its kind's fix round, as needing an
  answer (42); 48 names a found beside a failed submission as what
  sends a place back; 49 carries the question before the answer.
- Record 0011's amendment of 2026-11-06, the design agent's, decides
  it, with its five options, and core.27 takes it. The site's
  `architecture/rules.md` gives core.27 word for word as `core.md` does.

## What fails

`openspec/specs/stage-runs/spec.md`, "A stop-short ends the stage
short", reads:

> The core SHALL take a stop-short only under the identity of the
> place's current stage's live agent and that start's secret, saying
> whether it needs an answer or found work, SHALL end the stage short
> with the agent's words whole and its kind, and SHALL start no next
> stage, except that found work from the kind's last stage starts the
> kind's fix round (41, 42, 131).

After this change, found work from fix's code, the kind's last stage
and its fix round both, starts nothing: the place waits on an answer,
as `running-through`'s amended "Found work no stage before it can do
again needs an answer" and the code say. The `stage-runs` requirement's
exception says the opposite for every fix, every chore and every
repository's kind whose last stage names itself or none. The change's
proposal lists `running-through`, `agent-profile` and `command-line`
as its modified capabilities and carries no `stage-runs` delta, though
the code stage amended the same sentence in `needs`' documentation in
`crates/flume-core/src/agent_acts.rs` ("where the fix round is a stage
before it; found work from any other stage, and from a last stage that
is its kind's own fix round, is taken as needing an answer").

`openspec validate --strict` cannot see it: each spec is valid on its
own. The repository's rule is that a change carries every requirement
its work changes in its own branch, so the specs reconcile.

What the code stage must do: add a `stage-runs` delta modifying "A
stop-short ends the stage short", its exception narrowed as 131 now
reads (the fix round started only where it is a stage before the last
stage that found; found work otherwise taken as needing an answer),
its two scenarios kept; name `stage-runs` among the proposal's
modified capabilities; and validate and trial-archive again. No code
changes.

The other specs that speak of a found were read against the change
and hold: `caller-waits`, `settling`, `place-status`, `seam-contracts`,
`kinds-profile`, `carried-conversations` and `command-line`'s existing
requirement say nothing a found at its kind's own fix round
contradicts.

## The specs, scenario by scenario

### running-through

| Scenario | Held by |
|---|---|
| The fix round: A blamed test, then a landing | `settling.rs`'s "a change's failed submission sends it back to code with the words, verify runs again, then it lands", unchanged |
| Verify finds work | `verify_finds_work`, now citing 48: code starts at once with the findings, verify runs again |
| A fix's failed submission | `a_fixs_failed_submission`: code starts again with a fresh agent (start 2) and the words, two agents opened, goings-back one, no move waited on, and a second submission once code delivers |
| Construct says it found work | `construct_finds_work`, unchanged |
| A fix's code finds work | `a_fixs_code_finds_work`: one agent opened, code ended short with `found_work`, goings-back none, the move a retry of code for an answer with the findings, the line `waiting on an answer`, the end `code` ended short found work with the findings, `fix--one code found work` delivered to `ops`, and nothing moved by a later settle |
| A last stage that names no stage for a failed submission | `a_last_stage_naming_none_finds_work`: a kind `audit` of one stage `review`, no agent started, the move a retry of review for an answer with the words, goings-back none |
| The retry that answers a fix's found | `the_retry_that_answers_a_fixs_found`: code starts once more (start 2), its prompt holding the findings before the answer, goings-back none, one submission, the place landed and tidied |

The walkthrough `a_fixs_code_finding_work_waits_on_an_answer_and_its_retry_lands`
in `crates/flume/tests/profiles/tools/walkthroughs.rs` holds the same
course under the stand-in for Claude Code: one start, the move and the
line as above, `flume retry --stage code --words` starting code again
with the findings before the answer, and the fix landing on the remote.

### agent-profile

"The words a fix's code is told" is held by
`the_words_a_fixs_code_is_told` in
`crates/flume/tests/profiles/agent_claude.rs`: the text for a fix's
code agent and a change's verify agent is one, design 5's words, with
no "cannot deliver past" and nothing of either identity.

### command-line

"needs' help" is held by `needs_help` in `crates/flume/tests/commands.rs`:
`flume needs --help` gives `--found` as "The stage found work a stage
before its own must do again", and no "cannot deliver past".

## The design, read against the code

- Design 1: `fix_round_of` in `crates/flume-core/src/submit.rs` is the
  one reading of the kind's fix round. `fix_round` takes its target
  from it, and `waiting_on`'s past-bound branch in `status.rs` its
  stage.
- Design 2: `sent_back_by` holds only where `fix_round_of` is a stage
  before the one that found. `found_work` returns at once otherwise,
  leaving the agent as any stop-short's.
- Design 3: `waiting_on`'s `EndedShort` branch answers nothing only
  where `sent_back_by` holds, the same reading the settle takes, and a
  retry for an answer with the words otherwise. A place left past its
  bound by a found from its own fix round before this change still
  reads `PastBound` on its fix round.
- Design 5: Claude Code's `stop_short`, `--found`'s help, `StopKind`'s,
  `StopShort`'s and `Waits::Answer`'s documentation and the fake's
  words say a found is work a stage before the agent's own must do
  again. "cannot deliver past" stands only in fix@1's and chore@1's
  code prompts, kept as 156 and core.34 have them, and in the record's
  and rule's account of the old words.

## Tasks

All thirteen tasks of groups 1 to 4 are ticked. None of them names
`stage-runs`, which is the gap above.

## The checks

- `openspec validate a-found-at-its-kinds-own-fix-round-waits-for-an-answer --strict`
  passes.
- A trial archive on a copy of `openspec/` applies the three deltas,
  and `openspec validate --specs --strict` passes 44 of 44 there.
- `sluice exec main` at dc584c4 passes 23 of 23 tasks: root:format,
  root:adrs, root:changes, seams', fakes', core's, conformance's and
  flume's format, lint and test, conformance:coverage, plugin-claude:test,
  site:prose, site:build and flume:tools, 109 of 109, the new
  walkthrough among them.

The checks pass, and none of them reads one spec against another, so
the gap above is the report's alone.
