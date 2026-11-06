---
number: 52
title: A conversation lost with its terminal or its host goes on in its own session at its caller's retry
status: proposed
date: 2026-11-06
decision-makers:
- Chuck Swanberg
---

# A conversation lost with its terminal or its host goes on in its own session at its caller's retry

Status: proposed. It is stated on agentplot/blueprints' branch `design/flume-a-lost-conversation-resumes-its-session` at 1192a5a, cut from main at e4fab7b and not yet pushed (30, 43, 45, 49, 56, 77, 145, 147, 161, 179, C.4, C.5, S113 to S116). The construct stage of the change `a-lost-conversation-resumes-its-session` proposed it on 2026-11-06 from the brief that launched the change, which carries the user's want. It stays proposed until the user's yes.

Deciders: the user, who wants the six research conversations deneb lost on 2026-11-06 back with their history, as the brief carries it: "The user wants the six back with their history." The brief set the direction: "a retry of a stage whose kind keeps a conversation (`conversation = true`), lost with its terminal or its host, resumes the session its started ack names. It does that by carry's `--resume`, under a new start's identity, on the account that holds the transcript, as 179 does for a limit, and with the same check that the transcript is the same file." The writing agent's: what happens when that account is spent or gone, when the transcript is missing, why a stage that is not a conversation starts afresh, what flume does with an agent the terminal brought back bare in a conversation's pane, the words the agent is given, and what status and the end say.
Sources: `agentplot/blueprints:design/flume/requirements.md` at 1192a5a, clauses 30, 31, 35, 39, 43, 45, 49 to 52, 56, 77, 109, 114, 145 to 147, 161, 179, C.4 and C.5, and S12, S41, S78, S79 and S113 to S116; records 0016, 0018 and 0020 with its amendments; rules core.10 and core.32; `crates/flume-core/src/acts.rs` (`Engine::retry`), `crates/flume-core/src/start.rs` (`Engine::claim`, `StartReason`), `crates/flume-core/src/accounts.rs` (`Engine::choose_account`), `crates/flume-core/src/end.rs` (`Engine::read_end`, `Engine::end_foreign`), `crates/flume-core/src/uncarried.rs`, `crates/flume-seams/src/held.rs` (`AfterSpent`, `Conversation`, `AccountFrom`) and `crates/flume/src/profiles/agent/claude/carry.rs`; the held state of `a8147ee247e9/research--effect-in-the-control-plane` on deneb, its talk lost with its started ack naming a session and a transcript under its account's `CLAUDE_CONFIG_DIR`; deneb's configuration homes, each `projects` a link to one folder; herdr 0.9.5's `resume_agents_on_restore` and its Claude Code integration's `agent_session_id` and `resume_argv`.

## Context and problem statement

deneb rebooted on 2026-11-06 at about 16:25Z, after its user lookups failed. Six research places lost their talk agents, each a conversation (109) between the user and a research agent. flume read each talk lost (45, 77). Each start's started ack names its session and its transcript, which is still on disk under its account's configuration home. herdr's restore brought one of them back as a bare `claude --resume` in the `flume` session, with no `FLUME_*`, and flume ended it once it was quiet, as 77 says.

`flume retry --stage talk` starts a fresh agent with the stage's prompt (49). The user's conversation is then gone from the place. flume carries a conversation only for the start after an agent stopped at its account's usage limit (146, 179). A retry, a fix round and a start after one begun without its settings carry none (30; record 0020).

## Considered options

Which retry carries:

1. **A retry of a conversation's stage reported lost** carries the lost agent's conversation where its program can carry it (179).
2. **A retry of any stage reported lost** carries it, a construction stage's among them.
3. **Every retry of a conversation** carries its last start's conversation, one stopped by its caller or let go after its stop-short among them.

Which account a carried retry takes:

4. **The account the lost start ran under, while it is usable,** and otherwise the account 145 chooses, carrying where that account reaches the conversation and starting afresh where none does.
5. **The account 145 chooses**, as a start after a spent agent takes one.
6. **The account the lost start ran under alone,** waiting for its reset, or refusing the retry, while it is not usable.

An agent the terminal brought back bare in a conversation's pane:

7. **Leave it in its pane for a person,** say so in status and the end, and let a caller's retry, stop, drop or done end it.
8. **Adopt it** as the stage's agent.
9. **End it once quiet,** as 77 says of every such agent.

## Decision outcome

Options 1, 4 and 7.

