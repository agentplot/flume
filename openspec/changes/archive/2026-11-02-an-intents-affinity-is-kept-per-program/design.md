# Design

## Context

flume's main at f9a51c2 keeps an intent's affinity as one document of
the host's own, `intents/<digest>.json` under `$XDG_STATE_HOME/flume/`
(153). Its type is `Affinity` in `crates/flume-seams/src/store.rs`:
`stream`, `pool`, `account`, `moves` and `gone`. `Engine::choose_account`
in `crates/flume-core/src/accounts.rs` takes the affinity's account where
its pool is the stage's and it is usable for the stage's program, else
the strategy's choice. Where the affinity's account is spent or full, it
records an `AccountMove` `on` `MoveOn::Intent`, unless the affinity once
left the account chosen (`Affinity::left`). `Engine::affinity_after`
writes the document at the intent's first start and at each move, and
nothing else.

So an intent whose stages run two programs keeps one account. A review
on Codex finds the Claude Code account not usable for its program and
takes the strategy's choice, and no account is kept for Codex. Where the
Claude Code account is at its bound when the review starts, `short()`
reads it full and the intent moves to the Codex account. The next code
start finds that account not usable for Claude Code and moves the
intent again. Each move says the cache was lost.

A pool's accounts that run the stage's program are read by
`PoolRoom::usable`. Where none does, it answers `Usable::NoProgram`,
`Queue::wants` answers `RoomWanted::Program`, and the stage waits for
room. `Engine::start_waiting` passes it over at every settle, so it
waits for ever. The engine's scenario `an_account_for_another_program`
holds that today.

The launch resolves each stage's agent before anything is made
(`make_place` in `launch.rs`), so a program no agent profile binds, or a
mode its profile cannot map, refuses the launch (4, 32, 169). The claim
in `start.rs` refuses a stage at a later start the same way: a reason
put in its `refused_setting`, the stage `Refused` and waiting on its
caller's retry (27, 32). `Choice::Undeclared`, a pool the host does not
declare, is refused there already.

The statement's amendment is on agentplot/blueprints main at a15d927.
It amends 4, 32, 145, 147 and the definition of affinity, and adds S80
and S81, which follow the carry's S78 and S79. The build reads the
statement there (core.19).

Clause 173, a stage whose layers name no program running the program of
the account chosen, is not built: record 0026's change builds it
(`_open.md`). Until then flume's default layer names `claude` for every
stage (`DEFAULT_PROGRAM` in `profiles.rs`), and every stage resolves a
program. And `profiles.rs` binds Claude Code's profile alone, so the
binary runs no second program.

The rules this design relies on are core.1 (the core and the suite name
no program: the fake's programs `agent` and `other` stand for Claude
Code and Codex), core.2 (no seam's profile is asked anything new),
core.8 (a launch refused whole, nothing made), core.14 (the JSON form
only gains fields), core.16 and core.17 (the host's documents by
compare-and-set, each act safe to repeat), core.26 (a stage waits for
room, and a launch is never refused for want of it), core.30 (a refusal
the launch could make is made at the launch) and core.32 (accounts and
the affinity). Group 1 amends core.26's, core.30's and core.32's
details and core.32's rule, accepted with record 0020's second
amendment. The one rules page is `docs/architecture/core.md`. Where
flume has no rule, the kit standard's code profile binds (kit:specs.3,
kit:testing.5).

## Goals / Non-Goals

**Goals:**

- An intent keeps, for each program its agents run, the account the
  first of them on that program started on, and a start takes the one
  for its stage's program (147, S80).
- A start on one program changes nothing of another program's account
  and is no move; a spent or full account moves its own program's alone,
  and status says for which program (56, 147).
- A document written before reads as its account's program's affinity,
  so no intent loses its account at the upgrade (147, 153).
- A stage whose pool holds no account of its program is refused, at its
  launch or at a later start, naming the stage, the pool and the
  program, and never waits for room (4, 32, 145, S81).

**Non-Goals:**

