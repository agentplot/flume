# Verify: a-lost-conversation-resumes-its-session

The verify stage's report on `change/a-lost-conversation-resumes-its-session`
at c8aa98d, against the change's proposal, design, delta specs and tasks.

## Verdict

The work holds. Every requirement and scenario of the change's thirteen
delta specs is met by a test that asserts it, every task is ticked, and
the gate passes over the branch's head. The change is archived with
this report.

## The base and the statement

- main at 4b3d2d1 is an ancestor of the branch: main had not moved
  since task 6.1 merged it, and a fetch of origin's main read the same
  revision.
- `crates/flume-conformance/src/clauses.rs` reads agentplot/blueprints
  at c94c496. The statement there, read for this report, states 207
  (live) in Part B and S113 to S117 in section 11, and holds no 202 to
  206 and no S109 to S112, as the list's gaps and skips say (106;
  core.19).
- Record 0052 is `accepted`, on the user's card in Pending You on
  2026-11-06, and rules core.10, core.18 and core.32 cite it.

## Tasks

Every task of groups 1 to 6 is ticked, each with its check named in the
commits that ticked it.

## Requirements and scenarios

Each scenario of the delta specs maps to a test that asserts its THEN.
The engine's scenarios are in `crates/flume-conformance/src/engine/`,
run over the fakes; the profile's in `crates/flume/tests/profiles/`.

| spec | scenario | test |
|---|---|---|
| account-pools | The lost start's account while the intent keeps another | `account_pools.rs` `the_lost_starts_account_while_the_intent_keeps_another` |
| account-pools | The lost start's account spent | `account_pools.rs` `the_lost_starts_account_spent` |
| account-pools | The lost start's account gone from the pool | `account_pools.rs` `the_lost_starts_account_gone_from_the_pool` |
| account-pools | Every account spent | `account_pools.rs` `every_account_spent_at_a_lost_talks_retry` |
| agent-profile | A session record and its process | `agent_claude.rs` `a_session_record_and_its_process` |
| agent-profile | A reused process id | `agent_claude.rs` `a_reused_process_id` |
| agent-profile | A resume under a home not given | `agent_claude.rs` `a_resume_under_a_home_not_given` |
| agent-profile | A sessions folder that cannot be read | `agent_claude.rs` `a_sessions_folder_that_cannot_be_read` |
| agent-profile | The pinned release's session record read | `tools/claude_words.rs` `the_pinned_release_names_its_running_sessions_as_live_reads_them` (check-tools) |
| caller-acts | A bare agent at work in a lost talk's pane | `caller_acts.rs` `a_bare_agent_at_work_in_a_lost_talks_pane` |
| caller-acts | A session run in a pane flume did not make | `caller_acts.rs` `a_session_run_in_a_pane_flume_did_not_make` |
| carried-conversations | A retry carries nothing | `carried_conversations.rs` `a_retry_carries_nothing` |
| carried-conversations | The next stage carries nothing | `the_next_stage_carries_nothing` |
| carried-conversations | A lost code stage's retry carries nothing | `s116_a_lost_code_stages_retry_carries_nothing` |
| carried-conversations | A stopped conversation's retry carries nothing | `a_stopped_conversations_retry_carries_nothing` |
| carried-conversations | The prompt of a carried start | `the_prompt_of_a_carried_start` |
| carried-conversations | The prompt of a lost conversation's carried start | `the_prompt_of_a_lost_conversations_carried_start` |
| carried-conversations | No such conversation | the S79 scenario, "a carried start whose program finds no such conversation" |
| carried-conversations | Another session | `another_session` |
| carried-conversations | The fresh start after it begins otherwise | `the_fresh_start_after_it_begins_otherwise` |
| carried-conversations | The second account's projects a link to the first's | `agent_claude.rs` `a_conversation_carried_to_a_home_whose_projects_links_the_firsts` |
| carried-conversations | Two homes of their own | `agent_claude.rs` `two_homes_of_their_own` |
| carried-conversations | A start that never said it started | `agent_claude.rs` `a_start_that_never_said_it_started` |
| carried-conversations | A lost conversation on its own account | `agent_claude.rs` `a_lost_starts_conversation_carried_to_its_own_home` |
| carried-conversations | A lost conversation whose transcript is gone | the same test, the transcript removed: no with `UNREADABLE` |
| carried-conversations | A talk lost with its host comes back on its own account | `s113_a_talk_lost_with_its_host_comes_back_on_its_own_account` |
| carried-conversations | A lost talk waits for room and carries when it starts | `a_lost_talk_waits_for_room` |
| carried-conversations | A lost conversation's carried start that does not begin | `a_lost_conversations_carried_start_that_does_not_begin` |
| carried-conversations | Its transcript gone | `s115_a_lost_talk_whose_transcript_is_gone` |
| carried-conversations | Another account that does not reach it | `s114_a_lost_talk_whose_account_is_spent` |
| carried-conversations | A lost start that never acked | `a_lost_start_that_never_acked` |
| carried-conversations | A host with no pool | `a_host_with_no_pool` |
| carried-conversations | A lost talk whose session a person runs by hand | `s117_talks_resumed_by_hand_wait_for_their_sessions` |
| carried-conversations | The session ends | `the_session_ends` |
| carried-conversations | A session that cannot be read | `a_session_that_cannot_be_read` |
| carried-conversations | A spent agent's session resumed by hand | `a_spent_agents_session_resumed_by_hand` |
| carried-conversations | A lost talk that cannot be carried, its session still live | `a_lost_talk_that_cannot_be_carried_its_session_still_live` |
| clause-coverage | The clauses listed | `clauses.rs`'s test, at c94c496 |
| clause-coverage | A clause left out | `conformance:coverage`'s own test, unchanged |
| clause-coverage | The suite as this change leaves it | `conformance:coverage` over the suite |
| end-delivery | A lost talk with a bare agent | `end_delivery.rs` `a_lost_talk_with_a_bare_agent` |
| end-delivery | A lost talk with no agent | `end_delivery.rs` `a_lost_talk_with_no_agent` |
| held-state | A bare agent in a conversation's pane | `restart.rs` `a_bare_agent_in_a_conversations_pane` |
| held-state | A bare agent in a code stage's pane | `restart.rs` `a_bare_agent_in_a_code_stages_pane` |
| held-state | A stop ends it | `restart.rs` `a_stop_ends_the_agent_left` |
| place-line | A lost talk retried while its session runs | `place_line.rs` `a_lost_talk_retried_while_its_session_runs` |
| place-status | A carried agent | `place_status.rs` `a_carried_agent` |
| place-status | A conversation that stayed | `place_status.rs` `a_conversation_that_stayed` |
| place-status | A carried start that did not begin | `place_status.rs` `a_carried_start_that_did_not_begin` |
| place-status | A lost talk carried | `place_status.rs` `a_lost_talk_carried` |
| place-status | The conversation's account | `place_status.rs` `the_conversations_account` |
| place-status | A talk waiting on its session | `place_status.rs` `a_talk_waiting_on_its_session` |
| place-status | A bare agent left | `place_status.rs` `a_bare_agent_left` |
| seam-contracts | A profile that offers no carry | `seams/agent.rs`'s carry scenarios, unchanged |
| seam-contracts | The fake's carry over the seam's scenarios | `seams/agent.rs`'s carry scenarios, unchanged |
| seam-contracts | A held state with a carried start | `round_trip.rs` `a_held_state_with_a_carried_start` |
| seam-contracts | A held state written before | `round_trip.rs` `a_held_state_written_before_carried_starts` |
| seam-contracts | A lost conversation's wait and conversation | `round_trip.rs` `a_lost_conversations_wait_and_conversation` |
| seam-contracts | An account from the conversation and an agent left | `round_trip.rs` `an_account_from_the_conversation_and_an_agent_left` |
| seam-contracts | A profile that reads no running session | `round_trip.rs` `a_profile_that_reads_no_running_session`; `seams/agent.rs` `live_not_read` |
| seam-contracts | The fake's live over the seam's scenarios | `seams/agent.rs` `live_reads_the_processes` |
| seam-contracts | A wait on a live session | `round_trip.rs` `a_wait_on_a_live_session` |
| seam-fakes | A bare agent brought back | `fakes.rs` `a_bare_agent_brought_back` |
| seam-fakes | A scripted live session that ends | `fakes.rs` `a_scripted_live_session_that_ends` |
| the-list | A talk waiting on its session | `list.rs` `a_talk_waiting_on_its_session` |
| tidying | A dropped place whose session a person runs | `tidying.rs` `a_dropped_place_whose_session_a_person_runs` |

