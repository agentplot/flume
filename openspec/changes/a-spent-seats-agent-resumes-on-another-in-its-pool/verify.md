# Verify: a-spent-seats-agent-resumes-on-another-in-its-pool

Checked on 2026-11-02 at `change/a-spent-seats-agent-resumes-on-another-in-its-pool`
a99c4c0, cut from main at b6cd1b5. Main's head is now becc6ec (#84,
`chore/the-rules-and-records-match-the-statement`), which the branch
does not hold. The check covers this change's proposal, design, nine
specs and tasks.

**The work holds, and the change is archived.** Every task is ticked,
and every requirement is built. Every one of the specs' 43 scenarios
has a test, the gate passes at a99c4c0, and the archived specs pass the
strict check. One thing stands between the branch and its landing. It
conflicts with main in `docs/architecture/core.md`, in core.10's
details, where main's #84 and this change each amend the same line.
The submission will meet that conflict and go back to code with the
merge queue's words (core.27). That is a merge to make, not a
requirement unmet, so it is set out under "Main since the cut" and
does not hold the archive. The warnings below leave no requirement
unmet.

## Tasks

All 20 tasks of the six groups are ticked, `[x]`, and none is open. The
gate run below backs each group's closing check task (1.3, 2.4, 3.4,
4.3, 5.2, 6.3, 6.4).

Task 4.2 is ticked on its live walkthrough,
`s78_a_spent_agents_conversation_goes_on_under_the_pools_other_account`
in `crates/flume/tests/profiles/live/walkthroughs.rs`. It is written,
and it builds under `flume:lint`, which compiles the live harness under
`--all-features` and passes below. The user ruled on 2026-11-02 that
its run by hand under herdr with the pinned Claude Code is theirs, on a
test account, after the change lands. It was not run here: it needs a
test account's token, and no branch requires `check-live` (W3).

## Checks

- **The gate.** `sluice exec main` at a99c4c0, with the worktree
  clean, ran the checks the branch affects against main at becc6ec. It
  passed all 24 `#check-fast` and `#check-tools` tasks, none replayed
  from cache:
  - `seams`, `fakes`, `core`, `conformance` and `flume`: format, lint
    and test;
  - `conformance:coverage`;
  - `flume:tools`, 71 of 71, among them
    `claude_words::the_pinned_release_resumes_as_the_carry_reads_it`,
    `stand_in::the_stand_in_refuses_a_session_it_finds_no_transcript_of`,
    `walkthroughs::a_spent_agents_conversation_goes_on_under_the_pools_next_and_the_fix_lands`
    (S78) and
    `walkthroughs::a_spent_account_moves_the_stage_to_the_pools_next_and_the_fix_lands`
    (S55);
  - `plugin-claude`: format and test;
  - `site:prose` and `site:build`;
  - `root:format`, `root:changes` and `root:adrs`.

  Task 6.3 also names `plugin-claude:validate` and `root:specs`. The
  branch's diff from main affects neither. Under `plugins/claude/` it
  changes only `tests/`, which the plugin's validation does not read,
  and it changes nothing under `openspec/specs/`. The archive
  commit makes `root:specs` affected, and the trial archive below is
  that check over the archived specs.
- **The archive.** `openspec archive a-spent-seats-agent-resumes-on-another-in-its-pool --yes`
  ran over a copy of `openspec/` at a99c4c0. It applies 18 added, 3
  modified and 1 renamed requirement, the renamed one being
  account-pools' "A spent agent's stage starts again on another
  account". It creates `carried-conversations` with its Purpose as the
  change gives it. `openspec validate --specs --strict` over the result
  then passes 34 of 34.
- **The statement.** 72410b2, the revision the clause list reads, is
  the head of agentplot/blueprints' `origin/main` as this host's clone
  last fetched it. That revision states 179 as the carry part of Part
  B, and S78 and S79.

## Requirements and scenarios

Each scenario was matched to its test, and the test's assertions were
read against its WHEN and THEN. Every test named here passes in the
gate run above, except the live walkthrough (W3).

Short names: CC is
`crates/flume-conformance/src/engine/carried_conversations.rs`, AP
`engine/account_pools.rs`, PS `engine/place_status.rs`, SI
`engine/start_integrity.rs`, SA `crates/flume-conformance/src/seams/agent.rs`,
RT `crates/flume-seams/src/round_trip.rs`, AC
`crates/flume/tests/profiles/agent_claude.rs`, HC
`crates/flume/tests/profiles/hooks_claude.rs`, and WT
`crates/flume/tests/profiles/tools/walkthroughs.rs`.

