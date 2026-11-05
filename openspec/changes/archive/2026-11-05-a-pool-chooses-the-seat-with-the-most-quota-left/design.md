# Design

## Context

flume's main at cc93941 chooses an account at each start of an agent
whose stage has a pool (145). `Engine::queue` in
`crates/flume-core/src/room.rs` reads, under the room lock, each pool
the host's configuration declares, and gives each account an
`AccountRoom`: the `Account` as declared, flume's agents `running` on
it, and `spent_until` from its document `accounts/<pool>/<account>.json`
(146, 153). `AccountRoom::short` answers why an account cannot take a
start, `MoveWhy::Spent` or `MoveWhy::Full`, and `usable` adds that it
runs the stage's program (144). `PoolRoom::strategy` in
`crates/flume-core/src/accounts.rs` takes the first usable account
under `Strategy::FillFirst` and the one running fewest under
`Strategy::LeastBusy`, equals in order. `Engine::choose_account` takes
the intent's affinity for the stage's program first while it is
usable, and records the intent's move where it is spent or full
(147). With nothing usable, `PoolRoom::usable` answers
`Usable::None(until)`, the soonest `spent_until`, which status gives as
the pool's soonest time (`RoomWanted::Pool`).

The host's facts reach the core through the `Host` trait in
`crates/flume-seams/src/host.rs`, which is no seam of Part B: its
clock, its configuration's streams and pools, a file's write time, the
login environment, the room lock. `crates/flume/src/host/config.rs`
reads `[pools.<pool>]` with `strategy` and `accounts`, each account
with `name`, `program`, `env` and `agents`, and refuses any other key
(C.7). `settings::expand` in the core reads an `env` value's `~` and
`$VAR` against the login environment (29).

Nothing of flume's reads how much of an account's allowance is left.
The agent seam's `spent` part (151) reads one agent's own record once
its turn stopped at the limit. The maker's usage report, which gives
each window's use and reset, is asked only with the account's
sign-in, and claude-swap's way of asking it, renewing idle copies of
each login, kills the account's own (record 0035).

The change `each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`
reads the host's configuration in a managed layer and a local one
(record 0032; C.7 at 4b4c5e0). Pools and their accounts are the managed
layer's alone, so this change's pool and account keys are too, and the
local layer's keys gain nothing. Points and pace (165 to 167) are stated
and unbuilt, by record 0025's change.

The rules this design relies on are core.1 (the core and the suite
name no program and no host's reader: the fake's accounts stand for
any), core.2 (no seam's profile is asked anything new: the reading is a
host's fact), core.3 (an account's reading is read only where its entry
names one, never guessed from a folder), core.14 (the JSON form only
gains fields), core.15 (a fact read where it lives when asked, nothing
kept between asks), core.16 and core.17 (flume writes nothing of a
reading, and a choice is safe to repeat), core.26 (a stage waits for
room, and a launch is never refused for want of it), core.30 (no
configuration home's path in status, and `~` and `$VAR` read only
against the login environment), core.32 (accounts, the pool's
choice and the spent pause) and core.38 (points never counted by a
guess). Record 0030's core.43, proposed, holds too: the reading reaches
flume through the host's configuration, which names its file, and any
host's reader may write it. The design departs from one detail of
core.30, which rules out `~` or `$VAR` read anywhere but in an
account's `env`: an account's `usage` reads them too, against the same
login environment, since it names a file of the same host the account's
`env` names folders of (decision 10). The one rules page is
`docs/architecture/core.md`; group 1 amends core.32 and core.30's
detail once the statement holds the clauses. Where flume has no rule, the kit standard's code
profile binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- A pool may choose the usable account with the most of its allowance
  left, as the host's reading of each account gives it (145, 184).
- An account whose reading is at or past its pool's threshold in any
  window takes no new start, under every strategy, until that window
  resets; its running agents go on (31, 185).
- flume reads the reading where the host keeps it, and asks, fetches,
  writes, keeps and renews nothing for it (146, 184).
