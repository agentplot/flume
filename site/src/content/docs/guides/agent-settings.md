---
title: Agent settings
description: The settings each agent starts with and the six layers they come from; the mode an agent may act in; the streams a host runs its places in, the pools of accounts its agents run under, each account's reading of its allowance and its pool's threshold, and how a start is checked against what it was given.
sidebar:
  order: 6
---

Each agent flume starts runs with five settings, resolved at its start
from six layers, and under an account of a pool when the host names
one. The account gives the agent its environment. `flume status` gives
each setting of the current agent with the layer it came from, and the
account with what chose it.

## The settings

| Setting | What it is |
|---|---|
| `program` | the agent program: `claude`, or a path to Claude Code |
| `model` | the model, passed as `--model` |
| `instructions` | text appended to the agent's system prompt, written to a file of the start's own and passed as `--append-system-prompt-file`, so it may hold newlines |
| `beside` | a pane the agent's pane opens beside, under herdr |
| `mode` | how far the agent may act in its place without asking a person: one of flume's four modes, or a table of programs ([the mode](#the-mode)) |

`program` defaults to `claude`. Every other setting is empty unless a
layer gives it.

## The six layers

Each setting comes from the first of these layers that gives it:

| Layer | Status names it | Where it is written |
|---|---|---|
| the launch | `launch` | `flume launch --agent [<stage>.]<field>=<value>` |
| the host, for a repository and kind | `host-repository` | the host's `[repos."<repository>".kinds.<kind>.agents]` |
| the host, for a kind | `host-kind` | the host's `[kinds.<kind>.agents]` |
| the repository | `repository` | the repository's `[kinds.<kind>.agents]`, for a kind it does not declare |
| the kind | `kind` | the kind's own `agents` table and its stage's agent table |
| flume's default | `default` | the program `claude`, and nothing else |

Within a layer, a stage's own setting, in a further `.stages.<stage>`
table, or `--agent <stage>.<field>=<value>` at the launch, comes before
one given for every stage. In the kind's layer, a stage's own agent
table comes before the kind's `agents` table. Each setting is taken
whole from the first layer that gives it.

The host's two layers give `model`, `instructions` and `mode` alone,
and flume reads no other key of those tables. `beside` comes only from the launch
and the kind, since where a pane opens is the caller's to say or the
kind's to need. No layer gives an environment: an agent's environment
is its account's ([the environment](#the-environment)).

### When each layer is read

The host's layers are read from its configuration as it stands at each
agent's start. Change a model in the host's file and retry a stage, and
the next agent runs on the new model, with no new launch. A mode changed
the same way reaches the next agent's start the same way. Every other
layer is what the launch read: a place keeps its launch's settings, its
repository's and its kind's for its whole life.

## At the launch

```sh
flume launch --kind change --name csv-export --brief "Export the ledger as CSV." \
  --agent model=sonnet \
  --agent verify.model=opus \
  --agent code.instructions="Run the tests before every commit." \
  --agent code.mode=edits
```

The field is one of `program`, `model`, `instructions`, `beside` and
`mode`, with an optional stage before it. A setting for a stage the
kind does not hold refuses the launch, naming the stage, and so does a
field that is none of the five.

`mode=<mode>` gives one of flume's four modes. `mode.<program>=<value>`
gives one entry of a table of programs, and several such entries build
one table: `--agent mode.claude=auto`. A mode word and a table for the
same stage, or for every stage, refuse the launch naming both, since a
mode is one word or one table.

## On the host

The host's configuration, `~/.config/flume/config.toml`, gives
settings to a kind in every repository, and to a kind in one
repository. This host runs every change on opus and every fix on
sonnet, and tells one repository's fixes how to run its tests:

```toml
[kinds.change.agents]
model = "opus"

[kinds.fix.agents]
model = "sonnet"

[repos."~/src/acme".kinds.fix.agents]
instructions = "This repository's tests need the database: run them with `just test`."
```

A repository's key is its location, with a leading `~` read as your
home. Settings for a kind or a stage the place does not run are
ignored. Each agent runs under whichever account its stage's pool
gives.

### Two layers

The host's configuration is two files read as one. `config.toml` is
the managed layer, written by whoever manages the host.
`config.local.toml` beside it is the local layer, yours or a control
plane's. flume reads it over the managed one key by key at each
command: a key it sets replaces the managed one's, and an agents
table's `mode` table is taken whole. flume writes neither file.

The local layer sets only what runs the host's work from hour to hour:

- a stream's `agents`, `priority`, `session` and `multiplexer`, the
  `session` it names for a kind or a kind's stage, and its labels'
  `reserve` and `keep_free`;
- the `stream` and the `label` a repository's kind, a repository or a
  kind is given;
- a kind's agents tables, `[kinds.<kind>.agents]` and
  `[repos."<repository>".kinds.<kind>.agents]`, with their stages'.