| Spec | Scenarios | Tests | Met |
|---|---|---|---|
| account-pools | 4 | AP `the_stage_moves` (S55, the conversation stayed and why), `the_stage_moves_with_its_conversation` (S78), `another_agent_on_the_spent_account`, `a_stage_under_no_account`; WT's two spent walkthroughs | yes |
| carried-conversations | 15 | CC `s78_carried_to_the_pools_other_account`, `a_carried_start_waits_for_a_usable_account`, `a_pool_of_one_account`, `a_retry_carries_nothing`, `the_next_stage_carries_nothing`, `the_prompt_of_a_carried_start`, `accounts_that_share_nothing`, `a_profile_that_carries_nothing`, `s79_no_such_conversation`, `another_session`, `the_fresh_start_after_it_begins_otherwise`, and `by_the_limit_and_not_received` beside them; AC `a_conversation_carried_to_a_home_whose_projects_links_the_firsts`, `two_homes_of_their_own`, `a_start_that_never_said_it_started`; WT's two spent walkthroughs for the two scratch homes, each holding only what it held and the transcripts written | yes (W2) |
| agent-profile | 5 | AC `the_command_of_a_carried_start`, `a_background_task_the_stopped_agent_left`, `last_words_before_the_carried_agent_wrote_any`, `a_fresh_start_reads_its_whole_transcript`, `a_question_and_a_slash_command_of_the_stopped_agent`, `a_transcript_read_from_its_length`; `claude_words.rs` `the_pinned_release_resumes_as_the_carry_reads_it` | yes |
| claude-code-plugin | 4 | HC `a_carried_start_in_another_session`, `a_start_that_resumes_nothing`, `a_carried_starts_length_kept_at_a_compaction` (4,096 at the ack and kept at `compact`); `plugins/claude/tests/hooks.sh` with its `session-start-another-session.json` fixture | yes |
| place-status | 3 | PS `a_carried_agent`, `a_conversation_that_stayed`, `a_carried_start_that_did_not_begin`, in the JSON form; `print.rs` `a_carried_conversation`, `a_conversation_that_stayed`, `a_carried_start_not_begun`, `a_start_in_another_conversation`, `a_move_with_its_conversation_and_without` for the person's form | yes |
| seam-contracts | 4 | SA `carry_not_offered`, `carry_to_the_account_that_reaches_it`; RT `a_held_state_with_a_carried_start`, `a_held_state_written_before_carried_starts`, `a_start_begun_in_another_conversation_in_words`, `a_profile_that_carries_no_conversation` | yes (W2) |
| seam-fakes | 2 | CC `s78_carried_to_the_pools_other_account`, whose `carried` reads the fake's log line `carry … : yes s-1`, and `a_scripted_carried_start_that_ends_first`; `fakes.rs` `the_agents_carry_is_scripted_by_each_accounts_home`, `a_carried_start_scripted_to_end_before_its_word`, `the_agents_command_resumes_the_session_it_is_given`, `the_agents_word_names_the_session_it_resumes` | yes |
| start-integrity | 3 | SI `the_session_recorded`, `another_session`, `the_session_resumed` | yes |
| clause-coverage | 3 | `clauses.rs` `the_clauses_listed` and its unbuilt list; `coverage.rs`; `conformance:coverage` | yes |

The clause list reads revision 72410b2. Part A and Part B are exactly
as the spec gives them, Part A gaining 175 to 177 and Part B 178 and
179, with S1 to S79. Its unbuilt set is exactly the spec's: those of
records 0021, 0022, 0024's plan stage, 0025 and 0026 as before, and 175
to 178, S76 and S77 under records 0028 and 0029's change. 179 is cited
by SA's two scenarios and CC's. S78 is cited by CC's
`s78_carried_to_the_pools_other_account` and AP's
`the_stage_moves_with_its_conversation`, and S79 by CC's
`s79_no_such_conversation` and `a_scripted_carried_start_that_ends_first`.

## Design and rules

- **Decisions 1 to 3.** `Carry`, `Agent::carry` (`NotOffered` by
  default), `command`'s `resume`, `StartRecord::session`,
  `Unsettled::session`, `RoomWait::carry`, `AfterSpent`,
  `StartHeld::conversation`, `Conversation` and `AccountMove::carried`
  are built as decided, each new field read with a default and skipped
  where none. `read_spent` writes `AfterSpent { carry: true }`.
  `Engine::claim` asks the stage's profile only where the room carries
  an `AfterSpent` that may carry and an account was chosen, under the
  room lock. A claim left waiting for room keeps the room's `carry`. A
  carried start's prompt is design 3's words and the way to stop short,
  with nothing of the template filled. A retry clears the room's
  `carry` (`acts.rs`), so its start carries nothing.
- **Decision 4.** `carry.rs` follows the four steps, comparing by
  `dev` and `ino` with links followed. Its three `No` words are
  design 4's, each naming `CLAUDE_CONFIG_DIR` and no path. An empty
  `CLAUDE_CONFIG_DIR` reads as none. The command puts `--resume
  <session>` after `--plugin-dir`. Nothing writes in a home.
