## ADDED Requirements

### Requirement: settle --every keeps the host's watch

`flume settle --every <bound>` SHALL settle every place on the host as
`flume settle` naming no place does, recording first the host's watch
with its bound, a duration as `flume wait --timeout` takes it; it SHALL
be refused, naming why, beside a place's name or with a bound that is
not a positive duration (61, 119, 208).

#### Scenario: A watch over a scratch repository

- **WHEN** `flume settle --every 2m` runs on a scratch host holding one submitted place, and `flume status --json` runs after it
- **THEN** the settle stepped the place's base once, and status gives the host's `watch` with a bound of 120 seconds and `watch_settled` when the settle began (56, 208)

#### Scenario: A watch with a place named

- **WHEN** `flume settle fix--one --every 2m` runs
- **THEN** it is refused naming `--every` and the place, nothing is settled, and no watch is recorded (208)

## MODIFIED Requirements

### Requirement: status --line gives each place's line

`flume status [<place>] --line` SHALL give the line of the place named, or of every place on the host, each line's full form on a line of its own for a person and the lines as JSON with `--json`, reading only what a line holds and settling nothing, whether or not a caller keeps watch (57, 60, 61, 208).

#### Scenario: One place's line under the printed command

- **WHEN** `flume status fix--one --line` runs over a scratch repository whose place's code stage was started under the printed command, no caller keeping watch
- **THEN** it prints `fix--one · fix@1 · code · run 1 · starting` alone, `--json` gives the same line in `lines`, the place's held state is at the version it was before, and no agent was started (57)
