# Design

## Context

flume's main at b6cd1b5 runs a host's work on pools of accounts (record
0020). When a settle reads a live start's agent `spent`,
`Engine::read_spent` in `crates/flume-core/src/spent.rs` records the
account spent in the host's own documents. Once the agent is quiet it
closes the agent, puts the stage back in the queue with a `RoomWait`
holding `StartReason::Spent`'s words, and calls `start_stage`.
`Engine::claim` in `start.rs` chooses the account under the room lock
(`choose_account`), reads its environment (`account_env`), fills the
stage's prompt with those words, records the start's `StartRecord`
(program, mode, account, digests), and writes a `StartHeld`. `open`
passes the claim to `Agent::command`. `stage_moved` in `accounts.rs`
records the stage's move between accounts, and status gives it as a
`MoveStatus` with its cache lost.

The Claude Code profile (`crates/flume/src/profiles/agent/claude/`)
reads every fact of a start from its record in the place's folder,
`<folder>/claude/<identity>/`. The SessionStart hook writes the started
ack, `Started { session_id, transcript_path, source, plugin_version }`,
once `hook::compare` finds the program and every variable as the
start's record names them. Own work, last words, asking's answer and a
slash command's receipt read the whole transcript the ack names
(`transcript::Transcript::read`). `deliver.rs` reads a start never begun
(`never_begun`) when its agent stops before its started ack, at the
stage's limit, or when no sending of its prompt is received.
`unsettled.rs` ends an agent begun without its settings once quiet and
starts its stage afresh.

Record 0020's amendment, proposed 2026-11-02, is stated at blueprints
ec16d03 on the branch `design/flume-a-spent-seats-agent-resumes`. It
amends 30, 56, 88, 146, 161 and 171, C.4's command and started rows,
its paragraph on accounts and its common profile's table, and S55, and
adds 175, C.4's carry row, S76 and S77. The build reads the statement
from blueprints main once the user rules it (core.19).

What the pinned Claude Code, 2.1.296, does, read from the program
`devenv.nix` pins:

- `-r, --resume [value]`, "Resume a conversation by session ID", and
  `--fork-session`, "When resuming, create a new session ID instead of
  reusing the original". Without the second, a resumed session keeps
  its id.
- A session's transcript is `<home>/projects/<folder>/<session>.jsonl`,
  `<home>` the configuration home, `CLAUDE_CONFIG_DIR` or `~/.claude`
  (`function Df(){return S(be(),"projects")}`). `<folder>` is the
  working directory with every character that is not a letter or a
  digit written `-`, cut at 200 characters with a hash after it.
- `--resume <session>` looks for the session under that home's
  `projects` alone, and goes on in the transcript it loaded, under the
  same session. A session it does not find ends the program with exit
  status 1 and "No conversation found with session ID: <session>", and
  no session begins.
