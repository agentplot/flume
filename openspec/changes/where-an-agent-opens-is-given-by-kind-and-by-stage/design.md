# Design

## Context

### What the user asked, and what they ruled

The user, 2026-11-05: "I'm realizing that mixing construction sessions
from Flume with research and design sessions makes it hard to find and
keep track of the research and design sessions. We probably want to put
those into different Herdr sessions. I don't know if that is possible,
whether that has to be explicit at the stream level (so we need
different streams), or whether we can do it below the stream level".

It is possible, below the stream. On 2026-11-06, in the Plannotator
Inbox thread msg_01M6Q2N04BS0RPG16TA76W67W6 (their reply
msg_01M6Q4DZM9GFJJFVDX9JE7G1N9), the user ruled where the work opens and
where the host says it:

- **Which part of herdr:** "A session of its own beside the stream's,
  e.g. `madswan-design`".
- **Where the host says it:** "The stream's own table,
  `[streams.<stream>.kinds.<kind>]`".
- **What opens there**, on the example
  `[streams.madswan.kinds.change.stages.construct]`: "not this one. just
  any elaboration types".
- **How a person opens it**, on `flume open madswan --session
  madswan-design`: "This confuses me. Why wouldn't it just be madswan
  design? Why do you need the flag? Why wouldn't you just open madswan
  design?"
- **Approval**, 2026-11-06: the user approved this design, the kind's
  table alone, the kinds the host names by name and `flume open
  <session>` among it, and named each partition's research-and-design
  session `<partition>-design`.

This design carries those rulings: what the host writes, what a person
opens and how, what the terminal profile does, what status says, and the
statement's amendment.

### What stands now

On agentplot/blueprints main at 7bd1891, before this change's
amendment:

- **107.** Where in the terminal a stage's agent runs is one of its
  settings. A launch may put it beside a pane it names; otherwise it
  runs in its stream's own part of the terminal, the one the host names
  for the stream, else the one named by the stream's name, and flume's
  own for the host's default stream.
- **32.** Program, model, effort, mode, instructions and own settings
  resolve from eight layers. Where an agent runs is not among them: it
  "is read from the launch's settings and the stage's declaration, else
  from its place's stream".
- **143, 186 to 190.** A stream's words: its bound, priority, pool,
  points, pace, terminal (`multiplexer`), session, view, terminal app
  and title. `flume open` shows a stream's own session (186); every
  agent stays in the terminal it opened in (190).
- **A place runs in exactly one stream (143).**

On flume's main at 045361c, which this branch has merged, record 0031
built 107 and 143's session and record 0037 built 186 to 190:

- `Queue::session_of` (`crates/flume-core/src/room.rs`) takes a pane
  the settings name, then the launch's `session`, then
  `StreamRoom::session`: the stream's `session`, else its name, else the
  terminal's own for `default`. Every layer but the launch's has
  `session` taken away (`sessionless` in `settings.rs`). The start
  records `SessionHeld { name, from }` and opens through
  `OpenIn::Session { name }`.
- The herdr profile records each workspace and tab by its server,
  closes the place's tab in every session it made one in, and shows the
  base's count on each of its workspaces (records 0018, 0031).
- `Engine::open_stream(stream)` (`crates/flume-core/src/open.rs`) shows
  a stream's own session in its view through its multiplexer and
  terminal app, under its title; `flume open [<stream>]` asks it
  (core.48).

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

So a person who opens `madswan-design` sees every workspace in it, and
in each the tabs of the research and design places whose agents opened
there, and no construction agent.

### What "elaboration" names

Weir's statement names an **elaboration kind**: "a kind with an
elaboration section, read by whoever runs elaborations" (weir 55). Its
section is sounding's: "Whoever runs places reads the kind's stages and
nothing of the section" (weir 55, S11), and flume ignores every section
it does not know (core.4). The user's "elaboration types" are the kinds
whose whole work is research and design, as against a change, a fix or
a chore, which build. On this host today that is `research`; crew's
design and planning roles join it once they run as kinds, and any
elaboration kind sounding hands flume. The host names each, one table
per kind by its name (decision 2).

