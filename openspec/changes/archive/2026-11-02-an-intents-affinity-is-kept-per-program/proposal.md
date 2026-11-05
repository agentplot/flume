# An intent keeps an account per program

## Why

An intent's affinity is one account (record 0020; 147). A place's
stages may run different programs, code on Claude Code and review on
Codex, and one account cannot serve both. Today the review's start finds
the affinity's account not usable for its program and takes the
strategy's choice, so the second program never keeps an account and
loses its prompt cache from start to start. Where the affinity's account
is full when the review starts, flume records the intent moved to the
Codex account, and the next code start moves it back: status says the
intent moved and lost its cache when nothing of the kind happened.

A stage whose pool holds no account of its program waits for room for
ever (145). No reset ends that wait, and status points at the pool as
though one would.

On 2026-11-02 the user decided to fix both, on their Pending You card
req_f861af4d: "Fix both". The statement is amended on agentplot/blueprints
main at a15d927: an intent keeps an account per program, and a stage
whose pool holds no account that can run it is refused rather than left
waiting (4, 32, 145, 147, S80, S81). Record 0020 gains the decision as a
further amendment, accepted on the user's word.

## What Changes

- **The decision recorded.** Record 0020 gains a second amendment. The
  rules core.26, core.30 and core.32 are amended with it, and the site's
  rules page mirrors core.32's words. `_open.md` names this build until
  it lands, and gives record 0026's change the halves of 145 and 147
  that rest on 173.
- **An affinity per program (147).** The intent's document,
  `intents/<digest>.json` (153), keeps the stream and, for each program
  its agents run, the pool and account the first of them on that program
  started on. A start takes the account for the stage's program while it
  is usable. A start on one program leaves every other program's account
  as it was and is no move. A spent or full account moves its own
  program's account alone.
- **An older document reads as its account's program's.** A document
  written before holds one account and no program. flume reads its
  program from the pool the host's configuration declares, and writes
  the document in the new shape at its next change.
- **The intent's move names its program.** A move `on` `intent` gains
  `program`, and status says the intent moved for which program.
- **A pool that cannot run a stage refuses it (4, 32, 145).** A launch
  where a stage's pool holds no account of its program is refused,
  naming the stage, the pool and the program, with nothing made. At a
  later start, read against the host's configuration as it stands, the
  stage is refused the same way and starts no agent until a retry. A
  waiting stage whose pool loses its last account of its program is
  refused at the next settle, room or none. A pool that holds such an
  account, spent or full, still waits for room as today.
- **The queue's want of a program goes.** No stage waits on an account
  no pool holds, so the queue never gives `room` `program` again.
- **The site** says how the affinity keeps one account per program,
  what refuses a stage for its pool, and what status gives.
- **Conformance** reads the statement at a15d927, scenarios S1 to S81.
  S80 and S81 are each held by an engine scenario, and S81 also by a
  walkthrough under the printed command.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `account-pools`: the choice takes the affinity for the stage's
  program; the intent keeps an account per program; a pool that holds no
  account to run a stage refuses its launch or its start.
- `place-status`: the agent's account from the affinity for its
  program, and the intent's move for which program.
- `seam-contracts`: the affinity's type per program, reading a document
  written before, and the intent's move's program.
- `store-profile`: an intent's document written before reads from its
  file.
- `clause-coverage`: the suite's list at a15d927, S80 and S81 cited.

## Impact

- agentplot/blueprints: the statement's amendment on main at a15d927,
  accepted with the record, which the build reads (core.19). Nothing in
  weir changes, and no kind gains a field (core.4).
- `docs/adr/0020-…`: a second amendment section. `docs/architecture/core.md`:
  core.26's, core.30's and core.32's details and core.32's rule.
  `docs/architecture/_open.md`: an item for this build, and record
  0026's item.
- `crates/flume-seams`: `Affinity`'s `accounts` and `AffinityAccount`,
  its older `pool` and `account` read and never written, `Affinity::left`
  by program, and `MoveOn::Intent`'s `program`.
- `crates/flume-core`: `accounts.rs`'s choice, write and read of the
  affinity, and `Choice::NoAccount`; `start.rs`'s claim; `room.rs`'s
  `wants` and `start_waiting`; `launch.rs`'s check; `answer.rs`'s
  `RoomWanted` and `MoveStatus::words`.
- `crates/flume`: the person's form of status; the store profile's and
  the walkthroughs' tests.
- `crates/flume-conformance`: the clause list and the engine's
  scenarios in `account_pools.rs`, S80 and S81 among them; the store
  seam's scenario.
- `site/`: the agent-settings guide, the state and configuration
  references, and the rules page.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency.
