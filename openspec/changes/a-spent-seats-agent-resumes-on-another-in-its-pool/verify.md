# Verify: a-spent-seats-agent-resumes-on-another-in-its-pool

Checked on 2026-11-02 at `change/a-spent-seats-agent-resumes-on-another-in-its-pool`
de368b9. That head holds main at becc6ec (#84,
`chore/the-rules-and-records-match-the-statement`), merged at 68d2c0a,
and main's head is still becc6ec. The check covers this change's
proposal, design, nine specs and tasks, and the merge.

**The work holds, and the change is archived.** Every task is ticked,
and every requirement is built. Every one of the specs' 43 scenarios
has a test, the gate passes at de368b9 against main at becc6ec, and the
archived specs pass the strict check. The merged
`docs/architecture/core.md` differs from main's only by this change's
own words. The warnings below leave no requirement unmet.

## Tasks

All 21 tasks of the six groups are ticked, `[x]`, and none is open. The
gate run below backs each group's closing check task (1.3, 2.4, 3.4,
4.3, 5.2, 6.3, 6.4, 6.5).

Task 4.2 is ticked on its live walkthrough,
`s78_a_spent_agents_conversation_goes_on_under_the_pools_other_account`
in `crates/flume/tests/profiles/live/walkthroughs.rs`. It is written,
and it builds under `flume:lint`, which compiles the live harness under
`--all-features` and passes below. The user ruled on 2026-11-02 that
its run by hand under herdr with the pinned Claude Code is theirs, on a
test account, after the change lands. It was not run here: it needs a
test account's token, and no branch requires `check-live` (W2).

## The merge with main

Task 6.5 merged main at becc6ec, which conflicted in core.10's details.

- **core.md.** A word diff of `docs/architecture/core.md` from
  becc6ec to de368b9 gives the same words, in the same order, as the
  change's own diff from its cut, b6cd1b5, to a99c4c0. Those words are
  core.32's rule and details and core.10's details. Nothing of main's
  is lost, and nothing else is added.
