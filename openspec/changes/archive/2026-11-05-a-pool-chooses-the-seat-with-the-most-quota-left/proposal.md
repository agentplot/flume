# A pool chooses the account with the most of its allowance left

## Why

A pool chooses an account fill-first or least-busy (record 0020; 145).
Neither knows how much of an account's allowance is left. least-busy
counts flume's agents on each account. That count leaves out a person's
own sessions under it, the work of the hours before, and how far
through its week the account is. An account is set aside only once an
agent's own record says it reached its usage limit (146). So the agent
that finds the limit is one flume just started, and its turn stops
mid-work.

On 2026-11-05 the user decided, in the Plannotator Inbox, what to do
with claude-swap, a tool that switches a folder's login when an account
runs low: "Take its ideas into the shim and Flume's pools; don't install
it". claude-swap keeps its own copy of each login and renews the idle
copies, and each renewal kills the folder's own login. Of the four ideas
the user chose, flume's pools take two, as they were put to the user:

1. "Choose by quota left, not by agents running. Flume's pool and
   `claude-account use` prefer the seat with the most 5-hour and 7-day
   quota left, read without renewing anyone's login."
2. "A seat near its limit takes no new work. Past a threshold, say 90%
   of either window, new sessions and agents go to another seat. Running
   ones finish where they are, and no seat's login ever moves."

A seat in the user's words is an account in flume's. flume fetches
nothing over the network and never touches a login (146), so the host
reads each account's windows and writes what it read where flume reads
it. Record 0035, proposed, records the decision. The statement's
clauses come first: the design writes them for the user to approve, and
nothing is built until they stand on agentplot/blueprints main
(core.19).

## What Changes

- **The decision recorded.** Record 0035, proposed, quotes the user and
  gives the choices this design makes. Once the statement holds the
  clauses, rule core.32 is amended citing them, and the site's rules
  page mirrors it.
- **An account's reading (184).** An account may name `usage`, a file
  of the host's, read as an `env` value is. The host writes the file
  whole, in flume's form: `read_at`, and `windows`, each with a `name`,
  a `used` percentage and an optional `resets_at`. flume reads it at
  each choice and when status is asked. It never writes, keeps or renews
  anything of it. A reading is fresh for fifteen minutes. A window past
  its reset reads as nothing used. A missing or malformed file is no
  reading: status says why, and nothing is refused.
- **most-left (145).** A third strategy chooses the usable account with
  the most left, the least any window of its fresh reading leaves. After
  every account with a fresh reading come the accounts without one.
  Equals go by fewest agents running, then the pool's order. With no
  fresh reading anywhere, most-left chooses as least-busy does.
- **A threshold (185).** A pool's `threshold`, ninety percent where it
  states none. An account whose reading gives a window used to the
  threshold or past it takes no new start, under every strategy, until
  that window resets. The affinity moves off it, saying which window
  and until when. Its running agents go on and are never ended or moved
  for it.
- **What stands.** A spent account's pause, the affinity, the carry, the
  queue's order, fill-first and least-busy are unchanged. A reading
  never shortens or lengthens a spent pause. A host that names no
  reading runs as today.
- **Status (56)** gives each pool's threshold, each account's reading
  with its windows, age and freshness or why there is none, and a
  threshold that holds an account, with its window and its end. A move
  off an account past its threshold says so. The pool's soonest time
  counts a threshold's end.
- **The site** says how a host gives readings, the form of the file,
  the strategy and the threshold.
- **Conformance** holds S86 to S88 and the spec's scenarios over the
  fakes, and a walkthrough holds the choice and the threshold under the
  printed command.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `account-pools`: an account's reading; the most-left strategy; a
  pool's threshold, past which an account takes no new start; the
  affinity moving off an account past its threshold.
- `place-status`: each pool's threshold, each account's reading, and a
  move off an account past its threshold.
- `store-profile`: a pool's `strategy` and `threshold` and an account's
  `usage` in the host's configuration, and a reading read from its file.
- `seam-contracts`: the host's reading of a file, and the types of a
  reading, a window and a move past a threshold.
- `seam-fakes`: the fake host scripts each account's reading.
- `clause-coverage`: the suite's list at the revision that states 184,
  185 and S86 to S88.

## Impact

- agentplot/blueprints: the clauses the design writes, added on main
  once the user approves them, which the build reads (core.19). Nothing
  in weir changes, and no kind gains a field (core.4).
- `docs/adr/0035-…`: the record, proposed. `docs/architecture/core.md`:
  core.32's rule and details, amended once the statement holds the
  clauses.
- `crates/flume-seams`: `Strategy::MostLeft`, `Pool`'s `threshold`,
  `Account`'s `usage`, `Usage` and `Window`, `MoveWhy::Threshold` and a
  move's window and end, and `Host::usage`.
- `crates/flume-core`: `accounts.rs`'s reading of an account, its
  threshold, the strategy and the soonest time; `room.rs`'s queue and
  status; `answer.rs`'s pool and account status.
- `crates/flume`: the configuration's pool and account keys, the real
  host's reading of a file, the person's form of status, and the
  walkthrough.
- `crates/flume-fakes`: the fake host's readings.
- `crates/flume-conformance`: the clause list and the engine's
  scenarios in `account_pools.rs` and `place_status.rs`.
- `site/`: the agent-settings guide, the configuration and state
  references, and the rules page.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency: `chrono` already reads RFC 3339 in `crates/flume`.
- A host's reader is the host's own. The design gives the form any
  host's reader writes. flume ships none.