- A host that names no reading runs exactly as today.
- Status says why each account was or was not usable (56).

**Non-Goals:**

- A reader. Reading the maker's report is the host's, in whatever way
  keeps its logins whole. flume ships none and names none (core.43).
- An account whose login is refused, taken out of the pool. The host
  names a dead login to its operator its own way. In flume such an
  account's reading goes stale once its reader can no longer read it,
  and ranks after every account with a fresh one; a start on it is
  never begun at its sign-in (35; core.12), as today. A reading that
  says a login is refused is a further decision, not this one's.
- A window for one model alone, held against only that model's starts.
  Every window a reading gives holds every start on its account.
  Section 10's question on one model's share stays open.
- Any change to fill-first, least-busy, the spent pause, the carry,
  the affinity's keeping, the queue's order, points or pace (145 to
  147, 166, 167, 179; core.26).
- A walkthrough under herdr. The printed command's walkthrough holds
  the real host's reading, and nothing of the choice is the terminal's.

## Decisions

### 1. The statement's clauses

The user approves these before anything is built. They are numbered
184, 185 and S86 to S88, which agentplot/blueprints main leaves free
between 183 and 186 and between S85 and S89. Each amendment is
surgical: the sentence named is replaced, and the rest of the clause
stands.

**The vocabulary.**

- **pool** becomes: "a named list of accounts in order, with the
  strategy by which flume chooses one at each start and the threshold
  past which one takes no new start (144, 145, 185)."
- **usable** becomes: "of an account, that it is not spent, not past
  its threshold, and has room under the bound it states (145, 185)."
- After **usable**, two terms:
  - "**reading**: of an account, what the host last read of its
    allowance: for each of its usage windows, how much is used and when
    it resets, written by the host in the file its configuration names,
    and never by flume (184)."
  - "**past its threshold**: of an account, that its reading gives a
    window used to its pool's threshold or past it, until that window
    resets (185)."

**144.** Its first sentence becomes: "A pool names accounts in order, a
strategy, fill-first, least-busy or most-left (145), and a threshold
(185)." After "and may state how many agents run on it at once." it
gains: "An account may name its reading (184)."

**145.** Its second sentence becomes: "An account is usable while it is
not spent (146), is not past its threshold (185), and has room under
the bound it states." Its fourth becomes: "Otherwise, under fill-first,
the first usable account in the pool's order; under least-busy, the
usable account with the fewest of flume's agents running, equals in the
pool's order; under most-left, the usable account with the most left by
a fresh reading (184), what an account has left being the least that
any of its windows leaves, then each usable account with no fresh
reading, equals in either by the fewest of flume's agents running and
then in the pool's order." Its fifth becomes: "With none usable the
stage waits for room (70) until one is, and status gives the soonest
time an account of the pool is neither spent nor past its threshold."

**147.** "When the account is spent or has no room" becomes "When the
account is spent, is past its threshold (185) or has no room".

**184**, new, after 147:

> 184. An account may name its reading: a file of the host's, written
>      by something of the host's other than flume, saying when the
>      host last read the account's allowance and, for each of its
>      usage windows, how much of the window is used and when it resets
>      (C.7). flume reads it as it stands at each choice of an account
>      (145) and when status is asked (56), and never writes it, keeps
>      it or renews anything to make it: flume asks no agent program or
>      its maker how much of an allowance is left, reads no account's
>      sign-in, and fetches nothing (146). A reading is fresh for
>      fifteen minutes from when the host read it, by the host's clock.
>      A window whose reset has passed reads as nothing used. A file
>      that is missing, or that holds no reading in C.7's form, is no
>      reading: status says why, and it refuses no start and fails no
>      settle. An account that names no reading has none.

**185**, new, after 184:

