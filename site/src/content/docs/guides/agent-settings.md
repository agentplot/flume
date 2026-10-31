---
title: Agent settings
description: The settings each agent starts with and the six layers they come from; the mode an agent may act in; the streams a host runs its places in, the pools of accounts its agents run under, and how a start is checked against what it was given.
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
| `instructions` | text appended to the agent's system prompt, passed as `--append-system-prompt` |
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
  { name = "work-1", program = "claude", env = { CLAUDE_CONFIG_DIR = "~/.claude-work-1" }, agents = 4 },
  { name = "work-2", program = "claude", env = { CLAUDE_CONFIG_DIR = "~/.claude-work-2" } },
]

[pools.personal]
strategy = "least-busy"
accounts = [{ name = "personal", program = "claude", env = { CLAUDE_CONFIG_DIR = "~/.claude-personal" } }]

[repos."~/src/acme"]
stream = "work"
```

This host runs acme's places in `work`, up to five of its eight agents,
ahead of `personal`'s. They run on work-1 until it runs four or is
spent, and then on work-2.

### A place's stream

A place's stream is fixed at its launch, from the first of:

| Source | Status names it |
|---|---|
| `flume launch --stream <stream>` | `launch` |
| the stream of the intent's affinity ([affinity](#an-intent-keeps-to-its-account)) | `intent` |
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

## Pools of accounts

A pool, `[pools.<pool>]`, lists accounts in order. Each account is a
`name`, the `program` it runs, an `env` that is its agents'
environment, and an optional `agents`, the bound on agents running
under it. A stage's pool is its stream's `pool`, else the host's
`[host] pool`. With no pool named anywhere, flume chooses no account,
and agents run under the login environment alone.

An account is **usable** for a start while it is unspent and under its
own bound, and runs the stage's program. At each start
flume chooses an account under the host's room lock, so two starts
never take one account's last room:

| `strategy` | The account chosen |
|---|---|
| `fill-first` | the first usable account in the pool's order |
| `least-busy` | the usable account running the fewest of flume's agents, equals in the pool's order |

The intent's affinity comes first while its account is usable. With no
usable account the stage waits for room, and status names the pool and
the soonest time an account of it stops being spent. A stage whose
program no account of the pool runs waits the same way, and status says
so. Sessions you run yourself under an account are not counted:
flume counts its own agents.

Sign each account's configuration home in once, by running
`CLAUDE_CONFIG_DIR=~/.claude-work-1 claude` yourself, since flume
answers no sign-in on an agent's behalf.

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
Claude Code release flume pins writes them, such as `You've hit your
session limit · resets 5pm (America/New_York)`. The record is a
document of the host's own, so every flume process on the host skips
the account until then.

Once the spent agent is quiet, flume ends it and starts its stage
afresh on another usable account, or the stage waits for one. The fresh
agent's prompt says, in flume's own words, that the earlier agent
stopped at its account's usage limit and that what it committed and
left is in the place. No caller's word is needed, and the fresh start
counts as no send-back. Other agents on the spent account work on until
their own turn fails. flume asks no one whether a limit has reset: the
next settle after the reset finds the account usable.

## An intent keeps to its account

An **intent** is the work a place serves: the name `flume launch
--intent <name>` gives, else the place's repository and base. Its
agents keep to one account and one stream while that account is
usable, so they read what they share from that account's prompt cache.
A later place of the intent takes its stream.

The intent moves to another account when its account is spent or at its
bound, and it never moves back to an account it left. flume drops it an
hour after its last place is gone. Status lists each move with its
reason, and says that the agents after it start without the earlier
account's cache.

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
with the record before it says the agent started. A pane brought back
without its environment, a program started again by hand, or another
`claude` found earlier on `PATH` then differs. The hook acks nothing,
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
  stream work (from host-repository), intent of /home/you/src/acme on main
  agent flume:f5f5e14159a8:fix--the-parser-reads-tabs:code:1 (claude) on the account work-1 of work (from the strategy): working, in the mode edits (from host-kind) passed as acceptEdits
    env.CLAUDE_CONFIG_DIR = <set> (from account)
    mode = edits (from host-kind)
    model = sonnet (from host-kind)
    program = claude (from default)
```

A place gives its stream with where it came from, and its intent. Its
current agent gives its account, with `from` saying whether the
intent's affinity or the pool's strategy chose it, and each variable
`env.<NAME>` from `account` as `<set>`, and its `mode` with the value
passed and whether it was read back. The place lists its `moves` and
each start in `unsettled`. The host gives its `streams`, each with its
running agents, bound, room and queue, and its `pools`, each account
with its running agents, its bound and the time it is spent until. The
[state reference](/flume/reference/state/) lists the JSON fields.
