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
agents = 4
terminal = "herdr"

[kinds.change.agents]
model = "sonnet"
env = { CLAUDE_CONFIG_DIR = "~/.claude-work" }

[kinds.change.agents.stages.verify]
model = "opus"

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
| `agents` | a whole number from 0 | The bound on agents running at once on the host, every caller's counted together. A stage starts only while the host is under it and otherwise waits for room. With none, every stage starts at once and the host has no queue. |
| `terminal` | `"herdr"`, `"printed"` | Where agents run: in herdr panes, or as a command flume prints for you to run. `"printed"` where none is given. Any other name makes every command refuse, naming the terminals there are. |

### `[kinds.<kind>.agents]`

Settings for a kind's agents in every repository: `program`, `model`,
`instructions` and `env`, with `[kinds.<kind>.agents.stages.<stage>]`
for one stage's. A stage's own setting comes before one for every
stage. Each value is a string, and `env` a table of strings. In `env`,
a leading `~` is your home and `$VAR` or `${VAR}` a variable of your
login environment. [Agent settings](/flume/guides/agent-settings/)
gives the layers.

### `[repos."<repository>".kinds.<kind>.agents]`

The same, for a kind in one repository, named by its location; a
leading `~` in the key is your home. It comes before the host's
settings for the kind in every repository.

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
A repository's file gives no `env`: a launch whose repository sets one
for its kind is refused.

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
| `agents` | a table | settings for every stage's agents, and `agents.stages.<stage>` for one stage's | none |
| `stages` | an array of tables | its stages, at least one | refused |

### A stage

| Field | Values | Meaning | Left out |
|---|---|---|---|
| `name` | a non-empty string | unique in its kind | refused |
| `prompt` | a path | its prompt, relative to the declaring file's folder; for a repository, read from git at the same revision | refused |
| `deliverables` | a list of one spelling | what ends it: `commits`, `change`, `tasks`, `archived`, `report:<path>` or `committed:<path>` | refused, unless a conversation |
| `conversation` | `true`, `false` | a stage with no deliverable, ended by `flume finish` | `false` |
| `agents` | a list of one table | its agent's settings: `program`, `model`, `instructions`, `env`, `beside` | refused by flume, which runs one agent per stage |
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