> 185. A pool's threshold is a share of a window, ninety percent where
>      the pool states none (C.7). An account whose reading gives any of
>      its windows used to the threshold or past it is past its
>      threshold until that window resets, however old the reading,
>      since a window's use only grows until it resets; where the window
>      names no reset, only while the reading is fresh (184). An account
>      past its threshold takes no new start under any strategy, the
>      intent's affinity's among them (145, 147). Its agents running go
>      on where they are, and none is ended or moved for it (31): one
>      whose turn stops at the account's usage limit moves as 146 says.
>      flume changes nothing of the account's sign-in, and copies, links
>      and writes nothing of its environment's (146). A reading never
>      shortens or lengthens a spent account's pause, which the agent's
>      own record set (146), and flume records nothing of a reading in
>      its own documents (153). Status gives each account's reading,
>      each window with its use and its reset, when it was read and
>      whether it is fresh, or why there is none, and, while the account
>      is past its threshold, which window holds it and until when (56).

**56.** Its last part, "and for each pool, its accounts, each with its
agents running and, while spent, until when (146)", becomes "and for
each pool, its strategy and its threshold, and its accounts, each with
its agents running, while spent, until when (146), and its reading as
185 says."

**C.7.** The pools paragraph's "`[pools.<pool>]` with `strategy`,
`fill-first` or `least-busy`, and `accounts`, in order, each with a
`name`, a `program`, an `env` read as an agents table's is, and an
optional `agents`, the bound on agents running under it (143 to 145)"
becomes "`[pools.<pool>]` with `strategy`, `fill-first`, `least-busy`
or `most-left`, an optional `threshold`, a whole percentage from 1 to
100, 90 where it gives none (185), and `accounts`, in order, each with
a `name`, a `program`, an `env` read as an agents table's is, an
optional `agents`, the bound on agents running under it, and an
optional `usage`, the path of its reading, its `~` and `$VAR` read as
an `env` value's are (143 to 145, 184)." The example's two accounts of
`work` gain `usage = "~/.local/state/usage/work-1.json"` and
`"~/.local/state/usage/work-2.json"`, and its closing words "on work-1
until it runs four or is spent and then on work-2" become "on work-1
until it runs four, is spent or is past ninety percent of a window its
reading gives, and then on work-2". After the paragraph on each
account's spent time and each intent's affinity, a bullet:

> - An account's reading is one JSON document in the file its `usage`
>   names, which the host writes whole, a new file renamed over the
>   old, and flume reads and never writes:
>
>   ```json
>   {
>     "read_at": "2026-11-05T14:02:11Z",
>     "windows": [
>       { "name": "5-hour", "used": 42, "resets_at": "2026-11-05T17:00:00Z" },
>       { "name": "7-day", "used": 71.5, "resets_at": "2026-11-09T08:00:00Z" }
>     ]
>   }
>   ```
>
>   `read_at` is when the host read the allowance. Each window's `used`
>   is the percentage of it used, from 0 to 100, and `resets_at`, which
>   a window may leave out, when it resets; each time is RFC 3339's with
>   its offset. A window's `name` is the host's, which status gives and
>   flume reads no meaning into. flume ignores every key it does not
>   know, so a host keeps its own beside them, and the file holds no
>   credential. A file that is not this form is no reading, status
>   saying why (184).

**Section 10.** The question on the weekly allowance keeps its words to
"(144 to 146)?", and its answer becomes: "Not decided (record 0020).
Where an account names its reading, its pool starts nothing on it past
its threshold in any window the reading gives, its weekly one among
them (184, 185; record 0035); how one model's share of an account's
allowance is spent is still not decided, and an account is spent only
when its agent's own record says so."

**Scenarios**, after S85:

