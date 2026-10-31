# command-line Specification

## Purpose
flume's commands, each answering from the one core in one JSON form or
its printed text: launch, settle, status, the caller's acts on a place,
tidy, and the commands the plugin's hooks and an agent call.

## Requirements

### Requirement: Every command answers from the one core

The binary SHALL offer `launch`, `status`, `prompt`, `tell`, `stop`,
`retry`, `finish`, `drop`, `done`, `tidy`, `settle`, `needs` and
`priority`, each building the core over the host's profiles and
answering the core's answer, printed for a person or as its JSON with
`--json`, exiting non-zero on a refusal; no command SHALL compute a
stage's end, a status, a submission or a tidy of its own (56 to 61,
section 9, core.14).

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

A command SHALL name a place by its location or by `<kind>--<name>` in
the repository it runs in, the repository being the directory holding a
bare repository and its worktrees, or the main worktree.

#### Scenario: Holds and recipients at the launch

- **WHEN** `flume launch` runs with `--hold verify=after review --recipient madswan/planner` and then with neither, `FLUME_RECIPIENT` set
- **THEN** the first place's verify is on hold with the words `after review` and its recipient is `madswan/planner`, and the second's recipient is the variable's (7)

#### Scenario: Agent settings at the launch

- **WHEN** `flume launch --kind change --agent model=opus --agent verify.model=sonnet` runs, and then with `--agent env.A=1`
- **THEN** the first's construct and code start with `opus` and verify with `sonnet`, each at the `launch` layer, and the second is refused naming `env` as no setting `--agent` gives (7, 32)

#### Scenario: A place named from inside it

- **WHEN** `flume status fix--one --json` runs inside the place `places/fix--one`
- **THEN** it answers that place's status

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
