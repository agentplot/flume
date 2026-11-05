# Design

## Context

flume's main at f9a51c2 runs a host's work in streams (record 0020).
`crates/flume/src/host/config.rs` reads one file,
`$XDG_CONFIG_HOME/flume/config.toml`, into `HostConfig`: `[host]`'s
bound, terminal and pool, `[streams.<stream>]` with `agents`,
`priority` and `pool` (any other key refused), `[pools.<pool>]`, the
`stream` named in `[repos."<repository>"]`,
`[repos."<repository>".kinds.<kind>]` and `[kinds.<kind>]`, the agents
tables of the same kinds' tables, and `[roles]`. A fault anywhere in
the file refuses every command. `RealHost` (`host/mod.rs`) reads it
once per command and answers the host's facts from it (`Host` in
`flume-seams/src/host.rs`).

`crates/flume-core/src/room.rs` keeps the host's queue. `Engine::queue`
reads, under the room lock, each stream's bound, priority and agents
running, each pool's accounts, and every stage waiting for room, sorted
by `Waiting::rank`, the stream's priority then the place's, then the
order they came to wait. `wants` names the room a stage is short of,
`must_wait` holds a stage due to start behind a waiting one that ranks
as high and could start, and `start_waiting` claims the waiting stages
in order as room allows.

`Terminal::open` takes `beside: Option<&str>`, the pane a stage's
settings name. `Settings::beside` is given by the launch's settings and
the stage's declaration alone (`placeless` in `settings.rs` takes it
from every other layer). The herdr profile
(`crates/flume/src/profiles/terminal/herdr/mod.rs`) opens every other
pane on its one `server`, `Server::Session("flume")` on the host,
`SESSION`, started headless by `herdr --session flume server` where
`herdr session list --json` lists it not running. It records one
workspace per repository and base, `workspaces/<digest(repository,
base)>.json`, and one tab per place, `tabs/<digest(location)>.json`,
each holding its server's socket. `tidy`, `kept`, `show` and `shown_at`
read those records. The printed command prints the command whatever
the settings name.

A kind is read through weir-kinds (core.4). Its `Kind` holds the name,
version, branch prefix, `starts_unready`, `lands`, its agents and its
stages, and keeps every key it does not know in `sections`. The seam's
`Kind` is flume's own copy of what it reads, and holds no label. weir's
statement names no `label` for a simple kind (weir 52, C.4), and gives
a statechart kind's `label` the words a person reads for it (weir 94).

The statement's amendments are on agentplot/blueprints main: 7367604
(32, 107, 143, C.5's herdr row, C.7, S82), 4b4c5e0 (32, section 9,
C.7, S83) and 2c95a70 (70, 143, the vocabulary's label, reserve and
room, 180 to 183, C.2, C.7, section 10, S84, S85). Records 0031, 0032
and 0033 decide them, proposed until the user's review of this change
accepts them. The build reads the statement at 2c95a70 (core.19).

The change `an-intents-affinity-is-kept-per-program` changes
`room.rs`, `accounts.rs`, `start.rs` and the clause list, and states
S80 and S81. This change is built after it lands, on main as it leaves
them.

