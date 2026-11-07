# Verify: every-question-a-place-asks-reaches-someone-who-answers-it

Read at `f4ef27f5` on
`change/every-question-a-place-asks-reaches-someone-who-answers-it`,
against main at `f72e0784` (#150), which the branch holds. origin's main
is still at `f72e0784` after a fetch.

## Verdict

**Holds: archived.**

- All 44 tasks are ticked, group 12 among them.
- The gate passes.
- Each of the 139 scenarios has a test that shows it. The scenarios
  belong to the 53 requirements of the 13 delta specs: 33 added, 19
  modified and 1 removed, with 5 renamed.
- The specs that archiving writes reconcile with each other, with the
  code and with the rules.

## What holds

### Tasks and their checks

- `openspec validate every-question-a-place-asks-reaches-someone-who-answers-it --strict`:
  valid (openspec 1.14.3, the devenv's).
- The change archived over a copy of `openspec/` applies +33, ~19, −1
  and →5. After it, `openspec validate --specs --strict` passes 46 of
  46.
- `sluice exec main` at `f4ef27f5` ran the 26 checks the branch affects
  against main, 27 tasks with `site:install`, and all 26 passed:
  - `flume:tools`: 132 of 132, the S93 and S94 walkthroughs among them;
  - `flume:test`, `core:test`, `seams:test`, `fakes:test`,
    `conformance:test` and `plugin-claude:test`, with each project's
    lint, and `plugin-claude:format` and `plugin-claude:validate`;
  - `root:adrs`, `root:changes`, `root:format`, `site:prose` and
    `site:build`;
  - `conformance:coverage`: every clause of Parts A and B, I1 to I8 and
    S1 to S119 at `9c060ab` is cited by the suite's 855 scenarios, or
    named unbuilt by its change. None of 193, S93, S94, S98 or S99 is
    named unbuilt.

  sluice did not record the run. While it ran, the worktree held this
  report uncommitted beyond the head. The code under test was the
  head's.
- `root:specs` is affected only by an archive, so the trial archive
  stands in for it here. The archive's own commit is checked by the
  gate when it is submitted.

### The items found at b024d079

Each of the eight items in the report at `b024d079` is mended, and none
leaves a contradiction behind.

1. **The plugin's spec.** claude-code-plugin brings *SessionStart
   compares the start's record before it acks* under the delta as
   MODIFIED. Its scenario *A session flume did not start* now compares,
   acks and records nothing of a start, and runs `flume inbox`. The
   archived spec no longer says "writes nothing" for that session.
   `plugins/claude/tests/hooks.sh` and `hooks_claude.rs` run the inbox
   there and check the exit 0.
2. **end-delivery.** *The settle delivers each end once to each
   recipient* is MODIFIED, so each end goes to "its recipients, as 'An
   end's recipients' gives them". The added *An end's recipients* gives
   them for a stop-short that named a role and for a stall. Both of its
   scenarios cite their tests: `stage_runs.rs`'s question for its own
   stream's ops, and `caller_waits.rs`'s `another_streams_role_named`.
   No archived spec still says "each recipient the launch named".
3. **Points in a role's entry.** *A role's entry gives its run* and *A
   fault in a source's roles stays its own* refuse `points`, with the
   scenario *Points in a role's entry*. `host.rs`'s
   `an_entry_at_fault_declares_nothing_for_its_address` cites and tests
   it. Design 2 and its Risks say that statement 132 and C.7 hold once
   165 is built.
4. **core.54 and core.24.** core.54's rule reads "Every place but a
   role's run". It excepts the answerer's own run's stall, and its
   details allow a role's run with no recipient. core.24's details allow
   the answerer's own run's stall said to no one. core.28 has a run
   whose kind lands submitted. The site's rules page mirrors core.28 and
   core.54. `launch.rs` giving a role's run no recipient now keeps the
   rules.
5. **A role's run whose kind lands.** Design 2 records this as the code
   stage's own reading, with the path not taken. *A role's run is over
   at its delivery* says so, with the scenario *A run whose kind lands*.
   `role_waking.rs`'s `a_run_whose_kind_lands` shows the run
   submitted and live, with one submission, and gone once it lands. The
   exception for 132 is commit `0fdd314` on blueprints' branch
   `design/flume-a-roles-run-whose-kind-lands`. That commit sits on main
   at `b0df70b` and is unpushed, for the orchestrator (core.19).
6. **The answering roles.** history-and-host names a standing agent's
   `answers`. *The host's roles* and `the_roles_a_host_answers_with`
   hold `east/design` as a standing agent.
7. **`role.stream`.** Status's `addresses[].role` is now `RoleStatus`.
   It flattens the entry and adds `stream`, the address's own
   (`wake.rs`). `answers.rs`'s `FIELDS` lists `role.stream` and
   `role.source`, and the site's state page names the field. The form
   only gains fields (core.14).
8. **Two answers at once.** `Engine::claim_answer` claims the end by
   compare-and-set before anything is typed. It does this in
   `open_sessions.rs`'s `answer`, for every answer to the agent kept
   open, keyed or not. It also does it in `retry_answering`, before a
   fresh agent is started for a keyed answer.
   - The claim is taken only while `keyed` passes and no live claim
     holds. Otherwise the answer is refused, naming the claimer, with
     nothing typed.
   - The record clears the claim.
   - The `Claimed` guard's drop clears it only where it is still this
     claim, on this key, so it never clears a later stop-short's claim.
   - A claim lapses at the hand's bound plus five minutes.
   - `caller_acts.rs`'s `two_answers_to_one_key_at_once` gives the
     second answer during the first's hand. The second is refused with
     nothing typed, and the first is recorded with no claim left.
     `an_answer_not_taken_lets_its_claim_go` shows the claim let go.
   - `round_trip.rs`'s `an_answer_being_taken` reads the claim back,
     and reads an end written before as unclaimed.
   - Design 8, core.54's details and the site's callers-and-holds and
     state pages say so.

The two "seen" notes of that report are also mended. The misplaced doc
comment in `config.rs` now sits on `a_roles_table_makes_the_configuration_unreadable`.
The design's Risks now give a line to SessionStart's settle and to a
hook killed mid-inbox.

### The statement, the records and the rules

- The clause list pins agentplot/blueprints at `9c060ab`, and blueprints
  main holds it. At that revision the statement has 193, S93, S94, S98
  and S99, and 132 gives `<stream>/<role>`.
- Record 0039 is accepted. Records 0009, 0013, 0014 and 0040 each carry
  an amendment line naming it.
- core.24, core.25 and core.28 are amended, and core.54 is drawn, each
  citing record 0039. The site mirrors core.28 and core.54.
- Nothing added names a program, a terminal or a queue outside a profile
  (core.1). Nothing names a host, a team or a word of ours on the site,
  in the plugin or in the commands' words (core.35).

### The scenarios

Each scenario was traced to its test, and the test's assertions were
read against its THEN clauses. The tests are in:

- the conformance engine: `role_waking.rs`, `launching.rs`,
  `place_status.rs`, `caller_waits.rs`, `caller_acts.rs`,
  `stage_runs.rs` and `end_delivery.rs`;
- the binary's tests: `host.rs`, `commands.rs`, `command_line.rs`,
  `hooks_claude.rs`, and the S93 and S94 walkthroughs;
- `round_trip.rs`, `print.rs` and `clauses.rs`;
- `plugins/claude/tests/hooks.sh`.

The scenarios kept from the archive under MODIFIED keep their tests:
end-delivery's four in `end_delivery.rs`, and the plugin's two in
`hooks.sh`.

Every scenario is met. A few tests assert less than their scenario
says, and the code does the rest:

- *A hold's bound passes* does not show the second stall's key differing
  from an earlier one, since no stall comes before its hold.
- *A tell naming an answered key* runs its tell after the agent took the
  first answer.
- *An address in the environment* uses `planner`, not `east/planner`, at
  the hook's level. The engine's test uses the address.
- *An address's stream* never asserts `role_of("slc-2/ops.1")`.
- *A session flume did not start* is tested in `hooks.sh` and in
  `hooks_claude.rs`'s `session_start_in` cases. Those tests show the
  inbox, the added context and the exit 0, but assert nothing about
  acks or records. `hook.rs`'s `caller_session_start` reads no start's
  record, so it compares, acks and records nothing.

None of these is a failure.

## Seen, and not failures

- **An unkeyed answer that a fresh agent carries claims nothing.** It
  is taken as on main, so it can race a keyed answer's claim, and its
  record clears that claim. Design 8 and caller-acts' requirement name
  this reading: the claim covers every answer to the agent kept open,
  and a keyed answer a fresh agent carries.
- **Record 0039 still says** "A run is over once its turn delivers:
  flume declares it done". The landing exception is in the design, the
  rule and the statement's branch, but not in an amendment line on the
  record. crew's kinds land nothing, so no run takes the exception
  today. A line on record 0039 would be the capture's place for it, and
  it is cheap to add when 0fdd314 lands.
- **The notice's read command comes from the place, not the end**
  (`ends.rs:43`). This is design 9's reading, as before.
- **A role's run declared done but kept from its tidy still stands for
  its address**, as before. Status names why it was kept, and core.8
  keeps a second run from its name until it is tidied.
- Blueprints main has moved past `9c060ab`, to `b0df70b`, which takes
  clause 209 out. The clause-coverage spec pins `9c060ab`, so this
  change is not affected.

## Decisions in this stage

- **The verdict: archive.** This is verify's own call, recorded here.
  Each item of the earlier report is mended, the specs reconcile, and
  the gate passes. The path not taken was to send the change back for
  the record 0039 amendment line. It was not taken because the decision
  is written in the design, the rule and the statement's branch, and no
  run takes the exception today.
- **The unkeyed fresh-agent answer that claims nothing is not a
  failure.** This is verify's reading of design 8 and caller-acts,
  cheap to change, and recorded above.

No clause, record or rule was changed in this stage. Unlanded:

- this report and the archive, on this branch, until the change lands
  through sluice;
- blueprints' `0fdd314` on `design/flume-a-roles-run-whose-kind-lands`,
  which the orchestrator pushes.
