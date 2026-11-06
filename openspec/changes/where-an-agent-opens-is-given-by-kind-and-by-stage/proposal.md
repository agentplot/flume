# Where an agent opens is given by kind and by stage

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
partition opens in the partition's own herdr session: a change's
construct stage, which writes the change's design, a research place
and a plan held for review, beside every change's code and verify
agents, every fix and every chore.

A place runs in exactly one stream (143), and a change's construct,
code and verify stages are one place. A stream for research would part
research from the rest, but never a change's design from its code, and
it would split the partition's one stream, its bound, its pool and its
planning label's share, which the fleet keeps one per partition. So
the host needs a word below the stream: for a kind, and for one stage
of a kind, the session its agents open in, while the place keeps its
stream.

Record 0048, proposed, decides it. The statement's amendment is on
agentplot/blueprints' branch `design/flume-where-an-agent-opens`,
committed for the user's word (32, 107, 143, 186, 190, 199, C.5, C.7,
S106, S107; core.19). The code waits for the user's read.

## What Changes

- **A stream names a session for a kind and for a kind's stage (143,
  199, C.7).** `session` in `[streams.<stream>.kinds.<kind>]`, and in
  its `.stages.<stage>` table for one stage of the kind. Under herdr
  that is a herdr session of its own, laid out as every session is: a
  workspace per repository and base, a tab per place.
- **The order where an agent opens (107).** A pane the settings name;
  a session the launch names; the session the stream names for the
  place's kind's stage, then for its kind; then the stream's own, as
  record 0031 has it. Read at each start; an agent stays where it
  opened.
- **The place keeps its stream (70, 143, 180).** Its bound, priority,
  pool, points, pace and label's share are unchanged: only where its
  agents open differs.
- **What refuses (4, 182, 199).** A start that reads its stream's table
  for its kind or its stage holding any key but `session` and a kind's
  `stages` is refused naming the key.
- **The local layer (C.7).** It may set these `session` keys, each a
  key of its own, which the managed layer may pin.
- **Status (56).** Each agent's session with where it came from, two new
  values among them, `stream-stage` and `stream-kind`. Each stream's
  own session as before, and each session it names, with the kinds and
  stages it names it for.
- **The view (186).** `flume open [<stream>] --session <name>` shows a
  session the stream names for a kind or a stage, in the stream's view
  and terminal app, under the session's name as its window's title. A
  session the stream names for nothing is refused naming the stream's
  sessions, and nothing is started.
- **The terminal profile (190, C.5).** Nothing new is asked of herdr: a
  place has a tab in each session its agents open in, every call on an
  agent goes to its recorded pane, and the tidy closes each tab, as
  record 0031 built for a launch's session per stage. A test under the
  pinned herdr holds it for a change whose construct stage opens
  apart.
- **The conformance suite** holds the new scenarios citing 199, S106 and
  S107, and reads its clause list at the revision of blueprints main
  that lands the amendment.
- **The site** says how a host keeps a stream's research and design
  apart, to any host's operator (core.35).
- **Outside this change.** Which kinds and stages a host opens apart,
  and the sessions' names: the host's, the fleet's among them, whose
  configuration this change's design sketches for its own change to
  write.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `stream-sessions`: a stream's session for a kind and for a kind's
  stage, the order where an agent opens, the start that reads another
  key refused, the place keeping its stream, and status's two new
  origins and each stream's sessions.
- `host-configuration`: the local layer sets a stream's `session` for a
  kind and for a kind's stage.
- `stream-views`: `flume open` shows a session a stream names for a
  kind or a stage, or refuses one it names for nothing.
- `command-line`: `open`'s `--session`.
- `terminal-profiles`: under the pinned herdr, a change's construct
  stage in one of its stream's sessions and its code and verify stages
  in another, each tab closed at its tidy.
- `seam-contracts`: the host's facts give each stream's sessions for its
  kinds and stages, and a session's origin gains `stream-kind` and
  `stream-stage`, written before reading as it did.
- `clause-coverage`: the list at the revision holding 199, S106 and
  S107.

## Impact

- agentplot/blueprints: clauses 32, 107, 143, 186 and 190, C.5's herdr
  row and C.7 amended, and 199, S106 and S107 added, on the branch
  `design/flume-where-an-agent-opens` at 72a6f12, cut from main at
  cd8fd73, committed and not pushed (design 12).
- `docs/adr`: record 0048, proposed; rule core.44 amended with it, the
  site's rules page mirroring it.
- `crates/flume-seams`: `Stream::kinds` and `KindSessions`, `SessionFrom`'s
  `StreamKind` and `StreamStage`, and the stream status's sessions, each
  only added.
- `crates/flume-core`: `StreamRoom` keeps the stream's kinds' sessions,
  `Queue::session_of` takes the place's kind and stage, a start refused
  for its stream's table's other key, status's sessions, and
  `open_stream`'s session.
- `crates/flume`: `host/config.rs` reads `[streams.<stream>.kinds]` and
  adds its keys to `LOCAL_KEYS`, `open`'s `--session`, the person's form
  of status, and a test of the herdr profile under the pinned herdr.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake host's
  streams carry their kinds' sessions; new scenarios in
  `engine/sessions.rs` and the open's; the clause list.
- `site/`: the herdr and agent-settings guides, and the configuration,
  commands and state references.
- Built after `flume-opens-a-streams-multiplexer` lands, on its `open`
  and its stream's terminal (design 11).
- No new crate, plugin or site part, so no new moon project; no new
  dependency.
