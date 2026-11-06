## MODIFIED Requirements

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

## ADDED Requirements

### Requirement: list needs no repository and runs no tool

`flume list` SHALL take no argument, need no repository and ask the
core's list, settling nothing first, running no tool but each stream's
terminal's runs, writing nothing and exiting 0 (57, 200; core.52).

#### Scenario: A fix under the printed command, outside any repository

- **WHEN** a fix named one is launched over a scratch repository under the printed command, its code agent not yet said started, and `flume list` then runs outside any repository with `PATH` set to an empty scratch folder
- **THEN** it exits 0 having printed the fix's line, and leaves every file under the host's state as it was (57, 200, 201)

#### Scenario: Fifty places within a second

- **WHEN** a scratch host's state holds fifty places' documents in three repositories, written from a launched place's under fifty names, and `flume list --json` runs
- **THEN** it answers fifty places' lines within a second of its start and exits 0 (200, S108)

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
each stream, reading only what flume keeps, and that `flume status
<place>` gives a place's every fact (200; core.35).

#### Scenario: The list's help

- **WHEN** `flume list --help` runs
- **THEN** it says the list gives a line for each place and each stream, reading only what flume keeps, and names `flume status <place>` for a place's every fact (200)