The accounts, the pools, a stream's `pool`, `view`, `terminal_app` and
`title`, `[host]` and `[roles]` are the managed layer's alone. A local
file that sets any of them, or that is not TOML, is not taken at all.
flume then reads the managed layer alone, and status names each key the
file may not set, or its fault, until you mend it. The local layer may name a stream the managed one
does not, and that stream's places take `[host] pool`.

The managed layer may pin a key the local layer could set, by `pin`
under `[host]`:

```toml
# config.toml
[host]
pin = ["streams.work.agents"]

[streams.work]
agents = 6
pool = "work"
```

```toml
# config.local.toml
[streams.work]
agents = 3
priority = 30
```

The local priority is taken, and `work` ranks at 30. Its agents are
passed over, so `work` keeps six, and status names the key with its
pin. A pin names a key or a table of keys, as `"streams.work"` does.
A pin that names neither makes the managed file refuse. Status gives
every value of the configuration with the layer it came from, and an
account's variables by name and `<set>`.

## In the repository

A repository gives settings in its `.config/flume.toml`, in the kind's
own declaration or, for a kind it does not declare, in a table that
holds only `agents`:

```toml
[kinds.change.agents.stages.verify]
model = "opus"
instructions = "Check every scenario of the change's specs, not only its tasks."
```

A declaration gives no environment: an `env` in any agents table of a
repository's file or a launch's declaration refuses the launch, naming
the field. What a host's agents run under is that host's to say.

## The mode

`mode` says how far a stage's agent may act in its place without asking
a person. It is one of flume's four modes, which mean the same on every
program:

| Mode | The agent |
|---|---|
| `plan` | reads the place and changes nothing in it |
| `ask` | reads without asking, and asks a person before each change to a file and each command that is not a read |
| `edits` | changes files in its place without asking, and asks before any command that reaches beyond them |
| `full` | acts without asking |

Each program's agent profile maps each mode to the one setting of its
program that means it, never a wider or a narrower one, and refuses a
mode its program has no setting for. Claude Code's are its
`--permission-mode` choices:

| Mode | Passed to Claude Code as |
|---|---|
| `plan` | `--permission-mode plan` |
| `ask` | `--permission-mode manual` |
| `edits` | `--permission-mode acceptEdits` |
| `full` | `--permission-mode bypassPermissions` |

`plan` suits a conversation, whose agent plans with you and asks in its
pane to leave plan mode, which you answer there. A launch that gives
`plan` to any other stage is refused, naming the stage and its
deliverable, since an agent in plan mode could not write it.

### A host's mode over a repository's

A host that sets its agents' reach states the mode in its own layers,
which come before a repository's and a kind's. This host runs every fix
in `edits`, whatever a repository asks:

```toml
# the host's ~/.config/flume/config.toml
[kinds.fix.agents]
mode = "edits"
```

```toml
# the repository's .config/flume.toml
[kinds.fix.agents]
mode = "full"
```

A fix's agent here starts in `edits`, passed to Claude Code as
`--permission-mode acceptEdits`, and status names `host-kind` as where
its mode came from.

### A program's own value

In place of one of the four, a layer may give a table of programs, each
entry one of the four or a value of that program's own:

```toml
[kinds.review.agents]
mode = { claude = "auto" }
```

The table is taken whole from its layer, and the agent's entry is its
program's. A program the table names no entry for refuses the stage:
the mode never passes to a later layer. Claude Code's own values are the
`--permission-mode` choices of the release flume pins: `acceptEdits`,
`auto`, `bypassPermissions`, `manual`, `dontAsk` and `plan`, each passed
as written. A program's own value given bare, as `mode = "auto"`, is
refused where it is read, naming it: give it under its program's name.

### No mode

Where no layer gives a mode, flume passes none, and the account's own
configuration of its program decides, as it decides everything else
flume does not give. A host and a repository that say nothing change
nothing.

### What refuses

A launch is refused where a stage's agent resolves to a mode its
profile cannot run, or to a table that names no entry for its program.
The refusal names the mode as written, the program, the stage and the
layer.
A host's table changed after the launch leaves the stage refused at its
start in the same words, starting no agent until a retry.

### What a mode leaves to you

flume answers no question an agent puts, in any mode. An agent in `ask`
or `edits` that asks a person waits asking until someone answers it in
its pane, and status says it is asking, so keep those modes for work
someone watches. Claude Code warns before an account's first session in
`bypassPermissions`, and flume never answers that warning either:
accept it once yourself under the account, as
`CLAUDE_CONFIG_DIR=~/.claude-work-1 claude --permission-mode
bypassPermissions`, before its agents run `full`.

## Streams

A host runs its places in streams. Each stream has its own bound on
agents running, its own priority and its own pool of accounts, all
under the host's bound. The stream `default` always stands: it holds
every place nothing names a stream for, under the host's bound alone
unless a `[streams.default]` table gives it one of its own.