> - **S86.** A stream's pool is most-left over work-1, work-2 and
>   work-3. work-1's reading, two minutes old, gives its 5-hour window
>   40% used and its 7-day window 80%; work-2's gives 60% and 30%;
>   work-3 names no reading. Four places start, each of an intent of
>   its own. The first starts on work-2, which has 40 left to work-1's
>   20, and the second on work-2 too, its reading unchanged. Once
>   work-2's reading is twenty minutes old with none newer, the third
>   starts on work-1, the one account with a fresh reading; and once
>   work-1 is spent, the fourth starts on work-3, which runs fewer of
>   flume's agents than work-2 (145, 147, 184).
> - **S87.** A fill-first pool's work-1 runs two of flume's agents when
>   its reading gives its 5-hour window 92% used, resetting at 17:00,
>   and its pool states no threshold. Both agents work on, and the one
>   whose turn stops at work-1's usage limit moves as S55 says. The next
>   stage of an intent whose affinity is work-1 starts on work-2, and
>   status says the intent moved because work-1 was past its threshold
>   in its 5-hour window until 17:00. After 17:00, no newer reading
>   written, a place of another intent starts on work-1, while the
>   intent that moved stays on work-2; nothing of work-1's sign-in or
>   its folders was changed (31, 145, 147, 185).
> - **S88.** Every account of a stage's most-left pool is past its
>   threshold or spent: work-1's 7-day window 95% used until Monday at
>   08:00, and work-2 spent until 15:00 though its reading gives it 50%
>   used. The stage waits for room, on the host and not on its caller,
>   and status gives 15:00 as the pool's soonest time; at the first
>   settle after 15:00 it starts on work-2. A reading of work-2 written
>   at 14:00 saying 10% used leaves the pause standing until 15:00 (70,
>   130, 145, 146, 185).

### 2. The reading's form, for a host's reader

This is what any host gives flume; the host that asked for the change
builds its reader to it.

- **One file per account**, at the path the account's `usage` names in
  the managed `config.toml`, `~` and `$VAR` read against the host
  user's login environment. Two pools' entries of one account may name
  one file. Keep it outside flume's own directories and outside the
  account's configuration home, which are flume's and the program's.
- **Written whole**: a temporary file in the same directory, renamed
  over the old, so flume never reads half a file.
- **Written at least every five minutes** while the reader can read the
  account. flume takes a reading as fresh for fifteen minutes from its
  `read_at`, so one missed write costs nothing.
- **`read_at`**: when the reader read the maker's report, not when it
  wrote the file, RFC 3339 with its offset.
- **`windows`**: one entry per window that holds every model of the
  account, for Claude its 5-hour and its 7-day window. Leave out a
  window for one model alone: flume would hold every start on the
  account by it.
- **Each window**: `name`, any string, shown in status; `used`, the
  percentage of the window used, a number from 0 to 100, fractions
  allowed; `resets_at`, when the window resets, RFC 3339, left out where
  the report gives none.
- **Nothing secret.** No token, no credential. Any other key is ignored,
  so the reader's own fields, such as a login's state for its own list,
  may sit beside these.
- **A login the maker refuses**: the reader writes nothing new, or
  removes the file. flume then reads a stale reading, or none, and the
  account ranks after every account with a fresh one.
- **Read without renewing.** How the reader reads the report without
  renewing a login, or without asking at all, is the host's. flume's
  choice depends only on the file.

### 3. The types

In `crates/flume-seams/src/host.rs`:

```rust
pub enum Strategy { FillFirst, LeastBusy, MostLeft }   // "most-left"

pub struct Pool {
    pub strategy: Strategy,
    /// The whole percentage of a window past which an account takes no
    /// new start, none where the pool states none and 90 holds (185).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<u32>,
    pub accounts: Vec<Account>,
}

pub struct Account {
    // name, program, env, agents as today
    /// The path of the account's reading as the configuration wrote it,
    /// none where it names none (184).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
}

/// An account's reading as the host last wrote it (184, C.7).
pub struct Usage {
    pub read_at: Seconds,
    pub windows: Vec<Window>,
}

/// One usage window of an account's allowance (184).
pub struct Window {
    pub name: String,
    /// Hundredths of a percent of the window used, 0 to 10000.
    pub used: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resets_at: Option<Seconds>,
}
```

`used` is kept in hundredths so the types stay `Eq`; the JSON form of
status gives it as a percentage, as the file wrote it. `Strategy::name`
gives `most-left`.