- It also takes a transcript's path, `--resume <path>.jsonl`, which
  flume does not use (record 0020's amendment, option 4).

The rules this design relies on are core.1 and core.2 (the core names
no tool, and the agent seam alone carries a conversation), core.10 (one
fresh agent per start, taken on its own word), core.13 (no daemon),
core.14 (the JSON form only gains fields), core.16 and core.17 (held
state by compare-and-set, every act safe to repeat), core.30 (no
variable's value given), core.32 (accounts, spent pauses, and the
conversation carried, as amended) and core.35 (the site's neutral
voice). core.32's amendment and core.10's details are marked proposed
until the user's yes. The one rules page is `docs/architecture/core.md`.
Where flume has no rule, the kit standard's code profile binds
(kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- An agent stopped at its account's usage limit goes on with its own
  conversation on the account its pool chooses, where its program can
  carry it there (146, 175).
- flume reads whether the host made a conversation reachable from the
  chosen account, and changes nothing of any account (C.4; core.32).
- A carried start is a start like any other: its own identity, secret
  and record, taken only on its own word (30, 35, 161; core.10).
- Where a conversation cannot be carried, or its start does not begin,
  the stage starts afresh as today, with no caller's word and no bound
  counted, and status says why (112, 146, S55, S77).

**Non-Goals:**

- The common profile's carry, and status naming it degraded with the
  other parts (170, 171, 174): record 0026's change builds the common
  profile, with carry not offered. Until then only Claude Code runs.
- Carrying a conversation for any other reason: a retry, a fix round, a
  stage after it, or an agent ended by a caller (30).
- A host's setup. flume makes no link and offers no command that makes
  one; the site shows the host the link.
- Moving a session's other state, its file history among it, with the
  conversation. `--resume` needs only the transcript.
- Any change to the account's pause, the affinity or the strategies.

## Decisions

### 1. The agent seam's carry

`flume-seams` gains, in `agent.rs`:

```rust
pub enum Carry {
    Yes { session: String },
    No { why: String },
    NotOffered,
}
```

and `Agent::carry(&self, folder: &str, earlier: &Identity, env:
&BTreeMap<String, String>) -> Carry`, `NotOffered` by default. `earlier`
is the start whose agent stopped at its account's limit, read in the
place's own `folder` (128). `env` is the chosen account's environment as
the claim reads it, each value read against the login environment
(`account_env`). `why` names a variable and never its value (core.30).

`Agent::command` gains `resume: Option<&str>`, the session the start
resumes, after `mode`. A profile that answers no `Yes` is never given
one. Its doc says so (88, 175).

`StartRecord` gains `session: Option<String>`, the session the start
resumes, skipped where none and read with a default, so held state
written before reads with none. `Unsettled` gains `session: bool`, the
agent came up in another session than its record names. Its `words()`
say "in another conversation than the one its start resumes", naming no
session (161).

### 2. The stage's wait after a spent agent

`RoomWait` gains `carry: Option<AfterSpent>`, skipped where none:

```rust
pub struct AfterSpent {
    /// The start whose agent stopped at its account's usage limit.
    pub start: Identity,
    /// Whether the next start may carry its conversation.
    pub carry: bool,
    /// Why it does not, where it may not.
    pub why: Option<String>,
}
```

`read_spent` writes `carry: Some(AfterSpent { start, carry: true, why:
None })` with the room it writes today. Every other path that starts a
stage writes its own `RoomWait`, or claims at once with none: a retry,
a fix round, a stop-short's answer and `read_unsettled` of a start that
carried nothing. So no other start carries a conversation (30). A
stage that waits for a usable account keeps its `carry`: the claim
that leaves it waiting writes its new `RoomWait` with the `carry` the
room held, and the claim that finally starts it carries the
conversation then (145).

### 3. The claim

`Engine::claim` reads the place's `RoomWait` before its update, beside
the account it chose. Where the room carries an `AfterSpent` with
`carry` and the choice is an account, it asks the stage's agent profile
`carry(folder, &after.start, &env)`, under the room lock, after the
choice (145). The answer decides the start:

- **`Yes { session }`.** The start's prompt is
  `StartReason::Carried.words()`, then two newlines and the profile's
  `stop_short`, with nothing of the stage's template filled (28). The
  words: "Your last turn stopped at your account's usage limit. This
  session now goes on under an account of the pool, with its
  conversation as it stood: carry on from where you stopped." `StartRecord::session`
  is the session, and `Claim` gains `resume`, which `open` passes to
  `Agent::command`.
- **`No { why }` or `NotOffered`.** The start is today's: its prompt
  filled with `StartReason::Spent`'s words, and nothing resumed.
  `NotOffered`'s why is "its agent profile carries no conversation".
- **An `AfterSpent` without `carry`.** It is the fresh start after a
  carried one that did not begin (decision 5), afresh with its `why`.

`StartHeld` gains `conversation: Option<Conversation>`, skipped where
none: `Conversation { from: u32, carried: bool, why: Option<String> }`,
`from` the stopped start's number. It is written for every start whose
room carried an `AfterSpent`. `AccountMove` gains `carried:
Option<bool>`, skipped where none, which `stage_moved` sets from it, so
a stage's move says whether its conversation went with it. A repeating
stage's start keeps the group `group_at` gives, as a fresh start after a
spent agent does today (140). The account choice, the affinity, the
pause and the counts are unchanged (145 to 147, 112). A pool of one
account carries its agent's conversation across its own reset, since
the chosen account's home holds the transcript.

### 4. Claude Code's carry and its command

`crates/flume/src/profiles/agent/claude/carry.rs` holds the reading.
`Claude::carry` reads the earlier start's started ack,
`Record::at(folder, earlier).started()`:

1. No ack: `No`, "the agent that stopped never said it started, so its
   conversation is not known".
2. The ack's transcript cannot be read by `fs::metadata`: `No`, "its
   conversation's transcript cannot be read".
3. `<home>` is the chosen account's `CLAUDE_CONFIG_DIR` from `env`, else
   `$HOME/.claude` of flume's own environment, the host user's login
   environment. The candidate is `<home>/projects/<folder>/<file>`,
   `<folder>` and `<file>` the last two names of the ack's transcript
   path, so flume computes no name of Claude Code's.
4. The candidate and the ack's transcript are the same file, by
   `MetadataExt::dev` and `ino` with symbolic links followed: `Yes {
   session: ack.session_id }`. Otherwise `No`, "the chosen account's
   configuration home (`CLAUDE_CONFIG_DIR`) does not reach its
   conversation: its `projects` folder is not the stopped account's".

Nothing in a configuration home is written, linked or copied. The
command adds `--resume <session>` where `resume` is given (C.4).
`Claude::carry` reads only, so it is safe to ask at every claim (core.17).

### 5. A carried start that does not begin

`flume-core` gains `uncarried.rs`, `Engine::read_uncarried(id, index,
start, words)`, in the shape of `read_unsettled`. Where `deliver.rs`
would read a start whose `conversation` carried as never begun, at its
stop, at the stage's limit or on an unreceived prompt, it calls
`read_uncarried` instead. That records `UncarriedStart { stage, start,
at, words }` once on the place, `HeldState::uncarried`, and sends its
agent nothing more. Once the agent is quiet, at once where it stopped,
it is closed. The stage is put back in the queue with
`StartReason::Spent`'s words and `carry: Some(AfterSpent { start: the
stopped start, carry: false, why: Some(words) })`, and started (146).
`words` say what happened: "the start that carried its conversation
ended before it said it started", "it did not say it started by the
stage's limit", or "it did not receive its prompt". A start under a
dialog waits on a person as any start does (34).

`read_unsettled` of a start whose `conversation` carried writes the same
`AfterSpent`, `carry: false`, with the unsettled words as its `why`, in
place of a bare `RoomWait`. Its `again` is false, since the stopped
start before it began, so the fresh start follows and the record of the
unsettled start stays (161).

Neither path delivers an end, holds a caller, or counts toward the
kind's bound (112). A fresh start after either carries nothing, and is
read as any start is: one that begins without its settings again is
never begun (161).

### 6. The session compared, and each start's own part of its transcript

`hook::compare` takes the SessionStart input's `session_id`. Where the
start's record names a session and the input's differs, it sets
`Unsettled::session`, so the hook acks nothing and writes
`unsettled.json` as for a variable (161). `flume ack started` compares
the session it is given the same way.

`Started` gains `from: u64`, read with a default of 0: the transcript's
length in bytes when the start's first started ack came, 0 where no
transcript is written. `Record::ack_started` keeps the `from` of an ack
already written, so a later SessionStart of the start, a compaction's
or a clear's, never moves it. `transcript::Transcript::read_from(path,
from)` reads the records that begin at or after `from`, or all of a
file shorter than `from`, which Claude Code replaced. Own work, last
words, asking's answer and the slash-command receipt read through it
(43, 89, 90, 91). For a fresh start `from` is 0 and nothing changes. A
carried start then reads none of the stopped agent's turns, background
tasks or last words as its own. Without this, a background task the
stopped agent left running when it was ended would read as the carried
agent's own work for ever, and it would never be quiet (31).

### 7. Status and the person's form

The JSON form only gains fields (core.14). `AgentStatus` in
`answer.rs` gains `conversation`, the start's `Conversation`: `{"from":
1, "carried": true}`, or `{"from": 1, "carried": false, "why": "…"}`.
`MoveStatus` gives the move's `carried`. Each `UncarriedStart` is listed
with the place's `uncarried`, and an unsettled start names `session`
where it came up in another. The person's form prints the agent's line
"carries the conversation of start 1", or "starts afresh after start 1:
<why>". A stage's move reads "its conversation with it", or "its
conversation stayed behind" (56). No answer gives a configuration
home's path or a session's transcript path (core.30).

### 8. The stand-in

`crates/flume/tests/profiles/stand-in/claude.rs` takes `--resume
<session>` as 2.1.296 does. It looks for
`<home>/projects/<folder>/<session>.jsonl`, `<folder>` the working
directory as decision 4's facts name it, and where it finds none prints
"No conversation found with session ID: <session>" and exits 1 before
any hook. Otherwise its session is the one resumed, and it appends to
that transcript. Its SessionStart names `source` `resume`, and a
UserPromptSubmit names the session. Its doc comment says so beside the
acts.

### 9. Proof

- The seam's scenarios in `crates/flume-conformance/src/seams/agent.rs`
  take a profile that offers no carry, and the fake's scripted carry
  (175).
- The engine's scenarios in a new
  `crates/flume-conformance/src/engine/carried_conversations.rs` hold
  the carried-conversations spec's scenarios, S76 and S77 among them.
  The S55 scenario in `account_pools.rs` gains the conversation that
  stayed and why.
- `crates/flume/tests/profiles/agent_claude.rs` takes Claude Code's
  carry over scratch configuration homes, one sharing `projects` by a
  link and one not, the command's `--resume`, and the readings past
  `from` over a fixture transcript. `hooks_claude.rs` and
  `plugins/claude/tests/hooks.sh` take the session's comparison and
  `from` kept at a compaction.
- `crates/flume/tests/profiles/tools/claude_words.rs` gains a
  `check-tools` test pinning, byte for byte in the program `devenv.nix`
  pins, the fragments decision 4 rests on: `--resume`'s and
  `--fork-session`'s option text, `function
  Df(){return S(be(),"projects")}` and `No conversation found with
  session ID: `. A release that changes them fails the gate at the bump
  (core.22).
- `crates/flume/tests/profiles/tools/walkthroughs.rs` gains a fix whose
  code agent stops at the limit on a pool whose second account's
  `projects` is a link to the first's. The second start's command
  resumes the first's session, its prompt is the carry words, status
  says carried, and the place lands (S76). The existing spent
  walkthrough asserts the conversation stayed and why (S55).
- `crates/flume/tests/profiles/live/walkthroughs.rs` gains a fix under
  herdr with the pinned Claude Code over two scratch configuration
  homes signed in by the test account's token, the second's `projects`
  a link to the first's. No test account reaches its limit on demand,
  so once its code agent has taken its first turn, the test runs the
  plugin's StopFailure hook for that start with a usage limit's input,
  as 2.1.296 builds it. The carried agent resumes the session and the
  place lands. The operator runs it by hand, as `CLAUDE.md` says.

### 10. The site

`site/src/content/docs/guides/agent-settings.md`'s "A spent account"
gains the conversation carried. It covers how a host shares one
`projects` folder between accounts by a link, what that shares, the
agents' memory of each project among it, and what status says when the
conversation stayed behind. `guides/claude-code.mdx` gains the carry
and the session compared. `reference/state.md` gains the agent's
`conversation`, the move's `carried` and the place's `uncarried`. Each
is written to any host's operator, naming no host or team of ours
(core.35).

## Risks / Trade-offs

- The carried agent's first request reads its whole conversation with
  no prompt cache, which costs more than a fresh start's prompt for a
  long conversation. The user chose the conversation, and status says
  the cache was lost either way.
- A resumed session may begin in another mode than the one passed. The
  mode's comparison at its first prompt reads it, and the stage starts
  afresh, as for any start (161, 162).
- Sharing `projects` shares what Claude Code keeps of each project
  there, between accounts the host may have kept apart for a reason.
  Sharing stays the host's choice, and the guide says what it shares.
- 2.1.296's `projects` folder name and its resume's lookup are read from
  its minified source. The `check-tools` test fails the gate at a bump
  that changes them, before a conversation is said carried that the new
  release cannot find. A carried start that still finds nothing falls
  back to afresh (decision 5).
- The transcript's length is read when the ack comes, after Claude Code
  may have written records of the resume. Those records are its own,
  not the agent's turns, so leaving them out reads nothing wrong.
