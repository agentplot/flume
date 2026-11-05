# command-line Specification

## Purpose
flume's commands, each answering from the one core in one JSON form or
its printed text: launch, settle, status, the caller's acts on a place,
tidy, and the commands the plugin's hooks and an agent call.

## Requirements

### Requirement: Every command answers from the one core

The binary SHALL offer `launch`, `status`, `prompt`, `tell`, `stop`,
`retry`, `finish`, `drop`, `done`, `tidy`, `settle`, `needs`,
`priority` and `open`, each building the core over the host's profiles
and answering the core's answer, printed for a person or as its JSON
with `--json`, exiting non-zero on a refusal; no command SHALL compute a
stage's end, a status, a submission or a tidy of its own (56 to 61,
186, section 9, core.14).

#### Scenario: The same answer through the command line and the core

- **WHEN** `flume status <place> --json` runs over a scratch repository and the core is asked the same in a test over the same state
- **THEN** the two answers are equal (S14, 60)

#### Scenario: A refusal

- **WHEN** `flume done` names a fix place
- **THEN** the JSON is a refusal naming the place and the exit is non-zero (S26, 61)

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

`flume open [<stream>]` SHALL show the stream's view, `default`'s where
it names none, needing no repository, its multiplexer and terminal app
bound through the resolver for that stream alone, and printing the
printed window's line on a line of its own (61, 105, 186, 190, C.8).

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
