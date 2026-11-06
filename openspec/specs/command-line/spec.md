# command-line Specification

## Purpose
flume's commands, each answering from the one core in one JSON form or
its printed text: launch, settle, status, the caller's acts on a place,
tidy, and the commands the plugin's hooks and an agent call.

## Requirements

### Requirement: Every command answers from the one core

The binary SHALL offer `launch`, `status`, `list`, `prompt`, `tell`,
`stop`, `retry`, `finish`, `drop`, `done`, `tidy`, `settle`, `needs`,
`priority` and `open`, each building the core over the host's profiles
and answering the core's answer, printed for a person or as its JSON
with `--json`, exiting non-zero on a refusal; no command SHALL compute a
stage's end, a status, a submission or a tidy of its own (56 to 61,
186, 200, section 9, core.14).

#### Scenario: The same answer through the command line and the core

- **WHEN** `flume status <place> --json` runs over a scratch repository and the core is asked the same in a test over the same state
- **THEN** the two answers are equal (S14, 60)

#### Scenario: A refusal

- **WHEN** `flume done` names a fix place
- **THEN** the JSON is a refusal naming the place and the exit is non-zero (S26, 61)

#### Scenario: The list through the command line and the core

- **WHEN** `flume list --json` runs over a scratch host and the core is asked the list in a test over the same state
- **THEN** the two answers are equal (60, 200)

### Requirement: needs takes the agent's identity and secret from its environment

`flume needs "<words>"` and `flume needs --found "<words>"` SHALL send a
stop-short that needs an answer or found work, with the identity and the
secret from `FLUME_AGENT` and `FLUME_SECRET`, refused when either is
missing or not the current start's (41, C.4).

#### Scenario: needs from outside an agent

- **WHEN** `flume needs "which design?"` runs with no `FLUME_AGENT`
- **THEN** it is refused and nothing changes

### Requirement: A place named by its location or its kind and name

A command SHALL name a place by its location or by `<kind>--<name>` of
the repository it runs in, the repository being the directory holding a
bare repository and its worktrees, or the main worktree; a place named
by `<kind>--<name>` SHALL be the one at the location the place profile
gives it, or, where the host's store holds none there and holds one at
`<repository>/places/<kind>--<name>`, as an earlier flume made it, that
one (C.1; record 0027).

#### Scenario: Holds and recipients at the launch

- **WHEN** `flume launch` runs with `--hold verify=after review --recipient madswan/planner` and then with neither, `FLUME_RECIPIENT` set
- **THEN** the first place's verify is on hold with the words `after review` and its recipient is `madswan/planner`, and the second's recipient is the variable's (7)

#### Scenario: Agent settings at the launch

- **WHEN** `flume launch --kind change --agent model=opus --agent verify.model=sonnet` runs, and then with `--agent env.A=1`
- **THEN** the first's construct and code start with `opus` and verify with `sonnet`, each at the `launch` layer, and the second is refused naming `env` as no setting `--agent` gives (7, 32)

#### Scenario: A place named from inside it

- **WHEN** `flume status fix--one --json` runs inside the place `fix--one` of a clone, and `flume status ./fix--one --json` in the directory holding it
- **THEN** each answers that place's status

#### Scenario: Every place under flume's data directory

- **WHEN** `flume launch --kind fix --name one` runs in a clone, and in the main worktree of a bare repository beside its worktrees
- **THEN** each place is under `$XDG_DATA_HOME/flume/places/<id>/`, the two at two locations, nothing in either repository's directory and neither's status showing one, and `flume status fix--one` in the main worktree and in the place answers the bare repository's

#### Scenario: A place made in the repository's directory, named by its kind and name

- **WHEN** a place stands at `<repository>/places/fix--one`, as an earlier flume made it inside a clone's main worktree or beside a bare repository's worktrees, and `flume status fix--one`, `flume launch --kind fix --name one`, `flume drop fix--one` and `flume tidy` run in the repository
- **THEN** status answers that place, the launch is refused naming `fix/one` and makes nothing, and the drop and the tidy remove its worktree and its branch, nothing moved under flume's data directory

### Requirement: A launch naming no base is cut from the default branch

`flume launch` with no `--base` SHALL take the repository's default
branch as its base, the remote's as the binding's read names it, and
SHALL be refused, naming why, where the repository names no default
branch (2, 20).

#### Scenario: A bare repository's HEAD on a branch never made

- **WHEN** a repository made by `git init --bare`, its `HEAD` naming `master`, a branch never made, with `origin` fetched and no `origin/HEAD` set, launches fix twice naming no base
- **THEN** each place's base is `main` and its branch is cut from `origin`'s `main`