- **core.10's details.** They hold both sides. Main's side gives "or
  from one session to another", "two agents in one place of a simple
  kind", the mode's cite "(161, 162, C.4, S67; record 0024, as amended
  on the user's ruling of 2026-10-31)", and the "Not ruled out" for a
  statechart kind's sessions side by side (30, 33, 175; record 0028).
  This change's side gives the "Not ruled out" for a start after a
  spent account (146, 179; record 0020's amendment, accepted
  2026-11-02). The Source cites 146, 175, 179 and S78, and records
  0020 and 0028. The two senses of "session" read apart. Main's "from
  one session to another" is a statechart kind's session (175). The
  carry's "the session it resumes" is the agent program's, as "its
  Claude Code session is the same" is earlier in the same line. The
  carry's words also call a carried start "a fresh process under its
  own start's identity and secret", so the line leaves no doubt that a
  carried start is a fresh agent.
- **The other files #84 changed.** Records 0012, 0024, 0027, 0028 and
  0029 and `docs/architecture/_open.md` are main's, byte for byte.
  `_open.md` holds no item on record 0020's amendment. This branch
  added that item and removed it at 6.2, so it nets to nothing against
  the cut, and main never held it. Record 0020 differs from main's only
  by its amendment's status line and section.
- **The specs.** 3c8f53a brought the change out of the archive with its
  specs unsynced, so `openspec/specs/` is main's, byte for byte.
- **The site's rules.** `site/src/content/docs/architecture/rules.md`
  differs from main's only by core.32's rule, as `core.md` amends it.

## Checks

- **The gate.** `sluice exec main` at de368b9, with the worktree
  clean, ran the checks de368b9 affects against main at becc6ec. It
  passed all 24 `#check-fast` and `#check-tools` tasks, none replayed
  from cache, and recorded their results for de368b9:
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

  An earlier run at de368b9 failed `plugin-claude:test` on one check
  of main's, `hooks.sh`'s "pre-tool-use outside flume's agent". The
  check exited 127 because `target/debug/flume` was not there when it
  ran. Its hook said nothing wrong. That run was not recorded, since
  this report was being written in the worktree during it. Run again
  with the worktree clean, the task passed. See "Outside the change".

  Task 6.3 also names `plugin-claude:validate` and `root:specs`. The
  branch's diff from main affects neither. Under `plugins/claude/` it
  changes only `tests/`, which the plugin's validation does not read,
  and it changes nothing under `openspec/specs/`. The archive commit
  makes `root:specs` affected, and the trial archive below is that
  check over the archived specs.
- **The archive.** `openspec archive a-spent-seats-agent-resumes-on-another-in-its-pool --yes`
  ran over a copy of `openspec/` at de368b9. It syncs every one of the
  34 specs byte for byte as 6d29816 synced them, and it creates
  `carried-conversations` with its Purpose as the change gives it.
  `openspec validate --specs --strict` over the result then passes 34
  of 34.
- **The statement.** The clause list reads 72410b2, which states 179
  as the carry part of Part B, and S78 and S79. 72410b2 is on
  agentplot/blueprints' `origin/main` as this host's clone last fetched
  it. That `origin/main` is now a15d927, one commit later. It amends
  4, 32, 145 and 147 and adds S80 and S81 (record 0020's next
  amendment: an intent keeps an account per agent program, and a pool
  that holds no account that can run a stage refuses it). It leaves
  146, 179, S78 and S79 as this change builds them. The clause-coverage
  spec asks for 72410b2 "or a later one", so the list meets it. See
  "Outside the change".

## Requirements and scenarios

The code, tests, site and specs are as they stood at a99c4c0, the head
an earlier check verified. `git diff a99c4c0 de368b9` changes only
`docs/adr/`, `docs/architecture/` and this change's `tasks.md` and
`verify.md`. Every test named below exists under its name at de368b9.
Each was matched to its scenario, and the test's assertions were read
against the scenario's WHEN and THEN. CC's S78, waiting, pool of one,
retry, S79, another session and fresh-start tests, and Claude Code's
`carry.rs`, were read again for this check. Every test named here
passes in the gate run above, except the live walkthrough (W2).

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
| carried-conversations | 15 | CC `s78_carried_to_the_pools_other_account`, `a_carried_start_waits_for_a_usable_account`, `a_pool_of_one_account`, `a_retry_carries_nothing`, `the_next_stage_carries_nothing`, `the_prompt_of_a_carried_start`, `accounts_that_share_nothing`, `a_profile_that_carries_nothing`, `s79_no_such_conversation`, `another_session`, `the_fresh_start_after_it_begins_otherwise`, and `by_the_limit_and_not_received` beside them; AC `a_conversation_carried_to_a_home_whose_projects_links_the_firsts`, `two_homes_of_their_own`, `a_start_that_never_said_it_started`; WT's two spent walkthroughs for the two scratch homes, each holding only what it held and the transcripts written | yes (W1) |
| agent-profile | 5 | AC `the_command_of_a_carried_start`, `a_background_task_the_stopped_agent_left`, `last_words_before_the_carried_agent_wrote_any`, `a_fresh_start_reads_its_whole_transcript`, `a_question_and_a_slash_command_of_the_stopped_agent`, `a_transcript_read_from_its_length`; `claude_words.rs` `the_pinned_release_resumes_as_the_carry_reads_it` | yes |
| claude-code-plugin | 4 | HC `a_carried_start_in_another_session`, `a_start_that_resumes_nothing`, `a_carried_starts_length_kept_at_a_compaction` (4,096 at the ack and kept at `compact`); `plugins/claude/tests/hooks.sh` with its `session-start-another-session.json` fixture | yes |
| place-status | 3 | PS `a_carried_agent`, `a_conversation_that_stayed`, `a_carried_start_that_did_not_begin`, in the JSON form; `print.rs` `a_carried_conversation`, `a_conversation_that_stayed`, `a_carried_start_not_begun`, `a_start_in_another_conversation`, `a_move_with_its_conversation_and_without` for the person's form | yes |
| seam-contracts | 4 | SA `carry_not_offered`, `carry_to_the_account_that_reaches_it`; RT `a_held_state_with_a_carried_start`, `a_held_state_written_before_carried_starts`, `a_start_begun_in_another_conversation_in_words`, `a_profile_that_carries_no_conversation` | yes (W1) |
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
  carry it, and core.10's cites 146, 179 and S78. The code, tests,
  specs and design cite 179, S78 and S79, the numbers 72410b2 gives
  them.
- **core.35.** The site's diff names none of the words core.35 rules
  out. The guide shows the link with `work-1` and `work-2` and
  `~/.claude-work-<n>` homes, an example of the reader's own.

## Warnings

- **W1. Claude Code's variable outside a profile.** The engine
  scenarios in CC, AP and PS, and RT's round trip, script the fake's
  `No` words naming `CLAUDE_CONFIG_DIR`. At b6cd1b5 no source of those
  crates named Claude in any case. core.1's check (`no_tool.rs`) reads
  `CLAUDE_CONFIG_DIR` as one token and, by design, finds no tool's name
  in it, as it finds none in `x_queue`. The carried-conversations and
  place-status specs' own scenarios name the variable, and every use
  is test data passed through the fake, not code of the core. A
  stricter reading of core.1, as covering a program's variables too,
  would ask the scenarios to script a neutral variable name.
- **W2. The live walkthrough is not yet run.** S78's live walkthrough
  builds under `flume:lint`, but nothing has run it under herdr with
  the pinned Claude Code. That is the user's run, by hand after the
  change lands, as they ruled on 2026-11-02.
- **W3. The proposal's last sentence of "Why".** "The user reads this
  design before any code, and the change is built once they accept the
  amendment" still reads as written before the acceptance. Record
  0020's amendment and the paragraph above it say it is accepted, so
  nothing is left unsaid.

## Outside the change

None of these bears on this change's requirements, and none holds
its archive.

- **The statement's next amendment.** a15d927 on blueprints'
  `origin/main` amends record 0020 again: an intent's affinity is kept
  per agent program, and a stage whose pool holds no account that can
  run it is refused rather than left waiting (4, 32, 145, 147, S80,
  S81). Nothing in this repository builds or cites it yet. The change
  that moves the clause list past 72410b2 lists S80 and S81, and builds
  them or names them unbuilt.
- **The site's rules mirror lags on main.** On main, as on this branch,
  `site/src/content/docs/architecture/rules.md` gives core.4 and
  core.10 in their words before #84, and lacks core.40, core.41 and
  core.42. This change mirrors its own core.32 and adds no lag.
- **`plugin-claude:test` races the crates' cargo runs.**
  `plugins/claude/tests/hooks.sh` builds `flume` and then runs
  `target/debug/flume` from the workspace's shared target directory.
  The task names no dependency on the crates' tasks, so `flume:test` or
  `flume:tools` can relink that binary while the hooks run, and a check
  then finds no binary, which fits the first run's failure above. It is
  main's as much as this branch's, and a later change can give the
  script a binary of its own, or have the task depend on the build.
