---
title: Configuration
description: The host's configuration, the repository's .config/flume.toml, the kind format, and the variables flume reads.
sidebar:
  order: 2
---

flume reads three files: the host's configuration, the repository's
`.config/flume.toml`, and a launch's own declaration of kinds when it
names one. Every part of each is optional.

## The host's configuration

`$XDG_CONFIG_HOME/flume/config.toml`, `~/.config/flume/config.toml`
by default. flume reads it at each command, and its agents' settings at
each agent's start. A file that is not there is an empty one; a file
flume cannot read makes every command refuse, naming its fault.

```toml
[host]
agents = 8
terminal = "herdr"
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

[kinds.change.agents]
model = "opus"

[kinds.change.agents.stages.verify]
instructions = "Check every scenario of the change's specs."

[repos."~/src/notes"]
stream = "personal"

[repos."~/src/acme".kinds.fix.agents]
instructions = "Run the tests with `just test`."

[roles."acme/planner"]
repository = "~/src/acme"
kind = "planner"
priority = 5
```

### `[host]`

| Key | Values | Meaning |
|---|---|---|
| `agents` | a whole number from 0 | The bound on agents running at once on the host, every caller's and every stream's counted together. A stage starts only while the host is under it and otherwise waits for room. With none, and no stream's bound, every stage starts at once and the host has no queue. |
| `terminal` | `"herdr"`, `"printed"` | Where agents run: in herdr panes, or as a command flume prints for you to run. `"printed"` where none is given. Any other name makes every command refuse, naming the terminals there are. |
| `pool` | a pool's name | The pool of a place whose stream names none. With none, such a place's agents run under no account. |

### `[streams.<stream>]`

