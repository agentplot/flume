## Purpose

A place's one line: its kind and name, its kind's version, its current
stage with a repeating stage's group and how many times it started, its
change's tasks ticked, and its state in one word; read from the place's
held state, the change at its head and its agent's own record alone,
never the terminal or the merge queue, so it can be asked at every
render of an agent's pane; and the same words in status, the status
line, the sidebar and the popup.

## ADDED Requirements

### Requirement: A place's line holds its parts

The core SHALL give every place one line: `<kind>--<name>`, `<kind>@<version>`, its current stage as its label gives it, a repeating stage's group with it, `run <n>` for that stage's starts, `tasks <t>/<n>` for its change's tasks at the head, and its state, joined by ` · `, a part it has none of left out; and the same line less its name and kind as its short form (48, 56, 139, 142).

#### Scenario: A change@2 place at its code stage's first group

- **WHEN** a change@2 place named parser runs its code stage's first repetition, its change holding two task groups and three tasks, none ticked, its agent started
- **THEN** its line is `change--parser · change@2 · code 1/2 · run 1 · tasks 0/3 · working` and its short form `code 1/2 · run 1 · tasks 0/3 · working` (142)

#### Scenario: A fix with no change

- **WHEN** a fix place named parser runs its code stage's second start, its agent started
- **THEN** its line is `fix--parser · fix@1 · code · run 2 · working`, with no tasks part (48)

#### Scenario: A landed place

- **WHEN** a fix place named parser landed and was tidied, its document kept for an end pending
- **THEN** its line is `fix--parser · fix@1 · landed`, with no stage, run or tasks part (63)

### Requirement: A place's state in one word

The line's state SHALL be the first that holds of: being made; landed, dropped or done; stalled; held; waiting on an answer; asking; lost; never begun; not ready; refused; stopped; past its bound; submission refused; waiting on done; waiting for room; starting; asking, where the live agent's profile reads it asking; stuck; in conversation; working; submitted; else the current stage's state in words (12, 27, 34, 38, 42, 45, 47, 48, 51, 63, 70, 90, 112, 114, 118, 130).

#### Scenario: A hold, then a stall

- **WHEN** a place's code stage is on hold under no words, and five minutes later is stalled
- **THEN** its line's state is `held`, and then `stalled` (47, 130)

#### Scenario: A stop-short that needs an answer

- **WHEN** a place's construct stage stopped short needing an answer
- **THEN** its line's state is `waiting on an answer` (42)

#### Scenario: An agent asking a person

- **WHEN** a place's code agent has started and its profile's asking mark stands
- **THEN** its line's state is `asking`, and once the mark is cleared, `working` (90)

#### Scenario: Starting and waiting for room

- **WHEN** one place's code agent is opened and has not said it started, and another place's first stage waits for room on the host
- **THEN** the first's line's state is `starting` and the second's `waiting for room` (35, 70)

#### Scenario: Submitted, then landed

- **WHEN** a fix place's last stage delivered and flume submitted it, and a later settle reads its landing
- **THEN** its line's state is `submitted`, and after that settle `landed` (48, 63)

### Requirement: A line asked alone reads only what it holds

A line asked alone SHALL read the place's held state, the change at the place's head where its kind has a change, and the live agent's asking mark from its profile, and nothing else: no terminal operation, no merge-queue operation and no host reading. It SHALL write nothing and start nothing, and a change it cannot read SHALL leave the tasks and the group out (57, 58).

#### Scenario: Nothing called and nothing written

- **WHEN** a place's line is asked alone over the fakes while its code agent works
- **THEN** the fake terminal and the fake merge queue record no call, and the fake store's version of the place is unchanged (57)

#### Scenario: A change that cannot be read

- **WHEN** a change@2 place's line is asked while the fake change cannot read its change
- **THEN** the line is given with no tasks part and the stage's name alone (58)

### Requirement: A session that is not its stage's agent says so

Where the line is asked for a session whose identity is not the current stage's live start, its full form SHALL end `· this session is not its agent`, and its short form SHALL not (35, 140).

#### Scenario: A repetition's earlier agent

- **WHEN** a change@2 place's code stage moved to its second repetition while its first repetition's agent is still quiet, and the line is asked with the first agent's identity
- **THEN** the line names `code 2/2` and ends `· this session is not its agent` (140)

### Requirement: The line alone and in status agree

A place's line in status SHALL be the line asked alone at the same state, so the command line, the MCP status, the sidebar and the popup give one place's line in the same words (60; S14).

#### Scenario: Asked two ways

- **WHEN** status and the line alone are asked of one place over the same fakes
- **THEN** status's line equals the line asked alone (S14)