```toml
[host]
agents = 8

[streams.work]
agents = 5
priority = 10
pool = "work"

[streams.personal]
agents = 3
pool = "personal"

[pools.work]
strategy = "fill-first"
accounts = [
  { name = "work-1", program = "claude", env = { CLAUDE_CONFIG_DIR = "~/.claude-work-1" }, agents = 4, usage = "~/.local/state/usage/work-1.json" },
  { name = "work-2", program = "claude", env = { CLAUDE_CONFIG_DIR = "~/.claude-work-2" }, usage = "~/.local/state/usage/work-2.json" },
]

[pools.personal]
strategy = "least-busy"
accounts = [{ name = "personal", program = "claude", env = { CLAUDE_CONFIG_DIR = "~/.claude-personal" } }]

[repos."~/src/acme"]
stream = "work"
```

This host runs acme's places in `work`, up to five of its eight agents,
ahead of `personal`'s. They run on work-1 until it runs four, is spent
or is past ninety percent of a window its reading gives, and then on
work-2.

### A place's stream

A place's stream is fixed at its launch, from the first of:

| Source | Status names it |
|---|---|
| `flume launch --stream <stream>` | `launch` |
| the stream of the intent's affinity ([affinity](#an-intent-keeps-an-account-per-program)) | `intent` |
| `stream` in the host's `[repos."<repository>".kinds.<kind>]` | `host-repository-kind` |
| `stream` in the host's `[repos."<repository>"]` | `host-repository` |
| `stream` in the host's `[kinds.<kind>]` | `host-kind` |
| `default` | `default` |

A launch whose stream the host does not declare is refused, naming the
stream, with nothing made. A role's run takes `stream` from its
`[roles."<group>/<role>"]` table.

### The queue across streams

A stage starts only while the host, its stream and an account of its
pool all have room. Waiting stages start by their stream's priority,
then their place's priority, then the order they came to wait. A settle
passes over a stream at its bound, or one whose pool has no usable
account. Its stages keep their order and start first once it has room,
and they never hold the host's room from another stream's stages. flume ends no
agent to make room.

### A stream's sessions

Under herdr, a stream's agents open in herdr sessions of the stream's.
Its own is the one its `session` names, else the one named by the
stream's name. The `default` stream's own is flume's session, `flume`,
unless `[streams.default]` names one. A stream may also name, for a
kind, a session of its own for that kind's agents, and, for one stage
of a kind, one for that stage's agents. So a host keeps a stream's
research apart from the rest of its work:

```toml
[streams.work]
agents = 5

[streams.work.kinds.research]
session = "work-design"

[streams.work.kinds.release.stages.review]
session = "work-review"
```

At each start, an agent opens in the first of:

| Where | Status names it |
|---|---|
| beside a pane its settings name, by `beside`, in whatever session that pane is | `beside the pane` |
| the session the launch names, `--agent <stage>.session=<name>` before `--agent session=<name>` | `launch` |
| the session its stream names for its kind's stage | `stream-stage` |
| the session its stream names for its kind | `stream-kind` |
| the stream's own `session` | `stream` |
| the session named by the stream's name | `stream-name` |
| flume's own session, for `default` naming none | `own` |

Here a research place's agents open in `work-design`, a release's
review agent in `work-review`, the release's other agents in `work`,
and every other place's agents in `work`. A kind is named as
`[kinds.<kind>]` names it, whatever source and version it resolves to,
and a stage by its name in that kind. A kind's session takes every
stage of the kind but one its own table names.

These tables are the stream's word, not one of the six layers: no
agents table, repository's file or kind gives a session, and nothing
names one for a repository, a label or a stage of every kind. They are
read at each start, the local layer over the managed one, so a change
takes the next start and an agent stays where it opened. A table for a
kind or a stage the place does not run is ignored. A misspelt kind's
table opens nothing apart, and status, which gives each stream's
sessions with the kinds and stages it names them for, shows the name as
written.

A place whose agents open apart keeps its stream. They count in its
bound and the host's, take its pool's accounts and take their label's
share, wherever they open.

A start that reads one of these tables is refused, its stage waiting
for a retry once the file is mended, where:

- the table holds any key but `session` and, for a kind, `stages`.
  Only the starts of that kind, or that stage, on that stream are
  refused;
- the session it names is one another stream's agents open in by that
  stream's word: its own, or one it names for a kind or a stage. A
  session a stream names apart is that stream's alone, so `flume open`
  finds it in one stream's view. A stream may name its own session for a
  kind.

The refusal names the table's key, or both streams:

```text
streams.work.kinds.research.pool: a stream's table for a kind holds the session its agents open in and its stages' tables alone
```

A stage waiting for room whose table comes to refuse is refused at the
next settle, ahead of the room.

### A stream's multiplexer and view

