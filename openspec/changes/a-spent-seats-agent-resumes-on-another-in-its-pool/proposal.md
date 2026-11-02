# A spent account's agent goes on with its conversation

## Why

When an agent's turn fails at its account's usage limit, flume records
the account spent, ends the agent once it is quiet, and starts its stage
afresh on another account of its pool (record 0020; 146). The fresh
agent reads only what the earlier one committed and left in the place.
A stage half way through its work loses what its agent had worked out
and not yet written down. A conversation (109), an agent planning or
researching with its person, loses the talk itself.

On 2026-11-02 the user asked that the agent keep its conversation and
go on where it stopped. Record 0020's amendment, proposed the same day,
decides how. The start after a spent agent resumes that agent's own
conversation on the account the pool chooses, where its program can
carry it there, and starts afresh otherwise, status saying which. The
statement's amendment is on blueprints' branch
`design/flume-a-spent-seats-agent-resumes` at ec16d03 (30, 56, 88, 146,
161, 171, 175, C.4, S55, S76, S77). The user reads this design before
any code, and the change is built once they accept the amendment.

## What Changes

- **carry, a part of the agent seam (175).** Given the start whose
  agent stopped at its account's limit and the environment of the
  account chosen for the next start, a profile answers whether its
  program can carry that agent's conversation there, with the session
  to resume, or why not, or that it gives no way to, the default.
  `command` takes the session a start resumes.
- **The start after a spent agent carries its conversation.** The
  settle that reads an agent spent marks the stage's wait for room with
  the start that stopped. The claim asks the profile's carry of the
  account it chose, under the room lock. Where the answer is yes, the
  start resumes the session, its record names the session, and its
  prompt is flume's words alone with the way to stop short. Otherwise
  it starts afresh as today. A retry, a fix round and a start after one
  begun without its settings carry nothing (30, 146).
- **Claude Code's carry.** Yes where the earlier start's transcript is
  the same file, by device and inode, as the chosen account's
  `<home>/projects/<folder>/<file>`, `<home>` its `CLAUDE_CONFIG_DIR`,
  else `~/.claude`. The command then adds `--resume <session>`. flume
  copies, links and writes nothing in a configuration home (C.4).
- **The session compared at the started ack.** SessionStart compares
  the session it runs with the one the start's record names. A start
  that came up in another session began otherwise than its record
  (161).
- **Each start reads only its own part of its transcript.** The
  started ack records the transcript's length when it first came, and a
  later SessionStart keeps it. Own work, last words, asking's answer and
  a slash command's receipt read only what follows. Otherwise the
  earlier agent's unfinished background task would keep the carried
  agent from ever reading quiet (31, 91, C.4).
- **A carried start that does not begin.** Never begun, or begun in
  another session, it is not delivered. Its agent is ended once quiet
  and the stage starts afresh at once, counting no bound (146, S77).
- **Status.** The agent says whether it carries the conversation of
  the stage's start before it, or why not. A stage's move says whether
  its conversation went with it. No configuration home's path is given
  (56; core.30).
- **The stand-in** resumes a session by `--resume`, as the pinned
  2.1.296 does, and refuses one it cannot find.
- **Docs.** The site says how a host shares its accounts' `projects`
  folder, what that shares, and what status says.

## Capabilities

### New Capabilities

- `carried-conversations`: the carry part, which start carries a
  conversation, a carried start's prompt and record, the fresh start
  when a conversation cannot be carried or its start does not begin,
  and Claude Code's carry over a shared `projects` folder.

### Modified Capabilities

- `account-pools`: a spent agent's stage starts again on the account
  chosen, carrying its conversation where its program can, afresh
  otherwise.
- `start-integrity`: a start's record names the session it resumes,
  and the started ack is taken only in that session.
- `agent-profile`: Claude Code's `--resume`, its readings of a start's
  own part of its transcript, and its carry's facts pinned to the
  release the tree holds.
- `claude-code-plugin`: SessionStart compares the session and records
  the transcript's length once.
- `place-status`: the agent's conversation and a move's.
- `seam-contracts`: the agent seam's carry, `command`'s session and the
  types that hold a carried start.
- `seam-fakes`: the fake agent scripts carry and a carried start's
  session.
- `clause-coverage`: the suite's list at the revision that states 175,
  S76 and S77.

## Impact

- agentplot/blueprints: the statement's amendment at ec16d03 on the
  branch `design/flume-a-spent-seats-agent-resumes`, for the user's
  ruling. The build reads it from blueprints main once ruled (core.19).
  Nothing in weir changes, and no kind gains a field (core.4).
- `crates/flume-seams`: `Carry`, `Agent::carry`, `Agent::command`'s
  `resume`, `StartRecord::session`, `Unsettled::session`,
  `RoomWait::carry`, `AfterSpent`, `StartHeld::conversation`,
  `Conversation`, `AccountMove::carried`, `UncarriedStart`, status
  fields, only added.
- `crates/flume-core`: `spent.rs`, `start.rs`'s claim and
  `StartReason::Carried`, a new `uncarried.rs`, `deliver.rs`,
  `unsettled.rs` and `status.rs`.
- `crates/flume`: the Claude Code profile's `carry`, `command`,
  `record.rs`, `hook.rs` and `transcript.rs`; the person's form of
  status; the stand-in; the walkthroughs, the `check-tools` fragments
  test and one live walkthrough.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake agent's
  carry, the seam's and the engine's scenarios citing 175, S76 and S77,
  and the clause list.
- `plugins/claude`: the hooks' tests of the session's comparison.
- `site/`: the agent-settings and Claude Code guides and the state
  reference.
- `docs/architecture/_open.md`: the item on record 0020's amendment
  leaves once it is built.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.