- The halves of 145 and 147 for a stage whose layers name no program
  (173): the pool's accounts whose program can run the stage, and the
  affinity for the program the intent's last agent ran. No stage
  resolves no program until record 0026's change builds 173, which
  builds them, the affinity's document gaining the last program then.
  `_open.md`'s item on record 0026 says so.
- S80 walked through under the printed command. The binary runs Claude
  Code alone until record 0026's change binds a second program, and S80
  needs two. The engine's scenario holds it over the fake, as S56's
  holds S56, and record 0026's change adds the walkthrough.
- An affinity's account that the host's configuration no longer
  declares, which today leaves its starts to the strategy with no move.
  It stays as it is.
- Any change to the strategies, the spent pause, the carry or the
  queue's order (145, 146, 179; core.26).

## Decisions

### 1. The affinity's document

`Affinity` in `crates/flume-seams/src/store.rs` becomes:

```rust
pub struct Affinity {
    pub stream: String,
    /// For each program the intent's agents run, as its accounts name
    /// it, the account the first of them on that program started on, as
    /// moved since (147).
    #[serde(default)]
    pub accounts: BTreeMap<String, AffinityAccount>,
    pub moves: Vec<AccountMove>,
    pub gone: Option<Seconds>,
    /// The one account a document written before an intent kept one
    /// per program holds, and its pool; read and never written (147).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

pub struct AffinityAccount {
    pub pool: String,
    pub account: String,
}
```

The key is the program as the stage's layers resolve it and the
account's `program` names it, the same string that makes the account
usable for the stage (144). The program's file name, by which its
profile is bound (`program_name`), would not do: an account usable only
for `claude` could then be kept for a stage that names a path. Each
program keeps its own pool, since an intent's places may run in streams
of different pools (143).

`Affinity::left(account)` becomes `left(program, account)`: whether the
affinity for `program` once moved from `account` (147).

`MoveOn::Intent` becomes `Intent { program: Option<String> }`, the
program whose account moved, skipped where none, so a move recorded
before reads with none. Every new intent move names its program.

### 2. A document written before

The document does not name its account's program, and only the host's
configuration does. So the seam reads it as written, with its `pool` and
`account`, and the core gives it its program. `Engine::affinity` reads
the document and, where `account` is some, finds that account in the
pool the host's configuration declares as it stands (`Host::pool`). Its
program then keys `accounts`, unless `accounts` already holds that
program, and every intent move with no program is given it. Where the
pool no longer declares the account, the account is dropped and the
stream kept, since no stage could take it. Either way `pool` and
`account` read none.

Every caller sees the folded affinity. `write_affinity` writes what
`change` makes of it whole, so the next write puts the document in the
new shape. A read that changes nothing writes nothing, so the fold is
safe to repeat (core.17). A serde conversion in the seam cannot do it,
since the program is not in the document.

### 3. The choice

`choose_account` takes `kept`, the account of `affinity.accounts[program]`
where that entry's pool is the stage's, and otherwise as today: `kept`
while usable, from the affinity, else the strategy's choice. The intent's
move is recorded only where `kept` is spent or full, for `program`, and
not where `affinity.left(program, chosen)`. A program with no entry
moves nothing, so a start on Codex while the Claude Code account is full
is no move.

`affinity_after(place, program, account, moved)` writes, under
compare-and-set (core.16):

- no document: the stream and `accounts` holding `program` alone;
- no entry for `program`: the entry, the account the start took, and no
  move;
- a move: that program's entry moved to its `to`, the move pushed, and
  the stream the place's, as today;
- and `gone` cleared.

Every other entry stays as it stands. `claim` passes the stage's
resolved program, the one `choose_account` was given.

### 4. A pool that holds no account of the stage's program

`Choice` gains `NoAccount(String)`. `choose_account` answers it where the
pool holds no account whose program is the stage's, `Usable::NoProgram`,
and the claim refuses the stage as it refuses `Choice::Undeclared`: its
`refused_setting` the words, `Refused`, no agent opened, waiting on its
caller's retry (27, 32). The words name the stage, the pool and the
program, in one function `no_account(stage, pool, program)` the launch
shares:

> the stage review's agent runs codex, and the pool work holds no
> account of codex, so no room it gives could start it (145)