`Host` gains one fact:

```rust
/// The reading in the file at `path`, an absolute path, as it stands
/// now; or why there is none, in words that name no path (184, C.7).
fn usage(&self, path: &str) -> Result<Usage, String>;
```

The core expands the account's `usage` with `settings::expand` against
the login environment (29) and asks `usage` with the result. A `$VAR`
the login environment lacks is no reading, its words naming the
variable. The words never give the path, which may lie in an account's
configuration home, whose path status never gives (core.30, core.32).

In `crates/flume-seams/src/held.rs`, `MoveWhy` gains `Threshold`,
spelled `threshold`, and `AccountMove` gains `window` and `until`, each
skipped where none: for a move off an account past its threshold, the
window that held it and when that window resets. A move recorded before
reads with neither (core.14).

### 4. The host reads the file

`RealHost::usage` in `crates/flume/src/host/` reads the file and parses
it with `serde_json` into the form, its times by `chrono`'s RFC 3339
parser, already a dependency of `crates/flume`. It refuses, in words:

- a file that is missing or cannot be read: "the file is missing", "the
  file cannot be read: <error>";
- a document that is not JSON, or not an object;
- no `read_at`, or a `read_at` or `resets_at` that is not RFC 3339 with
  an offset, naming the key;
- no `windows`, or `windows` that is not a list, or a window with no
  `name`, or a `used` that is not a number from 0 to 100, naming the
  window by its place in the list, `windows[<n>]` from 0, as the
  configuration's refusals name an account.

Every other key is ignored, at the top and in each window. A refusal
fails nothing (184): the core reads the account as having no reading.

The fake host in `crates/flume-fakes/src/host.rs` holds a reading or a
refusal per path, set by a test (`set_usage(path, Result<Usage,
String>)`), and answers "the file is missing" for any other path. Its
seam scenario holds it (106).

### 5. The account's room

`AccountRoom` gains `reading: Reading`, read in `Engine::queue` under
the room lock for each account of each pool, beside its `spent_until`:

```rust
pub(crate) enum Reading {
    /// The account names no reading.
    None,
    /// It names one that cannot be read, with why.
    Unread(String),
    /// Its reading, as read now.
    Read(Usage),
}
```

From it, at the host's clock `now`, with the pool's threshold `t`,
`threshold.unwrap_or(THRESHOLD)`, `THRESHOLD` 90:

- a window's **use now** is none where its `resets_at` has passed, and
  its `used` otherwise;
- the reading is **fresh** while `now` is less than fifteen minutes past
  `read_at`, `FRESH` 900 seconds; a `read_at` later than `now` reads as
  `now`;
- the account is **past its threshold** where a window's use now is at
  least `t` percent, `t × 100` in hundredths: until its `resets_at` where it
  names one, whatever the reading's age, and where it names none while
  the reading is fresh, until `read_at` plus `FRESH`. Where two windows
  hold it, the one that holds it latest is named, with its end;
- what it has **left** is 10000 less the most use now of its windows,
  for a fresh reading with a window, and none otherwise.

`AccountRoom::short` answers `MoveWhy::Spent` first, then
`MoveWhy::Threshold`, then `MoveWhy::Full`. `usable` is unchanged in
shape: not short, and running the program. So everything that reads
usable, the affinity's keeping, `Queue::wants` and `start_waiting`,
takes the threshold with no change of its own.

An account's **back** is the latest of its `spent_until` and its
threshold's end, where it has either, and none where it is only full.
`Usable::None` gives the soonest back of the pool's accounts of the
program, so status's soonest time counts a threshold's end (145).

### 6. The choice

`PoolRoom::strategy` gains `Strategy::MostLeft`: of the usable accounts
of the program, the greatest by, in turn, having what it has left,
what it has left, the fewest of flume's agents running, and the
earliest in the pool's order. A most-left pool none of whose accounts
has a fresh reading so chooses as least-busy does. fill-first and
least-busy are as today, over accounts usable as section 5 reads them.

