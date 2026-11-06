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
account's usage limit (146, 179); a retry carries none (30). herdr's
restore brought `research--effect-in-the-control-plane` back as a bare
`claude --resume` in the `flume` session, with no `FLUME_*`, and flume
ended it once it was quiet (77).

The user wants the six back with their history. Record 0052, proposed,
decides how; the statement's amendment is on agentplot/blueprints'
branch `design/flume-a-lost-conversation-resumes-its-session` at
1192a5a, unpushed (30, 43, 45, 49, 56, 77, 145, 147, 161, 179, C.4,
C.5, S113 to S116). The change is built once the user accepts the
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
  it. The lost end and status say flume left it. A caller's retry,
  stop, drop or done ends it, once quiet or forced (50 to 52, 114),
  and the retry reads the lost start's pane again first, so an agent
  the terminal brought back after the stage was read lost is ended too.
  In a stage that is not a conversation it is ended once quiet, as
  today (S41).
- **Status and the end.** A start's conversation says whether the start
  it carries stopped at its account's limit or was lost. The account
  says it came from the conversation. The agent and the lost end say
  where flume left an agent the terminal brought back. The JSON form
  only gains fields and values (56, 61).
- **Docs.** The agent-settings and callers guides say what a retry of a
  lost conversation does and what a person in a pane herdr brought back
  loses: nothing they sent.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `carried-conversations`: a retry of a conversation reported lost
  carries its conversation on the account that holds it first, starts
  afresh where it cannot, and a stage that is not a conversation
  carries nothing.
- `caller-acts`: retry carries a lost conversation's session and ends
  an agent the terminal brought back in its pane first.
- `held-state`: after a restart, an agent the terminal brought back in
  a conversation's pane is left for a person.
- `account-pools`: a lost conversation's start takes the account its
  lost start ran under first.
- `end-delivery`: a lost conversation's end says flume left an agent
  in its pane.
- `place-status`: a start's conversation names its cause, the account
  names the conversation, and the agent says it was left.
- `seam-contracts`: the types that hold a lost conversation's carry, its
  account's choice and an agent left.
- `seam-fakes`: the fake agent and terminal script a bare agent and a
  carry after a loss.
- `clause-coverage`: the suite's list at the revision that states
  S113 to S116.

## Impact

- agentplot/blueprints: the statement's amendment, read by the build
  once on main (core.19). Nothing in weir changes, and no kind gains a
  field (core.4).
- `crates/flume-seams`: `held.rs`'s carry marker, `Conversation`,
  `AccountFrom`, `StageEnd` and the status types, only added to.
- `crates/flume-core`: `acts.rs`'s retry, `start.rs`'s claim and
  `StartReason`, `accounts.rs`'s `choose_account`, `end.rs`'s lost
  reading, `uncarried.rs`, `status.rs` and `deliver.rs`.
- `crates/flume`: the person's form of status and of a notice; a
  walkthrough under the stand-in; no profile's code, since carry's
  check is the same.
- `crates/flume-fakes`, `crates/flume-conformance`: the scenarios citing
  45, 49, 77, 145, 179 and S113 to S116, and the clause list.
- `site/`: the agent-settings and callers guides and the state
  reference.
- `docs/adr/0052`, rules core.10 and core.32: written with this change.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.