`Queue::wants` no longer answers for a program: a stage whose pool
holds no account of its program wants no room. `RoomWanted::Program`
goes. The JSON form loses no field, since `room` stays, and its value
`program` is not given any more because no stage waits on it (core.14).

`start_waiting` reads, ahead of the host's room, whether each waiting
stage's pool holds an account of its program (`PoolRoom::runs`), and
claims each that does not under the room lock. The claim refuses it, so
a stage the host's configuration left without an account is refused at
the next settle, whatever room the host has, and is never shown waiting
on a room that cannot come. A stage whose pool holds an account of its
program, spent or at its bound, waits as today, and status gives the
pool's soonest reset (145).

### 5. The launch

`make_place` checks each stage after its resolution and before anything
is made (core.8). Where the place's stream has a pool, as the host's
configuration names it now (`Host::pool_for`, `Host::pool`), and that
pool declares no account whose program is the stage's resolved one, the
launch is refused with `no_account`'s words. Where the stage's program
is one no agent profile binds, the launch is refused for that first, as
today (4, 32). Each later start reads the configuration again (decision
4), so a pool the host changes after the launch refuses the stage then
(32).

### 6. Status and the person's form

The JSON form gains `program` on a move `on` `intent` (core.14). The
agent's account, `from` `affinity`, is the affinity for the agent's
`program`, which status gives beside it.