`Engine::choose_account` is unchanged but for the move: where the
affinity's account for the program is short of `Threshold`, the
intent's move carries the window and end section 5 names. A stage's
move (`stage_moved`) does the same. Both record a move only as today:
the affinity's for its program, never back (147).

The choice stays under the room lock (145): `queue()` reads every
reading inside it, and `take_account` counts each start of the settle
against its account, so two starts at once see the same readings and
the second sees the first's agent. A reading does not change within a
settle, so most-left gives one settle's starts to the account with
most left, up to its own bound (section 9).

### 7. The configuration

`pools_declared` in `crates/flume/src/host/config.rs` reads a pool's
keys `strategy`, `threshold` and `accounts`, and an account's `name`,
`program`, `env`, `agents` and `usage`, and refuses any other as today.
It refuses, naming the key:

- a `strategy` other than `fill-first`, `least-busy` or `most-left`,
  its words naming the three;
- a `threshold` that is not a whole number from 1 to 100;
- a `usage` that is not a string, or is empty.

These are pool and account keys, the managed layer's alone (C.7): the
local layer's keys gain nothing, and a local layer that sets one is not
taken, as for every pool key today. The configuration's values give an
account's `usage` as `<set>`, as they give an `env` value, so status,
which gives every value with its layer, prints no reading's path
(section 8; core.30, core.32).

### 8. Status

The JSON form gains, all additive (core.14):

- `pools[].threshold`: the pool's threshold, 90 where it states none.
- `pools[].accounts[].reading`, where the account names one: `read_at`,
  `fresh`, and `windows`, each with `name`, `used` as a percentage, a
  whole number where it is one, `resets_at` where the reading gives one,
  and `past`, whether that window holds the account past its threshold;
  or `unread`, the words of why there is none.
- `pools[].accounts[].past`, while the account is past its threshold:
  `window` and `until`.
- On a move, `why` may be `threshold`, with `window` and `until`.
- `room` `pool`'s `until` counts a threshold's end, and its words say
  `the soonest back at <time>`.

The person's form prints every time as status prints a spent account's,
the host's clock in seconds; 17:00 below stands for that. It prints a
pool `pool work, most-left, threshold 90%:` and, under each account,
its reading: `5-hour 42% (resets 17:00) · 7-day 71% · read at 16:58`,
`stale, read at 16:40` where not fresh, or `no reading: <words>`; and
`past its threshold in 5-hour until 17:00`. A move off an account past
its threshold reads `the intent moved for claude from work-1 to work-2
at …, work-1 past its threshold in 5-hour until 17:00`. Status prints no
reading's path.

### 9. How this sits with least-busy, points and pace, and spent marks

- **least-busy** stays: it fits a host whose accounts are alike and
  that gives no reading. most-left reads the account's allowance
  itself. With no fresh reading anywhere it falls back to least-busy's
  count, so a host whose reader stops loses nothing it had. Fewest
  running is most-left's first tie-break, so accounts equal by their
  readings share the work as least-busy shares it. Under least-busy and
  fill-first the threshold holds wherever an account gives a reading:
  an account near its limit takes no new work whatever the strategy,
  as the user asked.
- **A burst on one account.** A reading moves only when the host
  writes it, so the starts between two readings all go to the account
  with most left. The intent's affinity already keeps an intent's
  agents on one account for its cache (147). An account's own `agents`
  bound caps how many run on it at once (144), and the threshold stops
  new work on it once a reading shows it near the limit. Dividing what
  is left by the agents running was not taken: it is choosing by agents
  running again, and guesses that every agent spends alike.
- **Points and pace** (166, 167) hold a stream's first starts by its
  callers' estimates; the reading holds which account a start takes.
  Neither reads the other: no points become a share of an allowance and
  no reading becomes points (core.38), and flume plans nothing across
  places (70). A stage held by its stream's pace waits for room, and so
  does one whose pool has no account under its threshold; status says
  which. The threshold holds every start, a later stage's and a fix
  round's among them, where points and pace hold first starts alone:
  every start is a new agent spending the account, and a place's later
  stages are its work begun. This change builds nothing of points.
