# Verify: every-question-a-place-asks-reaches-someone-who-answers-it

Read at `b024d079` on
`change/every-question-a-place-asks-reaches-someone-who-answers-it`,
against main at `f72e0784` (#150), which the branch holds. origin's main
is still at `f72e0784`.

## Verdict

**Falls short: not archived. The change goes back to code.** The build
holds:

- all 38 tasks are ticked;
- the gate passes;
- every one of the 125 scenarios of the 48 requirements in the 13 delta
  specs has a test that shows it.

But the specs that archiving would write contradict themselves and the
code in places, and two rules this change wrote contradict the design
the user approved. Archiving now would put those contradictions into
`openspec/specs/` and `docs/architecture/core.md`. Items 1 to 6 below
are text in the change's specs and rules, small to fix. Item 7 is a
field the status JSON lost. Item 8 is a race in the key's "taken once".

## What holds

### Tasks and their checks

- `openspec validate every-question-a-place-asks-reaches-someone-who-answers-it --strict`:
  valid.
- The change archived over a copy of `openspec/` (openspec 1.14.3, the
  devenv's) applies +30, ~17, −1 and →5. After it, `openspec validate
  --specs --strict` passes 46 of 46. Every MODIFIED, REMOVED and RENAMED
  header matches its archived requirement, and each MODIFIED block
  keeps the archived scenarios it should.
- `sluice exec main` at `b024d079` ran the 26 checks the commit affects
  against main, and all 26 passed (27 tasks in all, with
  `site:install`). sluice recorded the results under the key for
  `b024d07`:
  - `flume:tools`: 132 of 132, with
    `s93_a_question_for_easts_ops_is_answered_once_by_its_woken_run`
    and `s94_a_restarted_session_is_given_its_place_and_the_stall_reaches_the_door`;
  - `flume:test`, `core:test`, `seams:test`, `fakes:test`,
    `conformance:test` and `plugin-claude:test`, with each project's
    format and lint;
  - `root:adrs`, `root:changes`, `root:format`, `site:prose` and
    `site:build`;
  - `conformance:coverage`: every clause of Parts A and B, I1 to I8 and
    S1 to S119 at `9c060ab` is cited by the suite's 852 scenarios, or
    named unbuilt by its change. None of 193, S93, S94, S98 or S99 is
    named unbuilt.
- `root:specs` was not affected by this commit, so the gate did not run
  it. The trial archive's `openspec validate --specs --strict` stands in
  for it.

### The statement, the records and the rules

- agentplot/blueprints main holds `9c060ab`, which the clause list
  pins. At that revision the statement has 193 and S93, S94, S98 and
  S99, as amended for record 0039 (task 1.5). Clause 132 gives
  `<stream>/<role>`.
- Record 0039 is accepted. Its status line quotes the user's acceptance
  of 2026-11-05 and their approval of the design on 2026-11-06.
- Records 0009, 0013, 0014 and 0040 each carry an amendment line naming
  record 0039.
- `docs/architecture/core.md` amends core.24, core.25 and core.28, and
  draws core.54 at the next free id, since main holds no core.54.
  `site/src/content/docs/architecture/rules.md` mirrors core.28 and
  core.54.
- No core module, seam, conformance scenario, host reader or command
  added a program's, terminal's or queue's name (core.1). The site, the
  plugin and the commands' help name no host, team or word of ours
  (core.35). The one "conductor" is a test's recipient name in
  `print.rs`, which core.35 allows.

### The scenarios

Each scenario was traced to its test and the test's assertions read
against its THEN clauses. The tests are in the conformance engine
(`role_waking.rs`, `launching.rs`, `place_status.rs`, `caller_waits.rs`,
`caller_acts.rs`, `stage_runs.rs`, `end_delivery.rs`), the binary's
tests (`host.rs`, `commands.rs`, `command_line.rs`, `hooks_claude.rs`,
the S93 and S94 walkthroughs), `round_trip.rs`, `print.rs`, `clauses.rs`
and `plugins/claude/tests/hooks.sh`. Every scenario is met. A few tests
assert less than their scenario says, and the code does the rest:

- *A hold's bound passes* does not show the second stall's key differing
  from an earlier one, since no stall comes before its hold.
- *A tell naming an answered key* runs its tell after the agent took the
  first answer, so not while the agent is still kept open.
- *An address in the environment* uses the name `planner`, not
  `east/planner`, at the hook's level. The engine's test uses the
  address.
- *An address's stream* never asserts `role_of("slc-2/ops.1")`.

None of these is a failure.

## What fails

Ranked, most serious first. Each item names what to change.

1. **The plugin's spec would contradict itself.** This change makes
   SessionStart in a session flume did not start run `flume inbox`. That
   records the inbox's deliveries on each place it collects from, and
   settles every place while no caller keeps watch (`hook.rs`
   `caller_session_start` → `Engine::session_inbox` → `Engine::inbox`,
   `wait.rs:183`). The archived requirement *SessionStart compares the
   start's record before it acks* keeps its scenario *A session flume
   did not start*: "it compares nothing, writes nothing and exits 0"
   (`openspec/specs/claude-code-plugin/spec.md:96-99`). This change does
   not modify that scenario, so after the archive the spec says both.
   Bring that requirement under the delta as MODIFIED, so the scenario
   says what the hook does there: it compares nothing, acks nothing, and
   runs the inbox as the new requirement says.
2. **end-delivery would contradict stage-runs and caller-waits.** The
   archived *The settle delivers each end once to each recipient*
   (`openspec/specs/end-delivery/spec.md:11-18`) delivers "each end once
   to each recipient the launch named". This change sends a stop-short
   that names a role "to that address alone and not to the place's
   recipients" (stage-runs, *A stop-short may name the role it is for*).
   It also sends a stall to the role the stop-short named and to the
   answerer behind the stream (caller-waits). A launch's recipients now
   end at that answerer too, whom the launch did not name. Bring the
   requirement under the delta as MODIFIED, so it names each end's
   recipients as this change gives them.
3. **role-waking says an entry may give `points`, and the code refuses
   it.** *A role's entry gives its run* gives "an optional brief,
   points, name, intent, …" (`specs/role-waking/spec.md:186-189`).
   `crates/flume/src/host/roles.rs:223-231` refuses an entry that gives
   `points`, and `crates/flume/tests/host.rs:811` tests that refusal.
   Design 2 (`design.md:812-816`) records the refusal as the code
   stage's own reading, since this flume builds no points (165 is
   unbuilt). The spec was not brought to it, and neither its faults
   requirement nor any scenario names the refusal. Write the reading
   into the requirement and its faults, with a scenario. Statement 132
   and C.7 still list points among what a source declares. That is
   right for when 165 is built, but the design should say so, as it
   does for a stream's points.
4. **core.54 and core.24 contradict the role's run the design gives.**
   - core.54, which this change drew, says "Every place has a recipient
     its ends reach". Its details rule out "a launch taken with no
     recipient into a stream no role stands behind" and "a stall the
     answerer behind the place's stream is not delivered".
   - core.24, as this change amended it, keeps "a stalled place said to
     nobody" ruled out.
   - Design 2 (`design.md:796`), core.28 and statement 132 give a role's
     run no recipient. They give its stall to the answerer, and never to
     its own address, so the answerer's own run's stall goes to no one
     (role-waking, *The door's own run*; design Risks, "The door is its
     stream's last word").
   - `launch.rs` gives every role's run no recipients, in a stream no
     role stands behind too (`role_waking.rs` `a_turn_delivers`).
   - So the code breaks core.54 and core.24 as they are written, and
     `docs/architecture/` wins over the design. Amend core.54's rule and
     details, and core.24's details, to except a role's run as core.28
     and 132 give it. Mirror the change in the site's rules page.
5. **A role's run whose kind lands is built but written nowhere.**
   `crates/flume-core/src/settle.rs:330` (`run_over`) declares done
   only a role's run whose kind does not land. It submits one whose kind
   lands, as any place's, since 114 refuses a done on such a place. The
   code's comment cites design 2, but design 2 does not say this. *A
   role's run is over at its delivery* declares every role's run done
   at its delivery, with no exception, and so do core.28 and 132. No
   test covers the landing case. Write the reading into the design and
   the requirement, with a scenario. Where statement 132 needs the
   exception, put it to the orchestrator for blueprints (core.19).
6. **history-and-host misdescribes the answering roles.** *The roles a
   host answers with* gives "every address a source declares with
   `answers`, a run's or a carried one's"
   (`specs/history-and-host/spec.md:27-29`).
   `HostRoles::answering` (`crates/flume-seams/src/host.rs:674`)
   includes a standing agent's `answers` too. C.7 lets a standing entry
   give `answers`, and seam-contracts and `round_trip.rs` agree with the
   code. Say "a run's, a standing agent's or a carried one's".
7. **A field removed from the status JSON (core.14).** `RoleRun` lost
   its `stream` (`crates/flume-seams/src/host.rs:422-443`). On main, an
   entry that gave one serialized it at
   `status.host.addresses[].role.stream`. The stream is now the
   address's first part, so nothing is lost, but core.14 rules out "a
   field removed from the JSON form". Main's `FIELDS` never listed
   `role.stream`, so `every_field_listed` did not catch it. Give the
   address's stream there again, and list it in `FIELDS`.
8. **Two answers to one key at once can both be typed.** The
   kept-open path reads the key on a plain read (`acts.rs:231` for
   a retry, `acts.rs:675` for a tell). Then `answer()`
   hands the words to the session (`open_sessions.rs:75`) and only
   after that records them by compare-and-set (`open_sessions.rs:79`).
   `hand()` waits up to `TELL_BOUND`, ten minutes, for the agent to wait
   again.
   - So a second keyed answer whose check lands while the first is being
     handed passes the check, waits out the agent's turn on the first,
     and is typed.
   - Its write then finds the stop-short answered, and it is refused,
     "moved on before its answer was recorded", after its words reached
     the session.
   - If the agent stopped short again meanwhile, the second answer's
     words sit under the new question.
   - That breaks 75 for two acts at once, and core.54's "a second answer
     to one key taken". It is the very case the key exists for: two
     answerers of one question.
   - Every test is sequential. The unkeyed race of record 0046's change
     on main has the same shape, but this change promises "taken once".
   - Claim the answer on the end by compare-and-set before handing, and
     release the claim where the hand fails. A second answer then finds
     the claim and is refused with nothing typed. Add a test in which a
     second answer is checked while the first is handed.

## Seen, and not failures

- **The notice's read command comes from the place, not the end.**
  `ends.rs:43` names `flume inbox` wherever the place waits on a move
  when the notice is sent. So a delivery's notice sent late, once the
  place waits on the next stage's stop-short, names the inbox. Design 9
  reads it from the place as its held state stands when the notice is
  sent, and the inbox then gives the waiting place. The spec's "an end
  the place waits on a move for" bears that reading.
- **SessionStart's inbox settles every place while no caller keeps
  watch.** That is `flume inbox`'s way since record 0051. The hook's
  20-second timeout bounds it, which design 9 takes from sluice's
  session start. If Claude Code kills the hook after the inbox
  collected and before it printed, the collected ends stand delivered
  by the inbox and that session is not told them. The waiting places
  survive, since they are never marked. No test runs a slow settle
  there. Worth a line in the design's risks when the code stage is back
  in it.
- **A role's run declared done and kept from its tidy still stands for
  its address.** Uncommitted work or a session's process can keep it
  (`wake.rs` `run_of` sets aside only `Tidied`). No new run can be made
  under its name until it is tidied anyway (core.8), and status names
  why it was kept. The spec does not speak to the case.
- `crates/flume/src/host/config.rs:1819`: the doc comment `/// role-waking,
  "The roles in the host's configuration" (193, C.7).` sits on the
  unrelated test `a_keep_open_for_a_kind_and_for_one_stage`.
- Blueprints main has moved past `9c060ab` (`b0df70b` takes clause 209
  out). The clause-coverage spec pins `9c060ab`, so this change is not
  affected.

## Decisions in this stage

- **The verdict: send back.** This is verify's own call, recorded here.
  Items 1 to 6 would archive contradictions into the specs and rules,
  which the repository's rule on specs ("so the spec reconciles") and
  core.19 rule out. Items 7 and 8 are code. The path not taken was to
  archive and leave items 7 and 8 for a later change. It was not taken
  because items 1 to 6 alone keep the archive from reconciling.
- **The notice's read command and SessionStart's settle are not
  failures.** This is verify's reading of design 9, cheap to change,
  and recorded above.

No clause, record or rule was changed in this stage. The one commit is
this report, on this branch, unlanded until the change lands.