The person's form prints the agent's account `(from the affinity for
claude)` where it came from the affinity, and `(from the strategy)` as
today. An intent's move reads `the intent moved for claude from work-1
to work-2 at …`, and one recorded before with no program reads as today.
A refused stage gives its words as any refused stage does (56).

### 7. The record and the rules

Group 1 writes them first, as the user decided them.

- **Record 0020** gains a section, "Amendment, 2026-11-02: an intent
  keeps an account per program, and a pool that cannot run a stage
  refuses it". Its status says it was accepted by the user on
  2026-11-02, on their Pending You card req_f861af4d, "Fix both". Its
  deciders: the user, who chose to fix both; the document's shape, how a
  document written before reads, the move's program, and where the
  refusal is read are the writing agent's. Its options for the affinity:
  one account, a start on another program taking the strategy's choice
  and moving nothing; an account per program, chosen; an account per
  program and stage. Its options for a pool without the program: wait,
  status naming it, which is today's; refuse at the launch and at a
  later start, chosen; refuse at the launch alone. Its consequences: the
  statement's 4, 32, 145, 147, the definition and S80 and S81 at
  a15d927 (core.19); core.26, core.30 and core.32 amended; 173's halves
  left to record 0026's change. The record's status line names both
  amendments. The decision and the carry's amendment stand as written.
- **core.32's rule**: the intent's affinity for the stage's program
  while that account is usable, else the pool's strategy; a stage whose
  pool holds no account that can run it refused, at its launch or its
  start, and never left waiting. The amendment's note names both of
  record 0020's amendments. Its details rule out an intent's one account
  taken for another program, a start on one program read as a move or
  moving another program's account, a document written before read as
  no affinity, and a stage left waiting on a pool holding no account of
  its program. They leave open a stage waiting on a pool whose accounts
  of its program are spent or full, an intent's programs on two pools,
  and 173's halves, which record 0026's change builds. Source: 4, 32,
  145, 147, S80, S81.
- **core.30's details**: a pool that cannot run a stage found only at
  its start when the launch could have refused it is ruled out, beside
  the program no profile binds. A stage refused at a later start where
  the host's configuration dropped its pool's last account of its
  program is not (32, 145, S81).
- **core.26's details**: a launch, or a stage at a later start, refused
  where its pool holds no account that can run it is not ruled out,
  since that refusal is not for want of room: no room would end the wait
  (4, 32, 145, S81).
- **The site's rules page** mirrors core.32's rule as written.
- **`_open.md`** gains an item naming this build until it lands, and
  record 0026's item gains 173's halves of 145 and 147 and S80's
  walkthrough.

### 8. Proof

- `crates/flume-seams/src/round_trip.rs`: an affinity of two programs,
  an intent move naming its program, and `left` by program round trip;
  an older document's JSON reads with its `pool` and `account`, its move
  with no program, and is written with neither.
- `crates/flume-conformance/src/engine/account_pools.rs`, over the fake
  bound to `agent` and `other`:
  - S80: a fill-first pool of agent-1, agent-2, other-1 and other-2,
    and three places of one intent each running code on `agent` and
    review on `other`. Every code is on agent-1 and every review on
    other-1, the later ones from the affinity, with no move. Once agent-1
    is spent, the next code goes to agent-2, moving the intent for
    `agent` alone, and the next review stays on other-1.
  - A start on `other` while the intent's agent-1 is at its bound
    records no move and leaves `agent`'s account as it was.
  - A document written before, its account agent-2, reads as `agent`'s.
    Its move is given `agent`, and the next write holds `accounts` and
    no `pool` or `account`. One whose account the pool no longer
    declares reads with its stream alone.
  - S81: a launch whose review stage runs `other` on a pool of `agent`
    accounts is refused naming review, the pool and `other`, with
    nothing made. On a pool that holds other-1, the host's pool then
    drops other-1 while code works, and review is refused once code
    delivers, no agent opened; a retry after the pool gains it again
    starts review on it.
  - A review waiting for room on a spent other-1, on a host at its
    bound, is refused at the next settle once the pool drops other-1.
  - `an_account_for_another_program` holds its spec scenario as
    amended: a stage of `other` on a pool of an `agent` account and an
    `other` one starts on the second, where today it waits on a pool of
    `agent` accounts alone.
  - The S56, never-back, an-hour-after and no-intent scenarios pass,
    reading the affinity's `agent` entry.
- `crates/flume-conformance/src/seams/store.rs`: the store seam writes
  and reads an affinity of two programs.
- `crates/flume/tests/profiles/store_state_dir.rs`: an
  `intents/<digest>.json` in the older shape reads with its `pool` and
  `account`, and one written now holds no such keys.
- `crates/flume/src/commands/print.rs`: unit tests of the agent's
  account from the affinity for its program, and of an intent's move
  with a program and one without.
- `crates/flume/tests/profiles/tools/walkthroughs.rs` gains S81 under
  the printed command, with the programs the binary runs: every stage
  runs `claude`. A fix launched on a stream whose pool holds only
  `codex` accounts is refused, naming code, the pool and `claude`, with
  no branch and no place made. A change launched on a pool that holds
  the stand-in's account runs construct. The host's configuration then
  leaves the pool a `codex` account alone, and once construct delivers,
  code is refused naming the pool and `claude`, with no printed command
  for it. Once the configuration is mended, a retry starts code (4, 32,
  145).

### 9. The site

`site/src/content/docs/guides/agent-settings.md`'s "Pools of accounts"
says a stage waits only while its pool holds an account of its program,
and is refused otherwise, at its launch or at a later start. "An intent
keeps to its account" becomes "An intent keeps an account per program".
It covers one account per program the intent's agents run, a start on
one program moving nothing of another's, and the move naming its
program. "What status shows" gives the agent's line from the affinity
for its program. `reference/state.md` gives the intent's document's
`accounts`, a move's `program`, and the queue's wants without a
program's. `reference/configuration.md`'s account `program` says a pool
with no account of a stage's program refuses that stage. Each is written
to any host's operator, naming no host or team of ours (core.35).

## Risks / Trade-offs

- A host that removes a program's last account from a pool while its
  stage waits sees that stage refused rather than waiting, and must
  retry it once the account is back. The user chose the refusal, since
  that wait could not end, and status says why.
- The fold of a document written before reads the account's program
  from the configuration as it stands at the read. Where the host gave
  the account another program since, the account is kept for its new
  program. The account runs that program now, so it is usable only for
  it.
- An affinity keeps an entry for every program its intent ran for an
  hour after its last place, so its document grows by one entry a
  program. A host runs a handful of programs.
- The walkthroughs hold S81 with the programs swapped, since the binary
  runs Claude Code alone. The engine's scenario holds it as the
  statement words it, over the fake.