### Across the family and the changes in flight

- **Program settings, effort and `keep_open`.**
  `an-agents-program-settings-come-from-flumes-layers` (in code) and
  `an-agents-effort-is-a-setting-of-flumes-layers` fold settings through
  clause 32's eight layers; `an-answer-reaches-the-open-session-within-an-hour`
  folds `keep_open` from the launch's and the host's four layers. Where
  an agent opens is not one of those settings, and stays out of their
  fold (decision 2), so no two changes resolve one setting two ways.
- **`flume-opens-a-streams-multiplexer`** landed on main as #109 and is
  merged here: its `open`, the terminal's attach by view and the window
  seam (186 to 190, record 0037) are what decision 7 extends.
- **`flume-list-shows-each-place-and-stream-on-one-line`** (record
  0049, clause 200 on blueprints main at 7bd1891, its code in its own
  place): the list gives a line for each stream "with the session its
  agents run in", and says "The streams are what a person opens by name
  (186)". This amendment gives each stream's line its kinds' sessions
  too, each a name a person opens; whichever change's code lands on
  flume's main second builds that line (decision 11).
- **`every-question-a-place-asks-reaches-someone-who-answers-it`**
  (designing): crew's roles run as kinds in their partition's stream,
  each run a place of its role's kind. A host names a session for a
  role's kind as for any kind.
- **The fleet's streams**, afterthought/swancloud
  `each-stream-opens-in-a-session-the-host-chooses-and-gets-its-labels`
  (record 0053 there): one stream per partition, named by the
  partition, opening in the partition's own herdr session, with
  `planning` a label. Decision 12 says what the fleet would write
  beside that.
- **sluice and Boomdrive** name no terminal session: sluice's runs and
  the runner's work keep no agent in a pane. No other tool of the family
  decides where an agent opens, so there is nothing to reconcile.

### What rules what

Record 0048, accepted on the user's word of 2026-11-06, decides it,
carrying their rulings, and the statement's amendment is on blueprints
main at f676e8f (decision 13).
The rules this design relies on:

- core.1 and core.2: the core names no terminal; it hands the terminal
  `OpenIn::Session { name }` and the terminal profile makes it a herdr
  session.
- core.3 and core.45: the host's configuration, its local layer over
  its managed one, is the one place a session is read from.
- core.4: flume reads nothing of a kind's elaboration section.
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
- core.44 and core.48, as record 0048 amends them: where an agent
  opens, and what a person's open shows.
- core.49: every call on an agent goes to the terminal its start
  recorded.

The one rules page is `docs/architecture/core.md`; this change touches
no area another page would own.

## Goals / Non-Goals

**Goals:**

- A host keeps a stream's research and design places apart from its
  construction in the terminal, by kind, while every place keeps its
  one stream, its bound, its pool, its points, its pace and its label's
  share (70, 143, 180).
- A person opens the research and design agents together by the
  session's name alone, `flume open madswan-design` (186).
- Every agent is sent to, read and closed where it opened, and a place
  is tidied in every session it opened in (190, 137).
- Status says where each agent opened and why, and which sessions each
  stream has (56).
- Where an agent opens has one resolution, apart from clause 32's
  layered settings, so no sibling folds it a second way.

**Non-Goals:**

- A session for one stage of a kind. Of a change's construct stage the
  user said "not this one. just any elaboration types", and nothing else
  asks for one (decision 2).
- A terminal or a view per kind. A stream's terminal, view, terminal
  app and title stay the stream's (186, 190).
- A session for a repository, or a repository and kind, within a
  stream. Nothing asks for it, and a stream's sessions stay few.
- A workspace other than the repository and base's. The user ruled a
  session of its own.
- Moving an agent that runs. An agent stays where it opened (190).
- Which kinds a host opens apart, and what it names their sessions.
  Those are the host's; decision 12 sketches the fleet's.
