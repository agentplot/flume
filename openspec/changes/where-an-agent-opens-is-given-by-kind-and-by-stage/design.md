# Design

## Context

### What the user asked

The user, 2026-11-05: "I'm realizing that mixing construction sessions
from Flume with research and design sessions makes it hard to find and
keep track of the research and design sessions. We probably want to put
those into different Herdr sessions. I don't know if that is possible,
whether that has to be explicit at the stream level (so we need
different streams), or whether we can do it below the stream level".

It is possible, below the stream. This design says how a host's
configuration names where an agent opens by kind and by stage, what a
person is shown, what the terminal profile does, what status says, and
the statement's amendment.

### What stands now

On agentplot/blueprints main at cd8fd73:

- **107.** Where in the terminal a stage's agent runs is one of its
  settings. A launch may put it beside a pane it names; otherwise it
  runs in its stream's own part of the terminal, the one the host names
  for the stream, else the one named by the stream's name, and flume's
  own for the host's default stream.
- **32.** Program, model, effort, mode, instructions and own settings
  resolve from eight layers. Where an agent runs is not among them: it
  "is read from the launch's settings and the stage's declaration, else
  from its place's stream".
- **143, 186, 190.** A stream's words: its bound, priority, pool, points,
  pace, terminal (`multiplexer`), session, view, terminal app and title.
  `flume open` shows a stream's own session (186); every agent stays in
  the terminal it opened in (190).
- **A place runs in exactly one stream (143).** A change's construct,
  code and verify stages are one place.

On flume's main, record 0031 built 107 and 143's session:
`Queue::session_of` (`crates/flume-core/src/room.rs`) takes a pane the
settings name, then the launch's `session`, then
`StreamRoom::session`, the stream's `session` else its name, else the
terminal's own for `default`. Every layer but the launch's has
`session` taken away (`sessionless` in `settings.rs`). The start records
`SessionHeld { name, from }` and opens through
`OpenIn::Session { name }`. The herdr profile already holds a place
with tabs in several sessions: it records each workspace and tab by its
server, closes the place's tab in every session it made one in, and
shows the base's count on each of its workspaces (records 0018, 0031),
since a launch may name a session for one stage alone.

### How herdr names sessions and workspaces

Under herdr 0.9.5, which the tree pins (C.5):

- A **session** is a herdr server of its own with its own socket,
  `herdr --session <name> server`, listed by `herdr session list`. A
  person attaches one client to one session, `herdr session attach
  <name>`, and sees that session alone. flume's `open` gives that
  command to a window under a title (186 to 188).
- A **workspace** is a group within a session, listed in its sidebar.
  flume makes one per repository and base, labelled `<repository>
  <base>`, and gives it the base's count as a clue (139).
- A **tab** within a workspace holds one place's panes, `<kind>--<name>`.
- A **pane** is one agent.

So a person who opens a session sees every workspace in it, and in each
the tabs of the places whose agents opened there. The part a person
opens to find "the research and design sessions", meaning the agents
doing research and design, is a session of their own: one window shows
them all, a workspace per repository and base, a tab per place, and no
construction agent among them.

### Across the family and the changes in flight

- **Program settings, effort and `keep_open`.**
  `an-agents-program-settings-come-from-flumes-layers` (in code) and
  `an-agents-effort-is-a-setting-of-flumes-layers` fold settings through
  clause 32's eight layers; `an-answer-reaches-the-open-session-within-an-hour`
  folds `keep_open` from the launch's and the host's four layers. Where
  an agent opens is not one of those settings, and stays out of their
  fold (decision 2), so no two changes resolve one setting two ways.
- **`flume-opens-a-streams-multiplexer`** (verified, landing) builds a
  stream's multiplexer, `flume open`, the terminal's attach by view and
  the window seam (186 to 190, record 0037). This change adds a
  session to `open` and is built after it (decision 11).
- **`every-question-a-place-asks-reaches-someone-who-answers-it`**
  (designing): crew's roles run as kinds in their partition's stream,
  each run a place of its role's kind. A host names a session for a
  role's kind as for any kind.
- **The fleet's streams**, afterthought/swancloud
  `each-stream-opens-in-a-session-the-host-chooses-and-gets-its-labels`
  (record 0053 there): one stream per partition, named by the
  partition, opening in the partition's own herdr session, with
  `planning` a label of two agents. Decision 13 says what the fleet
  would write beside that.
