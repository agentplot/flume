# Where an agent opens is given by kind and by stage, and a person opens a session by its name

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
and "Yes: keep `[streams.<stream>.kinds.<kind>.stages.<stage>]` for
hosts that want it"; which kinds is the host's, "a […] config file that
we are going to be setting on the kinds in crew"; a person opens
"`flume open madswan-design`", since "in my mind, I thought the thing
being opened was the session". They approved the design and named each
partition's research-and-design session `<partition>-design`.

Record 0048 and its amendment decide it, accepted on the user's word and
their card. The statement's amendment is on agentplot/blueprints main at
f676e8f and its follow-up at fa40c94 (32, 107, 143, 186, 190, 199, 200, C.5,
C.7, S106, S107; core.19).

## What Changes

- **A stream names a session for a kind and for a kind's stage (143,
  199, C.7).** `session` in `[streams.<stream>.kinds.<kind>]`, and in
  its `.stages.<stage>` table for one stage of the kind. Under herdr that
  is a herdr session of its own, laid out as every session is: a
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
  `stages`, or naming a session that is another stream's, is refused
  naming why.
- **The local layer (C.7).** It may set these `session` keys, each a
  key of its own, which the managed layer may pin.
- **Status (56).** Each agent's session with where it came from,
  `stream-stage` and `stream-kind` new among them; each stream's own
  session as before, and each session it names, with its kinds and
  stages.
- **The view (186).** What a person opens is a session, by its name:
  `flume open [<session>]` takes a stream's own session's name, which
  is the stream's name where the host names none, or a session a stream
  names for a kind or a stage, and shows it in that stream's view and
  terminal app. A name no stream's word gives is refused naming each
  stream's sessions, and nothing is started.
- **The list (200, 201).** Each stream's line names each of its
  sessions, its own and each it names for a kind or a stage, with
  whether each runs, read by one runs of the stream's terminal over
  them all. `flume list` landed on main first, so this change builds
  it. runs (201) answers one or more parts together in one reading, as
  the answer to the code stage's stop-short of 2026-11-06 ruled, which
  keeps 200's reading of each stream's terminal once.
- **The terminal profile (190, C.5).** Nothing new is asked of herdr:
  every call on an agent goes to its recorded pane, and the tidy closes
  each tab in every session, as record 0031 built. A test through the
  binary under the pinned herdr holds a research place in its stream's
  design session, a stage in a session of its own, and `flume open` of a
  session by its name.
- **The conformance suite** holds the new scenarios citing 199, S106 and
  S107, and reads its clause list at the revision of blueprints main
  that holds the amendment and its follow-up.
- **The site** says how a host keeps a stream's research and design
  apart, and how a person opens a session, to any host's operator
  (core.35).
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
  key or another stream's session refused, the place keeping its
  stream, and status's two new origins and each stream's sessions.
- `host-configuration`: the local layer sets a stream's `session` for a
  kind and for a kind's stage.
- `stream-views`: `flume open` shows a session by its name, a stream's
  own or one it names for a kind or a stage, and refuses a name no
  stream's word gives.
- `command-line`: `open`'s one name, a session's.
- `seam-contracts`: the host's facts give each stream's sessions for its
  kinds and stages, and a session's origin gains `stream-kind` and
  `stream-stage`, written before reading as it did.
- `clause-coverage`: the list at the revision holding 199, S106 and
  S107 as the follow-up leaves them.
- `the-list`: each stream's line gives each of its sessions with whether
  each runs, the terminal asked once for them all, and the JSON form
  gains each stream's `sessions`.
- `seam-contracts`, `seam-fakes`, `terminal-profiles`: the terminal's
  runs answers one or more parts in one reading, herdr's from one
  session listing, the fake logging one runs for each call.

## Impact

- agentplot/blueprints: clauses 32, 107, 143, 186, 190 and 200, C.5's
  herdr row and C.7 amended, and 199, S106 and S107 added, on main at
  f676e8f, and amended again for a kind's stage and a session's open by
  the follow-up on main at fa40c94 (design 13); and 201 and C.5's runs
  amended on main at 12866a8, on the user's card of 2026-11-06
  (design 11).
- `docs/adr`: record 0048, accepted; rules
  core.44 and core.48 amended with it, the site's rules page mirroring
  them; record 0049's amendment, accepted on the user's card, for runs
  over several parts, with core.52's details.
- `crates/flume-seams`: `Stream::kinds`, `KindSessions` and
  `StageSession`, `SessionFrom::StreamKind` and `StreamStage`, and the
  stream status's sessions, each only added.
- `crates/flume-core`: `StreamRoom` keeps the stream's kinds' sessions,
  `Queue::session_of` takes the place's kind and stage, a start refused
  for its stream's table's other key or another stream's session,
  status's sessions, `open_stream` becoming `open_session`, taking a
  session's name, and the list's line giving each stream's sessions.
- `crates/flume-seams`, `crates/flume`: the terminal seam's `runs`
  taking one or more parts, herdr's and the printed command's beside it.
- `crates/flume`: `host/config.rs` reads `[streams.<stream>.kinds]` and
  adds its keys to `LOCAL_KEYS`, `open`'s session name, the person's
  form of status, and a test through the binary under the pinned herdr.
- `crates/flume-fakes`, `crates/flume-conformance`: the fake host's
  streams carry their kinds' and stages' sessions; new scenarios in
  `engine/sessions.rs` and `engine/open.rs`; the clause list.
- `crates/flume-core`'s settle: the stage after a delivered one starts
  only while the current stage still stands delivered, finished or
  repeating, so two settles at once never start past a refused start
  (75; task 11.1). The fake store gains a hook run after a read, and
  `engine/running_through.rs` a scenario for it.
- `site/`: the herdr and agent-settings guides, and the configuration,
  commands and state references.
- Built on `flume-opens-a-streams-multiplexer`, landed on main as #109
  and merged here (design 11).
- No new crate, plugin or site part, so no new moon project; no new
  dependency.
