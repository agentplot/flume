# Design

## Context

flume's main at c82438a runs a kind through on its own under one host
queue. `Queue` in `crates/flume-core/src/room.rs` counts the host's live
starts against `Host::bound` and orders what waits by the place's
priority, then arrival; `must_wait` and `start_waiting` take the room
under `Host::room`, the host-wide lock. `Engine::layers` in
`settings.rs` resolves an agent's settings from six layers, the host's
two read from `Host::settings` at each start. `HostConfig` in
`crates/flume/src/host/config.rs` reads `[host]`, `[kinds]`, `[repos]`
and `[roles]`. The Claude Code profile's StopFailure hook
(`crates/flume/src/profiles/agent/claude/hook.rs`) records a failed
turn's words as the agent's last words, and its SessionStart hook runs
`ack_started`, which takes the start's secret and nothing else.

Record 0020 (accepted 2026-10-29) is stated at blueprints 6e5aae2: 143
to 147, 151, 153 and S53 to S56 added, 2, 4, 7, 8, 32, 56, 70, C.4, C.7
and I6 amended. Record 0016, amended 2026-10-29 on the user's idea, is
stated at blueprints 8a11fbb: 35 and 135 amended, 161 and S65 added,
and C.4's started and spent rows say how Claude Code meets them.
Record 0020, amended 2026-10-29 as the user approved this plan, is
stated at blueprints 87a7458: accounts exist only in pools, an account
gives an agent its whole environment, and the host's tables give a
model and instructions (29, 32, 143, 144, C.2, C.7, S44). The
rules this design relies on are core.1 and core.2 (the core names no
tool, and no profile calls another), core.4 (the format is weir's),
core.10 (one fresh agent per start, taken on its word under its
identity and settings), core.13 (no daemon), core.16 (one document per
place, written by compare-and-set), core.26 (the queue across streams),
core.30 (six layers, the environment from the account alone, no
value printed), core.32 (accounts, spent
pauses, affinity) and core.35 (the site's neutral voice). The one rules
page is `docs/architecture/core.md`; this change touches no area
another page would own. Where flume has no rule, the kit standard's
code profile binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- A host runs its places in named streams, each under its own bound and
  priority within the host's, and a full stream never holds another's
  room (70, 143; core.26).
- A stage's agent runs under an account of its pool, chosen fill-first
  or least-busy, and a spent account moves the stage to another with no
  one's word (144 to 146, 151, 153; core.32).
- An intent's agents keep to one account while it is usable, so they
  read what they share from the prompt cache (147).
- No agent runs a stage under settings other than its start's (161).
- An agent's environment is its account's and nothing else's (32, 144).

**Non-Goals:**

- The weekly allowance overall, which section 10 leaves open: an account
  is spent only when its agent's own record says so.
- The intent's brief, the goals and the standing prompt (record 0021):
  this change gives an intent its name and nothing it says. Record
  0021's plan adds the brief to the same launch field.
- Kinds named by source, locks and packages (record 0022).
- Asking Claude Code or its maker whether a limit has reset, or a
  process of flume's that wakes at a reset: the next settle after it
  finds the account usable (core.13).
- A field of the kind format. No account, pool or stream is one: flume
  reads none from a kind's `agents` table, which weir-kinds returns as
  written (`KindAgents`), so nothing in weir changes (core.4).

## Decisions

### 1. Streams and pools in the host's configuration

`flume-seams` gains `Stream { agents: Option<u32>, priority: i64, pool:
Option<String> }`, `Pool { strategy: Strategy, accounts: Vec<Account>
}`, `Strategy::{FillFirst, LeastBusy}` spelled `fill-first` and
`least-busy`, and `Account { name, program, env, agents: Option<u32>
}`, each serde and round-tripping. The `Host` trait gains `streams()`,
the declared streams by name, `stream_for(repository, kind)`, the
stream the configuration names with the key it came from,
`pool_for(stream)`, the stream's pool, else `[host] pool`, with the key
it came from, and `pool(name)`. `RoleRun` gains `stream`. All are read
at each call as the configuration stands, as `settings` is (32, 144,
C.7).

`HostConfig` reads `[streams.<stream>]`, `[pools.<pool>]`, `[host]
pool`, `stream` in `[repos."<repository>"]`,
`[repos."<repository>".kinds.<kind>]` and `[kinds.<kind>]`, and `stream`
in `[roles."<address>"]`. An account's `env` is kept as written, its
`~` and `$VAR` read at each start against the login environment (29).
The read refuses, naming the key: a strategy other than the two; a
stream or `[host] pool` naming an undeclared pool; a pool with no
accounts, or two accounts of one name; an account with no `name` or
`program`; an `agents` that is not a positive integer. A configuration
that refuses leaves every launch and start refused naming it, as an
unreadable one does today. With no `[streams]` table the only stream is
`default`, held by `[host] agents` alone; with no pool named, no
account is chosen and agents run under the login environment alone
(144).

### 2. A place's stream and intent

`Launch` gains `stream: Option<String>` and `intent: Option<String>`,
in its JSON form and on `flume launch --stream <stream> --intent
<name>`. `PlaceRecord` gains `stream: StreamHeld { name, from }` and
`intent: IntentKey`, both kept in held state.

`IntentKey` is the launch's name, else the repository and base. Its
digest, sha256 over `name:<name>` or `base:<repository id>:<base>`,
names its document (decision 5). `Engine::launch` resolves the stream
once (8): the launch's, else the stream of the intent's affinity where
one stands, else `stream_for` for the repository and kind, for the
repository, for the kind, else `default`. `from` records which: `launch`,
`intent`, `host-repository-kind`, `host-repository`, `host-kind` or
`default`. A stream the host does not declare refuses the launch naming
it, leaving nothing made (4; core.8). `default` is always declared. A
role's woken run takes its `[roles]` table's `stream` as its launch's
(132).

### 3. The environment from the account, and six layers

`Settings` loses `env`, so no launch, host, repository or kind layer
gives a variable (32, 144; core.30). The host's agents tables are read
for `model` and `instructions` alone, and any other key there is not
read, as keys that are no setting are not read today. `--agent` and
the launch's JSON form give `program`, `model`, `instructions` and
`beside`. The kinds profile reads no `env` into a kind's settings, and
an `env` in any declaration's agents table refuses the launch naming
the field (C.2, 158), in place of `no_repository_env`.

`Engine::layers` keeps its six layers for the program, the model and
the instructions. The resolved program picks which of the pool's
accounts are usable (144). The agent's environment is the chosen
account's `env`, each value read against the login environment by
`expand`, and a variable the login environment lacks refuses the start
naming it and the account (27, 29). With no account the environment is
empty, and the terminal gives the login environment beneath it.
`Layer` gains `Account`, the source `from` names for each `env.<NAME>`,
with the value as the account wrote it.

### 4. The queue across streams

`Queue` gains, for each stream, its live starts, its bound and its
priority, and for each pool's accounts, their live starts and whether
each is spent. `Waiting` gains its stream and the stream's priority, and
the sort is the stream's priority, then the place's, then `order`, then
`since` (70). A stage may start only while the host, its stream and an
account of its pool all have room. `must_wait` asks that, and whether a
stage that ranks at least as high waits and could start now.
`start_waiting` walks the queue in order and passes over a stage that
cannot start, never stopping at it, until the host's room is spent. A
stage passed over keeps its order, so it starts first once its stream or
pool has room (core.26). `HostStatus` gains `streams`, each with its
running agents, bound, room and queue, and a stage waiting on its pool
names the soonest time an account of the pool stops being spent (56).

### 5. Accounts, the host's documents and affinity

The store seam gains its host documents (153): `read_host(key)` and
`write_host(key, at, next)`, compare-and-set on a version as a place's
held state is (102, 103). The state directory's profile keeps
`accounts/<pool>/<account>.json`, the spent time with the words and when
it was recorded, and `intents/<digest>.json`, the stream, pool and
account, each move, and when its last place went. Both live under
`$XDG_STATE_HOME/flume/`, written 0600 (C.7). The fake store keeps them
in memory.

`Engine::claim` chooses the account under the room lock, after the room
check, for a start whose stream or host has a pool (145). It takes the
intent's affinity account while that account is usable: not spent at
the host's clock, under its own `agents`, and running the stage's
program. Otherwise it applies the strategy over the pool's usable
accounts in order: fill-first the first, least-busy the fewest live
starts, equals in order. `StartHeld` gains `account: Option<AccountHeld {
pool, name, from }>`, `from` being `affinity` or `strategy`. With none
usable the stage waits for room, its words naming the pool and its
soonest reset (70).

The first start of an intent with a pool writes its document. A start
the strategy places elsewhere because the affinity's account was spent
or full moves the affinity and appends a move: from, to, why, when. A
move never returns to an account the affinity left. `Engine::tidy` of
the intent's last live place writes `gone` with the host's clock. A read
of an affinity whose `gone` is an hour past removes it by
compare-and-set and reads none, and a later place of the intent writing
it again clears `gone` (147). Status gives each move with its reason
and that the agents after it start without the earlier account's cache.

### 6. A spent account

The agent seam gains `spent(folder, agent) -> Spent`, `Unknown`, `No`,
or `Yes { until: Option<Seconds>, words }` (151). The fake agent scripts
it.

The Claude Code profile's StopFailure hook already records the failed
turn's words. It now also records the `error` and the time it ran in the
agent's record. `spent` reads `Yes` only where the `error` is
`rate_limit` and the words open with one of the usage-limit openings the
pinned 2.1.296 itself recognizes (its own list): "You've hit your",
"You've reached your", "You're out of usage credits", "Your org is out
of usage · add funds to continue", "Your org is out of usage · contact
your admin", "Your seat type doesn't include usage credits", "Your seat
type doesn't include usage", "Your usage allocation has been disabled
by your admin", "Your group's usage limit is set to $0", "Fable 5
requires usage credits", "You're out of extra usage" and "Your seat
type doesn't include extra usage". A `rate_limit` with other words, such
as "Opus is experiencing high load…", "Server is temporarily limiting
requests (not your usage limit)", or "No response requested.", reads
`No`, and the turn's failure is read as 43 reads it (C.4).

The reset is the words' ` · resets <when> (<zone>)` part, wherever it
falls among their ` · ` parts. 2.1.296 builds it as `` `You've hit your
${limit}${" · resets " + when}${" · progress saved"?}` ``, `limit` one
of `limit`, `usage limit`, `session limit`, `weekly limit`, `Opus
limit`, `Sonnet limit`, `Fable limit`, `team's shared budget`, `monthly
spend limit` and the org's limits. `when` is `$R(resetsAt, true)`: an
en-US time of day, `3pm` or `3:30pm`, where the reset is within a day,
else `Nov 2, 3pm`, with `, 2027` after the day in another year. `zone`
is the IANA name Claude Code's `Intl` resolved, such as
`America/New_York` or `UTC`. A time of day is the first such time in the
zone no more than a minute before the hook ran, since the words name no
seconds. A date and time is read in the zone, the year
the hook's own where none is given. The `chrono-tz` crate reads the
zone, pinned in `Cargo.lock`. Words with no such part, or a zone the
crate does not know, give `until: None`, and flume pauses the account
for an hour (146).

A `check-tools` test reads the pinned Claude Code, the file its
`claude` wrapper from devenv.nix execs, and requires the source
fragments above to be there byte for byte: the openings' list, the
`You've hit your ${e}${r}${h}` template, `` ` \xB7 resets ${y}` `` and
`$R`'s zone suffix. A Claude Code bump that changes the words, or only
renames what its minifier names in those fragments, then fails the gate
until the reading is made again and the fragments are taken afresh
(core.22, core.29's rule for herdr held for Claude Code).

When a settle reads a live start `spent` (`Engine::read_end` beside the
last words), it writes the account's document spent until `until`, or
the host's clock plus an hour, unless a later time is already recorded.
Every process's claim reads that document, so no start anywhere takes
the account before then (146, S55). Once the agent is quiet (31) the
settle ends it and starts the stage afresh through `start_stage`, with a
`StartReason::Spent` whose words are flume's own. The words say the
stage's earlier agent stopped at its account's usage limit, and that
what it committed and left is in the place. The start records the move
on the place, counts toward no bound of the kind's (112), and waits on
no caller. A repeating stage's fresh start takes the same group, as a
retry's does (140). A start under no account is read as 43 reads it,
unchanged. Agents running on a spent account are never ended for it,
since each moves when its own turn fails (31).

### 7. A start checked against its settings

`flume-seams` gains `StartRecord { program, account: Option<String>,
env: BTreeMap<String, String> }`, the `env` values digests, and
`StartHeld` gains `record`. `Engine::claim` writes it from the resolved
settings, account included. It covers each variable of the resolved
environment, and `FLUME_AGENT`, `FLUME_PLACE` and `FLUME_STAGE`. Each
digest is HMAC-SHA256 keyed by the start's secret over the variable's
name, a zero byte and its value, in hex (`hmac` over the tree's `sha2`).
A record read off the host then tests no guessed value without the
secret, and no value is kept (161; core.30). `PATH`, `SHELL`, the login
environment beneath the settings and `FLUME_SECRET` are not in it: the
terminal sets the first three for its own sake (record 0018), and the
secret is checked by the ack itself.

The SessionStart hook, before `ack_started`, asks the core for the
start's record by `FLUME_AGENT` and `FLUME_SECRET`, as `agent_folder`
does. It digests each named variable in its own environment, and reads
the program from its parent process. The process-table reading the herdr
profile does today (`executable`, `program_path`) moves to
`crates/flume/src/host/`, which both profiles call, so no profile calls
another (core.2). Where all match it acks as today. Where any differs it
writes the agent's record `unsettled.json`, naming each variable missing
or different by name and whether the program differed. It acks nothing,
and starts `flume settle <place>` detached, as the Stop hook's on-quiet
does (121). The hook still exits 0 within its bound (C.4).

The agent seam gains `started_without(folder, agent) ->
Option<Unsettled>`, with `None` as its default (135). A settle reading
it on a starting start marks the start unsettled with the names. Once
the agent is quiet the settle ends it and starts the stage afresh, with
a `StartReason::Unsettled` that counts toward no bound and sends no
prompt to the agent ended. Where the stage's previous start was also
unsettled, the fresh start is never begun instead, its end naming the
variables and the program, delivered as any never-begun end is (35, 43).
Status names the unsettled start and its variables, never a value
(S65).

A start that carries no `FLUME_*` acks nothing and is read as 77 reads a
bare resume (record 0016), unchanged.

### 8. Status, the JSON form and the command line

The JSON form only gains fields (core.14). A place gains `stream` with
`from`, and `intent`. Its current agent gains `account` with `from`,
and each of its variables reads from `account`.
The place gains `moves`, each with its stage or intent, from, to, why,
when, and `cache: lost`. Unsettled starts are listed with their names.
`HostStatus` gains `streams` (decision 4) and `pools`, each account with
its live starts, bound and spent time. The person's form prints a
place's stream and account on its agent's line, a move on a line of its
own, and the host's streams and pools under the queue. `flume launch`
gains `--stream` and `--intent`, and the launch's JSON form the same two
fields.

### 9. The site

The agent-settings guide gains streams, pools, the two strategies, a
spent account, affinity, the start check, the environment from the
account and the six layers, with the example of C.7 and a host giving
`change` opus and `fix` sonnet. The configuration reference gains every key of
decision 1. The state reference gains the host's documents, and the
commands reference `--stream` and `--intent`. They are written to any
host's operator, naming no team or host of ours (core.35).

## Risks / Trade-offs

- Claude Code could change a variable before a hook sees it. Every start
  would then be unsettled twice and never begun. The record holds each
  value as flume passed it, expanded, and the hook's tests run against
  the stand-in; a `check-live` run with the pinned Claude Code is the
  only proof against the real program, and is the operator's.
- The reset is read from words written for a person. The `check-tools`
  test pins them to the release the tree holds, and a words change
  fails the gate at the bump rather than pausing an account for an hour
  where it should have read the time.
- A pool's least-busy count is flume's own agents on the account, never
  sessions a person runs under the same account by hand.
- An affinity's hour runs from its last place's tidy, which a settle or
  a command runs. A host where nothing runs for a day keeps the document
  until the next read, which then drops it.
