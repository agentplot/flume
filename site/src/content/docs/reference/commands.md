---
title: Commands
description: Every flume command and flag, with what each answers and how it exits.
sidebar:
  order: 1
---

| Command | |
|---|---|
| `flume launch` | Make one place and start, hold or queue its first stage. |
| `flume status [<place>]` | Every fact of a place, or of every place, and of the host; with `--line`, each place's one line. |
| `flume list` | A line for each place on the host, under its repository, and each stream, reading only what flume keeps; `--all` adds the places that landed or were dropped. |
| `flume prompt <place> <stage>` | The exact prompt a stage would be sent at its next start, starting nothing. |
| `flume tell <place> <words>` | Hand the place's agent words as a prompt, once it waits; to an agent kept open for its answer, the answer, `--answers <key>` naming the stop-short it answers. |
| `flume hold <place> <words>` | Hold the place's next stage, `--stage`, or its submission, `--submission`, with words saying what for; or, `--waiting`, the move the place waits on, with words saying who makes it and an optional bound. |
| `flume retry <place>` | Start a stage again with a fresh agent, hand an answer to the agent kept open for it, or submit the place again; `--answers <key>` names the stop-short the words answer. |
| `flume stop <place>` | End the place's agent and nothing else. |
| `flume finish <place>` | End a conversation's agent; the conversation counts as delivered. |
| `flume drop <place>` | Give up the place's work, and tidy it. |
| `flume done <place>` | Declare complete the work of a place whose kind does not land, and tidy it. |
| `flume priority <place> <n>` | Set the place's priority in the host's queue. |
| `flume tidy [<place>]` | Remove what is over, for one place or every place. |
| `flume settle [<place>]` | Move a place, or every place, on; with `--every <time>`, every place as a watch over the host. |
| `flume wait <place>` | Wait on a stage's end or the submission's outcome, as a recipient. |
| `flume inbox [<place>]` | Collect every end pending for a recipient, on every place of the host, or on the place named alone. |
| `flume wake <address>` | Wake a role's address, `<stream>/<role>`: launch its run where none stands, and nothing where an agent already holds it. |
| `flume kinds lock <identity>…` | Record each kind at its content's hash in a lock of kinds, or take it out. |
| `flume open [<session>]` | Show a session of a stream's, by the session's name, in its stream's view: a window its terminal app opens, its multiplexer's own app, or the line to run. |
| `flume mcp` | Serve flume's status over MCP on standard input and output. |
| `flume needs <words>` | From an agent flume started: its stage cannot deliver without an answer, or found work; `--to` names the role the words are for. |
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
unless a caller keeps watch over the host with `flume settle --every`.
`flume status`, `flume list`, `flume prompt` and `flume mcp`
only read, `flume status --line` and `flume hook status-line` among
them. `flume open` settles nothing, and `flume kinds lock` writes its
lock and settles nothing.

## `flume launch`

```sh
flume launch --kind <kind> --name <name> [--brief <text> | --brief-file <file>] [flags]
```