- **Spent marks** (146) stand as the stronger word. An agent's own
  record that its account reached its limit is the program's, and a
  reading may lag it. So a spent account stays set aside until its
  reset whatever its reading says, and a reading neither shortens nor
  lengthens the pause. flume writes no spent mark from a reading
  either: a reading at 100% is past every threshold already, and is
  read where it lives (core.15). With readings, spent marks grow rare:
  an account past its threshold takes no new agent, so only the agents
  already on it can reach its limit.
- **Logins.** flume chooses only which account a new agent starts
  under. It switches, copies, renews and writes no login (146), and an
  account's sign-in is its configuration home's alone.

### 10. The record and the rule

Record 0035, proposed, is committed with this change: the user's words,
the options for the choice, the near limit and the reading, and why not
the others. Once the user approves this design, the statement gains
decision 1's clauses on agentplot/blueprints main, the record is
accepted on their words, and group 1 of the tasks amends the rule:

- **core.32's rule** gains, after "else the pool's strategy": "; an
  account past its pool's threshold by the host's reading takes no new
  start, under any strategy, and flume reads that reading where the
  host keeps it, writing, keeping and renewing nothing to make it". Its
  note names record 0035.
- **core.32's details** gain. Rules out: a reading asked of an agent
  program or its maker, or made by renewing or reading an account's
  sign-in; a reading kept, cached or written by flume, or read from a
  path the account's `usage` does not name; an account past its
  threshold taken by a start under any strategy or by the affinity; an
  agent running on such an account ended or moved for it; a spent pause
  shortened or lengthened by a reading, or a spent mark written from
  one; a stale reading ranked above a fresh one; a missing or malformed
  reading refusing a start or failing a settle; a reading's path in
  status; points read as a share of an allowance, or a reading as
  points (core.38). Not ruled out: a host that names no reading, its
  pools as before; a most-left pool with no fresh reading choosing as
  least-busy; a threshold of 100; an account held past its threshold by
  an old reading until its window resets; one settle's starts all going
  to the account with most left, up to its bound. Source: 145, 147,
  184, 185, S86 to S88; record 0035.
- **core.30's details**: "`~` or `$VAR` read anywhere but in an
  account's `env`" becomes "`~` or `$VAR` read anywhere but in an
  account's `env` and its `usage`", citing 184 and record 0035. Reading
  them against the caller's session stays ruled out.
- **The site's rules page** mirrors core.32's rule as written.

### 11. Proof

- `crates/flume-seams/src/round_trip.rs`: a pool with `most-left`, a
  threshold and an account's `usage`; a `Usage` of two windows, one with
  no reset; a move `why` `threshold` with its window and end; and a move
  recorded before, which reads with neither.
- `crates/flume-conformance/src/seams/` and the fake host: a reading
  set for a path reads back, a refusal reads as its words, and an
  unknown path reads "the file is missing".
- `crates/flume-conformance/src/engine/account_pools.rs`, over the
  fakes:
  - S86, as decision 1 writes it, the clock moved twenty minutes for the
    stale reading and work-1 recorded spent.
  - S87 over a fill-first pool: the two agents on work-1 neither ended
    nor moved, the intent's move `why` `threshold` with `5-hour` and
    17:00, work-1 taking a start once the clock passes 17:00 with no
    newer reading, and the fake agent's log showing nothing sent to
    work-1's folders.
  - S88: the wait for room, status's 15:00, the start on work-2 at the
    settle after 15:00, and a reading of 10% leaving the pause.
  - A window past its reset, reading as nothing used.
  - A stale reading past its threshold: held until its window's reset;
    and one whose window names no reset, held only while fresh.
  - A threshold of 100, and one of 50, each held at its own share.
  - Least-busy and fill-first pools whose accounts give readings, each
    passing over an account past its threshold, and choosing otherwise
    as S54 does.
  - A most-left pool with no reading anywhere, choosing as S54's
    least-busy does.
  - An account whose reading cannot be read: chosen as one with no
    reading, status giving the words, and the start not refused.
  - Every existing scenario passes unchanged.
