---
title: Configuration
description: The host's configuration, the repository's .config/flume.toml, the kind format, and the variables flume reads.
sidebar:
  order: 2
---

flume reads the host's configuration, the repository's
`.config/flume.toml`, and a launch's own declaration of kinds when it
names one. Every part of each is optional.

## The host's configuration

Two files in `$XDG_CONFIG_HOME/flume/`, `~/.config/flume/` by default,
read as one at each command, and its agents' settings at each agent's
start ([two layers](/flume/guides/agent-settings/#two-layers)):

- `config.toml`, the managed layer, written by whoever manages the
  host. A file that is not there is an empty one. A file flume cannot
  read makes every command refuse, naming its fault.
- `config.local.toml`, the local layer, read over the managed one key
  by key. It sets only a stream's `agents`, `priority`, `session` and
  `multiplexer`, a label's `reserve` and `keep_free`, the `stream` and
  `label` of a repository's kind, a repository or a kind, and the
  kinds' agents tables. A local file that sets any other key, or that
  flume cannot read, is not taken at all, and status names each key or
  the fault.

flume writes neither file.

```toml
[host]
agents = 8
multiplexer = "herdr"
terminal_app = "ghostty"
pool = "work"

[streams.work]
title = "Work Agents"

[streams.personal]
agents = 3
pool = "personal"
terminal_app = "terminal"
title = "Personal Agents"

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
| `multiplexer` | `"herdr"`, `"printed"` | The multiplexer a stream's agents run in where the stream names none: herdr panes, or a command flume prints for you to run. `"printed"` where none is given. Any other name makes every command refuse, naming the terminal profiles there are. |
| `terminal` | as `multiplexer` | The word written before `multiplexer`, read as it. A `[host]` giving both is refused. |
| `view` | `"terminal"`, `"app"` | How `flume open` shows a stream's agents where the stream names none: in a window of a terminal app, or in the multiplexer's own app. `"terminal"` where none is given. |
| `terminal_app` | `"ghostty"`, `"terminal"`, `"printed"` | The program that opens a stream's window where the stream names none: Ghostty, Terminal.app on macOS, or none, `flume open` printing the line to run. `"printed"` where none is given. Any other name refuses each stream's `flume open` that takes it, naming the window profiles there are; every other command runs. |
| `pool` | a pool's name | The pool of a place whose stream names none. With none, such a place's agents run under no account. |
| `pin` | a list of dotted keys, such as `"streams.work.agents"` | The keys the local layer may not set over the managed one. Each names a key the local layer could set, or a table of such keys, as `"streams.work"` does. A local key a pin holds is passed over, status naming it with its pin, and the rest of the local file is taken. |

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
| `session` | a string herdr takes as a session's name | the herdr session its agents open in ([a stream's session](/flume/guides/agent-settings/#a-streams-session)) | the stream's own name, and flume's own session for `default` |
| `multiplexer` | `"herdr"`, `"printed"` | the multiplexer its agents run in, read at each start; an agent stays in the one it opened in ([a stream's multiplexer](/flume/guides/agent-settings/#a-streams-multiplexer-and-view)) | `[host] multiplexer` |
| `view` | `"terminal"`, `"app"` | how `flume open` shows its agents | `[host] view` |
| `terminal_app` | `"ghostty"`, `"terminal"`, `"printed"` | the program that opens its window | `[host] terminal_app` |
| `title` | a string | its window's title, which a window manager can file the window by | the stream's name |

The local layer may set a stream's `agents`, `priority`, `session` and
`multiplexer`, so you can try a multiplexer on one stream of your host.
Its `pool`, `view`, `terminal_app` and `title` are the managed layer's.
A stream whose `multiplexer` names no terminal profile flume carries
refuses each launch and each start on it, naming the stream, the name
and the profiles there are, and every other stream runs. One whose
`terminal_app` names no window profile refuses its `flume open` alone.

### `[streams.<stream>.labels.<label>]`

A label's share of the stream's agents
([labels](/flume/guides/agent-settings/#labels)). The stream holds it
only where it states `agents`.

| Key | Values | Meaning | Left out |
|---|---|---|---|
| `reserve` | a whole number from 0 | the stream's agents owed to the label: while it runs fewer, its stages start before the stream's others | `0` |
| `keep_free` | a whole number from 0, no more than `reserve` | the stream's slots kept free for the label while it runs fewer agents than that | `0` |

### `[pools.<pool>]`

A pool of accounts
([pools of accounts](/flume/guides/agent-settings/#pools-of-accounts)).

| Key | Values | Meaning |
|---|---|---|
| `strategy` | `"fill-first"`, `"least-busy"`, `"most-left"` | how an account is chosen. `fill-first`: the first usable one in order. `least-busy`: the usable one running the fewest of flume's agents, equals in order. `most-left`: the usable one with the most of its allowance left by its reading ([an account's reading](/flume/guides/agent-settings/#an-accounts-reading)). Required. |
| `threshold` | a whole number from 1 to 100 | the percentage of any window of an account's reading at which the account takes no new start until that window resets, under every strategy ([a threshold](/flume/guides/agent-settings/#a-threshold)). `90` where left out. |
| `accounts` | a list of tables | the pool's accounts, in order, at least one |

Each account:

| Key | Values | Meaning | Left out |
|---|---|---|---|
| `name` | a string | the account's name, unique in its pool | refused |
| `program` | a string | the program it runs, such as `claude`; it takes only stages of that program, and a pool with no account of a stage's program refuses that stage | refused |
| `env` | a table of strings | its agents' environment, over the login environment | no variable |
| `agents` | a whole number above 0 | the bound on agents running under it at once | no bound of its own |
| `usage` | a path | the file of its reading, written by something of the host's and read by flume, never written | no reading |

In `env`, a leading `~` is your home and `$VAR` or `${VAR}` a variable
of your login environment, read at each start. A variable the login
environment lacks refuses that start, naming it and the account.

`usage` reads `~` and `$VAR` the same way, at each choice of an account
and whenever status is asked. A variable the login environment lacks
leaves the account with no reading, and status names the variable; it
refuses no start. A file that is missing or not in the reading's form
is no reading either, and status says why. Status gives `usage` as
`<set>`, never its path.

### `stream`

`stream` in `[repos."<repository>".kinds.<kind>]`, in
`[repos."<repository>"]` and in `[kinds.<kind>]` names the stream of a
place of that repository and kind, of that repository, or of that kind,
in that order, where the launch and the intent name none.

### `label`

`label` in `[repos."<repository>".kinds.<kind>]` and in
`[kinds.<kind>]` names the label of a place of that repository and
kind, or of that kind, in that order. It is one word of letters,
digits, `-` and `_`. A place's label is read at each start.

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

- a `strategy` other than `fill-first`, `least-busy` and `most-left`;
- a `threshold` that is not a whole number from 1 to 100;
- a `usage` that is not a string, or is empty;
- a stream's `pool` or `[host] pool` naming a pool no `[pools]` table
  declares;
- a pool with no accounts, or two accounts of one name;
- an account with no `name` or no `program`;
- an `agents` of a stream or an account that is not a whole number
  above zero;
- a `mode` in an agents table that is neither one of `plan`, `ask`,
  `edits` and `full` nor a table of programs whose values are strings,
  naming the table and `mode`;
- a key a stream, a pool or an account does not know;
- a `label` that is not one word of letters, digits, `-` and `_`;
- a `pin` that names no key the local layer could set and no table of
  such keys;
- a `[host]` giving both `terminal` and `multiplexer`;
- a `view` other than `terminal` and `app`;
- a `multiplexer`, `terminal_app` or `title` that is not a string.

A managed file that refuses leaves every launch and every start
refused, naming its fault, until it is mended. A local file that would
refuse, or that sets a key it may not set, is not taken: every command
runs on the managed layer alone, and status says why.

A file flume reads can still refuse a stage. A pool that holds no
account of the program a stage's layers resolve refuses that stage,
naming the stage, the pool and the program. It refuses the stage at its
launch, with nothing made, or at a later start, which reads the file as
it stands then. A
stage so refused waits for a retry
([a pool that cannot run a stage](/flume/guides/agent-settings/#a-pool-that-cannot-run-a-stage)).

A stream's labels can refuse each of its starts, naming the stream and
why. They refuse where their reserves together pass the stream's
`agents`, where a label keeps more free than its reserve, and where a
label's table holds another key, such as a `pool`. A stage so refused
waits for a retry once the file is mended.

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

The repository's default branch is its remote's: the branch
`origin/HEAD` names. Where the repository has `origin` and no
`origin/HEAD`, as a repository made by `git init --bare` and then given
a remote has none, flume asks `origin` which branch its `HEAD` names and
sets `origin/HEAD` to it, as `git clone` does. A repository with no
`origin` has its own `HEAD`'s branch as its default. A `HEAD` naming a
branch that holds no commit names no default branch, so a launch that
names no base, and the binding's read, are refused rather than taking
it.

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