- **sluice and Boomdrive** name no terminal session: sluice's runs and
  the runner's work keep no agent in a pane. No other tool of the family
  decides where an agent opens, so there is nothing to reconcile.

### What rules what

Record 0048, proposed, decides it, and the statement's amendment is on
blueprints' branch `design/flume-where-an-agent-opens` for the user's
word (decision 12). The rules this design relies on:

- core.1 and core.2: the core names no terminal; it hands the terminal
  `OpenIn::Session { name }` and the terminal profile makes it a herdr
  session.
- core.3 and core.45: the host's configuration, its local layer over
  its managed one, is the one place a session is read from.
- core.10: an agent stays where it opened, and one stage's agent is
  never handed to another session.
- core.14: the JSON form only gains fields and values.
- core.17: a start, an open and a tidy are safe to repeat.
- core.18: the tidy closes only what flume made, in every session.
- core.26 and core.46: room and a label's share are the stream's; a
  session moves no agent between them.
- core.29: every herdr call bounded, a name herdr refuses refused in
  herdr's words.
- core.30: clause 32's layers, which this design leaves as they are.
- core.35: the site speaks to any host's operator.
- core.44, as record 0048 amends it: where an agent opens.
- core.48 (on the multiplexer change): flume keeps nothing of a window.

The one rules page is `docs/architecture/core.md`; this change touches
no area another page would own.

## Goals / Non-Goals

**Goals:**

- A host keeps a stream's research and design agents apart from its
  construction agents in the terminal, by kind and by stage, while
  every place keeps its one stream, its bound, its pool, its points,
  its pace and its label's share (70, 143, 180).
- A person opens the research and design agents together in one view
  (186).
- Every agent is sent to, read and closed where it opened, and a place
  is tidied in every session it opened in (190, 137).
- Status says where each agent opened and why, and which sessions each
  stream has (56).
- Where an agent opens has one resolution, apart from clause 32's
  layered settings, so no sibling folds it a second way.

**Non-Goals:**

- A terminal or a view per kind or per stage. A stream's terminal,
  view, terminal app and title stay the stream's (186, 190).
- A session for a repository, or a repository and kind, within a
  stream. Nothing asks for it, and a stream's sessions stay few.
- A session for every kind's stage of one name (decision 2).
- A workspace other than the repository and base's (decision 1).
- Moving an agent that runs. An agent stays where it opened (190).
- Which kinds and stages a host opens apart, and what it names their
  sessions. Those are the host's; decision 13 sketches the fleet's.
- A statechart kind's sessions (175), which flume runs none of yet
  (core.41); its change names a stage's session by the stage as this
  does.

## Decisions

### 1. The part a person opens: a session of the stream's terminal

A stream may name, for a kind and for one stage of a kind, a session of
its own terminal for those agents to open in (199). Under herdr it is a
herdr session: a server of its own, started headless where none runs,
laid out as every session flume opens in, a workspace per repository and
base, a tab per place (C.5; record 0018). A person opens that session to
see the research and design agents together.

Alternatives considered:

