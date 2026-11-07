---
title: Configuration
description: The host's configuration and its sources of kinds and roles, the repository's .config/flume.toml, the locks of kinds, the kind format, and the variables flume reads.
sidebar:
  order: 2
---

flume reads the host's configuration with its sources of kinds, the
repository's `.config/flume.toml`, a launch's own declaration of kinds
when it names one, and the locks of kinds. Every part of each is
optional.

## The host's configuration

Two files in `$XDG_CONFIG_HOME/flume/`, `~/.config/flume/` by default,
read as one at each command, and its agents' settings at each agent's
start ([two layers](/flume/guides/agent-settings/#two-layers)):

- `config.toml`, the managed layer, written by whoever manages the
  host, whole by flume's Nix module on a host that installs flume
  with Nix ([install with Nix](/flume/install/#install-with-nix)). A
  file that is not there is an empty one. A file flume cannot
  read makes every command refuse, naming its fault.
- `config.local.toml`, the local layer, read over the managed one key
  by key. It sets only a stream's `agents`, `priority`, `session` and
  `multiplexer`, the `session` it names for a kind or a kind's stage, a
  label's `reserve` and `keep_free`, the `stream` and `label` of a
  repository's kind, a repository or a kind, and every agents table.
  Each of a program's own settings and each program's `file` is a key
  of its own there. A local file that sets any other key, or that
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

[streams.work.kinds.research]
session = "work-design"

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

[agents.programs.claude]
file = "~/.config/agent-launcher/claude-settings.json"

[agents.streams.personal]
model = "sonnet"
keep_open = "15m"

[kinds.change.agents]
model = "opus"
effort = "high"

[kinds.change.agents.stages.verify]
instructions = "Check every scenario of the change's specs."

# a clone whose origin is on no forge, named by its folder
[repos."~/src/notes"]
stream = "personal"

# a clone whose origin is git@github.com:acme/acme.git, named by its address
[repos."acme/acme"]
stream = "work"

[repos."acme/acme".kinds.fix.agents]
instructions = "Run the tests with `just test`."

[repos."acme/upstream-fork".agents.programs.claude.settings]
attribution = { commit = "", pr = "", sessionUrl = false }
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
| `session` | a string herdr takes as a session's name | the herdr session its agents open in, but those of a kind or a stage it names another for ([a stream's sessions](/flume/guides/agent-settings/#a-streams-sessions)) | the stream's own name, and flume's own session for `default` |
| `multiplexer` | `"herdr"`, `"printed"` | the multiplexer its agents run in, read at each start; an agent stays in the one it opened in ([a stream's multiplexer](/flume/guides/agent-settings/#a-streams-multiplexer-and-view)) | `[host] multiplexer` |
| `view` | `"terminal"`, `"app"` | how `flume open` shows its agents | `[host] view` |
| `terminal_app` | `"ghostty"`, `"terminal"`, `"printed"` | the program that opens its window | `[host] terminal_app` |
| `title` | a string | its window's title, which a window manager can file the window by | the stream's name |

The local layer may set a stream's `agents`, `priority`, `session` and
`multiplexer`, and the `session` of its tables for its kinds and their
stages, so you can try a multiplexer on one stream of your host.
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

### `[streams.<stream>.kinds.<kind>]`

The session the stream opens one kind's agents in, beside its own
([a stream's sessions](/flume/guides/agent-settings/#a-streams-sessions)).
The kind is named as `[kinds.<kind>]` names it, whatever source and
version it resolves to.

| Key | Values | Meaning | Left out |
|---|---|---|---|
| `session` | a string herdr takes as a session's name | the session the kind's agents open in, every stage of the kind but one its own table names | the stream's own |
| `stages` | a table of tables, one for each stage by its name in the kind | the sessions of the kind's stages | none |

### `[streams.<stream>.kinds.<kind>.stages.<stage>]`

| Key | Values | Meaning | Left out |
|---|---|---|---|
| `session` | a string herdr takes as a session's name | the session that stage's agents open in | the kind's, else the stream's own |

```toml
[streams.work.kinds.research]
session = "work-design"

[streams.work.kinds.release.stages.review]
session = "work-review"
```

Each table is read at each start of its kind's or its stage's agents,
and a table for a kind or a stage the place does not run is ignored. A
session a stream names here is that stream's alone. The local layer
may set each `session`, as `streams.work.kinds.research.session` and
`streams.work.kinds.release.stages.review.session`. The managed layer
may pin either, or a table holding them, as `"streams.work.kinds"`.

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

### A repository's name

`[repos."<repository>"]` and every table under it name a repository by
its address, `<owner>/<name>` as its clone's `origin` remote gives it
on its forge. An origin of `git@github.com:acme/acme.git`,
`https://github.com/acme/acme` or `ssh://git@git.example.org/acme/acme.git`
gives `acme/acme`. A key matches exactly, its case as written and with
no star. An origin that is a folder on the host, or whose path has one
part or three, gives no address. A clone with no address, or no origin,
goes by its folder, a leading `~` in the key your home.

Where the configuration has a table under a repository's address, that
table comes first and is the repository's whole table: a table under
its folder is not read for it. Where none names the address, the
folder's table is read, so a file that names repositories by folder
reads as before. A launch reads the address from the clone it runs in,
or the folder `--repository` names, and its place keeps it, so the
place's later starts read the tables for that address. flume finds no
clone by its address: `--repository acme/acme` is the folder
`acme/acme`. Status gives each place's address beside its folder.

### `stream`

`stream` in `[repos."<repository>".kinds.<kind>]`, in
`[repos."<repository>"]` and in `[kinds.<kind>]` names the stream of a
place of that repository and kind, of that repository, or of that kind,
in that order, where the launch and the intent name none. The
configuration names the repository by its address, `<owner>/<name>`
as its clone's `origin` gives it, matched exactly, else by its folder.
The address's table comes first, whole
([a repository's name](#a-repositorys-name)).

### `label`

`label` in `[repos."<repository>".kinds.<kind>]` and in
`[kinds.<kind>]` names the label of a place of that repository and
kind, or of that kind, in that order. It is one word of letters,
digits, `-` and `_`. A place's label is read at each start. The
configuration names the repository by its address, matched exactly,
else by its folder. The address's table comes first, whole
([a repository's name](#a-repositorys-name)).

### `[kinds.<kind>.agents]`

Settings for a kind's agents in every repository: `model`, `effort`,
`instructions`, `mode`, `keep_open` and `programs`, with
`[kinds.<kind>.agents.stages.<stage>]` for one stage's. A stage's own
setting comes before one for every stage. `model` and `instructions`
are strings. `effort` is a string, its agent program's own word, as
Claude Code's `low`, `medium`, `high`, `xhigh` and `max`
([the effort](/flume/guides/agent-settings/#the-effort)). `mode` is one of `plan`, `ask`, `edits` and `full`, or a
table of programs whose values are strings, as `mode = { claude =
"auto" }`. `keep_open` is a string, `<n>`, `<n>s`, `<n>m` or `<n>h`:
how long an agent is kept open for an answer once it stops short
needing one, `0` keeping none
([how long an agent is kept open](/flume/guides/agent-settings/#how-long-an-agent-is-kept-open)).
It is one time for every program, never a table of programs.
`programs` is a table of programs' own settings
([`programs`](#programs)). Any other key is not read.
[Agent settings](/flume/guides/agent-settings/) gives the layers and
[the mode](/flume/guides/agent-settings/#the-mode) what each mode
means.

### `[repos."<repository>".kinds.<kind>.agents]`

The same, for a kind in one repository. The key is the repository's
address, `<owner>/<name>` as its clone's `origin` gives it, matched
exactly, else its folder, a leading `~` in the key your home. The
address's table comes first, whole
([a repository's name](#a-repositorys-name)). It comes before the
host's settings for the kind in every repository.

### `[repos."<repository>".agents]`

Settings for every agent of every kind in one repository: `model`,
`effort`, `instructions`, `mode`, `keep_open` and `programs`. The key
names the repository as `[repos."<repository>"]` does, by its address,
matched exactly, else by its folder, the address's table first and
whole ([a repository's name](#a-repositorys-name)). It comes after the repository's tables for one
kind, and before the host's settings for the kind in every repository.
It is how a host speaks for a repository it does not write to, a fork
of another project among them
([what a commit says about who made it](/flume/guides/agent-settings/#what-a-commit-says-about-who-made-it)).
A `program`, `env`, `beside`, `stages` or `streams` in it refuses the
file.

### `[agents.streams.<stream>]`

Settings for every agent of the places in one stream: `model`,
`effort`, `instructions`, `mode`, `keep_open` and `programs`. It comes after the host's
settings for a repository and for a kind, and before `[agents]`. The
stream is one a `[streams.<stream>]` table declares, or `default`. A
`program`, `env`, `beside` or `stages` in it, or a stream the host does
not declare, refuses the file. A stream's own table,
`[streams.<stream>]`, keeps its bound, priority, pool and session and
gives no setting.

### `[agents]`

Settings for every agent on the host: `model`, `effort`,
`instructions`, `mode`, `keep_open` and `programs`, and `streams`, the tables above. It comes after every
other host table, and before the repository's and the kind's settings.
A `program`, `env`, `beside` or `stages` in it refuses the file: the
host chooses a program by its pools' accounts, and an agent's
environment is its account's. A pane beside is the launch's to name,
and a stage's name is its kind's.

### `programs`

Every agents table above gives `programs`, a table of programs' own
settings, each entry named for a program
([a program's own settings](/flume/guides/agent-settings/#a-programs-own-settings)):

| Key | Values | Meaning |
|---|---|---|
| `file` | a path | a settings file in the program's own words, JSON where its name ends `.json` and TOML where it ends `.toml`, its top level a table, its keys beneath `settings` |
| `settings` | a table | the program's own keys as it spells them, each taken whole |

```toml
[agents.programs.claude]
file = "~/.config/agent-launcher/claude-settings.json"
settings = { tui = "fullscreen" }

[kinds.fix.agents.stages.code.programs.claude]
file = "claude-fix-code.json"
```

The host reads a file with its configuration, at each start: an
absolute path, a path under `~/` as your home, or a path relative to
the configuration's own folder, `$XDG_CONFIG_HOME/flume/`. A link is
read where it leads, so a link at a path that stays names a file that
moves. flume reads no `$VAR` in a path, and no `~` or `$VAR` inside a
value. A key of Claude Code's that another setting owns, `env`,
`model`, `effortLevel`, `permissions.defaultMode` and the rest, is left out of a named
file
([keys another setting owns](/flume/guides/agent-settings/#keys-another-setting-owns)).
Given in a table's `settings`, it refuses the launch, or the start, of
each agent the table gives settings to. A credential goes in an
account's `env`, never in `programs`: a value shaped as one refuses
the same way, and status gives it as `<set>` among the configuration's
values.

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
- an `effort` in an agents table that is not a string, a table of
  programs among them, naming the table and `effort`;
- a `mode` in an agents table that is neither one of `plan`, `ask`,
  `edits` and `full` nor a table of programs whose values are strings,
  naming the table and `mode`;
- a `keep_open` in an agents table that is not a string of the form
  `<n>`, `<n>s`, `<n>m` or `<n>h`, a table of programs among them,
  naming the table and `keep_open`;
- a `programs` in an agents table that is not a table of entries, an
  entry's key other than `file` and `settings`, or a TOML date or time
  in one, naming the key;
- a `file` that cannot be read, does not parse, holds no table at its
  top, or whose name ends neither `.json` nor `.toml`, naming the
  table's `file` and the file;
- a `program`, `env`, `beside` or `stages` in `[agents]` or
  `[agents.streams.<stream>]`, or any of those or `streams` in
  `[repos."<repository>".agents]`, naming the table and the key;
- an `[agents.streams.<stream>]` naming a stream no `[streams]` table
  declares;
- a key a stream, a pool or an account does not know;
- a `label` that is not one word of letters, digits, `-` and `_`;
- a `pin` that names no key the local layer could set and no table of
  such keys;
- a `[host]` giving both `terminal` and `multiplexer`;
- a `view` other than `terminal` and `app`;
- a `multiplexer`, `terminal_app` or `title` that is not a string;
- a `session` in a stream's table for a kind or a stage that is not a
  string, or a kind's `stages` that is not a table of tables.

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

A stream's table for a kind or a stage refuses each start that reads
it, and no other, where it holds any key but `session` and a kind's
`stages`, naming the key, and where the session it names is one another
stream's agents open in by that stream's word, its own or one it names
for a kind or a stage, naming both streams. A stage so refused waits
for a retry once the file is mended, and status gives each such table
with its refusal.

### No roles

The host's configuration names no role and no answerer: a source of the
host declares each role, in the roles file its file in `sources/` names
([a source's roles](#a-sources-roles)). A `config.toml` that gives a
`[roles]` table cannot be read, and every command refuses, naming it and
the roles file.

## The host's sources

`$XDG_CONFIG_HOME/flume/sources/`, beside `config.toml`, holds one file
per source of kinds the host declares
([kinds from a source on the host](/flume/guides/kinds/#kinds-from-a-source-on-the-host)):

```toml
# ~/.config/flume/sources/acme.toml
kinds_dir = "~/.local/share/flume/kinds/acme"
roles = "~/.local/state/acme/flume/roles.toml"
```

| Key | Values | Meaning | Left out |
|---|---|---|---|
| `kinds_dir` | an absolute path, or one whose leading `~` is your home | the directory holding the source's packages, each at `<name>@<version>/` with its `kind.toml` at its root | refused |
| `roles` | an absolute path, or one whose leading `~` is your home | the file in which the source declares the roles it answers ([a source's roles](#a-sources-roles)) | the source declares no role |

The file's name, less `.toml`, is the source's name. flume reads every
file in the directory whose name ends in `.toml` and begins with no
`.`, a link as the file it leads to, and ignores every other entry. It
reads the directory as it stands at each launch, each wake of a role,
each `flume kinds lock` and each status, and writes nothing in it or in
a `kinds_dir`.

The sources belong to the host's configuration as its accounts do: a
person, or a program that manages the host, writes each file whole,
renames a new one over it, and deletes it. `config.local.toml` declares
no source. No sources directory is a host with no sources. A `sources`
that cannot be listed, a file of that name among them, makes every
command refuse, as a `config.toml` that cannot be read does.

### A source's faults

Each of these is its source's own: it refuses every launch that names
the source, naming the file and the fault, and no other launch. The
host's part of `flume status` names it.

- a file that is not TOML;
- no `kinds_dir`, a `kinds_dir` or a `roles` that is neither absolute
  nor under `~`, or a key other than `kinds_dir` and `roles`;
- a name that is not a legal source's name;
- the name `flume`, `repo` or `launch`, the sources of flume's own
  kinds, the repository's and a launch's, which no host declares.

A `kinds_dir` that does not exist is no fault of the file's, since a
program may write the file before the packages. A launch naming that
source is refused, naming the directory as missing.

### A source's roles

The file a source's `roles` names declares the roles the source
answers, each at its address, `<stream>/<role>`, on a stream the host
declares ([roles and wake](/flume/guides/roles-and-wake/)). The
source's own tool writes it, whole, and renames it over the old one.
flume reads it as it stands at each launch, wake, start, stall and
status, and writes nothing in it. It gives `roles` alone, a table of
entries by address, each one of three:

```toml
[roles."east/operator"]
kind = "operator"
repository = "~/src/acme-records"
name = "east"
intent = "east-operator"
answers = "Whatever reaches the stream's work from outside it, and a question no role took up."
answerer = true

[roles."east/operator".agents]
model = "opus"

[roles."east/planner"]
agent = "east-planner"
answers = "The work's scope: work outside the goal, the order of work."

[roles."north/planner"]
carried = true
answers = "The work's scope, for the stream north, which runs on another host."
```

**A run's entry**, the run flume launches for the address:

| Key | Meaning | Left out |
|---|---|---|
| `kind` | one of the source's own kinds by its name, `operator` or `acme/operator`, its version the locks' | refused |
| `repository` | the repository of the run's place; a leading `~` is your home | refused |
| `priority` | a whole number, the run's priority in the host's queue | `0` |
| `base` | the base the run's place is cut from | the repository's default branch |
| `brief` | the run's brief | none |
| `name` | the run's name, which names its place `<kind>--<name>` | `<stream>-<role>` |
| `intent` | the intent the run serves | the repository and base, as for any launch |
| `agents` | the run's agents' settings, as a launch's `--agent` gives them: `program`, `model`, `effort`, `instructions`, `beside`, `session`, `mode`, `keep_open` and `programs`, and each stage's under `stages.<stage>`; a file a program's own settings name is read beside the roles file | none |
| `answers` | what the role answers, in words a stage's agent reads to choose it; with none, no prompt lists the role | none |
| `answerer` | `true` where the role stands behind its address's stream ([the answerer behind a stream](/flume/guides/roles-and-wake/#the-answerer-behind-a-stream)) | `false` |

**A standing agent's entry**, an agent that already runs for the role
and takes its ends as notices ([an address an agent already
holds](/flume/guides/roles-and-wake/#an-address-an-agent-already-holds)):
`agent`, the name the multiplexer knows it by; `session`, the session it
lives in, else the session the address's stream opens its own agents
in; and `answers` and `answerer` as a run's.

**A carried address's entry**, for a stream that runs on another host:
`carried = true`, and `answers` where it gives it. flume launches
nothing for it and keeps its ends pending for the tool that carries
them.

No entry names a stream: its address names it. A run and a standing
agent run, and are found, in that stream's room, pool and terminal.

Each fault stays its own, named under the source in status, and none
refuses the host's configuration, another source or another entry:

- **a roles file** that cannot be read, is not TOML, or gives a key
  other than `roles`, declares no role;
- **an entry** declares nothing for its address where its key is no
  `<stream>/<role>` address, or where it:
  - names a kind not its source's, or a kind with a version;
  - gives an `env`, a `pool`, an `account` or a `stream`, which no role's
    entry gives, or `points`, which this flume takes for no place;
  - gives a key its form does not know, a setting no `--agent` sets
    among them;
  - names in its address a stream the host does not declare, and is not
    carried;
  - gives `agent` beside `kind` or another of a run's keys, two agents
    of one role;
  - gives `carried` beside anything but `answers`;
- **an address two entries declare**, of one source or two, and two
  addresses whose runs would be one place, of one repository, kind and
  name, are declared by neither, each named with the other.

An address declared by nothing wakes nothing: its ends stay pending,
and status says why under the host's addresses.

## The repository's `.config/flume.toml`

flume reads it from git at a revision, never from the working copy. It
reads the `[profiles]` table at the repository's default branch, after
one fetch of `origin` where the repository has it, and the kinds at the
base's current revision for each launch. A place keeps what its launch read.
flume never writes it.

It reads the repository's lock of kinds, `.config/flume.lock`, at the
base's current revision too, where a launch's kind needs a lock
([the locks of kinds](#the-locks-of-kinds)).

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

[agents.programs.claude.settings]
attribution = { commit = "Assisted-by: Claude Code", pr = "" }
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
since an agent's environment is its account's alone, and so does a
`keep_open`, since how long an agent is kept open for an answer is the
host's or the launch's to give. A file an agents
table names under `programs` is a file of the repository, relative to
`.config/`, read at the same revision; one outside the repository, by
`..`, an absolute path or a link, refuses the launch.

### `[agents]`

Settings for every agent of every kind the repository runs, its own
and flume's: `program`, `model`, `effort`, `instructions`, `mode` and
`programs`, as a kind's agents table gives them. It is flume's own table beside
`[profiles]`, read with the kinds at the base's current revision and
kept as the launch read it. The repository's tables for one kind come
before it, and it comes before a kind the repository declares. A
`stages`, `streams`, `env`, `keep_open` or `beside` in it refuses the
launch, naming the field.

## A launch's declaration

`flume launch --kinds <file>` names a file in the same format, read
from disk at the launch with its prompts beside it, the source `launch`.
Its kinds stand over the repository's and flume's own for a bare name,
for that launch alone, and need no lock. Its `[profiles]` and
`[agents]` tables, if any, are not read.

## The locks of kinds

A kind from a source on the host runs only where a lock records its
identity at its content's hash. A lock records flume's own kinds too,
to pick the version a launch that names none takes
([locks](/flume/guides/kinds/#locks)). flume reads two:

| Lock | Read |
|---|---|
| `.config/flume.lock` in the repository | from git at the base's current revision, as its kinds are |
| `$XDG_CONFIG_HOME/flume/flume.lock`, the host's | as it stands at each launch |

Each is TOML, one `[[kinds]]` table per entry, `kind` its identity and
`hash` its content's:

```toml
[[kinds]]
kind = "acme/review@1"
hash = "sha256:ab12…"

[[kinds]]
kind = "acme/review@2"
hash = "sha256:cd34…"
```

A lock that is not there records nothing. A lock that cannot be read
refuses each launch that needs it, naming the file and its fault, and
no other. flume writes a lock only at
[`flume kinds lock`](/flume/reference/commands/#flume-kinds-lock), and
never commits one: the repository's lock reaches its places once you
commit it on their base.

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
| `agents` | a table | settings for every stage's agents, and `agents.stages.<stage>` for one stage's: `program`, `model`, `effort`, `instructions`, `beside`, `mode`, `programs`; an `env` or a `keep_open` refuses the launch | none |
| `stages` | an array of tables | its stages, at least one | refused |

### A stage

| Field | Values | Meaning | Left out |
|---|---|---|---|
| `name` | a non-empty string | unique in its kind | refused |
| `prompt` | a path | its prompt, relative to the declaring file's folder; for a repository, read from git at the same revision | refused |
| `deliverables` | a list of one spelling | what ends it: `commits`, `change`, `tasks`, `archived`, `report:<path>` or `committed:<path>` | refused, unless a conversation |
| `conversation` | `true`, `false` | a stage with no deliverable, ended by `flume finish` | `false` |
| `agents` | a list of one table | its agent's settings: `program`, `model`, `effort`, `instructions`, `beside`, `mode`, `programs`; an `env` or a `keep_open` refuses the launch | refused by flume, which runs one agent per stage |
| `makes` | a list of names | what it makes besides its deliverable, each recorded as a line of `made.jsonl` | none |
| `wait` | `true`, `false` | the stage is held before it starts until a retry names it | `false` |
| `quiet` | a whole number then `s`, `m` or `h`, as `"45m"` | how long its agent may be quiet with nothing delivered before it is stuck | 30 minutes |
| `on_fail` | a stage's name | read from the last stage: where a failed or conflicting submission sends the place back, and found work from the last stage where it names a stage before the last; found work from a last stage that names itself or none waits on an answer | the last stage |
| `max_send_backs` | a whole number from 0 | read from the last stage: how many times the place may go back before it waits for you | 3 |

[Kinds](/flume/guides/kinds/) says what each deliverable needs and how
a kind runs through, and [prompt parts](/flume/reference/prompt-parts/)
lists what a prompt may ask for.

## Environment

| Variable | Read by | Meaning |
|---|---|---|
| `XDG_CONFIG_HOME` | every command | where the host's configuration lies, with its sources and its lock of kinds, `~/.config` where unset |
| `XDG_STATE_HOME` | every command | where flume keeps its state, `~/.local/state` where unset |
| `XDG_DATA_HOME` | every command that starts an agent | where flume writes its Claude Code plugin, `~/.local/share` where unset |
| `FLUME_RECIPIENT` | `launch`, `wait`, `inbox` | the recipient where a command names none |
| `FLUME_AGENT`, `FLUME_SECRET` | `needs`, `ack`, `hook` | the identity and secret of the agent's start, set by flume on each agent |
| `FLUME_PLACE`, `FLUME_STAGE` | `hook`, `wait`, `inbox` | the agent's place and stage; a command run in a role's run takes its role from `FLUME_PLACE` |
| `HERDR_*` | the herdr terminal | the pane a command runs in, read as its session; stripped from every herdr call flume makes and from `sluice submit` |