### Requirement: launch takes a stream and an intent

`flume launch` SHALL take `--stream <stream>` and `--intent <name>`, and
the launch's JSON form the same two fields, each passed to the core as
the launch's; the binary SHALL choose neither itself (7, 143, 147).

#### Scenario: A stream and an intent named

- **WHEN** `flume launch` runs with `--stream personal --intent the-surfaces` over a scratch repository on a host declaring `personal`
- **THEN** the place's status names `personal` from `launch` and the intent `the-surfaces` from `launch` (143, 147)

### Requirement: launch gives a mode by --agent

`flume launch --agent [<stage>.]mode=<mode>` SHALL give one of flume's
four for every stage or for one, and `--agent
[<stage>.]mode.<program>=<value>` one entry of a table of programs,
several building one table; a word and a table for the same stage, or
for every stage, SHALL be refused naming both, and a bare value that is
not one of the four SHALL be refused naming it (7, 162, 169, C.2).

#### Scenario: A mode and a table at the launch

- **WHEN** `flume launch --kind fix --agent mode=edits --agent code.mode.claude=auto --agent code.mode.other=ask` runs over a scratch repository
- **THEN** the launch's settings give every stage `edits` and code the table `{ claude = "auto", other = "ask" }`, each at the `launch` layer (7, 32)

#### Scenario: A word and a table for one stage

- **WHEN** `flume launch --kind fix --agent code.mode=edits --agent code.mode.claude=auto` runs
- **THEN** it is refused naming both, and nothing is launched (7, 169)

#### Scenario: A program's own value given bare

- **WHEN** `flume launch --kind fix --agent mode=auto` runs
- **THEN** it is refused naming `auto` and saying it goes under its program's name, as `--agent mode.<program>=auto`, and nothing is launched (169)

### Requirement: launch names a session by --agent

`flume launch --agent [<stage>.]session=<name>` SHALL name the session
its place's agents open in, for every stage or for one, at the
`launch` layer, and `--agent`'s help and its refusals SHALL name
`session` among the settings it gives (7, 32, 107).

#### Scenario: A session for every stage and for one

- **WHEN** `flume launch --kind change --agent session=review --agent verify.session=late` runs over a scratch repository
- **THEN** the launch's settings give every stage `review` and verify `late`, each at the `launch` layer (7, 107)

#### Scenario: The settings named in a refusal

- **WHEN** `flume launch --kind fix --agent colour=blue` runs
- **THEN** it is refused naming `colour`, and its words name `session` among the settings `--agent` gives, and nothing is launched (7, 32)

### Requirement: status --line gives each place's line

`flume status [<place>] --line` SHALL give the line of the place named, or of every place on the host, each line's full form on a line of its own for a person and the lines as JSON with `--json`, reading only what a line holds and settling nothing (57, 60, 61).

#### Scenario: One place's line under the printed command

- **WHEN** `flume status fix--one --line` runs over a scratch repository whose place's code stage was started under the printed command, the terminal keeping no watch
- **THEN** it prints `fix--one · fix@1 · code · run 1 · starting` alone, `--json` gives the same line in `lines`, the place's held state is at the version it was before, and no agent was started (57)

### Requirement: open names a stream or none

`flume open [<session>]` SHALL show the session of that name in its
stream's view, the default stream's own where it names none, needing no
repository, its stream's multiplexer and terminal app bound through the
resolver for that stream alone, and printing the printed window's line
on a line of its own; its help SHALL say the name is a session's, a
stream's own or one a stream names for a kind or a stage (61, 105, 186,
190, 199, C.8).

#### Scenario: open under the printed command

- **WHEN** `flume open work --json` runs on a host declaring `work` whose multiplexer is the printed command
- **THEN** the JSON is a refusal saying the printed command holds no session, and the exit is non-zero (187)

#### Scenario: open of an undeclared stream

- **WHEN** `flume open nightly` runs outside any repository on a host that declares no `nightly`
- **THEN** it is refused naming `nightly`, and the exit is non-zero (4, 186)

#### Scenario: A stream's view through the binary

- **WHEN** `flume open w` runs on a host whose multiplexer is herdr under a scratch session home, its terminal app `ghostty` over a stand-in, `[streams.w]` titled `Work Herdr`; then a fix is launched on `w` and `flume open w` runs again; then `flume open o` runs for `[streams.o]` naming the terminal app `terminal` and the title `Office Herdr`
- **THEN** the stand-in records two windows titled `Work Herdr`, each running the pinned herdr's `session attach w`, the fix's agent opens in the session `w`, and on macOS the stand-in `osascript` records `exec <herdr> session attach o` under `Office Herdr`, and elsewhere `o`'s open is refused saying Terminal.app runs only on macOS (186 to 189, S90)

