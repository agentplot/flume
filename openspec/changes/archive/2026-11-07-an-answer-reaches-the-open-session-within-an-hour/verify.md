# Verify: an-answer-reaches-the-open-session-within-an-hour

Verified on 2026-11-07 at fe00f3f, the branch
`change/an-answer-reaches-the-open-session-within-an-hour` cut from main
and holding main at f6a5ba7 merged (task 10.1).

**Verdict: the work holds.** Every requirement and scenario of the
change's twelve delta specs is met by a test that asserts what the
scenario says, every task is ticked, and the checks the tasks name pass.

## Tasks

All 25 tasks of groups 1 to 10 are ticked, each group closed by a commit
that runs its checks through sluice (`git log f6a5ba7..HEAD`, 36
commits, the merge of main at f6a5ba7 among them).

## The checks

- `openspec validate an-answer-reaches-the-open-session-within-an-hour --strict`:
  valid.
- `sluice exec main` at fe00f3f, the checks it affects run against main
  at 9a838ee: exit 0, 37 tasks passed across `seams`, `fakes`, `core`,
  `conformance`, `flume`, `plugin-claude`, `plugin-herdr`, `site` and
  `root`. `flume:tools` passed 120 tests, the four walkthroughs of
  `tools/open_sessions.rs`, the standing agent's walkthrough under herdr
  and the stand-in's `stand_in.rs` tests under herdr among them.
  `conformance:coverage` found every clause of Parts A and B, I1 to I8
  and S1 to S119 at 9c060ab cited by the suite's 743 scenarios, but
  those it names unbuilt by other changes. sluice kept no record of the
  pass, because this report was not yet committed in the worktree.
- A trial `openspec archive` over a copy of `openspec/` applies 18 added
  and 8 modified requirements across the twelve specs and leaves
  `openspec validate --specs --strict --no-interactive` passing 44 of 44.
  `openspec/specs/` on the branch is main's at f6a5ba7, and the deltas
  are as the change's earlier verify read them at 0f62d8d.
- Every MODIFIED requirement's header stands in main's specs
  (caller-acts' Retry and Tell, clause-coverage's two, place-status'
  waiting place, running-through's found work, stage-runs' stop-short
  and its limits), none of them changed on main since 376c17c, and none
  drops a scenario main holds. The four specs both changes add to keep
  both: end-delivery 11, place-status 24, seam-contracts 46 and
  seam-fakes 21 requirements after the trial archive, the standing
  address's among them.

## The merge of main at f6a5ba7

The branch's own work differs from what it was before the merge in the
eight files the merge queue found contested, and in no other: the
imports of `engine/place_status.rs`, `commands/print.rs` and
`host/config.rs` take the standing address's names beside
`SessionAnswer`, `ClosedBy`, `read_span` and `KEEP_OPEN`; the docs of
`seams/terminal.rs`, the fake host, `terminal_printed.rs` and
`tools/main.rs` name both changes' parts; and `tools/walk.rs` holds one
`UnderHerdr`, main's short link for a session's sockets beside this
change's scripts, prompts and panes, each herdr call bounded by
`herdr_within` (core.29, core.51).

## Requirements and scenarios

Each scenario maps to one test, named here by its file; the conformance
scenarios cite their clauses as data (106).

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

The walkthroughs under the pinned herdr, with the stand-in reporting its
state to herdr and taking later prompts typed into its terminal, are in
`crates/flume/tests/profiles/tools/open_sessions.rs`: S4's answer by a
retry then a tell into the same session, S104's time passing and late
retry, and S105's session ended and account spent, each answered by a
retry (design 10).

## The rules

- core.1: the branch's diff over main names no program, terminal or
  tool outside a profile in the core, seams, conformance, fakes,
  commands or host code.
- core.35: the site's pages name no host, team or word of ours.
- core.19: 198, S104 and S105 are cited by the scenarios that build them,
  and the clause list reads blueprints main at 9c060ab, which holds 198
  as the code builds it.
- core.27 and core.30 carry this change's details; record 0046 is
  accepted, its amendment (design 14) proposed.

## Open beside the work

These do not hold the archive back; each is for the orchestrator or the
user, not this stage.

- **Main moved on again.** flume's main is at 9a838ee, landing #144
  (`change/an-agents-effort-is-a-setting-of-flumes-layers`) a minute
  before this change's last commit, and sluice reads the branch
  conflicting with it in fourteen files: `flume-conformance/src/clauses.rs`,
  `engine/place_status.rs`, `flume-core/src/settings.rs`,
  `flume-core/src/status.rs`, `flume-fakes/src/agent.rs`,
  `commands/launch.rs`, `commands/mod.rs`, `host/config.rs`,
  `tools/stand_in.rs`, `docs/architecture/core.md`, and the site's
  `guides/agent-settings.md`, `reference/commands.md`,
  `reference/configuration.md` and `reference/state.md`. Its archive of
  the effort's specs into `openspec/specs/` meets this change's archive
  in the specs both touch.
  The submission that follows this stage reads that conflict and sends
  the place to its fix round with the merge queue's words (112;
  core.6, core.27).
- **Design 14's statement amendment is unlanded.** Clauses 49, 179 and
  198 carrying a conversation's asking agent lost with its host at its
  answer's retry are on the blueprints branch
  `design/flume-an-asking-conversation-lost-is-carried` at 6529c04, for
  the user's word on record 0046's amendment; blueprints main is at
  b0df70b without it. Until that word lands, the code's `carries_lost`
  in `crates/flume-core/src/acts.rs` goes past main's statement by the
  amendment's text.
- **Blueprints main moved past the clause list's revision.** b0df70b
  takes clause 209 out (record 0053 rejected); the list still names 209
  unbuilt by record 0053's change. The change that adopts b0df70b drops
  it.
