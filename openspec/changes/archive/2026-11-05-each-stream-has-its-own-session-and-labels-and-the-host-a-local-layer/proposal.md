# Each stream has its own session and labels, and the host a local layer

## Why

Record 0020 runs a host's work in streams, each with its own bound,
priority and pool. Three things it leaves out the user decided on
2026-11-03, on their Pending You cards, and each is stated on
agentplot/blueprints main:

- **Every stream's agents open in one herdr session**, flume's own. A
  person who wants to see one stream's agents sees every stream's. The
  user: "Yes: a session per stream, its name by default." (record 0031;
  7367604: 32, 107, 143, C.5, C.7, S82)
- **The host's configuration is one file.** A control plane that moves
  a stream's bound from hour to hour must rewrite the file a fleet
  manages, accounts and all. The user: "Yes: operational keys only,
  manager may pin." (record 0032; 4b4c5e0: 32, section 9, C.7, S83)
- **Within a stream, places compete by priority alone.** A burst of
  changes fills the stream and a fix to a running system waits for an
  agent to end. The user: "The kind declares it; the host may reassign
  it.", "No: seats are the stream's budget.", "Reserves an idle label
  lends; it takes the next free slot back." and "The same, and a label
  may hold a slot empty (operation)". (record 0033; 2c95a70: 70, 143,
  180 to 183, C.2, C.7, S84, S85)

The records are proposed until the user accepts them on their review of
this change. The change is built once they do.

## What Changes

- **A session per stream (107, 143).** Where the launch's settings and
  the stage's declaration name no place in the terminal, an agent opens
  in its place's stream's session: `session` in `[streams.<stream>]`,
  else the stream's name, and flume's own for the default stream naming
  none. It is read at each start. A launch names one by `--agent
  [<stage>.]session=<name>`, and a pane the settings name comes first.
  Status gives each agent's session with where it came from.
- **The terminal seam's open takes where the agent runs**: beside a
  pane, in a session by its name, or in the terminal's own.
- **herdr opens a stream's agents in herdr's session of that name**,
  its server started headless where none runs. Its workspace per base
  and tab per place are kept per session, and a place's tidy closes its
  tab in each session.
- **Two layers of the host's configuration (C.7).** `config.local.toml`
  beside `config.toml` is read over it key by key at each reading. It
  sets only C.7's keys: a stream's `agents`, `priority`, `points`,
  `pace` and `session`, a label's `reserve` and `keep_free`, the
  `stream` and `label` a kind or repository is given, and a kind's
  agents table. A local file that sets anything else, or cannot be
  read, is not taken at all, and status names why. `[host] pin` pins
  keys, and a pinned local key is passed over. flume writes neither
  file.
- **Status gives every value of the host's configuration with its
  layer**, an account's variables by name and `<set>`.
- **Labels (180 to 182).** A place's label is the host's `label` for
  its repository's kind, else for its kind, else its kind's own, else
  none, read at each start. `[streams.<stream>.labels.<label>]` gives a
  label's `reserve` and `keep_free`. Within a stream a stage of a label
  below its reserve starts first, whatever the priorities, and a stage
  starts only while its stream's room passes what other labels keep
  free and do not use. No agent is ended for a reserve. A stream whose
  reserves pass its bound, or whose label keeps more free than its
  reserve or names anything but those two keys, starts no stage, each
  refused naming why.
- **Status gives each place's label, each stream's session and labels,
  and a stage waiting on room another label keeps free.**
- **The conformance suite** holds S82 to S85, each by a test, and its
  clause list is read at 2c95a70, with 183 named unbuilt.
- **The site** says how a stream's session, the two layers and labels
  work, in the streams guide, the configuration and state references,
  the herdr guide, the commands reference and the roadmap.

## Capabilities

### New Capabilities

- `stream-sessions`: where a stream's agents open in the terminal, a
  launch's session, and status's session of each agent and stream.
- `host-configuration`: the managed and local layers, the keys the
  local layer may set, pins, and status's values with their layers.
- `stream-labels`: a place's label, a label's reserve and the slots it
  keeps free in the queue's order and room, what a stream's labels
  refuse, and status's labels.

### Modified Capabilities

- `host-queue`: the room and order of a stage that may start count the
  stream's labels.
- `terminal-profiles`: herdr opens an agent in the session the core
  names, and tidies a place's tabs in every session.
- `seam-contracts`: the terminal seam's open takes where the agent
  runs, and the host's facts give the sessions, labels and layers.
- `seam-fakes`: the fakes script sessions, labels and layers.
- `command-line`: `--agent [<stage>.]session=<name>`.
- `clause-coverage`: the suite's list at 2c95a70.

## Impact

- agentplot/blueprints: the statement's amendments on main at 7367604,
  4b4c5e0 and 2c95a70, which the build reads (core.19).
- agentplot/weir: C.2 puts a kind's `label` in its table, which weir's
  statement and crate do not hold for a simple kind, and weir 94 gives
  a statechart kind's `label` another meaning. Until weir reads one, a
  kind's own label reads as none and the host gives every label
  (core.4).
- Clause 183, an MCP server naming its labels, is not built: how a
  host's servers reach an agent is section 10's question.
- `crates/flume-seams`: `OpenIn`, `Terminal::open`'s `at`,
  `Settings::session`, `Stream::session` and `labels`, `Share`,
  `Kind::label`, `SessionHeld`, `LabelHeld`, `StartHeld::session` and
  `label`, `Host::label_for` and `configuration` with their types, only
  added, held state written before reading with none.
- `crates/flume-core`: `settings.rs`, `start.rs`, `room.rs`,
  `clues.rs`, `status.rs` and `answer.rs`.
- `crates/flume`: `host/config.rs` and `host/mod.rs`, the herdr and
  printed profiles, `commands/launch.rs` and `print.rs`, and their tests.
- `crates/flume-fakes`, `crates/flume-conformance`: the fakes' scripts,
  the seam and engine scenarios citing S82 to S85, and the clause list.
- `site/`: the agent-settings guide, the places-and-herdr guide, the
  configuration, state and commands references, the rules mirror and
  the roadmap.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency. The herdr plugin is unchanged: each session's server runs
  its watch, and `bases` lists each base once per workspace.