A stream the host runs places in
([streams](/flume/guides/agent-settings/#streams)). The stream
`default` always stands, holding every place nothing names a stream
for; a `[streams.default]` table gives it keys of its own.

| Key | Values | Meaning | Left out |
|---|---|---|---|
| `agents` | a whole number above 0 | the bound on the stream's agents running at once, within the host's | the host's bound alone |
| `priority` | a whole number, negative allowed | the stream's priority: waiting stages start by it first, then by their place's | `0` |
| `pool` | a pool's name | the pool its places' agents take accounts from | `[host] pool` |

### `[pools.<pool>]`

A pool of accounts
([pools of accounts](/flume/guides/agent-settings/#pools-of-accounts)).

| Key | Values | Meaning |
|---|---|---|
| `strategy` | `"fill-first"`, `"least-busy"` | how an account is chosen: the first usable one in order, or the usable one running the fewest of flume's agents, equals in order. Required. |
| `accounts` | a list of tables | the pool's accounts, in order, at least one |

Each account:

| Key | Values | Meaning | Left out |
|---|---|---|---|
| `name` | a string | the account's name, unique in its pool | refused |
| `program` | a string | the program it runs, such as `claude`; it takes only stages of that program | refused |
| `env` | a table of strings | its agents' environment, over the login environment | no variable |
| `agents` | a whole number above 0 | the bound on agents running under it at once | no bound of its own |

In `env`, a leading `~` is your home and `$VAR` or `${VAR}` a variable
of your login environment, read at each start. A variable the login
environment lacks refuses that start, naming it and the account.

### `stream`

`stream` in `[repos."<repository>".kinds.<kind>]`, in
`[repos."<repository>"]` and in `[kinds.<kind>]` names the stream of a
place of that repository and kind, of that repository, or of that kind,
in that order, where the launch and the intent name none.

### `[kinds.<kind>.agents]`

Settings for a kind's agents in every repository: `model`,
`instructions` and `mode`, with `[kinds.<kind>.agents.stages.<stage>]`
for one stage's. A stage's own setting comes before one for every
stage. `model` and `instructions` are strings. `mode` is one of `plan`,
`ask`, `edits` and `full`, or a table of programs whose values are
strings, as `mode = { claude = "auto" }`. Any other key is not read.
[Agent settings](/flume/guides/agent-settings/) gives the layers and
[the mode](/flume/guides/agent-settings/#the-mode) what each mode
means.

### `[repos."<repository>".kinds.<kind>.agents]`

The same, for a kind in one repository, named by its location; a
leading `~` in the key is your home. It comes before the host's
settings for the kind in every repository.

### What refuses

flume refuses the file, naming the key, for:

- a `strategy` other than `fill-first` and `least-busy`;
- a stream's `pool` or `[host] pool` naming a pool no `[pools]` table
  declares;
- a pool with no accounts, or two accounts of one name;
- an account with no `name` or no `program`;
- an `agents` of a stream or an account that is not a whole number
  above zero;
- a `mode` in an agents table that is neither one of `plan`, `ask`,
  `edits` and `full` nor a table of programs whose values are strings,
  naming the table and `mode`;
- a key a stream, a pool or an account does not know.

A file that refuses leaves every launch and every start refused,
naming its fault, until it is mended.

### `[roles."<group>/<role>"]`

The run the host launches for a role's address
([roles and wake](/flume/guides/roles-and-wake/)):

| Key | Meaning | Left out |
|---|---|---|
| `repository` | the repository of the run's place; a leading `~` is your home | refused |
| `kind` | the kind the run launches | refused |
| `priority` | a whole number, the run's priority in the host's queue | `0` |
| `base` | the base the run's place is cut from | the repository's default branch |
| `brief` | the run's brief | none |
| `stream` | the stream the run's place runs in | as for any launch |

A key that is no `<group>/<role>` address, or a key the table does not
know, makes the file unreadable.

## The repository's `.config/flume.toml`

flume reads it from git at a revision, never from the working copy. It
reads the `[profiles]` table at the repository's default branch, after
one fetch of `origin` where the repository has it, and the kinds at the
base's current revision for each launch. A place keeps what its launch read.
flume never writes it.

```toml
[profiles]
place = "git-worktree"
change = "openspec"
merge_queue = "sluice"

[kinds.docs]
branch = "docs"
lands = true

[[kinds.docs.stages]]
name = "write"
prompt = "prompts/docs.md"
deliverables = ["commits"]
agents = [{ model = "sonnet" }]
on_fail = "write"

[kinds.change.agents.stages.verify]
model = "opus"
```

### `[profiles]`

Binds the repository's tools, one per seam. flume reads this table and
nothing else to choose them, and never guesses a tool from another
tool's files.

| Key | Values | Meaning |
|---|---|---|
| `place` | `"git-worktree"` | How places are made: plain `git worktree`. The one place profile, and the one bound where none is named. |
| `change` | `"openspec"` | The change tool a change's stages read: OpenSpec, its change at `openspec/changes/<name>`. Needed by a kind with a `change`, `tasks` or `archived` deliverable. |
| `merge_queue` | `"sluice"` | The merge queue a landing kind's work lands through. Needed by a kind that lands. |

A name flume holds no profile for refuses the launch, naming it.

### `[kinds]`

The repository's own kinds, in the kind format below. A kind named
as one of flume's own stands over it in this repository. A
`[kinds.<kind>]` table that holds only `agents` gives settings to a
kind declared elsewhere, flume's own among them, and declares no kind.
An `env` in any agents table refuses the launch, naming the field,
since an agent's environment is its account's alone.

## A launch's declaration

`flume launch --kinds <file>` names a file in the same format, read
from disk at the launch with its prompts beside it. Its kinds stand
over the repository's and flume's own for that launch alone. Its
`[profiles]` table, if any, is not read.

## The kind format

Each kind is a table under `[kinds]`, its stages an array of tables in
order. flume ignores every table and key it does not know, so a file
can carry another tool's sections, and refuses a declaration whose
known fields fail, naming every field that does.

### A kind

| Field | Values | Meaning | Left out |
|---|---|---|---|
| `version` | a whole number from 1 | its version, shown as `<kind>@<version>` | `1` |
| `branch` | a non-empty string | the prefix of its places' branches | the kind's name |
| `starts_unready` | `true`, `false` | its first stage starts in a place whose readiness failed | `false` |
| `lands` | `true`, `false` | its work is submitted to the merge queue after the last stage | `false` |
| `agents` | a table | settings for every stage's agents, and `agents.stages.<stage>` for one stage's: `program`, `model`, `instructions`, `beside`, `mode` | none |
| `stages` | an array of tables | its stages, at least one | refused |

### A stage

| Field | Values | Meaning | Left out |
|---|---|---|---|
| `name` | a non-empty string | unique in its kind | refused |
| `prompt` | a path | its prompt, relative to the declaring file's folder; for a repository, read from git at the same revision | refused |
| `deliverables` | a list of one spelling | what ends it: `commits`, `change`, `tasks`, `archived`, `report:<path>` or `committed:<path>` | refused, unless a conversation |
| `conversation` | `true`, `false` | a stage with no deliverable, ended by `flume finish` | `false` |
| `agents` | a list of one table | its agent's settings: `program`, `model`, `instructions`, `beside`, `mode`; an `env` refuses the launch | refused by flume, which runs one agent per stage |
| `makes` | a list of names | what it makes besides its deliverable, each recorded as a line of `made.jsonl` | none |
| `wait` | `true`, `false` | the stage is held before it starts until a retry names it | `false` |
| `quiet` | a whole number then `s`, `m` or `h`, as `"45m"` | how long its agent may be quiet with nothing delivered before it is stuck | 30 minutes |
| `on_fail` | a stage's name | read from the last stage: where a failed or conflicting submission, or found work, sends the place back | the last stage |
| `max_send_backs` | a whole number from 0 | read from the last stage: how many times the place may go back before it waits for you | 3 |

[Kinds](/flume/guides/kinds/) says what each deliverable needs and how
a kind runs through, and [prompt parts](/flume/reference/prompt-parts/)
lists what a prompt may ask for.

## Environment

| Variable | Read by | Meaning |
|---|---|---|
| `XDG_CONFIG_HOME` | every command | where the host's configuration lies, `~/.config` where unset |
| `XDG_STATE_HOME` | every command | where flume keeps its state, `~/.local/state` where unset |
| `XDG_DATA_HOME` | every command that starts an agent | where flume writes its Claude Code plugin, `~/.local/share` where unset |
| `FLUME_RECIPIENT` | `launch`, `wait`, `inbox` | the recipient where a command names none |
| `FLUME_AGENT`, `FLUME_SECRET` | `needs`, `ack`, `hook` | the identity and secret of the agent's start, set by flume on each agent |
| `FLUME_PLACE`, `FLUME_STAGE` | `hook`, `wait`, `inbox` | the agent's place and stage; a command run in a role's run takes its role from `FLUME_PLACE` |
| `HERDR_*` | the herdr terminal | the pane a command runs in, read as its session; stripped from every herdr call flume makes and from `sluice submit` |
