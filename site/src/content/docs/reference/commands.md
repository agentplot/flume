---
title: Commands
description: Every flume command and flag, with what each answers and how it exits.
sidebar:
  order: 1
---

| Command | |
|---|---|
| `flume launch` | Make one place and start, hold or queue its first stage. |
| `flume status [<place>]` | Every fact of a place, or of every place, and of the host. |
| `flume prompt <place> <stage>` | The exact prompt a stage would be sent at its next start, starting nothing. |
| `flume tell <place> <words>` | Hand the place's agent words as a prompt, once it waits. |
| `flume hold <place> <words>` | Hold the place's next stage, `--stage`, or its submission, `--submission`, with words saying what for. |
| `flume retry <place>` | Start a stage again with a fresh agent, or submit the place again. |
| `flume stop <place>` | End the place's agent and nothing else. |
| `flume finish <place>` | End a conversation's agent; the conversation counts as delivered. |
| `flume drop <place>` | Give up the place's work, and tidy it. |
| `flume done <place>` | Declare complete the work of a place whose kind does not land, and tidy it. |
| `flume priority <place> <n>` | Set the place's priority in the host's queue. |
| `flume tidy [<place>]` | Remove what is over, for one place or every place. |
| `flume settle [<place>]` | Move a place, or every place, on. |
| `flume wait <place>` | Wait on a stage's end or the submission's outcome, as a recipient. |
| `flume inbox` | Collect every end pending for a recipient, on every place of the host. |
| `flume wake <address>` | Wake a role's address: launch its run where none stands. |
| `flume mcp` | Serve flume's status over MCP on standard input and output. |
| `flume needs <words>` | From an agent flume started: its stage cannot deliver without an answer, or found work. |
| `flume ack started`, `flume ack received` | From an agent flume started: it started, or it received a prompt. |
| `flume hook <event>` | From an agent program's hook: what flume does with the hook's input. |

`flume --help` and `flume <command> --help` print each command's own
words. `flume --version` prints the version.

## Every command

Two flags work with every command:

| Flag | |
|---|---|
| `--json` | Print the answer as JSON, one document. The form only ever gains fields, and every surface, the MCP server included, gives the same one. |
| `--repository <dir>` | The repository, where it is not the one the command runs in. |

**The repository** is the one the command runs in, found from any of
its worktrees, a place's included. In a bare layout,
`project/project.git` beside its worktrees, it is `project`.

**A place** is its location, or `<kind>--<name>` in the
repository, such as `fix--the-parser-reads-tabs`.

**Times** are written `<n>`, `<n>s`, `<n>m` or `<n>h`: `90`, `90s`,
`10m`, `1h`.

**Exits.** A command exits `0` with its answer, and `1` with
`refused: <reason>` when flume refused it, the reason in flume's words.
Words that name no command or flag exit `2`, with the usage on standard
error. `flume wait` has exits of its own.

**Settling first.** Every command that acts, `launch`, `tell`,
`hold`, `retry`, `stop`, `finish`, `drop`, `done`, `priority`, `tidy`,
`wait`, `inbox` and `wake`, settles every place on the host first,
unless herdr's watch keeps them moving. `flume status`, `flume prompt`
and `flume mcp` only read.

## `flume launch`

```sh
flume launch --kind <kind> --name <name> [--brief <text> | --brief-file <file>] [flags]
```

| Flag | |
|---|---|
| `--kind <kind>` | The kind, resolved from the launch's `--kinds` file, then the repository's `.config/flume.toml` at the base, then flume's own. Required. |
| `--name <name>` | The work's name, which names its branch, `<prefix>/<name>`, and its place, `<kind>--<name>`. Required. |
| `--base <branch>` | The branch the place is cut from and lands on; where none is named, the repository's [default branch](/flume/reference/configuration/#the-repositorys-configflumetoml), its remote's. |
| `--brief <text>` | What the work is, given whole to every stage whose prompt asks for `{{brief}}`. |
| `--brief-file <file>` | The brief, read from a file once at the launch. |
| `--kinds <file>` | A declaration of kinds the launch carries, read from disk with its prompts beside it. |
| `--hold <stage>=<words>` | Hold that stage before it starts, with words saying what it waits for. Repeatable. |
| `--hold-submission` | Hold the submission once the last stage delivers. |
| `--recipient <name>` | A recipient of the place's ends: a session or a role's address. Repeatable. With none, `$FLUME_RECIPIENT`, else the terminal session the launch runs in. |
| `--mark <mark>` | A mark the caller finds its places by, given back in status. |
| `--priority <n>` | The place's priority in the host's queue, a whole number, negative allowed; `0` where none is given. |
| `--stall-after <time>` | How long the place may wait on its caller under no hold's words before it is said stalled; five minutes where none is given. |
| `--agent [<stage>.]<field>=<value>` | One setting of every stage's agent, or of one stage's. The field is `program`, `model`, `instructions`, `beside` or `mode`: `mode=<mode>` gives one of `plan`, `ask`, `edits` and `full`, and `mode.<program>=<value>` one entry of a table of programs, several building one table. A mode word and a table for the same stage refuse the launch. Repeatable. |
| `--stream <stream>` | The stream the place runs in, one the host's configuration declares. Where none is named: the intent's stream, else the one the host's configuration names for the repository and kind, the repository or the kind, else `default`. Fixed at the launch. |
| `--intent <name>` | The intent the place serves, by name. Places of one intent keep to one account while it is usable. Where none is named, the repository and base name the intent. |