| Flag | |
|---|---|
| `--kind <kind>` | The kind by its name, `<name>` or `<name>@<version>`, resolved from the launch's `--kinds` file, then the repository's `.config/flume.toml` at the base, then flume's own. Or by its identity, `<source>/<name>` or `<source>/<name>@<version>`, from that source alone: `flume`, `repo`, `launch` or one the host declares ([where kinds come from](/flume/guides/kinds/#where-kinds-come-from)). Required. |
| `--name <name>` | The work's name, which names its branch, `<prefix>/<name>`, and its place, `<kind>--<name>`. Required. |
| `--base <branch>` | The branch the place is cut from and lands on; where none is named, the repository's [default branch](/flume/reference/configuration/#the-repositorys-configflumetoml), its remote's. |
| `--brief <text>` | What the work is, given whole to every stage whose prompt asks for `{{brief}}`. |
| `--brief-file <file>` | The brief, read from a file once at the launch. |
| `--kinds <file>` | A declaration of kinds the launch carries, read from disk with its prompts beside it. |
| `--hold <stage>=<words>` | Hold that stage before it starts, with words saying what it waits for. Repeatable. |
| `--hold-submission` | Hold the submission once the last stage delivers. |
| `--recipient <name>` | A recipient of the place's ends: a terminal's session by its name, or a role's address, `<stream>/<role>`, of any stream, whose run is woken for them. A name no pane holds is sent no notice: its ends wait for it, the place's record kept once it is tidied, until it collects them by `flume inbox` or `flume wait` as that name ([the end of a place's work](/flume/guides/callers-and-holds/#the-end-of-a-places-work)). Repeatable. With none, `$FLUME_RECIPIENT`, else the terminal session the launch runs in, else the answerer behind the place's stream ([who answers a place](/flume/guides/callers-and-holds/#who-answers-a-place)). Every stall reaches that answerer too. |
| `--mark <mark>` | A mark the caller finds its places by, given back in status. |
| `--priority <n>` | The place's priority in the host's queue, a whole number, negative allowed; `0` where none is given. |
| `--stall-after <time>` | How long the place may wait on its caller under no hold's words before it is said stalled; five minutes where none is given. |
| `--agent [<stage>.]<field>=<value>` | One setting of every stage's agent, or of one stage's. The field is `program`, `model`, `effort`, `instructions`, `beside`, `session`, `mode`, `keep_open` or `programs`. `effort=<word>` gives the agent's effort in its program's own word, as Claude Code's `low`, `medium`, `high`, `xhigh` or `max`, and a word its program does not take refuses the launch ([the effort](/flume/guides/agent-settings/#the-effort)). `session=<name>` opens the agent in that herdr session whatever its stream's, and `beside` opens it beside a pane before any session. `mode=<mode>` gives one of `plan`, `ask`, `edits` and `full`, and `mode.<program>=<value>` one entry of a table of programs, several building one table. A mode word and a table for the same stage refuse the launch. `keep_open=<time>` gives how long the agent is kept open for an answer once it stops short needing one, as a time, `0` keeping none; a `keep_open.<program>` refuses the launch ([how long an agent is kept open](/flume/guides/agent-settings/#how-long-an-agent-is-kept-open)). `programs.<program>.settings.<key>=<value>` gives one top-level key of a program's own settings, several building one entry. Its value is read as JSON where it parses and as the text written otherwise. A nested key, or one key given twice for one stage, refuses the launch. `programs.<program>.file=<path>` names a program's settings file, relative to the working directory, read once at the launch ([a program's own settings](/flume/guides/agent-settings/#a-programs-own-settings)). Repeatable. |
| `--stream <stream>` | The stream the place runs in, one the host's configuration declares. Where none is named: the intent's stream, else the one the host's configuration names for the repository and kind, the repository or the kind, else `default`. Fixed at the launch. |
| `--intent <name>` | The intent the place serves, by name. Places of one intent keep to one account while it is usable. Where none is named, the repository and base name the intent. |

The answer is the place's status. A launch is refused whole, leaving
nothing it made, when:

- the place, its branch or its directory already stands;
- the kind resolves to no declaration, or a declaration cannot be read;
- the kind names a source the host does not declare, or one whose file
  cannot be read, or a version its source does not hold;
- a kind from a source on the host is recorded in no lock, or its
  content's hash is not its lock's;
- the two locks give a kind two hashes, or a lock the launch needs
  cannot be read;
- the kind asks for more than flume runs, such as a stage with no agent
  or several;
- the kind lands and the repository binds no merge queue, or reads a
  change and binds no change tool;
- a stage's prompt asks for `{{brief}}` and the launch gives none;
- the name gives no valid branch;
- the base's current revision cannot be read, a failed fetch among the
  causes;
- a declaration sets `env` or `keep_open` in an agents table, or a
  setting names a stage the kind lacks, or a program no agent profile
  runs;
- a program's own setting is a key its agent profile reserves for
  another setting, holds a value shaped as a credential, or is given
  for a program whose profile takes none;
- the stream is one the host's configuration does not declare;
- the stream's multiplexer names no terminal profile flume carries,
  while a launch on another stream runs;
- it finds no recipient: no `--recipient`, no `$FLUME_RECIPIENT`, no
  terminal session it runs in, and no role standing behind its stream;
- it would take its stream's answerer, and two roles stand behind the
  stream, naming both;
- it names a role's address that no source declares, or two declare,
  and no role stands behind its stream to take its questions.

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
flume status [<place>] [--line]
```

With a place, its status; with none, every place on the host. Each
place gives:

- its line, the one `--line` prints;
- its kind, version and origin, and its phase, live or tidied;
- its kind's identity, where it came from, and for a package its hash
  and the lock that recorded it;
- its branch, base and head, and where the base's revision was read:
  where its merge queue lands it, at `origin`, or the local branch's,
  with why;
- whether each readiness step passed or failed, with the last ten lines
  a failed step printed, its terminal escapes stripped;
- each stage's state and starts;
- its current agent's identity, program, settings with their layers
  and state, and whether it started, is asking or has its own work
  running;
- its current agent's own settings, each with its layer and its file,
  the keys left out of each file with their owners, and how they were
  passed;
- whether the deliverable exists, and a change's tasks;
- its submission;
- what it waits on, since when and in whose words, and whether it is
  stalled;
- its stream, its label and its intent, its current agent's account,
  the multiplexer and the session it opened in, each move between
  accounts, and each start that began without its settings;
- its holds, priority, recipients and ends, and what flume could not
  read.

The host part gives the agents running, the bound, the room and the
queue. Each stream's part gives its session, its multiplexer, view,
terminal app and title, each with where it came from, and its labels.
Then come each pool's part, each source of kinds the host declares with
the kinds it holds or why it cannot be read, each base's count and the
role addresses with ends pending. Every address an agent already holds
is there too, with its agent, its session and where that came from, and
what the multiplexer found there when asked. Each multiplexer the host binds gives
what a tidy kept there. Then comes the host's configuration: whether
its local layer was taken, each value it set and each key passed over
with its pin. Last comes the host's watch: a caller keeping it, with
the bound it stated and when its newest settle began, or no caller
keeping it, and since when where one did. With none, places move on at
flume's commands and when their agents go quiet. Under that line it gives what the terminal's own configuration
lacks for flume's plugin there, such as herdr's key for the popup, in
words you can act on.

**`--line`** gives each place's one line and nothing else, reading only
what a line holds: the place's held state, the change at its head and
its agent's own record. It calls no terminal and no merge queue. Each
line holds the place, its kind and version, its current stage with a
repeating stage's group, how many times that stage started, a change's
tasks ticked and the place's state. A part the place has none of is
left out:

```console
$ flume status --line
change--parser · change@2 · code 1/2 · run 1 · tasks 0/3 · working
fix--the-parser-reads-tabs · fix@1 · code · run 2 · waiting on an answer
```

The words are the ones status gives each place, the herdr plugin's
sidebar and popup show, and every agent's status line pins under its
prompt ([the status line](/flume/guides/claude-code/#the-status-line)
lists the states). With `--json` the answer is
`{"lines":{"lines":[…]}}`, each line with the fields of status's
`line`.

flume reads each fact where it lives when asked, and keeps none
between asks. The [state reference](/flume/reference/state/) lists the
JSON fields.

## `flume list`

```sh
flume list [--all]
```

A line for each place on the host and each stream its configuration
declares, for a person to see at a glance what flume holds and what
`flume open` can show. A place that landed or was dropped has nothing
left to open, so the list leaves it out; `--all` lists every place,
those among them. It needs no repository and always exits `0`. `flume
status <place>` gives a place's every fact.

Each repository's path stands on a line of its own, its places' lines
under it. A place's line is `<kind>--<name> · <stage> · <state>`, the
stage its line names, without a repeating stage's group, and the state
in the words its line gives it. A place that is not live names no
stage. Then come the parts that hold:

- whom it waits on: `waits on its caller`, where its next move is its
  caller's, stalled or not; `waits on the host`, where a stage waits for
  room; `waits on its session`, where a stage waits while a process
  runs the conversation its start would carry; or `waits on the merge
  queue`, where it is submitted;
- what keeps a place whose work is over, in a word: `kept by <step>`,
  the teardown's step that failed, such as `sluice release`; `kept
  holding work`, `kept holding a lock` or `kept holding work and a
  lock`; `kept while its agent runs`; `kept while its session runs`,
  while a process runs its conversation; or `kept unread`. A place removed
  whose branch and document are still to go reads `being tidied`, and a
  tidied place's document kept for an end pending reads `tidied`. A
  place that landed or was dropped shows these only under `--all`; a
  place its caller declared done shows them always.

After the places, under `streams`, comes a line for each stream, in
status's order, `default` among them: `<stream> · session <name> ·
<runs>`. The session is the stream's own, which `flume open` shows by
its name: the stream's `session`, else its name, and, for `default`
naming none, the multiplexer's own, named as the multiplexer names it. The last part is
`runs`, `not running`, `not read` where the multiplexer could not be
read, or `no profile for <multiplexer>` where the stream's multiplexer
names no profile flume carries. Then comes `session <name> · <runs>`
for each other session the stream names for a kind or a stage, each
once, so the line names every session a person can open for the
stream. Where no multiplexer was asked, the sessions come first and
`no profile for <multiplexer>` once after them. Under the printed
command, which holds no session, a stream's line reads `<stream> · no
session`. A host with no places to list prints `no places` before `streams`.

```console
$ flume list
/home/you/src/parser
  change--tabs-are-read-as-spaces · design · working
  fix--a-long-line-wraps · code · waiting for room · waits on the host
/home/you/src/site
  change--the-search-is-rebuilt · construct · held · waits on its caller
streams
  work · session work · runs
  personal · session personal · not running
  default · session flume · runs
```

With `--all`, the places that landed or were dropped are listed among
the others, in the same order:

```console
$ flume list --all
/home/you/src/parser
  change--tabs-are-read-as-spaces · design · working
  chore--the-docs-are-moved · landed · kept by sluice release
  fix--a-long-line-wraps · code · waiting for room · waits on the host
  fix--the-old-parser-is-removed · dropped · tidied
/home/you/src/site
  change--the-search-is-rebuilt · construct · held · waits on its caller
streams
  work · session work · runs · session work-design · not running
  personal · session personal · not running
  default · session flume · runs
```

The list reads each place's held state, and a live agent's asking mark
from its own record, and asks each stream's multiplexer once whether
each of its sessions runs, all of them in one reading. It reads no worktree, head, deliverable, change or
submission, asks no merge queue, and starts no session. None of a
place's long words, a stop-short's, a hold's, a failure's or a
teardown's, is in its lines or its JSON. It says what the last settle
left: where no caller keeps watch over the host, places move on only
at flume's commands and their agents' quiet. It settles, starts, sends and writes nothing.

With `--json` the answer is `{"list":{"places":[…],"streams":[…]}}`,
the same lines and their parts. The [state
reference](/flume/reference/state/#the-answer-of-flume-list-json)
lists the fields.

## `flume prompt`

```sh
flume prompt <place> <stage>
```

The exact prompt the stage would be sent at its next start, every part
filled, with nothing started, sent, settled or written. A prompt that
asks for a part with nothing to fill it is refused, naming the part.

## `flume tell`

```sh
flume tell <place> <words> [--within <time>] [--answers <key>]
```

Hands the place's current agent the words as a prompt once it has said
it started and waits, read back as received or sent once more. flume
waits up to ten minutes, or `--within`, for the agent to wait. It
refuses with nothing typed when the place has no live agent, its agent
is asking a person a question, or the words are not delivered within
the bound. A tell starts and ends no agent, and moves no stage but
one it answers.

To a stage that ended short needing an answer, while its agent is kept
open for it, the tell is the answer. flume types the question and the
words into that agent's session and records the answer on the
stop-short. The stage carries on there under the same start
([the answer in its session](/flume/guides/callers-and-holds/#the-answer-in-its-session)).
Once the agent is no longer kept open, the tell is refused with
nothing typed, naming the retry that gives the answer:
`flume retry <place> --words <answer>`. Under the printed command every
tell is refused, since flume holds no pane to type into.

`--answers <key>` names the stop-short the words answer, by the key the
inbox and status give it, as `stage/construct/1/ended-short`. flume
takes the answer only while the place waits on that stop-short and no
answer was taken for it, and otherwise refuses with nothing typed,
naming the answer taken, who gave it and when, or the move the place
waits on now ([the answer's key](/flume/guides/callers-and-holds/#the-answers-key)).

## `flume hold`

```sh
flume hold <place> --stage <stage> <words>
flume hold <place> --submission <words>
flume hold <place> --waiting <words> [--for <time>]
```

Holds a stage not started since the current stage began, taking one
waiting for room out of the queue, or the submission not yet made. It
refuses a stage started, a submission made and undecided, a second hold
and a place whose work is over. Only a retry naming the stage or the
submission ends the hold.

`--waiting` holds the move the place waits on its caller for, with
words saying who makes it and how, so the place is not stalled while
the hold holds. A later hold on the move replaces the words, the bound
and since when. The hold ends once the place waits on another move or
none. `--for <time>` gives how long it holds, as `--stall-after`
takes a time, with no days, `24h` for a day: once that passes, the
settle that reads it lapses the hold, records when, and stalls the
place at once with the hold's words. With no `--for` it holds until the
move is made. It refuses a place that waits on no move of its
caller's, and a `--for` beside `--stage` or `--submission`
([a hold on the waiting move](/flume/guides/callers-and-holds/#a-hold-on-the-waiting-move)).

## `flume retry`

```sh
flume retry <place> [--stage <stage>] [--words <words>] [--answers <key>] [--force]
flume retry <place> --submission
```

| Flag | |
|---|---|
| `--stage <stage>` | The stage to start again; the current one where none is named. The stages after it run again in order. |
| `--words <words>` | Words for the stage's agent, after the question its stage stopped on; the answer, to an agent kept open for it. |
| `--answers <key>` | The key of the stop-short the words answer, as the inbox and status give it. Taken only while the place waits on that stop-short, once, with words and naming its stage; refused otherwise, before any agent is ended, naming the answer taken, who gave it and when, or the move the place waits on now ([the answer's key](/flume/guides/callers-and-holds/#the-answers-key)). |
| `--force` | End an agent that is not quiet. |
| `--submission` | Submit the place's head again, starting no agent. |

A retry ends the place's agent first, starts the stage with a fresh
agent, and ends a hold on it. It never ends an agent whose stage has
started or begun: the retry is refused, forced or not, naming the
agent, and `flume stop <place>` ends it first.

A retry with `--words` naming a stage that ended short needing an
answer, while its agent is kept open for it, is the answer instead. It
starts and ends no agent. flume types the question and the words into
that agent's session as it types a tell, within ten minutes, and
records the answer on the stop-short. The stage carries on there under
the same start. An answer the agent does not take is refused with nothing
recorded, and the stage still waits
([the answer in its session](/flume/guides/callers-and-holds/#the-answer-in-its-session)).
A retry with no words runs as above, the question alone in the fresh
agent's prompt.

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
  `git worktree remove`, then whatever a tool left in its folder, and
  its repository's folder under `places/` once that holds no place;
- its branch, where the base holds every commit of it;
- in each multiplexer its agents opened in, under herdr its tab, and
  its base's workspace once that holds nothing else of flume's or
  yours.

It keeps a place
holding uncommitted or unignored work, and a branch with commits the
base lacks, naming each. What it closed or kept in a multiplexer is
named with that multiplexer, `profile` in the JSON form.

```console
$ flume tidy
removed ~/.local/share/flume/places/f5f5e14159a8/chore--tidy-the-readme
kept the branch chore/tidy-the-readme of ~/.local/share/flume/places/f5f5e14159a8/chore--tidy-the-readme, 1 commits not landed
closed the tab chore--tidy-the-readme of ~/.local/share/flume/places/f5f5e14159a8/chore--tidy-the-readme in the terminal herdr
```

## `flume settle`

```sh
flume settle [<place>] [--on-quiet]
flume settle --every <time>
```

Moves a place, or every place, on. It reads each stage's deliverable,
starts the next stage or the waiting ones there is room for, and
submits. It collects the merge queue's outcomes, delivers pending
ends, wakes roles and tidies. Safe to repeat. `--on-quiet` first reads the place's
agent until it is quiet, for up to sixty seconds; the Stop hook runs it.
No multiplexer and no plugin of one runs it.

`--every <time>` settles every place as a watch over the host. `<time>`
is the bound its runs come within, as `flume wait --timeout` takes one:
`<n>`, `<n>s`, `<n>m` or `<n>h`. flume records in the host's state when
the settle began and the bound, then settles every place, an agent in
it or none, and steps each open submission's base. Run it on that bound
from a timer of the host's or whatever else keeps time there. Every
place then moves on within the bound with no command of yours, a place
whose submission waits on the merge queue among them. While the newest such
settle began less than twice its bound ago, a caller keeps watch and
flume runs unattended: the commands that act settle no place first, and
`flume wait` leaves the settling to the watch. Once its runs stop, the
watch lapses and flume runs attended again. `flume status` gives the
watch, its bound and when it last settled, or since when none is kept.
`--every` beside a place's name, or with a bound that is no positive
duration, is refused before anything is read:

```console
$ flume settle fix--one --every 2m
refused: --every settles every place as a caller's watch over the host, and takes no place: fix--one was named
```

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
flume inbox [<place>] [--recipient <name>]
```

Every end pending for the recipient, on every place of the host, a
tidied place's kept record among them, each recorded delivered in the
same write, each with its key, a drop's or a done's among them. Prints
`nothing pending for <recipient>` when there is none. The recipient
defaults as for `flume wait`.

Naming a place, as `flume status` names one, the inbox collects that
place's ends alone: those of its record, or, once it was tidied, of the
records kept for it, `<place>@<at>` naming one record. It reads them
before it moves anything on, so it collects nothing tidied since. A
name under which no record stands is refused, and nothing is marked
delivered.

Beneath them, `waiting` gives every place, or the place named, that
waits on its caller and on the recipient: one it is a recipient of, one whose stop-short named
it, or one whose stall of the move it waits on now reached it as the
answerer. Each comes with its kind, name, stream and brief's first
line, the move and since when, the stop-short's words, key and the
role it asked, and a hold on the move that still holds. The inbox marks
nothing for these
([flume inbox](/flume/guides/callers-and-holds/#flume-inbox)).

## `flume wake`

```sh
flume wake <address>
```

Wakes a role's address, `<stream>/<role>`: a role a source of the host
declares in its roles file, for a stream the host declares
([roles and wake](/flume/guides/roles-and-wake/)). Where no run of the
address stands, it launches the run its source's entry gives, in the
address's stream. Otherwise it makes nothing, saying the run is live or
waits. It is refused, saying why, for a name that is no address, and
for an address that no source declares or two declare, whose stream the
host does not declare, whose entry is at fault, or that its source
declares carried, its run on another host:

```console
$ flume wake east/reviewer
refused: east/reviewer wakes nothing: no source of the host declares east/reviewer
$ flume wake north/planner
refused: north/planner wakes nothing on this host: acme declares it carried, its run on another host
```

An address its source gives an agent that already runs
([an address an agent already holds](/flume/guides/roles-and-wake/#an-address-an-agent-already-holds))
launches nothing. The wake settles as any act does, then answers that
the agent holds the address. It names the agent, its session and where
that came from, and what the multiplexer found there, with the ends
pending for it beneath:

```console
$ flume wake east/design
east/design is held by the standing agent east-design in the session acme (the entry's), none of that name there, so nothing was launched; 1 end pending
  ~/.local/share/flume/places/f5f5e14159a8/fix--the-parser-reads-tabs: submission landed d185a879b51dba33555c036392b50443f4dfc5e8
```

With `--json` the answer is `{"woken":{…}}` for a run and
`{"held":{…}}` for an agent that holds the address, as the [state
reference](/flume/reference/state/#the-answer-of-flume-wake-json)
gives them.

## `flume kinds lock`

```sh
flume kinds lock [--host] [--remove | --hash <hash>] <identity>…
```

Records each kind named, whole as `<source>/<name>@<version>`, at its
content's hash, beside any other version of the same kind the lock
holds, and over an entry of the same identity. A source's kind is read
as the host holds it, flume's own as flume carries it.

| Flag | |
|---|---|
| `--host` | Write the host's lock, `$XDG_CONFIG_HOME/flume/flume.lock`. Without it, the lock is `.config/flume.lock` in the working copy the command runs in, or `--repository`'s, which you commit. |
| `--remove` | Take each named entry out, reading no package, so a kind whose directory is gone can be taken out. |
| `--hash <hash>` | With one kind named, record it only where its content hashes to `<hash>`. Otherwise refuse, naming both hashes, and leave the lock as it was. |

It refuses, writing nothing:

- outside a working copy without `--host`, naming `--host`;
- an identity whose source is `repo` or `launch`, which no lock records;
- a kind no source holds, naming what its source does hold;
- a lock that cannot be read, which it never overwrites;
- `--hash` with more than one kind named.

`--remove` and `--hash` are never given together.

It writes the lock whole, beside it and renamed over it, while it holds
`$XDG_STATE_HOME/flume/kinds-lock.lock`, so two writers on the host
never lose each other's entries. It fetches nothing and commits
nothing, and run twice it leaves the same lock. The answer names the
lock and each entry recorded or removed:

```console
$ flume kinds lock --host acme/review@1
recorded acme/review@1 at sha256:ab12… in ~/.config/flume/flume.lock
$ flume kinds lock --host --remove acme/review@1
removed acme/review@1 from ~/.config/flume/flume.lock
$ flume kinds lock --host --remove acme/review@1
~/.config/flume/flume.lock is as it was
```

## `flume open`

```sh
flume open [<session>]
```

Shows a session of a stream's, by the session's name, in its stream's
view, and the `default` stream's own where none is named. It needs no
repository. What it opens is a session, never a stream:

- **A stream's own session**, the one its `session` names, else the
  one named by the stream's name, and, for `default` naming none, the
  multiplexer's own. So `flume open work` shows `work`'s own where
  `work` names none. Its window is titled with the stream's `title`,
  else its name.
- **A session a stream names for a kind or a stage**
  ([a stream's sessions](/flume/guides/agent-settings/#a-streams-sessions)).
  Its window is titled with the session's name, and the answer gives
  the kinds and stages the stream names it for, each with its layer.
  Where it is the stream's own session too, it is shown as the stream's
  own.

Under herdr, a session not yet running is started first, as an agent's
start would start it, and stays running.

- **View `terminal`**, the default. The stream's multiplexer gives the
  command that attaches to the session, and the stream's terminal app
  opens a window running it, under its title. Where no terminal app is
  named, flume opens no window and prints the command on its last line,
  after `run: `, for you or a launcher of your own to run.
- **View `app`.** The multiplexer's own app shows the session. No
  title is given and no terminal app is read.

[Configuration](/flume/reference/configuration/#host) gives the keys,
and [seeing a stream's agents](/flume/guides/places-and-herdr/#seeing-a-streams-agents)
the views and the terminal apps.

```console
$ flume open work
opened a ghostty window titled "Work Agents" (the stream's title) on herdr's session work (the stream's name)
  stream work, multiplexer herdr (the host's), view terminal (the default), terminal app ghostty (the host's)
  its command /usr/local/bin/herdr session attach work
$ flume open personal
the terminal app printed opens no window; run this line to show herdr's session personal (the stream's name) titled "personal" (the stream's name):
  stream personal, multiplexer herdr (the host's), view terminal (the default), terminal app printed (the default)
  its command /usr/local/bin/herdr session attach personal
run: /usr/local/bin/herdr session attach personal
$ flume open work-design
opened a ghostty window titled "work-design" (the session's name) on herdr's session work-design (the stream's for its kind)
  stream work, multiplexer herdr (the host's), view terminal (the default), terminal app ghostty (the host's)
  the stream names the session work-design for the kind research (from managed)
  its command /usr/local/bin/herdr session attach work-design
```

The answer gives the stream, its multiplexer, session, view, title and
terminal app, each with where it came from, the kinds and stages the
stream names the session for, the command and what the window did. A variable the command needs is named, never its value:
the printed line gives it as `<name>=<set>`. With `--json` the answer
is `{"opened":{…}}`:

| Field | |
|---|---|
| `stream` | the stream's name |
| `multiplexer` | `profile` and `from`: `stream`, `host` or `neither` |
| `session` | `name`, `null` for the multiplexer's own, and `from`: `stream`, `stream-name`, `own`, `stream-kind` or `stream-stage` |
| `named` | the kinds and stages the stream names the session for: `name`, `kinds` and `stages`, each `name`, the kind or `<kind>.<stage>`, and `layer`, `managed` or `local`; left out where the stream names it for none |
| `view` | `view`, `terminal` or `app`, and `from` |
| `title` | `title` and `from`: `stream`, `stream-name`, or `session` for a session the stream names for a kind or a stage; `null` for the view `app` |
| `terminal_app` | `profile` and `from`; `null` for the view `app` |
| `command` | `program`, by its full path, `arguments`, and `environment`, the names of the variables it needs; `null` for the view `app` |
| `window` | `"opened"`, `{"printed":{"line":…}}` or `"app"` |

Each asking opens a window of its own. flume keeps nothing of a window:
it never finds, raises or closes one, and opens one only at your
asking, never at a launch, a settle or a start. It takes no lock and
writes nothing it holds. It is refused, opening nothing, when:

- the name is no session a stream's word gives, a stream's name that
  is not one of its sessions' among them, naming each stream's
  sessions, before the multiplexer or a window is asked anything;
- the session is given by two streams' words, naming both;
- the stream's multiplexer or terminal app names no profile flume
  carries, naming the profiles there are;
- the multiplexer holds no session to show, as the printed command,
  whose agents run in the commands it printed;
- the view is `app` and the multiplexer has no app of its own, as
  herdr;
- the terminal app refuses, in its own words: Ghostty not installed,
  Terminal.app not allowed to be controlled, or Terminal.app on a host
  that is not macOS.

## `flume mcp`

Serves MCP on standard input and output with one tool, `status`, which
takes an optional place and answers as `flume status --json` does. It
settles nothing. flume's Claude Code plugin runs it as its `flume`
server.

## Run by agents

These run inside an agent flume started, where `FLUME_AGENT` and
`FLUME_SECRET` name its start. Elsewhere they refuse, or in the hook's
case do nothing but `session-start`, which collects for the session
([a session of your own](/flume/guides/claude-code/#a-session-of-your-own)).

| Command | |
|---|---|
| `flume needs [--found] [--to <role or address>] <words>` | The stage ends short with the words: it needs an answer. `--found` says it found work a stage before its own must do again instead. `--to` names the role the words are for, one the stage's prompt lists: a role alone, `<role>`, is the place's own stream's, and an address, `<stream>/<role>`, any stream's. The end goes to that role alone, in place of the place's recipients; a name the prompt did not list is refused, naming those it did, and the stage goes on ([asking a role](/flume/guides/callers-and-holds/#asking-a-role)). Where its multiplexer can type into a running agent, the agent is kept open for the answer, but for found work that starts the kind's fix round ([the agent kept open](/flume/guides/callers-and-holds/#the-agent-kept-open)). |
| `flume ack started --session <id> --transcript <path> --plugin-version <v> [--source <how>]` | The agent started under its identity and secret, in a session with a transcript. `--source` says how the session began, `startup` where none is given. flume takes it only where the agent runs the program its start recorded and every variable of the record matches. Otherwise it refuses, naming each variable that differed and whether the program did, and starts a settle of the place. |
| `flume ack received --digest <hex> [--mode <mode>]` | The agent received a prompt, by the SHA-256 digest of its text with paste tags taken off and whitespace collapsed. At the start's first prompt, where its start passed a mode, flume takes it only where `--mode` names that mode as the program names it. Otherwise it refuses, naming the mode asked and the one given, records the start begun without its settings and starts a settle of the place. A later prompt compares no mode. |
| `flume hook <event>` | What the hook for the event does with the input on standard input: `session-start`, `user-prompt-submit`, `pre-tool-use`, `permission-request`, `post-tool-use`, `stop`, `stop-failure` or `session-end`. Any other event does nothing. It always exits `0`. |
| `flume hook status-line` | Prints the line of the place `FLUME_PLACE` names, as `flume status <place> --line` gives it, ending `· this session is not its agent` where `FLUME_AGENT` is not the current stage's live start. It needs `FLUME_PLACE` and `FLUME_AGENT` alone, and reads no standard input. It prints nothing where they are missing or the place cannot be read, writes nothing, settles nothing and always exits `0`. |

flume's Claude Code plugin runs `flume hook` for every event, and its
module runs `flume hook status-line` to pin the place's line;
[the Claude Code plugin](/flume/guides/claude-code/#what-each-hook-does)
says what each does.
