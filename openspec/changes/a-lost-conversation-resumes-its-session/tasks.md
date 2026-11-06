# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main and on the
groups before it. The change is built once the user accepts record 0052
and the statement's amendment, on agentplot/blueprints' branch
`design/flume-a-lost-conversation-resumes-its-session` at 1192a5a,
lands on blueprints main; the build reads the statement there. A
group's last task runs the checks it touched through sluice. Every
conformance scenario a group adds cites its clauses as data (106). The
engine's scenarios name no tool (core.1). Tests use fixtures only: no
real account, no configuration home or herdr session of the host's, and
no host configuration of the user's. Every configuration home, session
home and state directory a test names is a scratch one of the test's
own, and a herdr session a test starts is named by one letter of the
test's own.

## 1. The decision recorded

- [x] 1.1 Add record 0052, proposed, "A conversation lost with its terminal or its host goes on in its own session at its caller's retry", quoting the brief's words of 2026-11-06 and the user's want it carries, with its deciders, sources (the statement on blueprints' branch at 1192a5a; records 0016, 0018 and 0020), context, nine considered options, outcome, why not the others and consequences (30, 43, 45, 49, 56, 77, 145, 147, 161, 179, C.4, C.5, S113 to S116; core.19). True when `sluice exec main -- moon run root:adrs root:format` passes.
- [x] 1.2 Amend core.10 (an agent the terminal brought back bare in a conversation's pane ended only by its caller's act) and core.32 (the account a lost conversation's start ran under first; no retry but a lost conversation's carries) in `docs/architecture/core.md`, each with its details citing record 0052, and have `site/src/content/docs/architecture/rules.md` mirror both rules (core.19, core.35). True when `sluice exec main -- moon run root:format site:prose site:build` passes.
- [x] 1.3 Run the checks of the projects this group touched. True when `openspec validate a-lost-conversation-resumes-its-session --strict` and `sluice exec main -- moon run root:format root:adrs root:changes site:prose site:build` pass.

## 2. The seams and their fakes

- [ ] 2.1 In `crates/flume-seams/src/held.rs`, rename `AfterSpent` to `CarryFrom` and give it `cause: CarryCause` (`spent`, `lost`), read as `spent` where absent; give `Conversation` the same `cause`; give `AccountFrom` `Conversation`, written `conversation`; give `StageEnd` and the status agent `left_in_pane`, skipped where false. Re-export them, and add to `round_trip.rs` the seam-contracts spec's scenarios: a lost conversation's wait and conversation, a document with no cause read as spent, and an account from the conversation with an end that left an agent (43, 49, 56, 77, 145, 179; core.14, core.16). True when `sluice exec main -- moon run seams:test core:test conformance:test` passes, every existing test passing unchanged but for the type's new name.
- [ ] 2.2 Say in `Agent::carry`'s documentation that the start given is one stopped at its account's limit or a conversation's start reported lost, and that the environment is empty under no pool; the signature stays (49, 179). Keep the seam's carry scenarios in `flume-conformance`'s `seams/agent.rs` citing 49 beside 179. True when `sluice exec main -- moon run seams:test conformance:test` passes.
- [ ] 2.3 Give the fakes `bring_back_bare(identity)`, as design 9 has it: the fake terminal's `resume` and the fake agent's started ack read no, with the log line `bring back bare <identity>`; and add the seam-fakes spec's scenario to `crates/flume-fakes/tests/fakes.rs` (77, 106). True when `sluice exec main -- moon run fakes:test` passes.
- [ ] 2.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test conformance:format conformance:lint conformance:test core:test` passes.

## 3. The core

- [ ] 3.1 In `Engine::retry` (`acts.rs`), where the stage named is declared `conversation` and reads `Lost` with a last start, read that start's agent again through the terminal its start recorded where the start is not live, and end an agent standing there as `end_agent` ends one, refused naming it where it is not quiet and the retry is not forced; then write the stage's `RoomWait` under the room lock with the next order, the retry's words and a `CarryFrom` naming the lost start with the cause `lost`, before `start_stage`. Every other retry clears the marker as today (30, 49, 50, 77). True when `sluice exec main -- moon run core:test conformance:test` passes, `engine/caller_acts.rs` holding the caller-acts spec's two new scenarios.
- [ ] 3.2 In `choose_account` (`accounts.rs`), for a marker whose cause is `lost`, take the lost start's `StartHeld::account` first where the stream's pool declares it, of the stage's program and usable, as `AccountFrom::Conversation` and with no intent's move; otherwise choose as today. In `claim` (`start.rs`), ask carry under `Choice::NoPool` with an empty environment where the cause is `lost` (design 3, 4; 145, 147, 179). True when `sluice exec main -- moon run core:test conformance:test` passes, `engine/account_pools.rs` holding the account-pools spec's four new scenarios.
- [ ] 3.3 Give `StartReason` `Resumed` and `Lost` with design 5's words. Build a lost conversation's carried prompt from `Resumed`'s words, the retry's words and the way to stop short, and its fresh start's `{{words}}` from `Lost`'s words before the retry's. Have `uncarried.rs`'s `after_uncarried` keep the marker's cause and the fresh start's words follow it, and record `Conversation::cause` from the marker (27, 28, 49, 146, 179, S79). True when `sluice exec main -- moon run core:test conformance:test` passes, `engine/carried_conversations.rs` holding the carried-conversations spec's new and modified scenarios, the one S113 cites named `s113_a_talk_lost_with_its_host_comes_back_on_its_own_account`, S114's `s114_a_lost_talk_whose_account_is_spent`, S115's `s115_a_lost_talk_whose_transcript_is_gone` and S116's `s116_a_lost_code_stages_retry_carries_nothing`.
- [ ] 3.4 In `Engine::read_end` (`end.rs`), for a stage declared `conversation` read lost with an agent that is not the stage's still standing, leave it, closing and sending nothing, keeping its start live until the terminal reads it stopped, and write `left_in_pane` on the end; a stage that is not a conversation is read as today (31, 43, 77; core.10). True when `sluice exec main -- moon run core:test conformance:test` passes, `engine/restart.rs` holding the held-state spec's two new scenarios and `engine/end_delivery.rs` the end-delivery spec's two.
- [ ] 3.5 Give status the start's conversation's cause, the account from the conversation, and the agent's `left_in_pane`, read for a lost conversation through the terminal its last start recorded whether or not that start is live; give the lost end's notice its words where it left an agent; and add each field to `engine/answers.rs`'s field list (43, 56, 61, 77, 145; core.14, core.15). True when `sluice exec main -- moon run core:test conformance:test` passes, `engine/place_status.rs` holding the place-status spec's new and modified scenarios.
- [ ] 3.6 Run the checks of the projects this group touched. True when `sluice exec main -- moon run core:format core:lint core:test conformance:format conformance:lint conformance:test` passes.

## 4. The profiles and the command

- [ ] 4.1 Give `crates/flume/src/commands/print.rs` the person's form of design 8: "carries the conversation of start 1, lost", "starts afresh after start 1, lost: <why>", "<account> (its conversation's)", and the agent brought back and left in its pane with what a retry does; and the notice's words for an end that left an agent (43, 56, 77, 145; core.35). True when `sluice exec main -- moon run flume:test` passes, `print.rs`'s tests holding each form.
- [ ] 4.2 Hold Claude Code's carry for a lost start in `crates/flume/tests/profiles/agent_claude.rs` over scratch configuration homes: yes on the lost start's own home that shares `projects` with none, and no with `UNREADABLE`'s words once its transcript is removed, naming `CLAUDE_CONFIG_DIR` and no path (49, 179, C.4, S115; core.30). True when `sluice exec main -- moon run flume:test` passes.
- [ ] 4.3 Add design 10's walkthrough in the `check-tools` layer: under a scratch host and a kind of the test's own whose one stage is a conversation, the stand-in for Claude Code killed in its pane, the stage read lost, and its retry's start holding `--resume` and the lost start's session, the stand-in acking in that session and appending to the same transcript file by device and inode; then the same with the transcript removed first, the stage starting afresh with `Lost`'s words (45, 49, 77, 179, S113, S115). True when `sluice exec main -- moon run flume:tools` passes.
- [ ] 4.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test flume:tools` passes.

## 5. The site and the clause list

- [ ] 5.1 Say in `site/src/content/docs/guides/agent-settings.md`'s "The conversation carried" that a retry of a conversation reported lost carries it on the account that holds it first, starts afresh where it cannot and why, and that no other retry carries one; in `guides/callers-and-holds.md`'s `<stage> lost` that a lost conversation's retry goes on in its session and that an agent the terminal brought back in its pane is left until the retry ends it, so a person loses nothing they sent; and in `reference/state.md` the fields `cause`, `conversation` as an account's `from`, and `left_in_pane`; each to any host's operator (43, 49, 56, 77, 145; core.35). True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 5.2 Have `crates/flume-conformance/src/clauses.rs` read blueprints main at the revision that lands record 0052's amendment, or the later revision of main the build reads, with S113 to S116 among the scenarios, and S109 to S112 and 202 to 206 named unbuilt by the change that builds them where that revision holds them, as the clause-coverage spec has it (106; core.19). True when `sluice exec main -- moon run conformance:test conformance:coverage` passes, S113 to S116 each cited by this change's scenarios.
- [ ] 5.3 Run the checks of the projects this change touched. True when `sluice exec main -- moon run root:format root:adrs root:specs root:changes seams:test fakes:test core:test conformance:test conformance:coverage flume:test flume:tools site:prose site:build` passes.

## 6. The gate

- [ ] 6.1 Merge main into this branch where it has moved, resolving any file it contests, and run the gate over the merged tree. True when `openspec validate a-lost-conversation-resumes-its-session --strict` and `sluice exec main` pass.
