# Verify: a-found-at-its-kinds-own-fix-round-waits-for-an-answer

The verify stage's report on
`change/a-found-at-its-kinds-own-fix-round-waits-for-an-answer` at
af3658f, against the change's proposal, design, delta specs and tasks.

## Verdict

The work holds, and the change is archived with this report. Every
requirement and scenario of its four delta specs is met, by a test
where it names behaviour and by the trial archive where it names the
specs. All fourteen tasks are ticked. The checks the tasks name pass.

## The base and the statement

- The branch is cut from `bolt/a-fix-round-that-finds-work-asks` at
  376c17c, which a fetch of origin reads unmoved. Its thirteen commits
  are the change's own.
- The statement on agentplot/blueprints at 734d60c, read for this
  report, says what the change builds. 41 calls a found work a stage
  before the agent's own must do again. 131 starts the fix round on a
  found from the kind's last stage only where the fix round is a stage
  before the one that found, and takes found work from any other
  stage, and from a last stage that is its kind's fix round, as needing
  an answer (42). 48 names a found beside a failed submission as what
  sends a place back. 49 carries the question before the answer.
- Record 0011's amendment of 2026-11-06, the design agent's, decides
  it with its five options, and core.27 takes it. The site's
  `architecture/rules.md` gives core.27 word for word as `core.md`
  does.

## The specs, scenario by scenario

### running-through

| Scenario | Held by |
|---|---|
| The fix round: A blamed test, then a landing | `settling.rs`'s "a change's failed submission sends it back to code with the words, verify runs again, then it lands", unchanged |
| Verify finds work | `verify_finds_work`, now citing 48: code starts at once with the findings, and verify runs again |
| A fix's failed submission | `a_fixs_failed_submission`: code starts again with a fresh agent (start 2) and the words, two agents opened, goings-back one, no move waited on, and a second submission once code delivers |
| Construct says it found work | `construct_finds_work`, unchanged |
| A fix's code finds work | `a_fixs_code_finds_work`: one agent opened, code ended short with `found_work`, goings-back none, the move a retry of code for an answer with the findings, the line `waiting on an answer`, the end `code` ended short having found work with the findings, `fix--one code found work` delivered to `ops`, and nothing moved by a later settle |
| A last stage that names no stage for a failed submission | `a_last_stage_naming_none_finds_work`: a kind `audit` of one stage `review` whose `send_back_to` is none, one agent opened, the move a retry of review for an answer with the words, goings-back none |
| The retry that answers a fix's found | `the_retry_that_answers_a_fixs_found`: code starts once more (start 2), its prompt holding the findings before the answer, goings-back none, one submission, the place landed and tidied |

The walkthrough `a_fixs_code_finding_work_waits_on_an_answer_and_its_retry_lands`
in `crates/flume/tests/profiles/tools/walkthroughs.rs` holds the same
course under the stand-in for Claude Code: one start, the move and the
line as above, `flume retry --stage code --words` starting code again
with the findings before the answer, and the fix landing on the remote.

### stage-runs

"A stop-short ends the stage short" is modified so that found work from
the kind's last stage starts the fix round only where the fix round is a
stage before that last stage, and any other found is taken as needing an
answer, which is 131's reading. Its two scenarios are kept unchanged and
are held as before, in `crates/flume-conformance/src/engine/stage_runs.rs`:
"A construct stops short" by "stop-short: a construct stops short, and
no code stage starts", and "A stop-short with another start's secret"
by "stop-short: one carrying another start's secret or identity is
refused". The requirement now
agrees with `running-through`'s "Found work no stage before it can do
again needs an answer", with `needs`' documentation in
`crates/flume-core/src/agent_acts.rs`, and with `sent_back_by` and
`found_work` in `crates/flume-core/src/submit.rs`.

### agent-profile

"The words a fix's code is told" is held by
`the_words_a_fixs_code_is_told` in
`crates/flume/tests/profiles/agent_claude.rs`. The text for a fix's
code agent and for a change's verify agent is one, design 5's words,
with no "cannot deliver past" and nothing of either identity.

### command-line

"needs' help" is held by `needs_help` in `crates/flume/tests/commands.rs`:
`flume needs --help` gives `--found` as "The stage found work a stage
before its own must do again", and no "cannot deliver past".

### The specs the change leaves as they are

The other requirements that speak of a found were read against the
change in the trial archive and hold: `seam-contracts`' types,
`repeating-stages`' fresh start, `kinds-profile`'s "What flume's own
prompts ask", `place-status`' "Status says what a waiting place waits
on", `settling`'s failed submission, and `command-line`'s "needs takes
the agent's identity and secret from its environment". None says a
found at its kind's own fix round sends the place back.

## The design, read against the code

- Design 1: `fix_round_of` in `crates/flume-core/src/submit.rs` is the
  one reading of the kind's fix round. `fix_round` takes its target
  from it, and so does `waiting_on`'s past-bound branch in `status.rs`.
- Design 2: `sent_back_by` holds only where `fix_round_of` is a stage
  before the one that found. `found_work` returns at once otherwise,
  leaving the agent as any stop-short's.
- Design 3: `waiting_on`'s `EndedShort` branch answers nothing only
  where `sent_back_by` holds, the same reading the settle takes, and
  otherwise a retry for an answer with the words. A place left past its
  bound by a found from its own fix round before this change still
  reads `PastBound` on its fix round.
- Design 4: `Engine::retry` is unchanged, and the retry's prompt
  carries the findings before the answer through `answered`.
- Design 5: Claude Code's `stop_short`, `--found`'s help, `StopKind`'s,
  `StopShort`'s and `Waits::Answer`'s documentation and the fake's
  words say a found is work a stage before the agent's own must do
  again. "cannot deliver past" stands only in fix@1's and chore@1's
  code prompts, kept as 156 and core.34 have them, and in the record's
  and rule's account of the old words.

## Tasks

All fourteen tasks of groups 1 to 4 are ticked. Task 4.3 carries the
`stage-runs` delta and names `stage-runs` among the proposal's modified
capabilities, with no code changed.

## The checks

- `openspec validate a-found-at-its-kinds-own-fix-round-waits-for-an-answer --strict`
  passes.
- A trial archive on a copy of `openspec/` applies the four deltas:
  `agent-profile` one added, `command-line` one added, `running-through`
  one added, two modified and one renamed, and `stage-runs` one
  modified. `openspec validate --specs --strict` passes 44 of 44 there.
- `sluice exec main` at af3658f passes 23 of 23 tasks: root:format,
  root:adrs, root:changes, seams', fakes', core's, conformance's and
  flume's format, lint and test, conformance:coverage, plugin-claude:test,
  site:prose, site:build and flume:tools, 109 of 109, the new
  walkthrough among them. sluice did not record the run, since this
  report was being written in the worktree beside the head while it
  ran. The report is the only file the run did not check.