- A statechart kind's sessions (175), which flume runs none of yet
  (core.41); a host names a session for a statechart kind by its name
  as for any kind.

## Decisions

### 1. The part a person opens: a session of its own

The user: "A session of its own beside the stream's, e.g.
`madswan-design`". A stream may name, for a kind, a session of its own
terminal for that kind's agents to open in (199). Under herdr it is a
herdr session: a server of its own, started headless where none runs,
laid out as every session flume opens in, a workspace per repository and
base, a tab per place (C.5; record 0018).

Alternatives considered: a workspace within the stream's session,
holding the research and design places of every repository and base. One
window would show everything, but a workspace holding several bases'
places breaks record 0018's layout, which the base's count clue (139)
and the tidy of a base's workspace (138) rest on, and needs a part within
a part the terminal seam does not have. The user chose the session.

### 2. Where the host says it: the stream's table for a kind

The user: "The stream's own table, `[streams.<stream>.kinds.<kind>]`",
and, of a change's construct stage, "not this one. just any elaboration
types". The fleet's sketch:

```toml
[streams.madswan.kinds.research]
session = "madswan-design"
```

`[streams.<stream>.kinds.<kind>]` holds `session` and nothing else. The
kind is named as `[kinds.<kind>]` names it, by its name, whatever source
and version it resolves to (19, 143, 180). Every stage of the kind opens
there. It is not one of clause 32's settings: no agents table,
launch's `--agent`, repository's file or kind gives it, and the launch's
`session` keeps its place before it (decision 3).

Why a kind's table and no stage's: the user's words name the kind's
table, and of a change's construct stage they said "not this one". A
change's design stays with its code in the change's own
tab, which is where a person follows one change from its design to its
landing. A stage's table adds later without breaking a host: `stages` in
a kind's table is today another key, refused (decision 4), so no host
can come to rely on it meaning something else.

Why the stream's own word:

- **A session is the stream's.** A session is a part of the stream's
  terminal (190), and the stream already names its own (143). A kind's
  research session in one stream is not its research session in
  another: the fleet's partitions each keep their own.
- **32's host layers cannot say it.** Its layers for a repository and
  kind and for a kind name no stream, so a session given there opens
  every stream's research in one session, mixing two partitions, unless
  the host writes it for every repository.
- **One resolution.** The launch's settings, then the stream's table.
  The siblings' fold of 32's settings never sees a session.

Alternatives considered:

- **A stream per sort of work.** Splits the partition's bound, pool,
  points, pace and planning share, which the user kept one per
  partition.
- **`session` one of 32's settings**, given by the launch and the
  host's four layers as `keep_open` is. The user chose the stream's
  table.
- **Every elaboration kind at once**, one stream word, say
  `elaboration_session`, opening every kind weir calls an elaboration
  kind there. A new elaboration kind would join without the host's
  word, but flume would read whether a kind has weir's elaboration
  section, which weir 55 gives to whoever runs elaborations and core.4
  keeps from flume, so weir's statement would change first.
