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

- **WHEN** `flume launch --kind change --agent model=opus --agent verify.model=sonnet --agent env.A=1` runs
- **THEN** construct and code start with `opus`, verify with `sonnet`, every stage with `A=1`, each at the `launch` layer (7, 32)

#### Scenario: A place named from inside it

- **WHEN** `flume status fix--one --json` runs inside the place `places/fix--one`
- **THEN** it answers that place's status