- `crates/flume-conformance/src/engine/place_status.rs`: a pool's
  threshold, an account's reading fresh and stale, `unread`, `past`,
  and a move `why` `threshold`, in the JSON form.
- `crates/flume/src/host/config.rs`: the configuration with
  `most-left`, a threshold and `usage` read; a `threshold` of 0, 101 or
  `"90%"`, a `usage` of `7` and a strategy `round-robin` each refused
  naming the key; C.7's example as decision 1 amends it.
- `crates/flume/src/host/`'s reading module: the form read, with a
  fractional `used`, an unknown key at the top and in a window, a
  window with no `resets_at`; and each refusal of section 4.
- `crates/flume/tests/host.rs`: `RealHost::usage` over a scratch file,
  a missing one, and an account's `usage` with `~` read against a
  scratch login environment.
- `crates/flume/src/commands/print.rs`: unit tests of a pool's line
  with its threshold, an account's reading fresh, stale and unread, an
  account past its threshold, and a move off one.
- `crates/flume/tests/profiles/tools/walkthroughs.rs`: under the
  printed command, a most-left pool of two of the stand-in's accounts,
  each `usage` a scratch file written whole, named by `~` under the
  walk's home, the second in the pool's order leaving more. A fix starts
  on the account whose reading leaves more. Its reading is then written
  past ninety percent, and a second fix of the same intent starts on the
  other: the intent's affinity would keep it on the first account but
  for its threshold, so the intent's move gives `why` `threshold` with
  the window and its reset. `flume status --json` and the person's form
  give the pool's threshold, both readings and the first account past
  its threshold, with no reading's path. The fix running on the account
  past its threshold lands under it, and neither reading is written.

### 12. The site

`site/src/content/docs/guides/agent-settings.md`'s "Pools of accounts"
gains `most-left` in the strategies' table, and two sections: "An
account's reading", what a host writes, the form and how often, that
flume reads and never writes it and fetches nothing; and "A threshold",
an account near its limit taking no new start under any strategy until
its window resets, its running agents going on. "What status shows"
gives a pool's threshold and an account's reading.
`reference/configuration.md` gives `strategy`'s three values, a pool's
`threshold` and an account's `usage`, and the refusals.
`reference/state.md`, in "Elsewhere on the host", gives an account's
reading as the host's file flume reads and never writes, and, in the
answer of `flume status --json`, the new fields. The guide's streams
example gives work's accounts `usage` and its closing words as C.7's
example does, S86 is told as a story under "An account's reading", and
"What status shows" quotes the person's form of section 8. The
roadmap's "Built" table gains a row for a pool choosing by the reading
and starting nothing past its threshold. Each is written to any host's
operator, naming no host, team, tool or word of ours, "seat" among them
(core.35).

## Risks / Trade-offs

- **A reading lags.** Between two readings an account's use grows
  unseen. A fresh reading under the threshold may hide an account now
  past it. The threshold's margin and the spent pause cover it: an
  agent stopped at the limit moves as 146 says.
- **A threshold leaves allowance unspent.** An account past ninety
  percent of its week takes nothing new until the week resets. A host
  that wants it spent sets the threshold higher, up to 100.
- **A dead reader.** Where the host's reader stops, readings go stale
  and most-left falls back to least-busy, while an account past its
  threshold by a window with a reset stays held until that reset. That
  is the account's state, since use only grows within a window. A
  window with no reset stops holding fifteen minutes after its reading.
- **A host's clock.** Freshness and resets are read by the host's
  clock. A reader on the same host writes by the same clock.
- **The statement waits.** Nothing is built until the clauses stand on
  agentplot/blueprints main, which the user's approval of this design
  starts.
