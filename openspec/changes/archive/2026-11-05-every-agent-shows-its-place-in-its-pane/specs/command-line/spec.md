## ADDED Requirements

### Requirement: status --line gives each place's line

`flume status [<place>] --line` SHALL give the line of the place named, or of every place on the host, each line's full form on a line of its own for a person and the lines as JSON with `--json`, reading only what a line holds and settling nothing (57, 60, 61).

#### Scenario: One place's line under the printed command

- **WHEN** `flume status fix--one --line` runs over a scratch repository whose place's code stage was started under the printed command, the terminal keeping no watch
- **THEN** it prints `fix--one · fix@1 · code · run 1 · starting` alone, `--json` gives the same line in `lines`, the place's held state is at the version it was before, and no agent was started (57)