The answer is the place's status. A launch is refused whole, leaving
nothing it made, when:

- the place, its branch or its directory already stands;
- the kind resolves to no declaration, or a declaration cannot be read;
- the kind asks for more than flume runs, such as a stage with no agent
  or several;
- the kind lands and the repository binds no merge queue, or reads a
  change and binds no change tool;
- a stage's prompt asks for `{{brief}}` and the launch gives none;
- the name gives no valid branch;
- the base's current revision cannot be read, a failed fetch among the
  causes;
- a declaration sets `env` in an agents table, or a setting names a
  stage the kind lacks, or a program no agent profile runs;
- the stream is one the host's configuration does not declare.

```console
$ flume launch --kind deploy --name x --brief x
refused: the kind deploy resolves to no declaration
$ flume launch --kind chore --name 'bad..name' --brief x
refused: the name bad..name gives no valid branch: chore/bad..name
```

Every launch that is not refused is taken: one that finds the host at
its bound waits for room, and the answer says so.

## `flume status`

```sh
flume status [<place>]
```

With a place, its status; with none, every place on the host. Each
place gives:

- its kind, version and origin, and its phase, live or tidied;
- its branch, base and head, and whether each readiness step passed or
  failed, with the last ten lines a failed step printed, its terminal
  escapes stripped;
- each stage's state and starts;
- its current agent's identity, program, settings with their layers
  and state, and whether it started, is asking or has its own work
  running;
- whether the deliverable exists, and a change's tasks;
- its submission;
- what it waits on, since when and in whose words, and whether it is
  stalled;
- its stream and intent, its current agent's account, each move
  between accounts, and each start that began without its settings;
- its holds, priority, recipients and ends, and what flume could not
  read.

The host part gives the agents running, the bound, the room, the
queue, each stream's and each pool's part, the watch, each base's
count, the role addresses with ends pending, and what a tidy kept in
the terminal. Where the terminal keeps no watch, it says so: places
then move on only at flume's commands and when their agents go quiet.

flume reads each fact where it lives when asked, and keeps none
between asks. The [state reference](/flume/reference/state/) lists the
JSON fields.

## `flume prompt`

```sh
flume prompt <place> <stage>
```

The exact prompt the stage would be sent at its next start, every part
filled, with nothing started, sent, settled or written. A prompt that
asks for a part with nothing to fill it is refused, naming the part.

## `flume tell`

```sh
flume tell <place> <words> [--within <time>]
```

Hands the place's current agent the words as a prompt once it has said
it started and waits, read back as received or sent once more. flume
waits up to ten minutes, or `--within`, for the agent to wait. It
refuses with nothing typed when the place has no live agent, its agent
is asking a person a question, or the words are not delivered within
the bound. A tell starts, ends and moves no stage. Under the printed
command every tell is refused, since flume holds no pane to type into.

## `flume hold`

```sh
flume hold <place> --stage <stage> <words>
flume hold <place> --submission <words>
```

Holds a stage not started since the current stage began, taking one
waiting for room out of the queue, or the submission not yet made. It
refuses a stage started, a submission made and undecided, a second hold
and a place whose work is over. Only a retry naming the stage or the
submission ends the hold.

## `flume retry`

```sh
flume retry <place> [--stage <stage>] [--words <words>] [--force]
flume retry <place> --submission
```

| Flag | |
|---|---|
| `--stage <stage>` | The stage to start again; the current one where none is named. The stages after it run again in order. |
| `--words <words>` | Words for the stage's agent, after the question its stage stopped on. |
| `--force` | End an agent that is not quiet. |
| `--submission` | Submit the place's head again, starting no agent. |

A retry ends the place's agent first, starts the stage with a fresh
agent, and ends a hold on it. It never ends an agent whose stage has
started or begun: the retry is refused, forced or not, naming the
agent, and `flume stop <place>` ends it first.

## `flume stop`, `finish`, `drop` and `done`

```sh
flume stop <place> [--force]
flume finish <place> [--force]
flume drop <place> [--force]
flume done <place> [--force]
```