#### Scenario: The printed window through the binary

- **WHEN** `flume open w` runs on the same host naming no terminal app
- **THEN** it prints the line `<herdr> session attach w` preceded by `XDG_CONFIG_HOME=<set>`, the exit is zero, and nothing is run (188, S90)

#### Scenario: Two streams under two multiplexers through the binary

- **WHEN** the host's multiplexer is herdr, `[streams.t]` names the printed command, and a fix is launched on `w` and one on `t`, and then `flume status --json` runs
- **THEN** the `w` fix's agent opens in herdr's session `w`, the `t` fix's command is printed, and status gives each agent's terminal and each stream's multiplexer with where it came from (190, S91)

#### Scenario: A design session through the binary

- **WHEN** the same host's `[streams.w.kinds.research]` gives `session = "w-design"`, a research place and a fix are launched on `w`, `flume open w-design --json` runs, then `flume open w`, each place's agent is read by `flume status --json` and the places are tidied
- **THEN** the pinned herdr lists the sessions `w-design` and `w` running, the research agent's pane on `w-design`'s server in its place's tab and the fix's on `w`'s; the open's JSON gives the stream `w`, the session `w-design` from `stream-kind` named for research and the title `w-design`, and the stand-in records a window titled `w-design` running `session attach w-design` and one titled `Work Herdr` running `session attach w`; status gives each agent's session with its origin; and after the tidies no server holds a tab or a workspace of flume's (107, 137, 138, 186, 190, 199, S106, S107)

#### Scenario: A name no stream's word gives

- **WHEN** `flume open nightly` runs over a scratch host whose `work` names `work-design` for research
- **THEN** it exits refused naming `nightly`, `work` and `work-design`, and starts no session (186, 199)

### Requirement: list needs no repository and runs no tool

`flume list` SHALL take no argument but `--all`, need no repository and
ask the core's list, of every place where `--all` is given and
otherwise leaving out a place that landed or was dropped, settling
nothing first, running no tool but each stream's terminal's runs,
writing nothing and exiting 0 (57, 63, 200; core.52).

#### Scenario: A fix under the printed command, outside any repository

- **WHEN** a fix named one is launched over a scratch repository under the printed command, its code agent not yet said started, and `flume list` then runs outside any repository with `PATH` set to an empty scratch folder
- **THEN** it exits 0 having printed the fix's line, and leaves every file under the host's state as it was (57, 200, 201)

#### Scenario: Fifty places within a second

- **WHEN** a scratch host's state holds fifty places' documents in three repositories, written from a launched place's under fifty names, and `flume list --json` runs
- **THEN** it answers fifty places' lines within a second of its start and exits 0 (200, S108)

#### Scenario: A dropped place listed only with --all

- **WHEN** a scratch host's state holds the fix named one, launched under the printed command, and a document written from its as the fix named gone whose phase says it was dropped, and `flume list --json` and then `flume list --all --json` run outside any repository
- **THEN** the first gives `fix--one · code · starting` alone, the second `fix--gone · dropped` and then `fix--one · code · starting`, and each exits 0 (63, 200)

### Requirement: list prints each repository's places, then the streams

`flume list` SHALL print each repository's path on a line of its own
with its places' lines under it, then `streams` with each stream's line
under it, each line indented by two spaces, and `no places` before
`streams` where the host holds none; or the answer as JSON with
`--json` (61, 200).

#### Scenario: The person's form under the printed command

- **WHEN** `flume list` runs over the scratch host holding the fix named one, and over a scratch host holding no place
- **THEN** the first prints the repository's path, then `  fix--one · code · starting`, then `streams` and `  default · no session`; the second prints `no places`, then `streams` and `  default · no session` (200, 201)

### Requirement: list's help names status for a place's every fact

`flume list --help` SHALL say the list gives a line for each place and
each stream, reading only what flume keeps, that a place that landed or
was dropped is left out unless `--all` is given, that `--all` gives
every place, and that `flume status <place>` gives a place's every fact
(63, 200; core.35).

#### Scenario: The list's help

- **WHEN** `flume list --help` runs
- **THEN** it says the list gives a line for each place and each stream, reading only what flume keeps, a place that landed or was dropped left out unless `--all` is given, names `--all` for every place, those that landed or were dropped among them, and names `flume status <place>` for a place's every fact (63, 200)