- **A workspace within the stream's session** holding the research and
  design places of every repository and base. One window would show
  everything, the design work a click away in its sidebar. But a
  workspace that holds several bases' places breaks record 0018's
  layout, which the base's count clue (139) and the tidy of a base's
  workspace (138) rest on, and it needs a part within a part, which the
  terminal seam does not have (`OpenIn` names a session or the
  terminal's own). The user asked for sessions. Question 1 offers it.
- **A tab group or a pane title** within the place's tab. herdr 0.9.5
  has no grouping across workspaces, so the agents would stay scattered.

### 2. Where the host says it: the stream's own word

The stream's table names it, by kind and by stage:

```toml
[streams.work.kinds.research]
session = "work-design"

[streams.work.kinds.change.stages.construct]
session = "work-design"
```

`[streams.<stream>.kinds.<kind>]` holds `session` and `stages`;
`[streams.<stream>.kinds.<kind>.stages.<stage>]` holds `session`. The
kind is named as `[kinds.<kind>]` names it, by its name, whatever
source and version it resolves to (19, 143, 180), and a stage by its
name in that kind. It is not one of clause 32's settings: no agents
table, launch's `--agent`, repository's file or kind gives it, and the
launch's `session` keeps its place before it (decision 3).

Why the stream's own word:

- **A session is the stream's.** A session is a part of the stream's
  terminal (190), and the stream already names its own (143). A kind's
  research session in one stream is not its research session in
  another: the fleet's partitions each keep their own.
- **32's host layers cannot say it.** Its layers for a repository and
  kind and for a kind name no stream, so a session given there opens
  every stream's research in one session, mixing two partitions, unless
  the host writes it for every repository. Its layers for a stream and
  for every agent name no stage. And `[agents.streams.<stream>] session`
  would spell `[streams.<stream>] session` a second way.
- **One resolution.** The launch's settings, then the stream's table.
  The siblings' fold of 32's settings never sees a session.

Alternatives considered:

- **A stream per sort of work.** Splits the partition's bound, pool,
  points, pace and planning share, which the user kept one per
  partition, and still leaves a change's design with its code, since a
  place runs in one stream (143).
- **`session` one of 32's settings**, given by the launch and the
  host's four layers as `keep_open` is. Rejected for the reasons above.
- **A part word through 32's layers, `opens = "design"`, each stream
  mapping it to a session**, flume naming `<stream>-<part>` where a
  stream maps none. Two words and a mapping for one place, and a
  session name flume builds, which core.44 rules out. The fleet would
  still declare every session by name on every host.
- **A table of every kind's stage, `[streams.<stream>.stages.<stage>]`.**
  A stage's name means nothing across kinds: a repository's kind may
  hold a `plan` stage that builds. Clause 32 names a stage only within
  its kind. A host names `change`'s `construct` once, and change@1 and
  change@2 share the name.
- **A session per label.** A label is the kind's share of its stream
  and nothing else (180; core.46, the user's ruling), and a change's
  construct and code stages share its label.

Question 2 asks the user to choose between the stream's own word and
32's layers.

### 3. The order where an agent opens

At each start (107, 199):

1. a pane the settings name, the launch's `beside` or the stage's
   declaration's naming a launch's pane: a split beside it;
2. the session the launch's settings name, the stage's own before every
   stage's;
3. the session the place's stream names for the place's kind's stage;
4. the session the place's stream names for the place's kind;
5. the stream's own: the one `[streams.<stream>] session` names, else
   the stream's name, and, for `default` naming none, the terminal's
   own.

`Queue::session_of` takes the place's kind's name and the stage's name
beside the stream and the settings, and answers `SessionHeld` with
`from` one of `launch`, `stream-stage`, `stream-kind`, `stream`,
`stream-name` or `own`, or none beside a pane. A repeating stage's
repetitions each open where their stage's word says. A start that
carries a spent agent's conversation (179) opens where its own start
resolves, as every start does.

### 4. Read at each start; what is ignored, what refuses

- The stream's tables are read with the rest of the host's
  configuration at each start, the local layer over the managed one
  (32, C.7). A change to them takes the next start; an agent already
  started stays where it opened, and every call on it goes to the pane
  its start recorded (190).
- A table for a kind or a stage the place does not run is ignored, as
  the host's settings for a kind or a stage the place does not run are
  (C.7).
- A table holding any key but `session` and, for a kind, `stages` is
  kept with the key named, as a label's table is (`Share::unknown`), and
  a start that reads it is refused naming the key and the table, until
  the configuration is mended and the stage retried (4, 27, 182, 199).
  Only the starts of that kind, or that stage, on that stream are
  refused. A `session` that is not a string, or a `stages` that is not a
  table of tables, refuses the configuration naming the key, as a
  stream's `session` that is not a string does today.
- A session's name is herdr's to judge: one the pinned herdr refuses
  refuses the open in herdr's words, nothing of flume's left open
  (core.29, core.44).

### 5. The place keeps its stream

Nothing of room changes (70, 143, 180 to 182). A place's agents in its
stream's research session count toward its stream's bound and the
host's, take their accounts from its stream's pool, charge its points
and pace, and take their label's share, as every agent of the stream
does. Where an agent opens is read after the queue admits its start, as
today.

### 6. The local layer

`LOCAL_KEYS` gains `streams.*.kinds.*.session` and
`streams.*.kinds.*.stages.*.session`, each a key of its own, so an
operator moves a kind's session from hour to hour as they move a
stream's (C.7; record 0032). The managed layer may pin either,
`"streams.work.kinds.research.session"`, or pin a table holding them,
`"streams.work.kinds"`. A local layer that sets another key of these
tables is not taken, as for any key outside the list.

### 7. Status

- **Each agent** gives its session as today, `SessionHeld`, with `from`
  gaining `stream-stage` and `stream-kind`. The person's form says
  `session work-design (stream work, kind research)` or `(stream work,
  stage change.construct)`.
- **Each stream** gives its own session as today, and a new
  `sessions`: each session it names for its kinds and stages, with the
  kinds and the `<kind>.<stage>`s it names it for, and the layer of each,
  `managed` or `local` (56, C.7). A table's other key is given with the
  refusal it causes.
- The JSON form only gains a field and two values (core.14). A start
  written before reads with its `from` as written.

### 8. The view

`flume open [<stream>] [--session <name>]` (186):

- With no `--session`, the stream's own session, as the multiplexer
  change builds it.
- With `--session`, a session the stream names for a kind or a stage,
  shown in the stream's view through its terminal app, attached through
  its multiplexer, the session started headless where it does not run
  (187). Its window's title is the session's name, which the host chose,
  so a window manager files it apart from the stream's own window. The
  answer gives the session from `stream-kind` or `stream-stage` with
  the kinds and stages it is named for.
- `--session` naming the stream's own session is that session.
- `--session` naming anything else is refused naming the stream and its
  sessions, before the terminal is asked anything, so a mistyped name
  starts no stray session.

A person finds the research and design agents of `work` by `flume open
work --session work-design`: one window, every workspace in it a
repository and base, every tab a research place or a change whose
construct stage runs.

Alternative considered: `--kind research` resolving the session the
kind opens in. It saves remembering a name, but it puts a second
resolution of the same table on the command line, and a stage-only
session would need `--stage` too. Status lists each stream's sessions,
so a person reads the name there.

### 9. The terminal profile

Nothing new is asked of herdr, and the core names no terminal (core.1):

- **open** gets `OpenIn::Session { name }` with the session decision 3
  resolves. The herdr profile finds the session's server or starts it
  headless, then opens in that session's workspace of the place's
  repository and base, in the place's tab there, each made when first
  needed and recorded by id and server (C.5).
- **send, state, screen and close** go to the pane the start recorded by
  its id and server, found again by the agent's herdr name where the id
  no longer answers (98, 190).
- **tidy** closes the place's tab in every session it made one in, and
  each base's workspace once it holds no tab of a live place and nothing
  flume did not make (137, 138).
- **show** gives the base's count on each of its workspaces in every
  session (139).
- **watch**: each session's server runs flume's herdr plugin as each
  stream's does, and settles are safe to repeat (122; core.17).
- **attach** takes the session's name (187).
- **The printed command** names no session: open prints as before.

What is new is that a place regularly has tabs in two sessions of its
stream, a change's construct tab in one and its code and verify tab in
the other. A test of the herdr profile under the pinned herdr holds
it: each agent sent to, read and closed on its own server, and the tab
closed in both (task 6.2).

### 10. The seams and the core

- `flume-seams`: `Stream` gains `kinds: BTreeMap<String, KindSessions>`,
  `KindSessions { session, stages: BTreeMap<String, StageSession>,
  unknown }` and `StageSession { session, unknown }`, each read with
  none where written before and written only where set. `SessionFrom`
  gains `StreamKind` and `StreamStage`. The stream status gains
  `sessions: Vec<NamedSession>`, `NamedSession { name, kinds, stages }`.
- `flume-core`: `StreamRoom` keeps the stream's `kinds`.
  `StreamRoom::session` becomes `session_for(stream, kind, stage)`,
  answering the session with its origin or the refusal naming the key.
  `Queue::session_of` takes the kind and the stage; `Engine::claim`
  refuses the stage where it answers a refusal, as a stream's label
  fault does. `status.rs` gives each stream's sessions;
  `Engine::open_stream` takes a session.
- `flume`: `host/config.rs`'s `streams_declared` reads `kinds` beside
  `labels`, its keys in `LOCAL_KEYS`; `commands/open` takes
  `--session`; `print.rs` gives the new origins and each stream's
  sessions.
- `flume-fakes`: the fake host's `Stream` carries `kinds` as written by
  a test.

### 11. Agreement with the changes in flight

- **Order.** This change's code starts once the user has read this
  design, blueprints main holds decision 12's amendment, and
  `flume-opens-a-streams-multiplexer` has landed on flume's main, whose
  `open`, `Stream` and `StreamStatus` it extends. Group 1's first task
  merges main then.
- **Program settings, effort, `keep_open`.** None of them reads or
  writes a session: `session` stays out of `Settings::under`'s fold
  but the launch's, and out of every agents table. Should any of them
  land first, nothing here changes.
- **Questions.** A role's run is a place of its role's kind in its
  partition's stream; a host names a session for that kind as for any
  other. A role's run is reached by the pane its live start holds,
  wherever it opened (132).

### 12. The statement's amendment

On agentplot/blueprints, branch `design/flume-where-an-agent-opens`, cut
from main at cd8fd73, one commit, 72a6f12, not pushed, for the user's
word (core.19). Clause 199 and scenarios S106 and S107 are new: on main 186
to 192, 195 to 197, S90 to S92, S95 to S97 and S100 to S103 are taken,
and the designs in flight hold 193, 194, 198, S93 to S99, S104 and S105.
Each item gives the words a clause gains or its new text.

- **32**: "Where in the terminal an agent runs (107) is read from the
  launch's settings and the stage's declaration, else from its place's
  stream, by kind and by stage (143, 199)."
- **107**: "Where the settings name no place in the terminal, the agent
  runs in its place's stream's own: the session the stream names for
  the place's kind's stage, else for its kind (199), else the one the
  host's configuration names for the stream, else one named by the
  stream's name, so every agent of a stream is found in the stream's
  sessions and a person opens them by the stream's name. A place in the
  host's default stream runs in flume's own where its stream names none
  for it (143)."
- **143** gains, after "its own name where it states none (107)": "and,
  for a kind or one stage of a kind, the session of that terminal its
  agents of it run in instead (199)".
- **186**: flume shows "the stream's own part of its terminal, or the
  session the stream names for a kind or a stage that the person names
  (107, 190, 199)". Its title is the stream's "and, for a session the
  stream names for a kind or a stage, that session's name". It refuses
  "a session the stream names for no kind or stage … naming the
  stream's sessions, each before anything is started".
- **190**: "the one the host's configuration names for the stream, else
  the host's, whose parts the sessions the stream names for its kinds
  and stages are (199)."
- **199**, new: "A stream may name, for a kind and for one stage of a
  kind, the session of its terminal its agents run in. So a host keeps a
  stream's research and design apart from its other work in the
  terminal. Every place of the stream keeps the stream's bound,
  priority, pool, points, pace and labels alike (70, 143, 180 to 182).
  Where the settings name no place in the terminal, a stage's agent
  runs in the session its stream names for its kind's stage, else for
  its kind, else in the stream's own (107). A kind is named there by its
  name, whatever source and version it resolves to, as the host's
  configuration names a kind for a stream or a label (19, 143, 180), and
  a stage by its name in that kind. The host names nothing else there:
  no repository, label, stage of every kind or terminal. The session is
  read at each start from the configuration as it stands (32). A change
  to it takes the next start, and an agent already started stays in the
  session it opened in (190). A session named for a kind or a stage the
  place does not run is ignored. A start that reads a stream's table for
  its kind or its stage holding any key but the session and a kind's
  stages is refused naming the key, until the configuration is mended
  (4). A person is shown such a session as the stream's own is (186).
  Status gives each session a stream names, with the kinds and stages
  it names it for (56). It gives each agent's session with where it came
  from: the launch, the stream's word for its kind's stage or for its
  kind, the stream's own session, the stream's name, or flume's own."
- **C.5's herdr row**, open: "a pane in the herdr session the launch's
  settings name, else the one its place's stream names for the place's
  kind's stage or its kind (199), else the one its place's stream
  names, by default the stream's name, and flume's own for a place in
  the host's default stream naming none (107, 143): in each session,
  one workspace per repository and base … made when the first place on
  that base opens there, and in it one tab per place … whose panes are
  the place's agents that open in that session". The prose after it: "It
  closes a place's tab in each session it made one in when the place
  goes".
- **C.7**: the local layer may set "a stream's `agents`, `priority`,
  `points`, `pace` and `session` and `multiplexer`, and the `session`
  it names for a kind or a kind's stage (107, 143, 166, 167, 190,
  199)". A stream may state, after `session`: "in
  `[streams.<stream>.kinds.<kind>]`, `session`, the session of that
  terminal its agents of that kind run in, and, in its
  `.stages.<stage>` table, the one for that stage of the kind, the kind
  named as `[kinds.<kind>]` names it, each table holding nothing but
  these and the kind's `stages` (199)". The example gains `work`'s two
  tables of decision 2, and its reading "a research place's agents and
  a change's construct agent opening in the session `work-design` and
  work's other agents in `work`, every one of them counted in work's
  five".
- **S106** and **S107**, new: a stream's research and a change's
  construct stage in `work-design`, the rest in `work`, the room and
  the shares unchanged, a launch's session and a pane before it, status,
  and the tidy in both sessions; then a person's open of `work-design`,
  a session the stream names for nothing refused, the local layer moving
  research's next start, and a table's other key refusing research's
  next start on `work` alone.

### 13. What the fleet would write

Beside its streams design, which names each stream by its partition and
opens it in the partition's own session, the fleet's
`lib/flume-streams.nix` would render, for each partition stream with
research and design work, `madswan` shown:

```toml
[streams.madswan.kinds.research]
session = "madswan-design"

[streams.madswan.kinds.plan.stages.plan]
session = "madswan-design"

[streams.madswan.kinds.change.stages.construct]
session = "madswan-design"
```

and the same for `swancloud` and `agentplot`. A crew role whose run is
design work, the design agent and the planner, would add its kind's
table once crew's roles run as kinds, named as the fleet's
`[kinds.<kind>] label` tables name them. The fleet would also declare
each `<partition>-design` session on every Mac that holds a Flume file,
as `lib/herdr-hosts.nix` checks each stream's session today, and give a
person's launcher for it `flume open madswan --session madswan-design`.

Which kinds and stages, and the sessions' names, are the fleet's change
to write and the user's to rule: the user turned down `<name>-work`
sessions for a stream's own ("God I don't want ones with -work"), so a
design session's name is theirs to choose. This change adds nothing to
the fleet; its orchestrator is told.

### 14. The site

Written to any host's operator, naming no host, team or word of ours
(core.35):

- `guides/places-and-herdr.mdx`: a stream's sessions for its kinds and
  stages, how a person opens one, and a place with tabs in two sessions.
- `guides/agent-settings.md`: where an agent opens, in order, and that
  it is the stream's word, not a layered setting.
- `reference/configuration.md`: `[streams.<stream>.kinds.<kind>]` and
  its `stages`, the local layer's keys and pins.
- `reference/commands.md`: `flume open --session`.
- `reference/state.md`: `SessionFrom`'s two values and each stream's
  `sessions`.

## Risks / Trade-offs

- **[A second window to watch]** → The design session is its own
  server, so a person keeps two windows per stream. That is the ask:
  the research and design agents found apart. Question 1 offers the
  workspace for one window.
- **[A change's tab in two sessions]** → A change's construct tab sits
  in the design session until the place's tidy. Its agent ends with its
  stage; the tab and the base's workspace close at the tidy, as for a
  launch's session per stage today.
- **[More herdr servers]** → Each named session is a server with the
  herdr plugin's watch. Settles are safe to repeat; a few servers per
  host cost a few more settles.
- **[A typo in a session name]** → It opens a session of that name at
  the next start, which status shows by name and origin; `flume open`
  refuses a name the stream does not use, so a person's typo starts
  nothing.
- **[A typo in a kind's or a stage's name]** → The table is ignored as
  for a kind the place does not run, so agents open in the stream's
  own; status gives each stream's sessions with the kinds named, which
  shows the misspelt kind. Refusing an unknown kind is not possible: a
  kind's name resolves only at a launch (19).

## Migration Plan

Additive. A host that names no session for a kind or a stage runs as
before. A host that adds one finds the next start of that kind or stage
there; every agent running stays where it opened, and its place's tabs
in each session are closed at its tidy. Rollback is removing the
tables: the next starts open in the stream's own session.

## Open Questions

Asked of the user in the Plannotator Inbox, the design standing on the
recommended answers until they rule:

1. **Which part** a person opens for the research and design agents: a
   session of its own beside the stream's (recommended), or a workspace
   within the stream's session, one window holding both (decision 1).
2. **Where the host says it**: the stream's own word for its kinds and
   stages (recommended), or one of clause 32's settings given by the
   host's agents tables, as `keep_open` is (decision 2).
