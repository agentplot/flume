## MODIFIED Requirements

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

### Requirement: list's help names status for a place's every fact

`flume list --help` SHALL say the list gives a line for each place and
each stream, reading only what flume keeps, that a place that landed or
was dropped is left out unless `--all` is given, that `--all` gives
every place, and that `flume status <place>` gives a place's every fact
(63, 200; core.35).

#### Scenario: The list's help

- **WHEN** `flume list --help` runs
- **THEN** it says the list gives a line for each place and each stream, reading only what flume keeps, a place that landed or was dropped left out unless `--all` is given, names `--all` for every place, those that landed or were dropped among them, and names `flume status <place>` for a place's every fact (63, 200)
