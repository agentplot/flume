# A conversation lost with its terminal resumes its session on retry

## Why

deneb rebooted on 2026-11-06 at about 16:25Z, after its user lookups
failed. Six research places lost their talk agents, each the user's
conversation with a research agent (109), and flume reports each as
`waits on its caller to retry talk (lost)` (45, 77):

- `061708ef066d/research--the-tools-nested-life-cycles`
- `64e8990632a7/research--the-brand-mockups-are-revised`
- `a4061d73ce52/research--the-constitution-interview`
- `a8147ee247e9/research--effect-in-the-control-plane`
- `b3e4c15d5ebe/research--mcp-servers-for-flumes-agents`
- `ea85d06ebd3e/research--board-shows-flume-sluice-and-timber`

Each start's started ack names its session and its transcript, which is
still on disk under its account's `CLAUDE_CONFIG_DIR/projects/`. But
`flume retry --stage talk` starts a fresh agent with the stage's
prompt, so the conversation is gone from the place. flume carries a
conversation only for the start after an agent stopped at its
account's usage limit (146, 179); a retry carries none (30).

Since 17:00Z all six sessions run again outside flume: five the
orchestrator resumed by hand in herdr workspaces of their own, as
`claude --resume <session> --permission-mode bypassPermissions` on
another account with no `FLUME_*` and no flume plugin, and one herdr's
restore brought back bare. The user may be typing in any of them. A
retry must never start a second agent on a session a process runs.

The user wants the six back with their history. Record 0052, proposed,
decides how; the statement's amendment is on agentplot/blueprints'
branch `design/flume-a-lost-conversation-resumes-its-session` at
6b7d3fc, unpushed (30, 43, 45, 49, 56, 64, 77, 145, 147, 161, 179, 207,
C.4, C.5, S113 to S117). The change is built once the user accepts the
record and the amendment lands on blueprints main.

## What Changes

- **A retry of a conversation reported lost carries its conversation
  (49).** The retry marks the stage's start as carrying the lost
  start's conversation, as a spent agent's settle does. Its start
  resumes the lost agent's session by the carry part (179), under its
  own identity and secret (30, 33), and is the stage's only on its own
  word that it started in that session (161). Its prompt is flume's
  words, that its session was lost and goes on here and that what it
  had running then has stopped, then the retry's words, then the way
  to stop short (28).
- **No carried start beside a live session (30, 207).** A new part of
  the agent seam, live, reads whether a process of the program runs a
  conversation: for Claude Code, a `sessions/<pid>.json` naming the
  session in a home the host declares, whose process runs with its
  `pid` and `procStart`, or a `claude` whose arguments resume the
  session. A start that carries a conversation, after a loss or after a
  spent agent, opens only while none runs it. A lost conversation's
  retry is taken at once, and its stage waits as for room, never
  stalled, until a settle reads the session ended, while flume adopts,
  ends and sends nothing to the process.
- **The account that holds the conversation first (145).** The start
  takes the account the lost start ran under while it is usable, of the
  stage's program and in the stream's pool, moving no intent's
  affinity (147). Otherwise it takes the account 145 chooses, waiting
  for one as 145 says, and carries where that account reaches the
  conversation. A start under no pool carries where the login
  environment's configuration home reaches it.
- **Where the conversation cannot be carried, the stage starts afresh.**
  Its transcript gone, its start never acked, or no account it takes
  reaching it, the stage starts with its prompt, flume's words that its
  earlier agent was lost and its conversation could not be carried
  standing before the retry's. A carried start that does not begin is
  read as 146 reads one (S79).
- **A stage that is not a conversation starts afresh, as today.** A
  construction stage's retry carries nothing: its work is what the
  place holds, and its lost agent stopped at no turn's end.
- **An agent the terminal brought back bare in a conversation's pane
  is left for a person (77).** flume sends it nothing and does not end
  it; a caller's stop, drop or done ends it, and a retry waits for it to
  end. In a stage that is not a conversation it is ended once quiet, as
  today (S41).
- **A place kept while a process runs its conversation (64).** Its tidy
  keeps the worktree, `kept while its session runs`, naming the
  process.
- **Status, the line, the list and the end.** A waiting stage names each
  process running its session; its line reads `waiting on its session`
  and the list `waits on its session`. A start's conversation says
  whether the start it carries stopped at its account's limit or was
  lost, the account says it came from the conversation, and the lost end
  says where flume left an agent in its pane. The JSON form only gains
  fields and values (56, 61).
- **Docs.** The agent-settings and callers guides say what a retry of a
  lost conversation does, that it waits while a person's session runs,
  and that a person loses nothing they sent.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `carried-conversations`: a retry of a conversation reported lost
  carries its conversation on the account that holds it first, opens no
  carried start while a process runs the conversation, starts afresh
  where it cannot carry, and a stage that is not a conversation carries
  nothing.
- `caller-acts`: a lost conversation's retry is taken at once, ends
  nothing that runs its session, and waits for it to end.
- `held-state`: after a restart, an agent the terminal brought back in
  a conversation's pane is left for a person.
- `account-pools`: a lost conversation's start takes the account its
  lost start ran under first.
- `agent-profile`: Claude Code reads a session live from 2.1.296's
  `sessions/<pid>.json` and the host's processes, pinned to the release.
- `tidying`: a place is kept while a process runs its conversation.
- `end-delivery`: a lost conversation's end says flume left an agent
  in its pane.
- `place-status`: a waiting stage names each process running its
  session; a start's conversation names its cause; the account names the
  conversation.
- `place-line`: `waiting on its session`.
- `the-list`: `waits on its session`.
- `seam-contracts`: the agent seam's live, and the types that hold a
  lost conversation's carry, its account's choice, a live session and an
  agent left.
- `seam-fakes`: the fakes script a bare agent and a live session.
- `clause-coverage`: the suite's list at the revision that states 207
  and S113 to S117.

## Impact

- agentplot/blueprints: the statement's amendment, read by the build
  once on main (core.19). Nothing in weir changes, and no kind gains a
  field (core.4).
- `crates/flume-seams`: `Agent::live` with a default, `Live`, and
  `held.rs`'s carry marker, `RoomWait`, `Conversation`, `AccountFrom`,
  `StageEnd` and the status types, only added to.
- `crates/flume-core`: `acts.rs`'s retry, `start.rs`'s claim and
  `StartReason`, `room.rs`'s `start_waiting`, `accounts.rs`'s
  `choose_account`, `end.rs`'s lost reading, `uncarried.rs`, `tidy.rs`,
  `line.rs`, `list.rs`, `status.rs` and `deliver.rs`.
- `crates/flume`: the Claude Code profile's `live.rs` and its fragments,
  the person's form of status and of a notice, the stand-in's
  `sessions/<pid>.json`, and a walkthrough under the stand-in.
- `crates/flume-fakes`, `crates/flume-conformance`: the scenarios citing
  30, 45, 49, 64, 77, 145, 179, 207 and S113 to S117, and the clause
  list.
- `site/`: the agent-settings and callers guides and the state
  reference.
- `docs/adr/0052`, rules core.10, core.18 and core.32: written with this
  change.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.
