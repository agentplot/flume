---
title: Agent settings
description: The settings each agent starts with, the six layers they come from, how the environment merges, and running a kind's agents under an account of its own.
sidebar:
  order: 6
---

Each agent flume starts runs with five settings, resolved at its start
from six layers. `flume status` gives each setting of the current agent
with the layer it came from.

## The settings

| Setting | What it is |
|---|---|
| `program` | the agent program: `claude`, or a path to Claude Code |
| `model` | the model, passed as `--model` |
| `instructions` | text appended to the agent's system prompt, passed as `--append-system-prompt` |
| `env` | variables set in the agent's environment, one at a time |
| `beside` | a pane the agent's pane opens beside, under herdr |

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
table comes before the kind's `agents` table.

`beside` comes only from the launch and the kind, since where a pane
opens is the caller's to say or the kind's to need.

### The environment merges

`program`, `model`, `instructions` and `beside` are each taken whole
from the first layer that gives them. `env` merges one variable at a
time: each variable comes from the first layer that names it. So the
host can name an agent's configuration home while a launch adds a
variable of its own, and neither replaces the other.

### When each layer is read

The host's layers are read from its configuration as it stands at each
agent's start. Change an account in the host's file and retry a stage,
and the next agent runs under the new account, with no new launch.
Every other layer is what the launch read: a place keeps its launch's
settings, its repository's and its kind's for its whole life.

## At the launch

```sh
flume launch --kind change --name csv-export --brief "Export the ledger as CSV." \
  --agent model=sonnet \
  --agent verify.model=opus \
  --agent env.LOG_LEVEL=debug \
  --agent code.instructions="Run the tests before every commit."
```

The field is one of `program`, `model`, `instructions`, `env.<NAME>`
and `beside`, with an optional stage before it. A setting for a stage
the kind does not hold refuses the launch, naming the stage.

## On the host

The host's configuration, `~/.config/flume/config.toml`, gives
settings to a kind in every repository, and to a kind in one
repository:

```toml
# Every change's agents, in every repository.
[kinds.change.agents]
model = "sonnet"

[kinds.change.agents.stages.verify]
model = "opus"

# Every fix's agents in one repository.
[repos."~/src/acme".kinds.fix.agents]
instructions = "This repository's tests need the database: run them with `just test`."
env = { DATABASE_URL = "postgres://localhost/acme_test" }
```

A repository's key is its location, with a leading `~` read as your
home. Settings for a kind or a stage the place does not run are
ignored.

In the host's `env` values, a leading `~` is your home and `$VAR` or
`${VAR}` is a variable of your login environment. A variable your login
environment lacks refuses the stage's start, naming it. Status shows
each value as written, never expanded.

## In the repository

A repository gives settings in its `.config/flume.toml`, in the kind's
own declaration or, for a kind it does not declare, in a table that
holds only `agents`:

```toml
[kinds.change.agents.stages.verify]
model = "opus"
instructions = "Check every scenario of the change's specs, not only its tasks."
```

A repository's file gives no environment: a launch whose repository
sets `env` for its kind is refused, since what one machine needs is
that machine's to say. `~` and `$VAR` mean nothing in a repository's
values.

## Accounts per kind

Claude Code keeps its sign-in and settings in its configuration home,
`CLAUDE_CONFIG_DIR`. To run one kind's agents under one account and
another kind's under another, give each kind its own home on the host:

```toml
[kinds.change.agents]
env = { CLAUDE_CONFIG_DIR = "~/.claude-work" }

[kinds.fix.agents]
env = { CLAUDE_CONFIG_DIR = "~/.claude-personal" }
```

Sign each home in once, by running `CLAUDE_CONFIG_DIR=~/.claude-work
claude` yourself, since flume answers no sign-in on an agent's behalf.
A launch from a session signed in to a third account changes nothing:
the agent's environment never comes from the launching session.

## What the agent's environment holds

An agent starts with your login environment, never the launching
session's:

- `HOME`, `USER`, `LOGNAME`, `SHELL`, `TMPDIR` and `LANG` as they stand
  for your user, and the `PATH` your login shell sets;
- the resolved `env`;
- flume's own `FLUME_AGENT`, `FLUME_SECRET`, `FLUME_PLACE` and
  `FLUME_STAGE` ([the Claude Code plugin](/flume/guides/claude-code/)).

A pane's shell reads no startup files. Text beyond ASCII in a prompt
reaches the agent whole only when that environment sets a UTF-8 locale,
such as `LANG=en_US.UTF-8`.

## What status shows

```text
  agent flume:f5f5e14159a8:fix--the-parser-reads-tabs:code:1 (claude): unknown
    program = claude (from default)
```

and with `--json`, under the agent's `settings`, each value as written
and a `from` table giving the layer of each setting and of each
variable, such as `"env.CLAUDE_CONFIG_DIR": {"layer": "host-kind", …}`.