The rules this design relies on: core.1 and core.2 (the core names no
terminal, and one interface per seam), core.4 (a kind's fields read
through weir's crate alone), core.13 (no daemon), core.14 (the JSON
form only gains fields), core.16 and core.17 (held state by
compare-and-set, every act safe to repeat), core.18 (flume removes only
what it made), core.19 (the clauses cited), core.26 as amended (the
queue's room and order), core.29 (every herdr call bounded, herdr's
words read from the pinned release), core.30 (no variable's value in
status), core.35 (the site's neutral voice), core.38 (points and pace
left to record 0025's change), and core.44, core.45 and core.46, drawn
with this change's records. The one rules page is
`docs/architecture/core.md`. Where flume has no rule, the kit
standard's code profile binds (kit:specs.3, kit:testing.5).

## Goals / Non-Goals

**Goals:**

- A stream's agents open in a session of the stream's own, named by
  its configuration or its name, flume's own for the default stream;
  a launch may name another (107, 143, S82).
- The host's configuration in two layers, the local one setting only
  C.7's keys, the managed one pinning, every value given with its layer
  (32, C.7, S83).
- Labels: a place's label resolved at each start, reserves lent and
  taken back, slots kept free, what a stream's labels refuse, and
  status (70, 180 to 182, S84, S85).

**Non-Goals:**

- A kind's own label read from a real declaration. weir-kinds reads
  none, flume reads no field of the format itself (core.4), and the
  field's name waits on the user's ruling, since weir 94 gives `label`
  another meaning. The seam's kind carries a label, the fakes set it,
  and the weir profile gives none until weir-kinds reads one.
- Clause 183, an MCP server naming its labels: how a host's servers
  reach an agent is section 10's question.
- A stream's `points` and `pace`, record 0025's change. The local layer
  may set them once that change reads them. Until then the reader
  refuses them in either layer, as it does today.
- A change to the herdr plugin. Each session's server runs the plugin's
  watch, and `bases` gives each base once per workspace.

## Decisions

### 1. Where an agent opens

The terminal seam gains where an agent opens, and `Terminal::open`
takes it in place of `beside`:

```rust
pub enum OpenIn {
    /// A split beside the pane the settings name (107).
    Beside { pane: String },
    /// The terminal's part of this name, a herdr session under herdr.
    Session { name: String },
    /// The terminal's own, flume's session under herdr.
    Own,
}

fn open(&self, place: &PlaceRef, folder: &str, command: &AgentCommand,
        identity: &Identity, at: &OpenIn, session: Option<&str>) -> Opened;
```

The last argument stays the session name a role run's agent is
reached by (132). The seam names no terminal (core.1).

`Settings` gains `session: Option<String>`, read with a default and
skipped where none, so held settings written before read with none.
`--agent [<stage>.]session=<name>` gives it (section 8). The launch's
layer alone keeps it: `placeless` takes `session` away as it takes
`beside`, and the kind's layer takes it away too, since a kind names
nothing a host owns (158). The weir profile reads no `session` from a
declaration, and the host's agents tables read none, as today: each
ignores a key it does not know.

`Stream` gains `session: Option<String>`. Held state gains:

```rust
pub enum SessionFrom { Launch, Stream, StreamName, Own }
pub struct SessionHeld { pub name: Option<String>, pub from: SessionFrom }
```

`name` is none for `Own`. `StartHeld` gains `session:
Option<SessionHeld>`, none for an agent opened beside a pane and in
held state written before.

The claim resolves where the agent opens, under the room lock, from
the stage's resolved settings and the place's stream as the queue read
it (32, 107, 143):

1. `beside` in the settings: `OpenIn::Beside`, no `SessionHeld`.
2. `session` in the launch's settings, the stage's own before every
   stage's: from `launch`.
3. The stream's `session` as the host's configuration stands now: from
   `stream`.
4. A stream other than `default`: the stream's name, from
   `stream-name`.
5. Otherwise `OpenIn::Own`, from `own`.

`StreamRoom` carries the stream's `session` for this. The claim writes
the `SessionHeld` on the start, and `open` builds the `OpenIn` from it
and the settings' `beside`, so a start's open and its record never
disagree. A stream's session changed between starts takes the next
start, and an agent started stays where it opened (32). A stream a live
place runs in that the configuration no longer declares opens under its
name.

Status gives `AgentStatus::session` from the start, and
`StreamStatus::session`, the stream's own resolution, steps 3 to 5
(56). The person's form says `session work, the stream's name`, `in
the terminal's own session`, and so on.

### 2. herdr's sessions

`Herdr::open` reads `at`:

- `Beside`: the split beside the named pane, as today.
- `Own`: the profile's own server, as today: `flume` on the host, the
  test's socket in a test.
- `Session { name }`: `Server::Session(name)`'s socket, found by
  `herdr session list --json` and started headless by `herdr --session
  <name> server` where none runs. Today's `socket()` and
  `start_session()` become `socket_of(&Server)`, used for both. A name
  the pinned herdr refuses refuses the open with herdr's words, and the
  profile names no rule of its own for a session's name (core.29). herdr
  0.9.5 refuses such a name before its server starts, its words on the
  server's stderr, which the profile keeps under its records'
  `servers/`, so a server that ends before herdr lists it running is
  refused in those words.

Within the server so found, the layout is record 0018's: the
repository and base's workspace, labelled `<repository> <base>`, and
the place's tab `<kind>--<name>`, each made when first needed.

The records are kept by server. A workspace's record is
`workspaces/<digest(socket, repository, base)>.json`, and a tab's
`tabs/<digest(location, socket)>.json`, each holding its server's
socket, the repository and base or the place, as today's do. A record
at today's key, `workspaces/<digest(repository, base)>.json` or
`tabs/<digest(location)>.json`, is read as the record for the server it
names, so a place opened before this change is read, shown and tidied
where it stands (138).

- `tidy(place)` closes the place's tab on every server a record names,
  then each base's workspace on that server once it holds no tab of a
  live place and no pane flume did not open, keeping and naming it
  otherwise (137, 138). A place that has no tab, its agents opened beside
  a pane, tidies the base's workspace on every server, as before.
- `kept()` reads every workspace record, of every server.
- `show(place, clues)` titles the live agent's pane on its own server,
  and gives the base's count to each of the base's workspaces, on every
  server (139).
- `shown_at(place, agent)` gives the tab and the base's workspace on
  the live agent's server, else the place's newest tab's, and the base's
  newest workspace where the place has no tab. `ShownAt` gains `bases:
  Vec<String>`, each workspace the base has on any server, as
  `herdr:<id>@<socket>`.

`Engine::bases` (`clues.rs`) gives a base once for each workspace
`shown_at`'s `bases` names, each with the base's one count, and once
with none where it names none. A place's clues leave `bases` out, as
they leave `base`. The herdr plugin's watch already refreshes each
entry whose `shown_at` is on its own server, so it refreshes each
session's workspace with no change.

A test's profile reaches named sessions under a scratch home, never
the host's. `Herdr::with_session_home(path)` runs every herdr call and
every session server the profile starts with that home in the
environment the pinned herdr reads its sessions from, and the tools
test reads which variables those are from the pinned release
(core.29). herdr 0.9.5 reads `XDG_CONFIG_HOME`, else `~/.config`, and
keeps a session at `<it>/herdr/sessions/<name>`, its socket
`herdr.sock` there.

Each tools test that starts a named session has a session home of its
own: a short link in the run's own scratch, `SLUICE_TMP` under sluice
and the temporary folder run directly, to a folder of the test's, so
two tests may each name a session `work`. Its sessions are named as the
spec's scenarios name them, `work` and `personal`. sluice's sandbox lets
a check write `SLUICE_TMP` alone, and since sluice 0.66.0 a place's
scratch is `/tmp/sluice-<uid>/<repository's key>/<place's key>`, 45
bytes on macOS. herdr 0.9.5 binds a named session's two sockets,
`herdr.sock` and `herdr-client.sock`, in `herdr/sessions/<name>` under
the session home, whatever `HERDR_SOCKET_PATH` or
`HERDR_CLIENT_SOCKET_PATH` says, and a socket's address holds 103 bytes
of path on macOS. `personal`'s client socket is the link's path and 42
bytes, about 97 in all, and a test whose path leaves no room fails
naming its bytes and the limit. The user ruled it on 2026-11-05, once
sluice gave a place's scratch a short path: "return the session tests
to running under sluice as designed, check herdr's two sockets fit
under macOS's limit there". The named-session tests run by hand
outside the gate verify nothing, as the user ruled on 2026-11-03: "a
test run by hand is no verification."

### 3. The printed command and the fakes

The printed command takes `at` and prints as before: a session names
nothing there. Status still gives the agent's session as resolved.

The fake terminal logs each open with where it was asked, on a line
of its own after the open's, `at <identity> in session work`, `in
own`, `beside operator`, so every scenario reading the open's line
reads it as before, and models a workspace per session and base, so
`shown_at` gives each base's workspaces.

The fake host scripts each stream's session and labels' shares, the
labels the configuration gives kinds, and the configuration's values
with their layers (section 5). The fake kinds script a kind's label.
No fake reads a configuration file (106).

### 4. The host's configuration in two layers

`config.rs` gains `local_config_file()`,
`$XDG_CONFIG_HOME/flume/config.local.toml`, and
`HostConfig::read_layers(managed, local)`, which `of_host` calls. No
command writes either file (core.16).

**The local layer's keys.** One list in `config.rs`, `LOCAL_KEYS`,
holds C.7's keys as paths of segments, `*` any one name:

| key | |
|---|---|
| `streams.*.agents`, `.priority`, `.points`, `.pace`, `.session` | 107, 143, 166, 167 |
| `streams.*.labels.*.reserve`, `.keep_free` | 181 |
| `repos.*.stream`, `repos.*.kinds.*.stream`, `kinds.*.stream` | 143 |
| `repos.*.kinds.*.label`, `kinds.*.label` | 180 |
| `kinds.*.agents.*`, `kinds.*.agents.stages.*.*`, and the same under `repos.*.kinds.*` | 32 |

Each key is one unit, its value taken whole, a `pace` table or a
`mode` table of programs among them (core.30). A later change that
gives the local layer another key adds it to the list.

**Reading the local file.** It is parsed as TOML and walked. A path
that is a key of the list is a unit. A table whose path leads to some
key of the list is walked further. Any other path is a key the local
layer may not set. Each such key is named once, by the shortest part of
its path that leads to no key of the list, and at least two segments
where the path has two: `pools.work` for anything in `[pools.work]`,
`host.agents`, `roles."a/b"`, `streams.work.pool`. A file with any such
key, or that is not TOML, is not taken.

**Pins.** `[host] pin` is a list of strings, each a dotted key in
TOML's spelling, read by parsing `<pin> = 0` as a document. The
repository segment after `repos` is compared with its `~` read as the
home, as a repository's key is. A pin names a key of the list, or a
table leading to some, as `"streams.work"` or `"streams"` does. A pin
that names neither refuses the configuration naming it, as any fault
of the managed layer does. A local unit whose path the pin equals or
lies under is passed over, named with its pin.

**Merging.** Each unit not passed over is set into the managed table,
replacing the managed value whole. A unit under `repos` is set into the
managed layer's table for the same repository, the two keys compared
with `~` read as the home, so `~/Code/acme` in one layer and its full
path in the other name one repository. The merged table is read by today's
reader, extended (section 5). A reading of the merged table that fails
where the managed table alone reads is a fault of the local layer: the
local layer is not taken, its fault named. A fault of the managed
layer refuses every command, as today. A stream only the local layer
names is declared by it, with no `pool`, so its places take `[host]
pool` (144). A host with no local file reads the managed table alone.

**Each value's layer.** The reading keeps, by dotted path, the layer of
each unit the local layer set. Every other value is the managed
layer's. The host's facts give:

```rust
pub enum ConfigLayer { Managed, Local }
pub struct ConfigValue { pub key: String, pub value: String, pub layer: ConfigLayer }
pub enum LocalLayer {
    NoFile,
    Taken,
    NotTaken { keys: Vec<String>, fault: Option<String> },
}
pub struct PinnedKey { pub key: String, pub pin: String }
pub struct HostConfiguration {
    pub local: LocalLayer,
    pub values: Vec<ConfigValue>,
    pub passed_over: Vec<PinnedKey>,
}
```

`values` holds every leaf of the merged table as TOML writes it, a
unit of the list whole, an array of tables by index
(`pools.work.accounts[0].name`), and the values of every `env` table
as `<set>`, an account's among them, so a variable's value written
where the reader ignores it is never given either (core.30). `Host`
gains `fn configuration(&self) ->
HostConfiguration`.

### 5. Labels and sessions in the configuration

- `streams_declared` takes `session` and `labels` beside `agents`,
  `priority` and `pool`. `session` is a string. `labels` is a table of
  labels, each a table read into
  `Share { reserve: u32, keep_free: u32, unknown: Vec<String> }`,
  `reserve` and `keep_free` whole numbers from zero, zero where left
  out. Any other key is kept in `unknown` rather than refused, since
  182 refuses the start that reads it and not the configuration. A
  value of the wrong type refuses its layer, as a stream's other values
  do. `Stream` gains `session` and `labels: BTreeMap<String, Share>`.
- `label` in `[kinds.<kind>]` and `[repos."<repository>".kinds.<kind>]`
  is read beside `stream`. It is one word of letters, digits, `-` and
  `_`, and any other value refuses its layer naming the key (180).
  `HostConfig::label_for(repository, kind)` gives the repository's
  kind's, else the kind's, as `stream_for` does, and `Host` gains `fn
  label_for(&self, repository: &str, kind: &str) -> Option<LabelHeld>`.
- The seams gain `LabelFrom { HostRepositoryKind, HostKind, Kind }` and
  `LabelHeld { name, from }`, and the seam's `Kind` gains `label:
  Option<String>`, read with a default. The weir profile gives none,
  since weir-kinds reads no label (core.4), and its doc comment names
  the open item. flume's own kinds declare none (180).

### 6. Labels in the queue

**A place's label.** `Engine::label_of(place)` is the host's
`label_for(repository, kind)`, else the place's kind's own, from
`kind`, else none (180). The claim writes it on the start,
`StartHeld::label: Option<LabelHeld>`, and status gives the place's
label as it would resolve now (56).

**What the queue reads.** `StreamRoom` gains `session`, `labels:
BTreeMap<String, LabelRoom { reserve, keep_free, running }>`, `held`
and `fault`. A label's `running` counts the live starts of the stream
whose `StartHeld::label` names it, so an agent counts toward the label
it started under until it ends (180). A label the stream states no
table for is owed nothing and counted nowhere. `held` is true where the stream
states a bound and has no fault. Where it states none its reserves are
passed over (181). `fault` is the first of:

- a label's table holding another key, named by its dotted key as the
  stream-labels spec has it: `streams.work.labels.operation.pool: a
  label's table holds its reserve and what it keeps free alone; its
  stream's pool, points and pace are its budget` (182);
- a label keeping more free than its reserve: `the label operation of
  the stream work keeps 2 free of a reserve of 1, and a reserve is at
  least what it keeps free` (181);
- reserves past the bound: `the stream work's reserves, operation 3 and
  construction 3, 6 in all, pass its bound of 5` (181).

`Waiting` gains `label: Option<String>`, resolved by `label_of` as the
queue reads it.

**Room.** Where the stream's reserves are held, `kept_for(stream,
label)` is the sum over the stream's other labels of `keep_free`
less their `running`, none below zero; a place of no label counts every
label's. `wants(stream, label, program)` answers, after a stream at its
bound and before the pool and the host, `RoomWanted::KeptFree { stream,
labels }` where the stream's room is no more than `kept_for`, naming
the labels keeping it (70, 181). Its words: `the stream work's room,
the rest kept free for operation`.

**Order.** The queue's order is today's: the stream's priority, then
the place's, then the order the stages came to wait. Within each
stream, its stages then take the positions its stages hold in that
order, a stage whose label runs fewer than its reserve first, each part
in today's order. So a label below its reserve goes first in its
stream, whatever the priorities, and the streams' turns keep their
places (70, 181). `must_wait` places a stage due to start where a
waiting stage would stand in that order, and holds it where a stage
before it could start now.

**One settle.** `start_waiting` takes the first stage in the order that
wants no room, claims it, counts its agent toward the host, its stream,
its label and its account, then orders what is left again, until the
host has no room or nothing left can start. A label that reaches its
reserve mid-settle so goes back among its stream's others (181).

**What refuses.** The claim reads the stream's `fault` before its
resolution, under the room lock. A fault leaves the stage refused,
`refused_setting` holding the words, as a setting that cannot run does
(4). A retry once the configuration is mended starts it. No agent is
ended for a reserve or a fault (31).

### 7. Status

The JSON form gains, and loses nothing (core.14):

- `PlaceStatus::label: Option<LabelHeld>`.
- `AgentStatus::session: Option<SessionHeld>`.
- `StreamStatus::session: SessionHeld`, `labels: Vec<LabelStatus { name,
  reserve, keep_free, running, waiting }>`, `reserves: held |
  passed-over | none`, and `refused: Option<String>`. `reserves` is
  `none` where the stream gives no label a share, `passed-over` where
  it states no bound, and `held` otherwise. A fault is given by
  `refused` alone.
- `Queued::label`, and `RoomWanted::KeptFree`.
- `HostStatus::configuration: HostConfiguration`.
- `BaseClues` once per workspace (section 2).

The person's form (`commands/print.rs`) gives the place's label and
where from, the agent's session and where from, each stream's session,
its labels with their reserves, what they keep free, running and
waiting, the reserves passed over and what the stream refuses, and the
configuration's local layer: taken, with each value it set, or not
taken, naming each key or the fault, and each key passed over with its
pin; nothing where the host has no local file, so a host with one
layer prints as before. A place's label goes on its stream's line, a
stream's labels after its session's line, and the local layer after
the pools. It names no variable's value (core.30).

### 8. The command line

`--agent`'s `FIELDS` gains `session`, read as `beside` is, for every
stage or for `<stage>.session`. `SETTINGS`, the words its refusals and
`--help` give, names it. The JSON a launch's settings are kept in gains
the field.

### 9. How it is held

- `flume-seams/src/round_trip.rs`: the new types, and held state, a
  kind and settings written before reading with none.
- `flume-fakes`: the fake terminal's log of where it opened, the fake
  host's and the fake kinds' scripts.
- `flume-conformance/src/engine/sessions.rs`: the stream-sessions
  spec's scenarios, S82 among them, over the fakes.
- `flume-conformance/src/engine/labels.rs`: the stream-labels spec's
  scenarios, S84 and S85 among them, and host-queue's two new ones.
- `flume-conformance/src/engine/host_layers.rs`: S83's core side, the
  fake host's values and layers given by status and its bound taken at
  the next start.
- `flume-conformance/src/engine/places.rs`: one base in two sessions.
- `crates/flume/src/host/config.rs`'s unit tests: the local keys, each
  key named, a fault, pins, the merge, each value's layer and `<set>`,
  labels' tables and words, `session`.
- `crates/flume/tests/host.rs`: `RealHost` over both files in a scratch
  `XDG_CONFIG_HOME`, its `configuration` and `label_for`, and neither
  file written.
- `crates/flume/tests/commands.rs`: S83 through the binary, `flume
  status --json` and the person's form; `--agent session`; and a
  launch, its settles, status and its tidy leaving both files as they
  were.
- `crates/flume/tests/profiles/kinds_weir.rs` and `built_in.rs`: a
  stage's agents `session` gives no setting, and flume's own kinds
  declare no label.
- `crates/flume/tests/profiles/tools/terminal_herdr.rs`: two streams'
  sessions under the pinned herdr with a scratch session home, S82
  among them, a session name herdr refuses, a place tidied in two
  sessions, and records written before.

### 10. The site

Every page speaks to any host's operator (core.35).

- `guides/agent-settings.md`: under "Streams", a stream's session and
  labels, with C.7's example and S84 told as a story. Under "On the
  host", the two layers and pins.
- `reference/configuration.md`: the two files, `[host] pin`,
  `[streams.<stream>] session`, `[streams.<stream>.labels.<label>]`,
  `label` in the kinds' tables, and "What refuses" with the local layer
  not taken and a stream's labels refused.
- `guides/places-and-herdr.mdx`: a session per stream, `herdr
  --session work`, and flume's own for the default stream.
- `reference/state.md`: the status fields of section 7.
- `reference/commands.md`: `--agent [<stage>.]session=<name>`.
- `roadmap.md`: the three built, and decided and not built, a kind's
  own label, waiting on the kind format, and an MCP server's labels,
  waiting on how a host's servers reach an agent.

### 11. The clause list

`clauses.rs` reads the statement at 2c95a70: Part A gains 180 to 183,
the scenarios run to S85, and 183 is named unbuilt, by the answer to
section 10's question on how a host's MCP servers reach an agent. 180
to 182 and S82 to S85 are cited by this change's scenarios.

## Risks / Trade-offs

- **A kind's own label is unread.** A repository that declares `label`
  in its kind gets no label until weir reads one, and status says the
  place has none. The host's tables give every label meanwhile, and
  flume's own kinds need them anyway (180).
- **A control plane's bad write is quiet.** A local layer not taken
  runs the host on its managed layer, and only status says so. The
  other path, refusing every command, would let one bad write stop
  every place (record 0032).
- **More watches.** Each session's server runs the herdr plugin's
  watch, each settling every place. Settles are safe to repeat
  (core.17), and the cost is a settle per watch's tick.
- **A place across two sessions.** A stream's session changed mid-place
  puts its next agent in another session, and the place's earlier tab
  stays until its tidy.
- **A reserve held while its label is idle is lent, not kept.** A label
  that wants a slot at once keeps it free, and its other agents start
  at the next agent's end, never by ending one (31).

## Migration Plan

- Held state, kinds and settings gain fields read with defaults. A
  place launched before reads with no session, no label and no
  `session` setting.
- A host with no streams runs every place in `default`, so nothing
  moves: its agents stay in flume's own session.
- A host with streams opens each stream's next start in the stream's
  session. Agents running stay where they opened, and their tabs and
  workspaces, recorded at the old keys, are read and tidied where they
  stand.
- A host with no `config.local.toml` reads as today, and status says
  the local layer has no file.

## Open Questions

- What weir names a simple kind's label, given weir 94's statechart
  `label`: the user's ruling, in the statement first (core.19). This
  change builds the host's labels and the seam's kind label either way.
- How a host's MCP servers reach an agent, section 10's question, on
  which 183 waits.
