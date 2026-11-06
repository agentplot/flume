# Design

## Context

flume's main at b5dcf96 carries a conversation in one case. When a
settle reads a live start's agent `spent`, `Engine::read_spent` in
`crates/flume-core/src/spent.rs` records the account spent and, once
the agent is quiet, closes it. It then writes the stage's `RoomWait`
with `StartReason::Spent`'s words and an `AfterSpent { start, carry:
true, why: None }` naming the stopped start, and calls `start_stage`.
`Engine::claim` in `start.rs` reads that marker, chooses the account
under the room lock (`choose_account` in `accounts.rs`: the intent's
affinity for the stage's program while usable, else the strategy), and
asks the agent profile's `carry` of the chosen account's environment.
Where it answers yes, the start resumes the session: its `StartRecord`
names the session, its prompt is `StartReason::Carried`'s words and the
way to stop short, and `StartHeld::conversation` records
`Conversation { from, carried: true }`. Otherwise it fills the stage's
prompt with the spent words and records why. `uncarried.rs` reads a
carried start that ended before its started ack, or came up in another
session, and starts the stage afresh at once.

Claude Code's carry (`crates/flume/src/profiles/agent/claude/carry.rs`)
answers yes where the stopped start's started ack names a session and
its transcript is the same file, by device and inode, as
`<home>/projects/<folder>/<file>`, `<home>` the chosen account's
`CLAUDE_CONFIG_DIR`, else `~/.claude`. Its words name
`CLAUDE_CONFIG_DIR` and no path: `UNKNOWN` where the start never acked,
`UNREADABLE` where its transcript cannot be read, `UNREACHED` where the
chosen home holds another file or none.

`Engine::retry` in `acts.rs` refuses while the stage's agent started or
began (`working`), ends the place's live agent (`end_agent`: closed
once quiet, refused naming it otherwise unless forced), and starts the
stage with the retry's words after any stop-short's question
(`answered`). It clears any `RoomWait::carry`, so a retry carries
nothing (30).

`Engine::read_end` in `end.rs` reads a live start's agent. One that
stopped, or that is no agent of the stage's (`started == No` or
`own_record == No`, as an agent herdr brought back bare is), ends the
stage `Lost`; `end_foreign` then closes a foreign agent once it is
quiet, at this settle and each one after (77). A conversation is read
for nothing else (109).

On deneb, each of the six lost talks' held state names its start's
account (`madswan/amadden`), and its started ack names a session and a
transcript under `/Users/chuck/.claude-sub-amadden/projects/`. Every
configuration home's `projects` there is a link to one folder.
herdr 0.9.5 resumes an agent on its restore, where its configuration
sets `resume_agents_on_restore`, by the `agent_session_id` and
`resume_argv` its Claude Code integration reported. The agent comes
back without the pane's environment (record 0018), so it runs under the
login environment's `~/.claude`, in the same session and transcript.

Record 0052, proposed, decides this change; its statement amendment is
on agentplot/blueprints' branch
`design/flume-a-lost-conversation-resumes-its-session` at 1192a5a
(30, 43, 45, 49, 56, 77, 145, 147, 161, 179, C.4, C.5, S113 to S116).
The rules this design relies on are core.1 and core.2 (the core names
no tool; carry is the agent seam's), core.10 (one fresh agent per
start, taken on its own word; a bare agent in a conversation's pane
ended only by its caller's act, as amended), core.14 (the JSON form
only gains), core.16 and core.17 (held state by compare-and-set, every
act safe to repeat), core.30 (no variable's value or configuration
home's path given), core.32 (accounts and the carried conversation, as
amended) and core.35 (the site's neutral voice).

## Goals / Non-Goals

**Goals:**

- A caller's retry of a conversation reported lost goes on in the lost
  agent's session, under a new start's identity, on the account that
  holds its transcript where that account is usable (49, 145, 179).
- Where the conversation cannot be carried, the stage starts afresh
  with words saying so, and status says why (49, 56).
- An agent the terminal brought back in a conversation's pane is left
  for a person, and what the person sent to it goes on with the retry
  (77).
- deneb's six talks come back with their history at their retry.

**Non-Goals:**

- A retry of a stage that is not a conversation carries nothing, as
  today (49). Section 4 of record 0052 says why.
- A conversation its caller stopped (51), or one let go after its
  stop-short's hour (198), starts afresh at its retry, as today. Each
  was ended by a deliberate act with the talk at a known point; whether
  either should carry is a question of its own.
- A start after one begun without its settings (161) carries nothing,
  as today. herdr's restore loads no plugin, so it never reaches that
  path.
- flume starts no lost stage on its own (45, 48): the retry is the
  caller's.
- No profile's code changes: Claude Code's carry check is the same,
  given the lost start in place of a spent one.

## Decisions

### 1. One carry marker for a spent start and a lost one

`AfterSpent` becomes `CarryFrom { start, cause, carry, why }`, `cause`
one of `spent` and `lost` (`CarryCause`), read as `spent` where a
document written before holds none. It stays under `RoomWait::carry`,
so a held state written before reads unchanged (core.14, core.16).
`Conversation` gains `cause` the same way, and status gives it.

The alternative, a second marker for a lost start beside `AfterSpent`,
doubles every reading of the marker in `claim` and `uncarried.rs` for
one difference: which words a start gets and which account it takes
first.

### 2. The retry marks a lost conversation's start

In `Engine::retry`, after `end_agent` and before `start_stage`, where
the stage named is declared `conversation` and its state is `Lost`
with a last start, the retry writes the stage's `RoomWait` under the
room lock with the next order, the retry's words, and
`carry: Some(CarryFrom { start: <the lost start>, cause: Lost, carry:
true, why: None })`, as `read_spent` writes one (49). For every other
stage the retry clears the marker as today, so no other retry carries
(30).

Before it marks the start, the retry reads the lost start's agent once
more through the terminal its start recorded (core.49), where that
start is no longer live. An agent that stands in its pane again, as
when the terminal brought it back after the settle read the stage
lost, is ended as `end_agent` ends one: closed once quiet, and
otherwise refused naming it, unless forced (50, 77). A second agent in
the same session would write the transcript beside the carried one.

### 3. The account that holds the conversation first

`choose_account` takes the account the marker's start ran under, read
from that start's `StartHeld::account`, where the marker's cause is
`lost`. It takes that account first where it is declared in the pool
the stream takes now, under the same pool, of the stage's program, and
usable as 145 reads it: not spent, not past its threshold, with room
under its own bound (145, 185). The choice is
`AccountFrom::Conversation`, and no intent's move is made by it (147).
`affinity_after` runs as for any start, so an intent with no account
yet for the program keeps this one, and one with an account keeps
its own. Otherwise the choice is 145's as today, and a move of the
stage from the lost start's account to the chosen one is recorded with
its carried flag by `stage_moved` (56).

For a spent start the holding account is the spent one and is never
usable when the start is made, so the rule is not applied to it:
146's choice stands unchanged.

### 4. Carry asked as for a spent start, under no pool too

`claim` asks the profile's carry of the chosen account's environment,
as today. Where the stage's stream has no pool (`Choice::NoPool`) and
the marker's cause is `lost`, it asks carry with an empty account
environment, so the profile reads the login environment's home; a
spent start always ran under an account (146). Claude Code's carry then
answers:

- yes on the lost start's own account, whose home holds the transcript;
- yes on another account whose `projects` reaches the same file;
- `UNREADABLE` where the transcript is gone (S115);
- `UNKNOWN` where the lost start never acked;
- `UNREACHED` where the chosen home reaches no such file.

### 5. The words of a carried start and of a fresh one

`StartReason` gains two values:

- `Resumed`, the carried start's words: "Your session was lost and
  goes on here, with its conversation as it stood. Anything you had
  running then has stopped: read the place again before you carry on."
- `Lost`, the fresh start's words: "The stage's earlier agent was lost,
  and its conversation could not be carried, so this agent starts the
  stage afresh. What it committed is
  on the place's branch and what it left is in the place: read them and
  carry the stage on from there."

A carried start's prompt is `Resumed`'s words, then the retry's words
where it has any, then the way to stop short, each joined by a blank
line (28). A fresh start fills the stage's prompt with `Lost`'s words
before the retry's words as its `{{words}}` (27, 49). The spent words
stay as they are.

### 6. A carried start that does not begin

`uncarried.rs` reads a lost conversation's carried start that ended
before its started ack, or came up in another session, as it reads a
spent one's (S79). `after_uncarried` keeps the marker's cause, and the
fresh start's words follow it: `Lost`'s and the retry's for a lost one,
the spent words for a spent one.

### 7. An agent the terminal brought back in a conversation's pane

`read_end` reads such an agent as today: the stage ends `Lost` and its
start stays live while the agent stands. Where the stage is declared
`conversation`, `end_foreign` is not called, at that settle or after:
the agent is left in its pane, sent nothing (core.11), and its start
goes not live once the terminal reads it stopped, as when a person ends
it. The stage's end gains `left_in_pane: true`, written when the
settle reads it lost with such an agent standing (43). A stage that is
not a conversation is read as today: its agent is ended once quiet
(S41, S116).

The agent counts among the host's agents running while its start is
live, as any live start does (70): it runs on the host, and the retry
ends it first anyway. The place waits on its caller, said stalled after
five minutes, as any lost stage is (130; core.24).

### 8. Status and the end

- **The agent.** `left_in_pane: true` where the current stage is a
  conversation reported lost and the terminal reads an agent standing
  in its last start's pane, live or not, so one the terminal brought
  back after the settle read the stage lost is named too. The person's form says "brought back by the
  terminal and left in its pane; a retry ends it once quiet and carries
  its conversation" (56, 77).
- **The conversation.** `cause` with `from`, `carried` and `why`. The
  person's form says "carries the conversation of start 1, lost" or
  "starts afresh after start 1, lost: <why>" (56, 179).
- **The account.** `from: "conversation"` beside `affinity` and
  `strategy`. The person's form says "work-1 (its conversation's)"
  (145).
- **The end.** `left_in_pane` on the lost end. A notice of the end adds
  "flume left the agent the terminal brought back in its pane" (43).

Each is a field or a value added, never one removed (61; core.14).
`engine/answers.rs`'s field list gains them.

### 9. The fakes

The fake terminal brings an agent back by `resume`, and the fake agent
reads its started ack `No` by `set_started`; together they are a bare
agent. The fake agent's `carry_to` scripts carry per account, and its
log names each `carry` it answers. The seam-fakes delta adds one
helper, `bring_back_bare(identity)`, doing both, so the engine's
scenarios read as the statement's.

### 10. The walkthrough under the stand-in

`crates/flume/tests/profiles/` gains a walkthrough in the `check-tools`
layer with the stand-in for Claude Code. It uses a scratch host and a
kind of the test's own whose one stage is a conversation. A scratch
configuration home is that host's only account. The test kills the
stand-in's process in its pane, and the settle reads the stage lost.
The test retries the stage, and the new start's command holds
`--resume` and the lost start's session. The stand-in acks in that
session and appends to the same transcript file, read by device and
inode. A second run removes the transcript first, and the stage starts
afresh with `Lost`'s words. Fixtures only: no real account, no host
configuration of the user's, and no herdr session but one the test
starts and names (core.29).

## Risks / Trade-offs

- **A bare agent runs outside every pool until its caller acts.** It
  runs under the login environment's account. Status names it, and the
  stalled notice reaches the place's recipients after five minutes.
  Ending it at once loses a person's unsent draft, and adopting it
  takes an agent flume cannot read or guard (record 0052).
- **Two writers of one transcript.** A retry that started a carried
  agent while the bare one still ran would have both append to one
  session. The retry ends every agent in the lost start's pane first,
  as decision 2 reads it, and refuses while one works, unless forced.
- **A spent account on a host that shares nothing loses the talk from
  the place.** The transcript stays on disk and status says why, so a
  person can resume it by hand. Waiting for the reset was weighed and
  declined (record 0052, option 6).
- **The settles design and where-opens are in flight.** The build
  merges main first. Where where-opens has landed, the account and
  terminal of a carried start come from the start's own records as
  they do today. The clause list reads the blueprints revision the build
  finds on main, S109 to S112 and 202 to 206 named unbuilt as the
  settles design's where main holds them.

## Migration Plan

Held state written before reads with no cause, read as `spent`, and no
`left_in_pane`. A lost conversation's start written before this change
carries at its retry from its own `StartHeld::account` and started
ack, which every start since record 0020 holds: deneb's six among them.
Nothing is rewritten. Once the change lands and the host runs it, the
caller retries each of the six with `flume retry <place> --stage talk`.

## Open Questions

None. The user's acceptance of record 0052 and the amendment's landing
on blueprints main gate the build.
