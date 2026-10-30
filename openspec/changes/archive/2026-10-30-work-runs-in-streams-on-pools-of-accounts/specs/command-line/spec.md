## MODIFIED Requirements

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

## ADDED Requirements

### Requirement: launch takes a stream and an intent

`flume launch` SHALL take `--stream <stream>` and `--intent <name>`, and
the launch's JSON form the same two fields, each passed to the core as
the launch's; the binary SHALL choose neither itself (7, 143, 147).

#### Scenario: A stream and an intent named

- **WHEN** `flume launch` runs with `--stream personal --intent the-surfaces` over a scratch repository on a host declaring `personal`
- **THEN** the place's status names `personal` from `launch` and the intent `the-surfaces` from `launch` (143, 147)