| Command | Ends the agent, then |
|---|---|
| `stop` | nothing more: the stage waits for a retry or a drop |
| `finish` | the conversation counts as delivered |
| `drop` | starts nothing more and tidies the place |
| `done` | declares the work complete and tidies the place; refused for a kind that lands |

Each ends an agent only once it is quiet, unless `--force` is given.
Under the printed command flume cannot end an agent, and the act is
refused until the agent's session ends where it runs.

## `flume priority`

```sh
flume priority <place> <n>
```

Sets the place's priority, a whole number, negative allowed. A settle
starts waiting stages by their stream's priority, then their place's,
highest first, equals in the order they came to wait.

## `flume tidy`

```sh
flume tidy [<place>]
```

Removes what is over, for each place whose work landed, was dropped
or is done and whose agent stopped:

- its worktree, through the repository's teardown and
  `git worktree remove`;
- its branch, where the base holds every commit of it;
- under herdr, its tab, and its base's workspace once that holds
  nothing else of flume's or yours.
 It keeps a place
holding uncommitted or unignored work, and a branch with commits the
base lacks, naming each.

```console
$ flume tidy
removed ~/.local/share/flume/places/f5f5e14159a8/chore--tidy-the-readme
kept the branch chore/tidy-the-readme of ~/.local/share/flume/places/f5f5e14159a8/chore--tidy-the-readme, 1 commits not landed
```

## `flume settle`

```sh
flume settle [<place>] [--on-quiet]
```

Moves a place, or every place, on. It reads each stage's deliverable,
starts the next stage or the waiting ones there is room for, and
submits. It collects the merge queue's outcomes, delivers pending
ends, wakes roles and tidies. Safe to repeat. `--on-quiet` first reads the place's
agent until it is quiet, for up to sixty seconds; the Stop hook runs it.
herdr's watch runs `flume settle` whenever an agent changes state,
comes up or goes.

## `flume wait`

```sh
flume wait <place> --stage <stage> [--recipient <name>] [--timeout <time>]
flume wait <place> --submission [--recipient <name>] [--timeout <time>]
```

Waits as a recipient on the current start's end of a stage, or on the
submission's outcome. An end pending for the recipient is answered at
once and recorded delivered. One already delivered is read again,
delivering nothing. Otherwise the wait follows the host's state until
an end comes or the timeout passes. With no timeout it waits until an
end comes.

| Exit | |
|---|---|
| `0` | a stage delivered or finished, or a submission landed |
| `2` | any other end |
| `3` | still waiting when the timeout passed |
| `1` | refused |

The recipient is `--recipient`, else `$FLUME_RECIPIENT`, else the role
a role's run serves, else the terminal session the command runs in.

## `flume inbox`

```sh
flume inbox [--recipient <name>]
```

Every end pending for the recipient, on every place of the host, a
tidied place's kept record among them, each recorded delivered in the
same write. Prints `nothing pending for <recipient>` when there is
none. The recipient defaults as for `flume wait`.

## `flume wake`

```sh
flume wake <address>
```

Wakes a role's address, `<group>/<role>`: launches the run the host's
`[roles]` table names for it where none stands, and otherwise makes
nothing, saying the run is live or waits. Refused for a name that is no
address, or an address the table names nothing for.

## `flume mcp`

Serves MCP on standard input and output with one tool, `status`, which
takes an optional place and answers as `flume status --json` does. It
settles nothing. flume's Claude Code plugin runs it as its `flume`
server.

## Run by agents

These run inside an agent flume started, where `FLUME_AGENT` and
`FLUME_SECRET` name its start. Elsewhere they refuse, or in the hook's
case do nothing.

| Command | |
|---|---|
| `flume needs <words>` | The stage ends short with the words: it needs an answer. `--found` says it found work it cannot deliver past instead. |
| `flume ack started --session <id> --transcript <path> --plugin-version <v> [--source <how>]` | The agent started under its identity and secret, in a session with a transcript. `--source` says how the session began, `startup` where none is given. flume takes it only where the agent runs the program its start recorded and every variable of the record matches. Otherwise it refuses, naming each variable that differed and whether the program did, and starts a settle of the place. |
| `flume ack received --digest <hex> [--mode <mode>]` | The agent received a prompt, by the SHA-256 digest of its text with paste tags taken off and whitespace collapsed. At the start's first prompt, where its start passed a mode, flume takes it only where `--mode` names that mode as the program names it. Otherwise it refuses, naming the mode asked and the one given, records the start begun without its settings and starts a settle of the place. A later prompt compares no mode. |
| `flume hook <event>` | What the hook for the event does with the input on standard input: `session-start`, `user-prompt-submit`, `pre-tool-use`, `permission-request`, `post-tool-use`, `stop`, `stop-failure` or `session-end`. Any other event does nothing. It always exits `0`. |

flume's Claude Code plugin runs `flume hook` for every event;
[the Claude Code plugin](/flume/guides/claude-code/#what-each-hook-does)
says what each does.