- **A session per label.** A label is the kind's share of its stream
  and nothing else (180; core.46, the user's ruling).

### 3. The order where an agent opens

At each start (107, 199):

1. a pane the settings name, the launch's `beside` or the stage's
   declaration's naming a launch's pane: a split beside it;
2. the session the launch's settings name, the stage's own before every
   stage's;
3. the session the place's stream names for the place's kind;
4. the stream's own: the one `[streams.<stream>] session` names, else
   the stream's name, and, for `default` naming none, the terminal's
   own.

`Queue::session_of` takes the place's kind's name beside the stream and
the settings, and answers `SessionHeld` with `from` one of `launch`,
`stream-kind`, `stream`, `stream-name` or `own`, or none beside a pane.
A repeating stage's repetitions each open where their kind's word says.
A start that carries a spent agent's conversation (179) opens where its
own start resolves, as every start does.

### 4. Read at each start; what is ignored, what refuses

- The stream's tables are read with the rest of the host's
  configuration at each start, the local layer over the managed one
  (32, C.7). A change to them takes the next start; an agent already
  started stays where it opened, and every call on it goes to the pane
  its start recorded (190; core.49).
- A table for a kind the place does not run is ignored, as the host's
  settings for a kind the place does not run are (C.7).
- **Another key.** A table holding any key but `session` is kept with
  the key named, as a label's table is (`Share::unknown`), and a start
  that reads it is refused naming the key and the table, until the
  configuration is mended and the stage retried (4, 27, 182, 199). Only
  the starts of that kind on that stream are refused. A `session` that
  is not a string refuses the configuration naming the key, as a
  stream's `session` that is not a string does today.
- **Another stream's session.** A session a stream names for a kind is
  that stream's alone. Where it is a session another stream's agents
  open in by that stream's word, its own session, its name where it
  names none, or one it names for a kind, a start that reads it is
  refused naming both streams and the session, until the configuration
  is mended (4, 199). So a person opens a kind's session by its name
  alone and it shows one stream's agents in that stream's view
  (decision 7), and two partitions' research never share a session. A
  stream may name its own session for a kind; that is its own session,
  from `stream-kind`.
- A session's name is herdr's to judge: one the pinned herdr refuses
  refuses the open in herdr's words, nothing of flume's left open
  (core.29, core.44).

The path not taken for another stream's session: letting two streams
name one session for their kinds, `flume open` refusing that name as
ambiguous. A session two streams' agents open in has two views, two
terminal apps and two titles to choose from, and a person could not
open it by its name; nothing asks for it.

### 5. The place keeps its stream

Nothing of room changes (70, 143, 180 to 182). A place's agents in its
stream's research session count toward its stream's bound and the
host's, take their accounts from its stream's pool, charge its points
and pace, and take their label's share, as every agent of the stream
does. Where an agent opens is read after the queue admits its start, as
today.

### 6. The local layer

`LOCAL_KEYS` gains `streams.*.kinds.*.session`, a key of its own, so an
operator moves a kind's session from hour to hour as they move a
stream's (C.7; record 0032). The managed layer may pin it,
`"streams.work.kinds.research.session"`, or pin a table holding it,
`"streams.work.kinds"`. A local layer that sets another key of these
tables is not taken, as for any key outside the list.

### 7. The view: a person opens a session by its name

The user: "Why wouldn't it just be madswan design? Why do you need the
flag? Why wouldn't you just open madswan design?" No flag is needed.
`flume open [<name>]` (186) reads the one name it is given:

- **A stream's name**, or none for `default`: the stream's own session,
  as record 0037 builds it, under the stream's title, else its name.
- **A session a stream names for a kind**: that session, in its
  stream's view, through its stream's multiplexer and terminal app,
  started headless where it does not run (187). Its window's title is
  the session's name, which the host chose, so a window manager files it
  apart from the stream's own window. The answer gives the stream, the
  session from `stream-kind` and the kinds it is named for. Where that
  session is the stream's own, it is shown as the stream's own, under the
  stream's title.
- **Anything else** is refused naming the streams the host declares and
  the sessions each names for its kinds, before the terminal or a window
  is asked anything, so a mistyped name starts no stray session.

A stream's name is read first. A name cannot be both a stream's and a
session another stream names for a kind: decision 4 refuses that
stream's starts and status names the fault, and `open` of that name is
refused naming both streams, until the configuration is mended.

A person finds madswan's research and design agents by `flume open
madswan-design`: one window, every workspace in it a repository and
base, every tab a research or design place. `flume open madswan` opens
madswan's own session as before.

The session's name is the host's whole word, `madswan-design`, as the
user named it, `<partition>-design`: flume builds no name from a stream
and a part (core.44).

Alternatives considered: `flume open madswan --session madswan-design`,
naming the stream and then the session. It names one session twice and
asks the person to know which stream owns it; the user asked why it
needs the flag, and it does not, since each such session is one
stream's (decision 4). `flume open madswan research`, the stream and a
kind, resolving the kind's session: a second reading of the same table
on the command line, and the person thinks of the session they want,
not the kind that opens there.

### 8. The terminal profile

Nothing new is asked of herdr, and the core names no terminal (core.1):

- **open** gets `OpenIn::Session { name }` with the session decision 3
  resolves. The herdr profile finds the session's server or starts it
  headless, then opens in that session's workspace of the place's
  repository and base, in the place's tab there, each made when first
  needed and recorded by id and server (C.5).
- **send, state, screen and close** go to the pane the start recorded by
  its id and server, found again by the agent's herdr name where the id
  no longer answers (98, 190; core.49).
- **tidy** closes the place's tab in every session it made one in, and
  each base's workspace once it holds no tab of a live place and nothing
  flume did not make (137, 138). A place's agents open in one session
  by their kind's word; a place has tabs in two only where its kind's
  session changed between its starts or a launch named a session for
  one stage, as record 0031 already built.
- **show** gives the base's count on each of its workspaces in every
  session (139).
- **watch**: each session's server runs flume's herdr plugin as each
  stream's does, and settles are safe to repeat (122; core.17).
- **attach** takes the session's name (187).
- **The printed command** names no session: open prints as before.

A test through the binary under the pinned herdr holds it end to end: a
research place's agent opening in herdr's session `w-design`, a fix's in
`w`, `flume open w-design` attaching `w-design` under that title, each
agent read and closed on its own server, and each tab closed at its tidy
(task 6.2).

### 9. Status

- **Each agent** gives its session as today, `SessionHeld`, with `from`
  gaining `stream-kind`. The person's form says `session madswan-design
  (stream madswan, kind research)`.
- **Each stream** gives its own session as today, and a new
  `sessions`: each session it names for its kinds, with the kinds it
  names it for and the layer of each, `managed` or `local` (56, C.7). A
  table's other key, and a session that is another stream's, are given
  with the refusal each causes.
- The JSON form only gains a field and a value (core.14). A start
  written before reads with its `from` as written.

### 10. The seams and the core

- `flume-seams`: `Stream` gains `kinds: BTreeMap<String, KindSession>`,
  `KindSession { session, unknown }`, read with none where written
  before and written only where set. `SessionFrom` gains `StreamKind`,
  serialized `stream-kind`. The stream status gains `sessions:
  Vec<NamedSession>`, `NamedSession { name, kinds }`, each kind with its
  layer.
- `flume-core`: `StreamRoom` keeps the stream's `kinds`.
  `StreamRoom::session` becomes `session_for(stream, kind, others)`,
  answering the session with its origin, or the refusal naming the
  table's other key, or naming the other stream whose session it is.
  `Queue::session_of` takes the kind; `Engine::claim` refuses the stage
  where it answers a refusal, as a stream's label fault does.
  `status.rs` gives each stream's sessions. `Engine::open_stream` takes a
  name: a stream's, else a session a stream names for a kind.
- `flume`: `host/config.rs`'s `streams_declared` reads `kinds` beside
  `labels`, its key in `LOCAL_KEYS`; `commands/open`'s argument is a
  name, its help saying a stream or a session a stream names for a
  kind; `print.rs` gives the new origin and each stream's sessions.
- `flume-fakes`: the fake host's `Stream` carries `kinds` as written by
  a test.

### 11. Agreement with the changes in flight

- **Order.** The user approved this design on 2026-11-06, and
  blueprints main holds decision 13's amendment at f676e8f, so the code
  starts now. The multiplexer change it extends has landed and is
  merged; group 1 merges main again.
- **Program settings, effort, `keep_open`.** None of them reads or
  writes a session: `session` stays out of `Settings::under`'s fold
  but the launch's, and out of every agents table. Should any of them
  land first, nothing here changes.
- **The list (200, record 0049).** Its stream lines name each stream's
  own session and say the streams are what a person opens by name.
  Clause 200 gains, in this amendment, each session the stream names
  for a kind, with whether it runs (201). Where `flume list` is on
  flume's main when this change's group 5 starts, this change gives
  each stream's line those sessions (task 5.2); otherwise the list's
  change builds it from 200 as amended, and is told so.
- **Questions.** A role's run is a place of its role's kind in its
  partition's stream; a host names a session for that kind as for any
  other. A role's run is reached by the pane its live start holds,
  wherever it opened (132).

### 12. What the fleet would write

Beside its streams design, which names each stream by its partition and
opens it in the partition's own session, the fleet's
`lib/flume-streams.nix` would render, for each partition stream, one
table per elaboration kind it runs, `madswan` shown:

```toml
[streams.madswan.kinds.research]
session = "madswan-design"
```

and the same for `swancloud` and `agentplot`, each in
`<partition>-design`, the name the user gave each partition's
research-and-design session on 2026-11-06. A crew role
whose run is design work, the design agent and the planner, adds its
kind's table once crew's roles run as kinds, named as the fleet's
`[kinds.<kind>] label` tables name them; so would an elaboration kind
of sounding's, once the fleet runs one. The fleet's nix could render a
table for each kind it gives the `planning` label, so the two lists
never drift; that is its own choice. The fleet would also declare each
`<partition>-design` session on every Mac that holds a Flume file, as
`lib/herdr-hosts.nix` checks each stream's session today, and give a
person's launcher for it `flume open madswan-design`. This change adds
nothing to the fleet; its orchestrator is told.

### 13. The statement's amendment

On agentplot/blueprints main at f676e8f, on the user's word of
2026-11-06 (core.19). Clause 199 and scenarios S106 and S107 are new,
beside 186 to 192, 195 to 197, 200, 201, S90 to S92, S95 to S97, S100 to
S103 and S108, and the designs in flight hold 193, 194, 198, S93 to S99,
S104 and S105. Each item gives the words a clause gains or its new
text.

- **32**: "Where in the terminal an agent runs (107) is read from the
  launch's settings and the stage's declaration, else from its place's
  stream, by its kind (143, 199)."
- **107**: "Where the settings name no place in the terminal, the agent
  runs in its place's stream's own: the session the stream names for
  the place's kind (199), else the one the host's configuration names
  for the stream, else one named by the stream's name, so every agent
  of a stream is found in the stream's sessions and a person opens each
  by its name. A place in the host's default stream runs in flume's own
  where its stream names none for it (143)."
- **143** gains, after "its own name where it states none (107)": "and,
  for a kind, the session of that terminal the kind's agents run in
  instead (199)".
- **186**: flume shows "the stream's own part of its terminal, or a
  session the stream names for a kind (107, 190, 199), the person
  naming the stream, or the session by its name alone". Its title is
  the stream's "and, for a session the stream names for a kind, that
  session's name". It refuses "a name that is no stream the host
  declares and no session a stream names for a kind … naming the
  streams and their sessions, each before anything is started".
- **190**: "the one the host's configuration names for the stream, else
  the host's, whose parts the sessions the stream names for its kinds
  are (199)."
- **199**, new: "A stream may name, for a kind, the session of its
  terminal that kind's agents run in. So a host keeps a stream's
  research and design apart from its construction in the terminal,
  every stage of such a kind running there. Every place of the stream
  keeps the stream's bound, priority, pool, points, pace and labels
  alike (70, 143, 180 to 182). Where the settings name no place in the
  terminal, a stage's agent runs in the session its stream names for
  its kind, else in the stream's own (107). A kind is named there by its
  name, whatever source and version it resolves to, as the host's
  configuration names a kind for a stream or a label (19, 143, 180).
  The host names nothing else there: no stage, repository, label or
  terminal. A session a stream names for a kind is that stream's alone:
  one another stream's agents run in by that stream's word, its own or
  one it names for a kind, refuses the starts that read it, naming both
  streams, until the configuration is mended (4). The session is read
  at each start from the configuration as it stands (32). A change to
  it takes the next start, and an agent already started stays in the
  session it opened in (190). A session named for a kind the place does
  not run is ignored. A start that reads a stream's table for its kind
  holding any key but the session is refused naming the key, until the
  configuration is mended (4). A person opens such a session by its
  name, and is shown it as the stream's own is (186). Status gives each
  session a stream names, with the kinds it names it for (56). It gives
  each agent's session with where it came from: the launch, the
  stream's word for its kind, the stream's own session, the stream's
  name, or flume's own."
- **200** gives each stream's line "with the session its agents run in
  (107) and each session it names for a kind (199), and whether each
  runs now (201). The streams, and the sessions they name for their
  kinds, are what a person opens by name (186)."
- **C.5's herdr row**, open: "a pane in the herdr session the launch's
  settings name, else the one its place's stream names for the place's
  kind (199), else the one its place's stream names, by default the
  stream's name, and flume's own for a place in the host's default
  stream naming none (107, 143): in each session, one workspace per
  repository and base … made when the first place on that base opens
  there, and in it one tab per place … whose panes are the place's
  agents that open in that session". The prose after it: "It closes a
  place's tab in each session it made one in when the place goes".
- **C.7**: the local layer may set "a stream's `agents`, `priority`,
  `points`, `pace` and `session` and `multiplexer`, and the `session`
  it names for a kind (107, 143, 166, 167, 190, 199)". A stream may
  state, after `session`: "in `[streams.<stream>.kinds.<kind>]`,
  `session`, the session of that terminal the kind's agents run in, the
  kind named as `[kinds.<kind>]` names it, the table holding nothing
  else (199)". The example gains `[streams.work.kinds.research]
  session = "work-design"`, and its reading "a research place's agents
  opening in the session `work-design`, which a person opens by that
  name, and work's other agents in `work`, every one of them counted in
  work's five".
- **S106** and **S107**, new: a stream's research in `work-design`, a
  change and a fix in `work`, the room and the shares unchanged, a
  launch's session and a pane before it, status, and the tidy in each
  session; then a person's `flume open work-design`, a name that is no
  stream and no session refused, the local layer moving research's next
  start, a table's other key, and another stream's session, each
  refusing research's next start on `work` alone.

### 14. The site

Written to any host's operator, naming no host, team or word of ours
(core.35):

- `guides/places-and-herdr.mdx`: a stream's sessions for its kinds, and
  how a person opens one by its name.
- `guides/agent-settings.md`: where an agent opens, in order, and that
  it is the stream's word, not a layered setting.
- `reference/configuration.md`: `[streams.<stream>.kinds.<kind>]`, the
  local layer's key and its pins.
- `reference/commands.md`: `flume open <stream or session>`.
- `reference/state.md`: `SessionFrom`'s new value and each stream's
  `sessions`.

## Risks / Trade-offs

- **[A second window to watch]** → The design session is its own
  server, so a person keeps two windows per stream. That is the ask:
  the research and design agents found apart.
- **[A new elaboration kind opens in the stream's own session]** → A
  kind joins the design session only once the host names it. Status
  gives each stream's sessions with their kinds, so a missing kind
  shows.
- **[More herdr servers]** → Each named session is a server with the
  herdr plugin's watch. Settles are safe to repeat; a few servers per
  host cost a few more settles.
- **[A typo in a session name]** → It opens a session of that name at
  the next start, which status shows by name and origin; `flume open`
  refuses a name that is no stream and no session, so a person's typo
  starts nothing.
- **[A typo in a kind's name]** → The table is ignored as for a kind the
  place does not run, so agents open in the stream's own; status gives
  each stream's sessions with the kinds named, which shows the
  misspelt kind. Refusing an unknown kind is not possible: a kind's name
  resolves only at a launch (19).

## Migration Plan

Additive. A host that names no session for a kind runs as before. A host
that adds one finds the next start of that kind there; every agent
running stays where it opened, and its place's tabs in each session are
closed at its tidy. `flume open <stream>` opens as before. Rollback is
removing the tables: the next starts open in the stream's own session.

## Open Questions

None: the user approved this design on 2026-11-06.