1. **A retry of a conversation reported lost carries its conversation (49).** Its start is one fresh agent under its own identity and secret (30, 33), resuming the lost agent's session by the carry part (179), with the settings and the standing prompt resolved at that start (32, 148). It is the stage's only on its own word that it started in the session its record names (35, 161). Its prompt is flume's words, then the retry's words where it has any, then the way to stop short (28). The words say that its session was lost and goes on here, and that anything it had running then has stopped. A retry counts as a start of the stage as any retry does (48), and the stages after it run again in order (49).
2. **The account that holds the conversation first (145).** The start takes the account the lost start ran under while it is usable, of the stage's program and in the stream's pool as the host's configuration stands then. Its configuration home holds the transcript, so the carry needs no shared `projects`. Taking it moves no intent's affinity (147), and status gives the account as the conversation's (56). Where that account is spent, past its threshold, full or gone from the pool, the start takes the account 145 chooses, the affinity first, waiting for one as 145 says. It carries where that account reaches the conversation by 179's check, the transcript the same file by device and inode, as a host that shares `projects` makes it (C.4). The stage's move between accounts says whether its conversation went with it (56).
3. **Where the conversation cannot be carried, the stage starts afresh.** Its transcript gone, its start never acked, or no account it takes reaching it, the stage starts with its prompt as for any retry. flume's words stand before the retry's: its earlier agent was lost and its conversation could not be carried, and what it committed and left is in the place. Status says why, naming `CLAUDE_CONFIG_DIR` and no path (56; core.30). A carried start never begun (35), as when its program finds no such conversation, or whose agent came up in another session (161), is read as 146 reads one. flume ends its agent once quiet and starts the stage afresh at once, with the same words, waiting on no caller.
4. **A stage that is not a conversation starts afresh.** Its retry carries nothing, and its agent starts from what the place holds: the commits, the change and what the agent left in the worktree (39). A construction stage's work is what its agents committed and left. Its lost agent stopped at no turn's end, killed with its terminal or its host while a tool ran, an edit was half written or a server or a background task of its own ran. Its conversation then tells of work the place may no longer hold, and no person reads it with the agent to set it right. A fresh agent reads the place as it is. A start after a spent agent is different: it stopped at a turn's end, with the place as its conversation says, and 146 carries it.
5. **An agent the terminal brought back bare in a conversation's pane is left for a person (77).** It is never the stage's agent (35, 161). flume sends it nothing, leaves it in its pane and does not end it, since a person may be talking in it. Status names it with the agent, and the lost stage's end says that flume left it (43, 56). A caller's retry ends it first, once it is quiet or when forced (50), as a stop, a drop or a done does (51, 52, 114). herdr's restore resumes the agent's own session by the session id its integration reported, under the login environment's configuration home, whose `projects` reaches the transcript on a host that shares the folder (C.5). What a person says to it is then in that transcript, and the retry's carry goes on with it. An unsent draft in its input stays the person's until the retry, which they or their caller choose the time of. A bare agent in a stage that is not a conversation is ended once quiet, as before (S41).

## Why not the others

Option 2 resumes a construction stage's conversation after a kill at no turn's end, as section 4 says. Option 3 overrides a caller who stopped a conversation on purpose, and a conversation let go after its stop-short's hour (198) is the stop-short's own question. Neither was asked, and each can be decided apart.

Option 5 sends a lost conversation to the affinity's or the strategy's account, which on a host whose accounts share nothing never reaches it. The account the lost start ran under always holds the transcript. Option 6 holds the person's talk for the hours, or the days of a weekly limit, until one account resets, with no way to start afresh but a drop. A pool exists so that a stage waits on no one account (record 0020, option 5). A host that wants every lost conversation to go on under any account shares `projects` once, as it does for a spent agent.

Option 8 takes an agent flume cannot read or guard as the stage's. It runs under the login environment rather than its account, with no identity, secret, guard, stop check or acks (35, 41, 116, 161), and 77, 161 and core.10 rule it out. Option 9 closes a pane a person may be typing in, at the first moment herdr reads it idle, which an unsent draft does not change. What the person sent is in the transcript either way, so option 7 loses nothing that option 9 keeps.

## Consequences

- Good: a conversation lost with its terminal or its host comes back whole at its caller's retry, on the account that holds it, on any host.
- Good: what a person says to an agent herdr brought back goes on in the conversation the retry carries.
- Good: a construction stage's retry is as before.
- Bad: an agent the terminal brought back in a conversation's pane runs under the login environment until a caller ends it, outside every pool. Status names it, and the place waits on its caller and is said stalled after five minutes (130).
- Bad: a conversation whose account is spent, on a host whose accounts share nothing, starts afresh at its retry. Its transcript stays on disk, and status says why.
- The statement's 30, 43, 45, 49, 56, 77, 145, 147, 161 and 179, C.4's carry row and C.5's herdr notes are amended, and S113 to S116 added, on blueprints' branch `design/flume-a-lost-conversation-resumes-its-session` at 1192a5a (core.19). S109 to S112 are the settles design's, held for the user's read.
- Rules core.10 and core.32 are amended with this record.
- The change `a-lost-conversation-resumes-its-session` builds it. Once it lands, the caller retries deneb's six research talks.
