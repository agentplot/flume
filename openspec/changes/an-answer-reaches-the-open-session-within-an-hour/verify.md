# Verify: an-answer-reaches-the-open-session-within-an-hour

Verified on 2026-11-07 at 86297b4, the branch
`change/an-answer-reaches-the-open-session-within-an-hour` cut from main
and holding main at 9a838ee merged (task 11.1). flume's main is at
9a838ee still, so the branch holds all of it.

**Verdict: the work holds.** Every requirement and scenario of the
change's twelve delta specs is met by a test that asserts what the
scenario says, every task is ticked, and the checks the tasks name pass.

## Tasks

All 26 tasks of groups 1 to 11 are ticked. Each group ends with a commit
that ran its checks through sluice (`git log 9a838ee..HEAD`, 41
commits, the merges of main at f6a5ba7 and at 9a838ee among them).

## The checks

- `openspec validate an-answer-reaches-the-open-session-within-an-hour --strict --no-interactive`:
  valid.
- `sluice exec main` at 86297b4, the checks it affects run against main
  at 9a838ee: exit 0, 37 tasks passed across `seams`, `fakes`, `core`,
  `conformance`, `flume`, `plugin-claude`, `plugin-herdr`, `site` and
  `root`. `flume:tools` passed 123 tests, among them the four
  walkthroughs of `tools/open_sessions.rs`, the effort's walkthroughs
  under the stand-in, and the stand-in's `stand_in.rs` tests under herdr
  beside its effort test. `conformance:coverage` found every clause of
  Parts A and B, I1 to I8 and S1 to S119 at 9c060ab cited by the suite's
  760 scenarios, except those the list names as unbuilt by other
  changes. sluice kept no record of the pass, because this report was
  not yet committed in the worktree.
- A trial `openspec archive` over a copy of `openspec/` applies 18 added
  and 8 modified requirements across the twelve specs and leaves
  `openspec validate --specs --strict --no-interactive` passing 45 of 45.
  `openspec/specs/` on the branch is main's at 9a838ee, byte for byte.
- No MODIFIED requirement drops a scenario main holds: caller-acts'
  Retry (2 on main, 3 here) and Tell (1, 2), clause-coverage's two (1
  and 2, each kept), place-status' waiting place (1, 2), running-through's
  found work (1, 2), and stage-runs' stop-short (2, 3) and its limits
  (1, 2). Of these, #144 touched only clause-coverage's two scenarios,
  which each change rewrites to name its own clauses as the spec says.
  Every requirement #144 added stays after the trial archive, place-status'
  "Status gives an agent's effort" and the effort's requirements in
  launching, command-line, store-profile, kinds-profile, seam-contracts and
  seam-fakes among them. Each spec gains exactly this change's added
  requirements: caller-acts 8 to 9, command-line 15 to 16, end-delivery 9
  to 11, kinds-profile 14 to 15, launching 12 to 13, place-status 23 to
  25, seam-contracts 45 to 48, seam-fakes 20 to 22, stage-runs 15 to 19,
  store-profile 11 to 12.

## The merge of main at 9a838ee

The merge commit, 65b9138, differs from both parents only where the two
changes meet, and each such place keeps both sides:

- `effort` and `keep_open` are both in `FIELDS`, which now holds nine
  fields, and in `--agent`'s help and refusal (`commands/launch.rs`,
  `commands/mod.rs`). Both are folded whole in `Engine::resolve`'s
  `whole` list beside the model (`flume-core/src/settings.rs`). Both are
  read in every host agents table (`host/config.rs`). Both are taken in
  `Settings::under` (`flume-seams/src/kinds.rs`).
- The imports take `EffortStatus` beside `SessionAnswer` and `KEEP_OPEN`
  in `engine/place_status.rs`, `flume-core/src/status.rs` and
  `commands/print.rs`.
- The docs of the fake agent, the resolution, the host's tables and the
  stand-in's tests name both changes' parts. `tools/stand_in.rs` holds
  the effort's test beside this change's two herdr tests.
- The clause list at blueprints 9c060ab names neither 197 and S103 nor
  198, S104 and S105 unbuilt. Its count test drops both changes' terms.
- core.30's details hold record 0044's effort and record 0046's
  `keep_open`. The site's agent-settings, commands, configuration and
  state pages name both.

## Requirements and scenarios

Each scenario maps to one test, named here by its file. The conformance
scenarios cite their clauses as data (106). `engine/open_sessions.rs`
holds 33 scenarios, each citing 198, S104 and S105 where it builds them.