Design 13's walkthroughs, in the `check-tools` layer under the stand-in
for Claude Code, are
`a_talk_resumed_by_hand_waits_at_its_retry_and_goes_on_in_its_session_once_it_ends`
and `a_lost_talk_whose_transcript_is_gone_starts_afresh_with_flumes_words`
in `tools/walkthroughs.rs`.

## The code read against the design

- `Engine::retry` ends no agent where the stage named is a conversation
  read lost with a start, and writes its wait with a `CarryFrom` of
  cause `lost` and the retry's words under the room lock; every other
  retry ends the place's agent and clears the marker as before (30, 49,
  77).
- `claim` asks `live` of every home the host's pools declare for the
  program and the login environment's, before a carried start and a
  lost conversation's fresh start, and leaves the stage waiting with
  what it read; it also holds the start while the agent the terminal
  brought back in the lost start's pane stands. `start_waiting` asks
  again at each settle whatever the room, and the queue passes over a
  stage so held (49, 70, 207).
- `choose_account` takes the lost start's account first where its pool
  declares it, of the program and usable, as `conversation`, moving no
  affinity; under no pool, carry is asked with an empty environment
  (145, 147, 179).
- `read_end` leaves an agent brought back bare in a conversation's
  pane, sending nothing, its start going not live once the terminal
  reads it stopped, and writes `left_in_pane`; a stage that is not a
  conversation is read as before (77; core.10).
- `tidy` keeps a place whose last conversation start's session live
  reads running, or cannot read, as `SessionRuns` with its processes
  (64, 207; core.18).
- Claude Code's `live.rs` reads `sessions/<pid>.json` matched by
  `sessionId`, counted only where the `pid` runs started at its
  `procStart`, and any `claude` resuming the session by `--resume`,
  `--resume=` or `-r` from one listing of the host's processes; unknown
  names `CLAUDE_CONFIG_DIR` or the host's processes and no path (207,
  C.4; core.30).

## Checks

- `openspec validate a-lost-conversation-resumes-its-session --strict`:
  valid.
- Each requirement the delta specs modify or rename stands under its
  old name in `openspec/specs/`, so the archive applies.
- `sluice exec main` at c8aa98d against main at 4b3d2d1: passed, 23
  tasks of 24 run, `flume:tools` 104 of 104 in the `check-tools` layer,
  the two walkthroughs above among them, and `flume:test`,
  `core:test`, `seams:test`, `fakes:test`, `conformance:test`,
  `conformance:coverage`, `site:prose` and `site:build` with them.
  sluice recorded no result for the run, since this report stood
  uncommitted in the worktree beside it.