Beside its session, a stream names the multiplexer its agents run in,
and how `flume open` shows them: its view, its terminal app and its
window's title. Each of the first three is the stream's, else the
host's, else the default:

```toml
[host]
multiplexer = "herdr"
terminal_app = "ghostty"

[streams.work]
session = "office"
title = "Work Agents"

[streams.trial]
multiplexer = "printed"
```

| Key | Neither names one | Status names it |
|---|---|---|
| `multiplexer` | the printed command | `stream`, `host` or `neither` |
| `view` | `terminal`, a window of a terminal app | `stream`, `host` or `neither` |
| `terminal_app` | the printed line, no window | `stream`, `host` or `neither` |
| `title`, on a stream alone | the stream's name | `stream` or `stream-name` |

Here `work`'s agents open in herdr's session `office`, and `flume open
office` opens a Ghostty window titled `Work Agents` on it. `trial`'s
agents run as commands flume prints, while every other stream runs
under herdr. A stream's multiplexer is read at each start, as its
session is, and an agent stays in the one it opened in. No launch,
kind, repository or label names a multiplexer: the stream is where you
try one. The local layer may set a stream's `multiplexer`, as it sets
its `session`; its `view`, `terminal_app` and `title` are the managed
layer's. [Seeing a stream's agents](/flume/guides/places-and-herdr/#seeing-a-streams-agents)
says what each view and terminal app does.

### Labels

A place's label gives it a share of its stream's agents, and nothing
else. A stream owes a label a `reserve` of its agents, and may keep
some of its slots free for it:

```toml
[streams.work]
agents = 5

[streams.work.labels.operation]
reserve = 2
keep_free = 1

[streams.work.labels.construction]
reserve = 2

[kinds.fix]
label = "construction"

[repos."~/src/acme".kinds.deploy]
label = "operation"
```

Within its stream, a stage of a label running fewer agents than its
reserve starts before every stage of a label running its reserve or
more, whatever their priorities. A label keeps `keep_free` of its
stream's slots free while it runs fewer agents than that: no stage of
another label, and no stage of no label, starts in them. flume ends no
agent for a reserve. A reserve another label borrowed comes back as
that label's agents end.

Take `work` above with no operation place waiting. Four construction
places run, and a place of no label waits, since the one slot left is
kept free for operation. An operation place launched then starts at
once in it. A second operation place waits until the next agent ends,
then starts before the place of no label and every construction place
waiting, whatever their priorities.

A place's label is read at each start, from the first of:

| Source | Status names it |
|---|---|
| `label` in the host's `[repos."<repository>".kinds.<kind>]` | `host-repository-kind` |
| `label` in the host's `[kinds.<kind>]` | `host-kind` |

An agent counts toward the label it started under until it ends, so a
label changed while it runs takes the place's next start. A label is
one word of letters, digits, `-` and `_`. flume's own kinds declare
none.

A stream holds its labels' reserves only where it states a bound, and
in one that states none status says they are passed over. A stream
starts no stage while its labels' reserves together pass its bound, a
label keeps more free than its reserve, or a label's table holds any
key but `reserve` and `keep_free`. Each of its starts is refused,
naming the stream and why, until the configuration is mended and the
stage retried. A label has no pool, account or bound of its own: those
are its stream's.

## Pools of accounts

A pool, `[pools.<pool>]`, lists accounts in order. Each account is a
`name`, the `program` it runs, an `env` that is its agents'
environment, an optional `agents`, the bound on agents running under
it, and an optional `usage`, the file of its reading ([an account's
reading](#an-accounts-reading)). A stage's pool is its stream's `pool`,
else the host's `[host] pool`. With no pool named anywhere, flume
chooses no account, and agents run under the login environment alone.

An account is **usable** for a start while it is unspent, is not past
its pool's threshold ([a threshold](#a-threshold)) and is under its own
bound, and runs the stage's program. At each start flume chooses an
account under the host's room lock, so two starts never take one
account's last room:

| `strategy` | The account chosen |
|---|---|
| `fill-first` | the first usable account in the pool's order |
| `least-busy` | the usable account running the fewest of flume's agents, equals in the pool's order |
| `most-left` | the usable account with the most of its allowance left by a fresh reading, then each with no fresh reading. Equals go by the fewest of flume's agents running, then the pool's order |

The intent's account for the stage's program comes first while it is
usable ([affinity](#an-intent-keeps-an-account-per-program)). With no
usable account the stage waits for room, and status names the pool and
the soonest time an account of it is back, neither spent nor past its
threshold. `least-busy` counts flume's own agents alone, and not the
sessions you run yourself under an account. A reading counts all the
account has used, your own sessions' use among it.

Sign each account's configuration home in once, by running
`CLAUDE_CONFIG_DIR=~/.claude-work-1 claude` yourself, since flume
answers no sign-in on an agent's behalf.

### An account's reading

flume asks no one how much of an account's allowance is left. It reads
what your host last read, from the file the account's `usage` names:

```toml
[pools.work]
strategy = "most-left"
accounts = [
  { name = "work-1", program = "claude", env = { CLAUDE_CONFIG_DIR = "~/.claude-work-1" }, usage = "~/.local/state/usage/work-1.json" },
  { name = "work-2", program = "claude", env = { CLAUDE_CONFIG_DIR = "~/.claude-work-2" }, usage = "~/.local/state/usage/work-2.json" },
]
```

In `usage`, as in `env`, a leading `~` is your home and `$VAR` or
`${VAR}` a variable of your login environment. The file holds one JSON
document:

```json
{
  "read_at": "2026-11-05T14:02:11Z",
  "windows": [
    { "name": "5-hour", "used": 42, "resets_at": "2026-11-05T17:00:00Z" },
    { "name": "7-day", "used": 71.5, "resets_at": "2026-11-09T08:00:00Z" }
  ]
}
```

- `read_at` is when your host read the account's allowance, not when it
  wrote the file.
- `windows` gives one entry for each usage window that holds every
  model of the account: for Claude Code, its 5-hour window and its
  7-day window. Leave out a window that holds one model alone, since
  flume holds every start on the account by every window the file
  gives.
- Each window's `name` is yours: status gives it, and flume reads no
  meaning into it. `used` is the percentage of the window used, from 0
  to 100, fractions allowed. `resets_at` is when the window resets, and
  a window may leave it out.
- Each time is RFC 3339's, with its offset.
- flume ignores every other key, so your host may keep its own beside
  these. Put no token or credential in the file.

Something of your host's writes the file, and flume reads it and never
writes it. flume keeps no copy of it, fetches nothing to make it, asks
no agent program or its maker what an account has left, and reads no
account's sign-in. How your host learns an account's use is yours to
choose: choose a way that leaves each account's sign-in as it is.

Write the file whole: a new file in the same folder, renamed over the
old, so flume never reads half of one. Write it at least every five
minutes while your host can read the account. A reading is **fresh**
for fifteen minutes from its `read_at`, by the host's clock, so one
missed write costs nothing. Keep the file outside flume's own folders
and outside the account's configuration home. Two pools' entries for
one account may name one file.

flume reads each account's file as it stands at each choice of an
account and whenever status is asked. A window whose `resets_at` has
passed reads as nothing used. A file that is missing, or that is not
this form, is no reading: status says why, and no start is refused and
no settle fails for it. Where your host can no longer read an account,
write nothing new for it, or remove its file. Its reading then goes
stale, or there is none.

What an account has **left** is the least any of its windows leaves:
40% of its 5-hour window used and 80% of its 7-day window leave it 20.
Take a `most-left` pool over work-1 with that reading, work-2 whose
reading gives 60% and 30%, and work-3, which names no reading. A first
place starts on work-2, which has 40 left to work-1's 20, and a second
place of another intent starts there too while the reading stands.
Once work-2's reading is twenty minutes old with none newer, the next
place starts on work-1, the one account with a fresh reading. Once
work-1 is spent, the next starts on work-3, which runs fewer of flume's
agents than work-2. A pool with no fresh reading anywhere chooses as
`least-busy` does.

A reading moves only when your host writes it, so every start between
two readings goes to the account with most left, up to its own
`agents`. An intent keeps its agents on one account in any case
([affinity](#an-intent-keeps-an-account-per-program)), and the
threshold stops new work on an account once a reading shows it near
its limit.

### A threshold

A pool's `threshold` is a whole percentage of a window, 90 where the
pool states none:

```toml
[pools.work]
strategy = "fill-first"
threshold = 85
```

An account whose reading gives any of its windows used to the threshold
or past it is **past its threshold**. It takes no new start, under any
strategy or by the intent's affinity, until that window resets. So
under `fill-first` and `least-busy` too, an account near its limit
takes no new work wherever it gives a reading. A window's use only
grows until it resets, so an account stays past its threshold until the
window's `resets_at`, however old the reading. A window that names no
reset holds the account only while the reading is fresh.

Its agents running work on where they are: flume ends and moves none of
them for the threshold. One whose turn stops at the account's usage
limit moves as a spent account's agent does ([a spent
account](#a-spent-account)). An intent whose account is past its
threshold moves to another account of the program, never back, and
status says which window held the account and until when.

A reading neither shortens nor lengthens a spent account's pause. An
account spent until 15:00 stays spent until then, though a reading at
14:00 gives it 10% used, and flume records no account spent from a
reading. Each holds the account on its own, so it takes a start again
only once its pause has passed and no window holds it past its
threshold. A threshold leaves the last of each window unspent: set it
higher, up to 100, to spend more of each before new work moves on.

### A pool that cannot run a stage

A stage waits for room only while its pool holds an account of its
program that is spent, past its threshold or at its bound. Only then can
a reset or an agent's end make room for it. A pool that holds no
account of the program the stage's layers resolve refuses the stage,
naming the stage, the pool and the program:

```text
the stage code's agent runs claude, and the pool personal holds no account of claude, so no room it gives could start it (145)
```

flume reads the pool at the launch, from the host's configuration as it
stands then, and refuses the launch with nothing made. It reads the pool
again at each later start. So where you take a pool's last account of a
program away after the launch, the stage of that program is refused
when it comes to start. A stage already waiting for room is refused at
the next settle, whatever room the host has. A stage so refused
starts no agent, and its place waits for your retry: put an account of
the program back in the pool, then run `flume retry <place>`.

## The environment

An agent's environment is your login environment with its account's
`env` over it, and nothing else gives one: not the launch, not the
host's agents tables, not the repository or the kind. A launch from a
session signed in to another account changes nothing, since the agent's
environment never comes from the launching session.

In an account's `env`, a leading `~` is your home and `$VAR` or
`${VAR}` is a variable of your login environment, read at each start. A
variable your login environment lacks refuses that start, naming it and
the account. Status names each variable and never shows its value.

The agent starts with:

- `HOME`, `USER`, `LOGNAME`, `SHELL`, `TMPDIR` and `LANG` as they stand
  for your user, and the `PATH` your login shell sets;
- its account's `env`, where it runs under an account;
- flume's own `FLUME_AGENT`, `FLUME_SECRET`, `FLUME_PLACE` and
  `FLUME_STAGE` ([the Claude Code plugin](/flume/guides/claude-code/)).

A pane's shell reads no startup files. Text beyond ASCII in a prompt
reaches the agent whole only when that environment sets a UTF-8 locale,
such as `LANG=en_US.UTF-8`.

## A spent account

When an agent's turn fails at its account's usage limit, flume records
the account **spent** until the reset the limit's words name, or for an
hour where they name none. It reads Claude Code's own words, as the
Claude Code release flume pins writes them. A reset within a day is a
time of day, as in `You've hit your session limit · resets 5pm
(America/New_York)`. A later one is a date and a time, as in `You've hit
your weekly limit · resets Nov 8 at 12am (America/New_York)`, with the
year after the day where it is not this year's. A limit on a budget
names its plan limit's reset, `your weekly limit resets Nov 8 at 12am
(America/New_York)`. Each is read in the zone the words name. The record
is a document of the host's own, so every flume process on the host
skips the account until then.

Where the words name a reset in a form flume does not read, the account
is spent for an hour, as where they name none, and status gives the
words beside the account's spent time, so a person can see what flume
could not read.

Once the spent agent is quiet, flume ends it and starts its stage again
on the account its pool chooses, or the stage waits for one. No
caller's word is needed, and the start counts as no send-back. Other
agents on the spent account work on until their own turn fails. flume
asks no one whether a limit has reset: the next settle after the reset
finds the account usable.

### The conversation carried

The start after a spent agent goes on with that agent's own
conversation where the agent program can carry it to the account
chosen. Claude Code carries it when the chosen account's configuration
home reaches the transcript the spent agent wrote ([the Claude Code
plugin](/flume/guides/claude-code/#a-conversation-carried)). flume then
starts the agent in that session, and its prompt is flume's words alone,
with the way to stop short:

```text
Your last turn stopped at your account's usage limit. This session now goes on under an account of the pool, with its conversation as it stood: carry on from where you stopped.
```

The carried start is a start like any other, with its own identity,
secret and record, and its record names the session it resumes. flume
takes its agent as the stage's only once the agent says it started in
that session ([the start check](#the-start-check)). A pool of one
account carries its agent's conversation across the account's own
reset, since its home holds the transcript.

Where the conversation cannot go to the account chosen, the start is
afresh. The fresh agent's prompt says, in flume's own words, that the
earlier agent stopped at its account's usage limit and that what it
committed and left is in the place. The agent reads only that.

Only the start after a spent agent carries a conversation. A retry, a
fix round, a later stage and the start after an agent a caller ended
start afresh.

A carried start may not begin: its agent stops before it says it
started, says nothing by the stage's limit, never receives its prompt,
or comes up in another session. flume then sends it nothing more, ends
it once it is quiet, and starts the stage afresh at once, with no
caller's word and no send-back counted.

### Sharing one projects folder

flume copies, links and writes nothing in an account's configuration
home. Whether a conversation can go from one account to another is
yours to set up, once, with a link. Claude Code keeps each session's
transcript under its configuration home's `projects` folder, so
accounts that share one `projects` folder reach each other's
conversations. To let `work-2` carry on what `work-1` began, move any
`projects` folder of `work-2`'s aside, then link it to `work-1`'s:

```sh
mv ~/.claude-work-2/projects ~/.claude-work-2/projects.before
ln -s ~/.claude-work-1/projects ~/.claude-work-2/projects
```

Link each further account of the pool the same way. The link carries
both ways: an agent that began under `work-2` carries on under `work-1`
as well.

The link shares everything Claude Code keeps under `projects`: every
session's transcript under those accounts, your own sessions among them,
and what it keeps of each project there, its memory of each project
among it. Accounts you keep apart for a reason stay apart only while
their `projects` folders do. A pool whose homes share nothing works as
before: each spent agent's stage starts afresh, and status says why.

## An intent keeps an account per program

An **intent** is the work a place serves: the name `flume launch
--intent <name>` gives, else the place's repository and base. It keeps
one stream, which a later place of the intent takes. For each program
its agents run, it keeps the account the first of them on that program
started on. Its agents on that program keep to that account while it is
usable, so they read what they share from its prompt cache. An intent
whose stages run two programs keeps one account for each.

A start takes the intent's account for its stage's program. It leaves
every other program's account as it was, so a start on one program is
no move of the intent, even while another program's account is spent or
at its bound. The account for a program moves to another account of
that program when it is spent, past its threshold or at its bound, and
it never moves back to an account it left. flume drops the intent an
hour after its last place is gone. Status lists each move with the
program whose account moved and its reason, and says that the agents
after it start without the earlier account's cache.

An intent an earlier release recorded holds one account. flume reads it
as the account for the program the host's configuration gives that
account, and keeps the stream alone where the pool no longer declares
the account.

## The start check

At each start flume records what the agent is to run under: its
program, its account, and a digest of each variable of its account's
environment and of `FLUME_AGENT`, `FLUME_PLACE` and `FLUME_STAGE`.
Where a layer gives a mode, the record holds it as passed. The
start's secret keys each digest, so the record holds no value and
tests no guess. `PATH`, `SHELL` and the rest of the login environment
are not in it. Nor is a credential the agent program keeps from the
programs it starts, its hooks among them: Claude Code keeps
`CLAUDE_CODE_OAUTH_TOKEN` and its other tokens to itself, so an
account's token reaches the agent and is never compared.

The agent's SessionStart hook compares its own program and environment
with the record before it says the agent started, and, where the record
names a session the start resumes, the session it runs in. A pane
brought back without its environment, a program started again by hand,
another `claude` found earlier on `PATH`, or a carried start that came
up in another conversation then differs. The hook acks nothing,
records what differed by name, and starts a settle. flume takes that
agent as no stage's agent: it sends it no prompt, ends it once it is
quiet, and starts the stage afresh under its settings. A second
difference in a row, or an agent still not quiet at the stage's limit,
ends the stage **never begun**, naming the variables.

Claude Code names the mode a session runs in only with a prompt, so its
mode is compared at the start's first prompt. The plugin's
UserPromptSubmit hook compares the session's mode with the one its start
passed, Claude Code's `default` read as `manual`. Where the account's
configuration disables the mode asked, Claude Code begins in another,
and the hook then blocks the prompt, so the agent takes no turn, and
records both modes. flume treats that start as it treats a variable
that differed: it ends the agent once quiet and starts the stage
afresh. A second start in another mode ends the stage never begun,
naming the mode asked and the mode it began in. Once a start's first
prompt is taken, its session's mode is its own. A later prompt, and a
session resumed or compacted, compare none, so a mode you move in the
pane stops nothing. Where a program's profile cannot read its mode
back, the start records the mode passed and compares none, and status
says the mode was not read back.

Status lists each such start with each variable's name, whether the
program differed and the modes asked and begun, never a value.

## What status shows

```text
  stream work (from host-repository), label construction (from host-kind), intent of /home/you/src/acme on main
  agent flume:f5f5e14159a8:fix--the-parser-reads-tabs:code:1 (claude) on the account work-1 of work (from the strategy): working, in the mode edits (from host-kind) passed as acceptEdits
    in the session work, the stream's name
    its terminal herdr, the host's
    env.CLAUDE_CONFIG_DIR = <set> (from account)
    mode = edits (from host-kind)
    model = sonnet (from host-kind)
    program = claude (from default)
```

A place gives its stream and its label, each with where it came from,
and its intent. Its current agent gives the session and the
multiplexer it opened in, each with where it came from. An agent that
opened in a session its stream names for its kind or its stage says
which, as `in the session work-design (stream work, kind research)` or
`(stream work, stage release.review)`. Its
current agent gives its account, with `from` saying whether the
intent's affinity for the agent's program or the pool's strategy chose
it, and each variable `env.<NAME>` from `account` as `<set>`, and its
`mode` with the value passed and whether it was read back. The place
lists its `moves` and each start in `unsettled`.

A change whose code started on work-2 once the intent's account for
`claude`, work-1, was spent runs its verify on work-2 from the
affinity:

```text
  agent flume:f5f5e14159a8:change--csv-export:verify:1 (claude) on the account work-2 of work (from the affinity for claude): working
    env.CLAUDE_CONFIG_DIR = <set> (from account)
    program = claude (from default)
  moved: the intent moved for claude from work-1 to work-2 at 1793291334, work-1 spent, its agents after starting without work-1's cache
```

A stage its pool cannot run reads `refused` on its line, with the
refusal's words after `refused:` ([a pool that cannot run a
stage](#a-pool-that-cannot-run-a-stage)).

The agent of a start after a spent agent says whether it carries that
agent's conversation, and the stage's move says whether the
conversation went with it. Where it stayed behind, the agent's line
says why, naming the variable and never a path:

```text
  agent flume:f5f5e14159a8:fix--the-parser-reads-tabs:code:2 (claude) on the account work-2 of work (from the strategy): working
    env.CLAUDE_CONFIG_DIR = <set> (from account)
    program = claude (from default)
    starts afresh after start 1: the chosen account's configuration home (`CLAUDE_CONFIG_DIR`) does not reach its conversation: its `projects` folder is not the stopped account's
  moved: code moved from work-1 to work-2 at 1793291334, work-1 spent, its conversation stayed behind, its agents after starting without work-1's cache
```

Where it went, the agent's line reads `carries the conversation of
start 1` and the move `its conversation with it`. A carried start that
did not begin is listed with what happened, `carried and not begun:
code's start 2 at 1793291340: the start that carried its conversation
ended before it said it started, and its stage started afresh`.

The host gives its `streams` and its `pools`. Each stream gives its
running agents, bound, room, queue, session, multiplexer, view,
terminal app, title and labels, and each session it names for its
kinds and stages, with the kinds and the `<kind>.<stage>`s it names it
for and the layer each came from. Each pool gives its strategy and
threshold, and each account with its agents running, bound, spent time,
reading, and what holds it past its threshold ([a pool's
accounts](#a-pools-accounts-and-their-readings)). A stream gives each
label's reserve, what it keeps free, its agents running and its stages
waiting, and a stage waiting gives its label. A stream whose labels or
multiplexer refuse its starts says `it starts no stage:` with why, and
one whose terminal app names no profile says `it opens no view:`, its
stages starting as before. A table for a kind or a stage that refuses
the starts that read it says `it starts no stage that reads <table>:`
with the refusal ([a stream's sessions](#a-streams-sessions)). Then comes the
local layer, with each value it set and each key passed over with its
pin, or, where it was not taken, each key it may not set or its fault:

```text
  stream work, priority 30: 5 agents running of 5, room for 0
    its agents open in the session work, the stream's name
    it names the session work-design for the kind research (from managed)
    it names the session work-review for the stage release.review (from local)
    multiplexer herdr (the host's), view terminal (the default), terminal app ghostty (the host's), title "Work Agents" (the stream's title)
    label construction: a reserve of 2, keeping 0 free, 4 agents running, 0 waiting
    label operation: a reserve of 2, keeping 1 free, 1 agents running, 1 waiting
    waiting for room: /home/you/.local/share/flume/places/f5f5e14159a8/deploy--rollout's code in work as operation, priority 0, short of the stream work's room, at its bound of 5
  the host's local layer is taken, setting:
    streams.work.kinds.release.stages.review.session = "work-review"
    streams.work.priority = 30
    streams.work.agents passed over: the managed layer pins streams.work.agents
```

### A pool's accounts and their readings

A pool gives its strategy and its threshold, then each account with its
reading under it. A reading gives each window's use and reset, then
when the host read it, `stale` where it is no longer fresh. An account
whose reading cannot be read says `no reading:` and why. An account past its threshold says which window
holds it and until when. Status never gives a reading's path: the
configuration's values give an account's `usage` as `<set>`, as they
give its variables.

```text
  pool work, most-left, threshold 90%:
    work-1 (claude): 2 agents running of 4
      5-hour 93% (resets 1793304000) · 7-day 55% (resets 1793520000) · read at 1793291300
      past its threshold in 5-hour until 1793304000
    work-2 (claude): 1 agents running, no bound of its own
      5-hour 40% (resets 1793304000) · 7-day 35% (resets 1793520000) · read at 1793291280
    work-3 (claude): 0 agents running, no bound of its own
      no reading: the file is missing
```

A move off an account past its threshold names the window and its
reset:

```text
  moved: the intent moved for claude from work-1 to work-2 at 1793291334, work-1 past its threshold in 5-hour until 1793304000, its agents after starting without work-1's cache
```

A stage waiting for an account of its pool names the soonest time an
account of the pool is back, neither spent nor past its threshold:
`short of a usable account of the pool work, the soonest back at
1793304000`.

The [state reference](/flume/reference/state/) lists the JSON fields.