| Spec | Scenarios | Where |
|---|---|---|
| seam-contracts | the printed command; herdr | `crates/flume/tests/profiles/terminal_printed.rs` `the_printed_command_hands_a_running_agent_no_words`; the herdr profile's unit test `herdr_hands_a_running_agent_words` |
| seam-contracts | an agent let go; two answers; state written before | `crates/flume-seams/src/round_trip.rs` `an_agent_let_go`, `two_answers_in_one_session`, `an_ends_key_with_and_without_answers`, `state_written_before_an_agent_kept_open` |
| seam-fakes | a fake terminal that hands no words; a fake agent's session ended; another process at the next sleep | `crates/flume-fakes/tests/fakes.rs`, three tests of those names |
| store-profile | a kind and one stage; a stream and every agent; another form; a table of programs | `crates/flume/src/host/config.rs`, its `keep_open` tests |
| kinds-profile | a repository's keep_open; a kind's keep_open | `crates/flume/tests/profiles/kinds_weir.rs`, over a scratch repository and a scratch package |
| command-line | a stage's time; another form; a program's time | `crates/flume/src/commands/launch.rs` unit tests, and every refusal naming `keep_open` |
| launching | from the host's kind; kind over stream; launch over host; no layer; a host change after the launch | `crates/flume-conformance/src/engine/agent_settings.rs`, five scenarios; the launch's `0` ending its agent once quiet is open_sessions' "kept open for no time" |
| place-status | waiting on an answer; its agent let go; two answers; from the host's table for every agent | `engine/place_status.rs`, four scenarios; the person's form in `crates/flume/src/commands/print.rs` unit tests (kept open, let go and ended, let go and to be ended, stopped, a bare agent left in its pane, two answers, the inbox line) |
| end-delivery | the notice of a question; waiting again after an answer | `engine/open_sessions.rs` |
| running-through | construct finds work; found work from the last stage | `engine/open_sessions.rs` |
| stage-runs | kept open (an hour by default, a stream's time, `0`, a terminal that hands no words, found work before the last); let go (the time passing with the freed room taken, not quiet, dying, account spent, past the threshold, a late answer, two settles at once); a conversation's agent lost (bare in a conversation's pane, bare in another stage, the answer carrying the conversation, stopped by its caller); carried on (a second stop-short, limits after an answer, spent after an answer, unsettled after a retry) | `engine/open_sessions.rs`, one scenario each; the stage-runs scenarios main holds stand unchanged |
| caller-acts | a retry with words after the agent was let go; a retry with no words while kept open; a tell once let go; a retry's answer; a tell's answer; an answer taken before its session ends; a repeating stage's answer; an answer the agent does not take; an answer after the time passed | `engine/caller_acts.rs` (the retry with words) and `engine/open_sessions.rs` |
| clause-coverage | the clauses listed; the suite as this change leaves it | `crates/flume-conformance/src/clauses.rs` at blueprints main 9c060ab, 198 in Part A, S104 and S105 among the scenarios, cited by open_sessions' scenarios; `conformance:coverage` |

The walkthroughs under the pinned herdr are in
`crates/flume/tests/profiles/tools/open_sessions.rs`, four tests. In
each, the stand-in reports its state to herdr and takes later prompts
typed into its terminal. They cover S4's answer by a retry then a tell
into the same session, S104's time passing and late retry, and S105's
session ended and account spent, each answered by a retry (design 10).

## The rules

- core.1: the branch's diff over main names no program, terminal or
  tool outside a profile in the core, seams, conformance, fakes,
  commands or host code.
- core.35: the site's pages name no host, team or word of ours.
- core.19: the scenarios that build 198, S104 and S105 cite them. The
  clause list reads blueprints main at 9c060ab, which holds 198 as the
  code builds it.
- core.27 and core.30 carry this change's details. Record 0046 is
  accepted. Its amendment (design 14) is proposed.

## Open beside the work

These do not hold the archive back. Each is for the orchestrator or the
user, not this stage.

- **Design 14's statement amendment is unlanded and unpushed.** It
  amends clauses 49, 179 and 198 so that a conversation's asking agent
  lost with its host is carried at its answer's retry. The amendment is
  on the blueprints branch `design/flume-an-asking-conversation-lost-is-carried`
  at 6529c04, in the local clone
  `~/Code/github_agentplot/blueprints/blueprints.git` and not on
  agentplot/blueprints. It waits for the user's word on record 0046's
  amendment. Blueprints main is at b0df70b without it. Until that word
  lands, the code's `carries_lost` in `crates/flume-core/src/acts.rs`
  goes past main's statement by the amendment's text.
- **Blueprints main moved past the clause list's revision.** b0df70b
  takes clause 209 out (record 0053 rejected), but the list still names
  209 unbuilt by record 0053's change. The change that adopts b0df70b
  drops it.