- **Decision 5.** `uncarried.rs` records an `UncarriedStart` once,
  sends nothing more, closes the agent once quiet and puts the stage
  back with `AfterSpent { carry: false, why }`. `deliver.rs` calls it at
  the agent's stop, at the stage's limit and on a prompt not received.
  `read_unsettled` of a carried start does the same with the unsettled
  words. No end is delivered and no send-back is counted.
- **Decision 6.** `hook::compare` sets `Unsettled::session` only where
  the record names a session, and `flume ack started` goes through the
  same `ack_started`. `Started::from` is written at the first ack and
  kept by `Record::ack_started`. `unsettled.json` records its own
  `from`. Own work, last words, asking's answer and the received
  reading read through `Transcript::read_from`.
- **Decisions 7 and 8.** Status gains `conversation`, `carried` and
  `uncarried` as only-added fields (core.14). The person's form prints
  "carries the conversation of start 1", "starts afresh after start 1:
  …", the move's "its conversation with it" and "its conversation
  stayed behind". The stand-in refuses an unknown session with
  2.1.296's words and exit status 1.
- **core.2, core.10, core.17, core.30 and core.32.** The core reaches
  the carry only through the agent seam. A carried start has its own
  identity, secret and record, and is taken only on its own word in the
  session its record names. `carry` reads only. No answer, record or
  words give a configuration home's path or a transcript's. The
  walkthroughs show each scratch home holding only what it held and
  the transcripts the stand-in wrote.
- **core.19.** Record 0020's amendment reads accepted by the user on
  2026-11-02. core.32, as amended, and core.10 and core.32's details
  carry it, and core.10's cites 146, 179 and S78. `docs/architecture/_open.md` no longer holds an item on the
  amendment. The code, tests, specs and design cite 179, S78 and S79,
  the numbers 72410b2 gives them.
- **core.35.** The site's diff names none of the words core.35 rules
  out. The guide shows the link with `work-1` and `work-2` and
  `~/.claude-work-<n>` homes, an example of the reader's own.

## Main since the cut

Main moved from b6cd1b5 to becc6ec by #84 (a4203f6, 8f8e476,
9195565), which changes only `docs/adr/` and `docs/architecture/`.
`git merge-tree` of a99c4c0 and becc6ec merges every file but one.
`docs/architecture/core.md` conflicts in core.10's details, which both
sides rewrote as one line:

- **Main's side.** It changes "Rules out: an agent handed from one
  stage to the next" to "…, or from one session to another" and "two
  agents in one place" to "… of a simple kind". It cites the mode
  compared at the first prompt as "(161, 162, C.4, S67; record 0024,
  as amended on the user's ruling of 2026-10-31)". It adds a "Not
  ruled out" for a statechart kind's sessions side by side, citing 175
  and record 0028.
- **This change's side.** It adds the "Not ruled out" for a start
  after a spent account that resumes its earlier agent's conversation
  (146, 179; record 0020's amendment). Its Source gains 146, 179, S78
  and record 0020.

The resolution keeps both. One point of wording comes with it. Main's
"session" in "from one session to another" is a statechart kind's
session (175). This change's "the session it resumes" is the agent
program's conversation. The merged line should leave a reader in no
doubt that a carried start is a fresh agent, not one handed between
sessions. core.32's details and the rest of the tree merge clean. #84
touches nothing under `openspec/`, so the archive drops nothing of
main's.

## Warnings

- **W1. The merge with main.** See "Main since the cut". The submission
  meets the conflict in core.10's details, and the stage the kind names
  for a failed submission merges main.
- **W2. Claude Code's variable outside a profile.** The engine
  scenarios in CC, AP and PS, and RT's round trip, script the fake's
  `No` words naming `CLAUDE_CONFIG_DIR`. At b6cd1b5 no source of those
  crates named Claude in any case. core.1's check (`no_tool.rs`) reads
  `CLAUDE_CONFIG_DIR` as one token and, by design, finds no tool's name
  in it, as it finds none in `x_queue`. The carried-conversations and
  place-status specs' own scenarios name the variable, and every use
  is test data passed through the fake, not code of the core. A
  stricter reading of core.1, as covering a program's variables too,
  would ask the scenarios to script a neutral variable name.
- **W3. The live walkthrough is not yet run.** S78's live walkthrough
  builds under `flume:lint`, but nothing has run it under herdr with
  the pinned Claude Code. That is the user's run, by hand after the
  change lands, as they ruled on 2026-11-02.
- **W4. The proposal's last sentence of "Why".** "The user reads this
  design before any code, and the change is built once they accept the
  amendment" still reads as written before the acceptance. Record
  0020's amendment and the paragraph above it say it is accepted, so
  nothing is left unsaid.
