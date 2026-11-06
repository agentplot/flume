# Where an agent opens is given by its kind, in a session of its own

## Why

The user, 2026-11-05: "I'm realizing that mixing construction sessions
from Flume with research and design sessions makes it hard to find and
keep track of the research and design sessions. We probably want to put
those into different Herdr sessions. I don't know if that is possible,
whether that has to be explicit at the stream level (so we need
different streams), or whether we can do it below the stream level".

Each stream's agents open in one session of its terminal, the one the
host names for the stream, else the stream's name (107; record 0031).
The fleet names each stream by its partition, so every agent of a
partition opens in the partition's own herdr session: research and
design places beside every change, fix and chore.

A place runs in exactly one stream (143). A stream for research would
split the partition's one stream, its bound, its pool and its planning
label's share, which the fleet keeps one per partition. So the host
needs a word below the stream.

The user ruled on 2026-11-06, in the Plannotator Inbox thread
msg_01M6Q2N04BS0RPG16TA76W67W6: the research and design agents open in
"A session of its own beside the stream's, e.g. `madswan-design`",
named in "The stream's own table, `[streams.<stream>.kinds.<kind>]`",
for "just any elaboration types", a change's construct stage "not this
one"; and of `flume open madswan --session madswan-design`: "Why do you
need the flag? Why wouldn't you just open madswan design?"

Record 0048, proposed, decides it. The statement's amendment is on
agentplot/blueprints' branch `design/flume-where-an-agent-opens`,
committed for the user's word (32, 107, 143, 186, 190, 199, 200, C.5,
C.7, S106, S107; core.19). The code waits for the user's answers to the
design's three questions.

## What Changes

- **A stream names a session for a kind (143, 199, C.7).** `session` in
  `[streams.<stream>.kinds.<kind>]`, every stage of the kind opening
  there. Under herdr that is a herdr session of its own, laid out as
  every session is: a workspace per repository and base, a tab per
  place.
- **The order where an agent opens (107).** A pane the settings name;
  a session the launch names; the session the stream names for the
  place's kind; then the stream's own, as record 0031 has it. Read at
  each start; an agent stays where it opened.
- **The place keeps its stream (70, 143, 180).** Its bound, priority,
  pool, points, pace and label's share are unchanged: only where its
  agents open differs.
- **What refuses (4, 182, 199).** A start that reads its stream's table
  for its kind holding any key but `session`, or naming a session that
  is another stream's, is refused naming why.
- **The local layer (C.7).** It may set a kind's `session`, which the
  managed layer may pin.
- **Status (56).** Each agent's session with where it came from,
  `stream-kind` new among them; each stream's own session as before,
  and each session it names, with its kinds.
- **The view (186).** `flume open <name>` takes a stream's name, as
  before, or the name of a session a stream names for a kind, and shows
  it in that stream's view and terminal app under the session's name. A
  name that is neither is refused naming the streams and their
  sessions, and nothing is started.
- **The terminal profile (190, C.5).** Nothing new is asked of herdr:
  every call on an agent goes to its recorded pane, and the tidy closes
  each tab in every session, as record 0031 built. A test through the
  binary under the pinned herdr holds a research place in its stream's
  design session and `flume open` of that session by its name.
- **The list (200).** Each stream's line names the sessions it names
  for its kinds, built here where `flume list` is on main first.
- **The conformance suite** holds the new scenarios citing 199, S106 and
  S107, and reads its clause list at the revision of blueprints main
  that lands the amendment.
- **The site** says how a host keeps a stream's research and design
  apart, to any host's operator (core.35).
- **Outside this change.** Which kinds a host opens apart, and the
  sessions' names: the host's, the fleet's among them, whose
  configuration this change's design sketches for its own change to
  write.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `stream-sessions`: a stream's session for a kind, the order where an
  agent opens, the start that reads another key or another stream's
  session refused, the place keeping its stream, and status's new
  origin and each stream's sessions.
- `host-configuration`: the local layer sets a stream's `session` for a
  kind.
- `stream-views`: `flume open` shows a session a stream names for a kind
  by its name, or refuses a name that is no stream and no such session.
- `command-line`: `open`'s one name, a stream's or a session's.
- `seam-contracts`: the host's facts give each stream's sessions for its
  kinds, and a session's origin gains `stream-kind`, written before
  reading as it did.
- `clause-coverage`: the list at the revision holding 199, S106 and
  S107.

## Impact

- agentplot/blueprints: clauses 32, 107, 143, 186, 190 and 200, C.5's
  herdr row and C.7 amended, and 199, S106 and S107 added, on the branch
  `design/flume-where-an-agent-opens` at dc4abac, cut from main at
  7bd1891, committed and not pushed, which also gives clause 200's list
  each stream's sessions for its kinds (design 13).
- `docs/adr`: record 0048, proposed; rules core.44 and core.48 amended
  with it, the site's rules page mirroring them.
- `crates/flume-seams`: `Stream::kinds` and `KindSession`,
  `SessionFrom::StreamKind`, and the stream status's sessions, each
  only added.
- `crates/flume-core`: `StreamRoom` keeps the stream's kinds' sessions,
  `Queue::session_of` takes the place's kind, a start refused for its
  stream's table's other key or another stream's session, status's
  sessions, and `open_stream`'s name.
- `crates/flume`: `host/config.rs` reads `[streams.<stream>.kinds]` and
  adds its key to `LOCAL_KEYS`, `open`'s name, the person's form of
  status, and a test through the binary under the pinned herdr.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake host's
  streams carry their kinds' sessions; new scenarios in
  `engine/sessions.rs` and `engine/open.rs`; the clause list.
- `site/`: the herdr and agent-settings guides, and the configuration,
  commands and state references.
- Built on `flume-opens-a-streams-multiplexer`, landed on main as #109
  and merged here (design 11).
- No new crate, plugin or site part, so no new moon project; no new
  dependency.
