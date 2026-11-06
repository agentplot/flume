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
session, and starts the stage afresh at once. A stage waiting for room
is started by any settle on the host, which passes over one whose
stream or pool has no room (`room.rs`, `Engine::start_waiting`).

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

On deneb at 17:00Z on 2026-11-06, each of the six lost talks' sessions
ran again outside flume. The orchestrator resumed five, each in a herdr
workspace of its own, as `claude --resume <session> --permission-mode
bypassPermissions` on another account, with no `FLUME_*` and no flume
plugin. herdr's restore ran the sixth. Each process is named in a
configuration home's `sessions/<pid>.json`, with its `pid`,
`sessionId`, `cwd` and `procStart`, and each holds `--resume <session>`
in its arguments. Every configuration home's `projects` there is a link
to one folder, so each process appends to the transcript its talk's
started ack names. The pinned Claude Code, 2.1.296, writes and reads
the same `sessions/<pid>.json` record (`procStart`, `sessionId`,
`cwd`, `kind`). herdr 0.9.5 resumes an agent on its restore, where its
configuration sets `resume_agents_on_restore`, by the `agent_session_id`
and `resume_argv` its Claude Code integration reported, without the
pane's environment (record 0018).

Record 0052, proposed, decides this change; its statement amendment is
on agentplot/blueprints' branch
`design/flume-a-lost-conversation-resumes-its-session` at 6b7d3fc
(30, 43, 45, 49, 56, 64, 77, 145, 147, 161, 179, 207, C.4, C.5, S113 to
S117). The rules this design relies on are core.1 and core.2 (the core
names no tool; carry and live are the agent seam's), core.10 (one fresh
agent per start, taken on its own word; no carried start beside a
process that runs its session; a bare agent in a conversation's pane
ended only by its caller's stop, drop or done; as amended), core.13 (no
process of flume's waits), core.14 (the JSON form only gains), core.16
and core.17 (held state by compare-and-set, every act safe to repeat),
core.18 (no place removed while a process runs its conversation, as
amended), core.26 (a wait for room is never stalled), core.30 (no
variable's value or configuration home's path given), core.32 (accounts
and the carried conversation, as amended) and core.35 (the site's
neutral voice).

## Goals / Non-Goals

**Goals:**

- A caller's retry of a conversation reported lost goes on in the lost
  agent's session, under a new start's identity, on the account that
  holds its transcript where that account is usable (49, 145, 179).
- No start, carried after a loss or after a spent agent, opens while a
  process runs the conversation it carries, whoever started it (30,
  207).
- flume adopts, ends and sends nothing to such a process, and keeps the
  place while it runs, since a person may be typing in it (49, 64).
- Where the conversation cannot be carried, the stage starts afresh
  with words saying so, and status says why (49, 56).
- deneb's six talks come back with their history at their retry, each
  once the user ends the session run by hand.

**Non-Goals:**

- A retry of a stage that is not a conversation carries nothing, as
  today (49). Record 0052's section 5 says why.
- A conversation its caller stopped (51), or one let go after its
  stop-short's hour (198), starts afresh at its retry, as today.
- A start after one begun without its settings (161) carries nothing,
  as today. herdr's restore loads no plugin, so it never reaches that
  path.
- flume starts no lost stage on its own (45, 48): the retry is the
  caller's.
- A session run under a home the host does not declare and resumed by
  `/resume` inside another session is not read live (record 0052,
  consequences).
- The terminal is not asked which of its parts runs a live process:
  status names the process, which a person finds in their own panes.

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

### 2. The agent seam's live

`Agent` gains `live(folder, start, envs) -> Live`, `start` the start
whose conversation a start would carry and `envs` the environments of
the accounts the host's configuration declares for the program, the
login environment's among them as an empty one (207). `Live` is
`Runs(Vec<LiveProcess>)`, each `{ pid, started }`; `None`; or
`Unknown { what }`. The default answers `Unknown` with "its agent profile
reads no running session", so a program whose profile cannot read it
never carries a conversation beside a process that runs it (171). The
answer's words name no path (core.30).

Claude Code's profile, in a new `crates/flume/src/profiles/agent/claude/live.rs`:

- reads the session from the start's started ack; none, and no session
  is live;
- for each environment's home (`carry::home`), lists `sessions/*.json`,
  takes each whose `sessionId` is the session, and counts it where the
  host runs a process with its `pid` started at its `procStart`. A
  record whose process is gone, or another process under a reused id,
  counts nothing;
- lists the host's processes once, `ps -axo pid=,lstart=,args=`, and
  counts each whose program is `claude`, by the last name of its first
  argument or its wrapped program's as `wrapper.rs` reads one, and
  whose arguments hold the session after `--resume` or `-r`;
- answers `Unknown` naming `CLAUDE_CONFIG_DIR` where a home's
  `sessions` folder exists and cannot be read, or naming the host's
  processes where they cannot be listed.

`fragments()` pins what it reads to 2.1.296's program, as `carry.rs`
pins its own: the record's fields `sessionId`, `pid` and `procStart`,
the `sessions` folder under the configuration home, and `--resume`'s
short form `-r`, checked by the `check-tools` fragments test (core.22,
core.39).

### 3. No carried start beside a live session

`claim` asks `live` before it opens any start that carries a
conversation, under the room lock, after the account is chosen and
carry is read, for a spent cause and a lost one alike. Where it answers
`Runs` or `Unknown`, the start does not open. The stage stays, or goes,
waiting for room, its `RoomWait` keeping the marker and gaining `live:
Vec<LiveProcess>` or `live_unread: Option<String>` as last read
(core.16). `start_waiting` passes over such a stage at each settle, as
it passes over one whose stream has no room, and asks `live` again when
it reaches it, so the first settle that reads `None` starts it, room
allowing (70; core.13). The wait is a wait for room: never stalled and
never a wait on the caller (130; core.24, core.26).

Where carry answers no, the start would open afresh, and a lost
conversation's fresh start would still put a second talk beside the
person's. So `live` is asked for a lost cause whatever carry answered,
and the fresh start waits the same way.

### 4. The retry marks a lost conversation's start, and ends nothing that runs it

In `Engine::retry`, where the stage named is declared `conversation`
and reads `Lost` with a last start, the retry ends no agent of that
stage. Its live start, where one stands (a bare agent in flume's own
pane), is no agent of the stage's, and `end_agent` is skipped for it. It
writes the stage's `RoomWait` under the room lock with the next order,
the retry's words and `CarryFrom { start: <the lost start>, cause:
Lost, carry: true, why: None }`, as `read_spent` writes one, and calls
`start_stage`. decision 3 then holds the start while the session runs
(49, 77). The retry answers at once, saying the stage waits for its
conversation to end, with each process, where it does. For every other
stage the retry ends the place's agent and clears the marker as today,
so no other retry carries (30, 50).

### 5. The account that holds the conversation first

`choose_account` takes the account the marker's start ran under, read
from that start's `StartHeld::account`, where the marker's cause is
`lost`. It takes that account first where it is declared in the pool
the stream takes now, under the same pool, of the stage's program, and
usable as 145 reads it: not spent, not past its threshold, with room
under its own bound (145, 185). The choice is
`AccountFrom::Conversation`, and no intent's move is made by it (147).
`affinity_after` runs as for any start, so an intent with no account
yet for the program keeps this one, and one with an account keeps its
own. Otherwise the choice is 145's as today, and a move of the stage
from the lost start's account to the chosen one is recorded with its
carried flag by `stage_moved` (56).

For a spent start the holding account is the spent one and is never
usable when the start is made, so the rule is not applied to it:
146's choice stands unchanged.

### 6. Carry asked as for a spent start, under no pool too

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

What a person said to the session while it ran by hand is in that file
wherever the home it ran under reaches it, as every home on deneb does,
so the carried start goes on with it (S117).

### 7. The words of a carried start and of a fresh one

`StartReason` gains two values:

- `Resumed`, the carried start's words: "Your session was lost and
  goes on here, with its conversation as it stood. Anything you had
  running then has stopped: read the place again before you carry on."
- `Lost`, the fresh start's words: "The stage's earlier agent was lost,
  and its conversation could not be carried, so this agent starts the
  stage afresh. What it committed is on the place's branch and what it
  left is in the place: read them and carry the stage on from there."

A carried start's prompt is `Resumed`'s words, then the retry's words
where it has any, then the way to stop short, each joined by a blank
line (28). A fresh start fills the stage's prompt with `Lost`'s words
before the retry's words as its `{{words}}` (27, 49). The spent words
stay as they are.

### 8. A carried start that does not begin

`uncarried.rs` reads a lost conversation's carried start that ended
before its started ack, or came up in another session, as it reads a
spent one's (S79). `after_uncarried` keeps the marker's cause, and the
fresh start's words follow it: `Lost`'s and the retry's for a lost one,
the spent words for a spent one. Its fresh start waits on `live` as
decision 3 says.

### 9. An agent the terminal brought back in a conversation's pane

`read_end` reads such an agent as today: the stage ends `Lost` and its
start stays live while the agent stands. Where the stage is declared
`conversation`, `end_foreign` is not called, at that settle or after:
the agent is left in its pane, sent nothing (core.11), and its start
goes not live once the terminal reads it stopped, as when a person ends
it. The stage's end gains `left_in_pane: true`, written when the
settle reads it lost with such an agent standing (43). A caller's stop,
drop or done ends it through `end_agent` as the place's agent (51, 52,
114). A stage that is not a conversation is read as today: its agent is
ended once quiet (S41, S116).

The agent counts among the host's agents running while its start is
live, as any live start does (70).

### 10. A place kept while a process runs its conversation

The tidy (`tidy.rs`) asks `live` of a place whose kind holds a
conversation, for its last conversation start, before it removes the
place, and keeps the place where it answers `Runs` or `Unknown`, as it
keeps one whose agent is live. The kept word is `kept while its session
runs`, and status names each process (64; core.18). The next tidy after
the process ends removes it.

### 11. Status, the line, the list and the end

- **The waiting stage.** With a place waiting for room to carry a
  conversation, each process `live` read, `{ pid, started }`, or what
  could not be read, and since when it waits. The person's form says
  "waits for its conversation to end: process <pid>, started <time>"
  (56, 207).
- **The line and the list.** A place whose stage waits on a live
  session reads `waiting on its session`, ahead of `waiting for room`,
  and the list says `waits on its session` in the place of `waits on
  the host` (139, 200).
- **The conversation.** `cause` with `from`, `carried` and `why`. The
  person's form says "carries the conversation of start 1, lost" or
  "starts afresh after start 1, lost: <why>" (56, 179).
- **The account.** `from: "conversation"` beside `affinity` and
  `strategy`. The person's form says "work-1 (its conversation's)"
  (145).
- **The end.** `left_in_pane` on the lost end. A notice of the end adds
  "flume left the agent the terminal brought back in its pane" (43).
- **A kept place.** `kept while its session runs`, with each process.

Each is a field or a value added, never one removed (61; core.14).
`engine/answers.rs`'s field list gains them.

### 12. The fakes

The fake terminal brings an agent back by `resume`, and the fake agent
reads its started ack `No` by `set_started`; together they are a bare
agent, given by one helper, `bring_back_bare(identity)`. The fake agent
scripts `live` per session: `live_runs(session, processes)`,
`live_unknown(session, what)`, and `None` otherwise, its log naming
each `live` it answers.

### 13. The walkthrough under the stand-in

`crates/flume/tests/profiles/` gains a walkthrough in the `check-tools`
layer with the stand-in for Claude Code. It uses a scratch host and a
kind of the test's own whose one stage is a conversation. A scratch
configuration home is that host's only account. The stand-in writes
`sessions/<pid>.json` as 2.1.296 does. The test kills the stand-in's
process in its pane, and the settle reads the stage lost. It then
starts a stand-in of its own, `--resume <session>` under a second
scratch home whose `projects` links to the first, in a pane flume did
not make. The test retries the stage, which waits, and flume closes
nothing. The test ends that process, and the next settle starts the
stage holding `--resume` and the lost start's session. The stand-in
acks in that session and appends to the same transcript file, read by
device and inode. A second run removes the transcript first, and the
stage starts afresh with `Lost`'s words. Fixtures only: no real
account, no host configuration of the user's, and no herdr session but
one the test starts and names (core.29).

## Risks / Trade-offs

- **A session a person keeps open holds its place's stage.** The stage
  waits as long as the process runs, under that process's account and
  mode, outside every pool. Status names the process and the line reads
  `waiting on its session`. Ending it at once loses a person's unsent
  draft, and adopting it takes a process flume cannot read or guard
  (record 0052).
- **A session `live` cannot see.** A session run under a home the host
  does not declare and resumed by `/resume` inside another session has
  no record flume reads and no argument naming it. A retry can then
  resume it beside that process. On deneb every session runs under a
  declared home or names its session in its arguments.
- **An unreadable home holds a stage.** `Unknown` never opens a start,
  so a home whose `sessions` folder cannot be read keeps the stage
  waiting. Status says what could not be read.
- **One process listing per waiting stage at each settle.** `ps` runs
  once for each stage that waits on a session, bounded by the waits on
  the host, six on deneb.
- **A spent account on a host that shares nothing loses the talk from
  the place.** The transcript stays on disk and status says why, so a
  person can resume it by hand. Waiting for the reset was weighed and
  declined (record 0052, option 6).
- **The settles design and where-opens are in flight.** The build
  merges main first. The clause list reads the blueprints revision the
  build finds on main, S109 to S112 and 202 to 206 named unbuilt as the
  settles design's where main holds them.

## Migration Plan

Held state written before reads with no cause, read as `spent`, no
`left_in_pane` and no live processes. A lost conversation's start
written before this change carries at its retry from its own
`StartHeld::account` and started ack, which every start since record
0020 holds: deneb's six among them. Nothing is rewritten. Once the
change lands and the host runs it, the caller retries each of the six
with `flume retry <place> --stage talk`. Each waits while its session
runs by hand, and goes on under flume at the first settle after the
user ends it.

## Open Questions

None. The user's acceptance of record 0052 and the amendment's landing
on blueprints main gate the build.
