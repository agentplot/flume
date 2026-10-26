# Seam contracts

## Purpose

The seven seams of Part B as one interface each, with the types they
pass, so every profile of Part C codes against them and the core reaches
no tool but through them (core.1, core.2).

## ADDED Requirements

### Requirement: One interface per seam

There SHALL be exactly one interface for each of the seven seams of
Part B: the place (78 to 83), the kinds (84, 85), the change (86, 87),
the agent (88 to 93, 115, 116, 121), the terminal (94 to 98, 122), the
merge queue (99, 101, 117, 125) and the store (102 to 104). Each SHALL
offer the operations its section names and no operation of the core's,
and no interface SHALL name a tool (core.1, core.2).

#### Scenario: Each seam's operations

- **WHEN** each seam's scenarios run against its fake
- **THEN** each operation its section of Part B names is exercised, and each scenario cites the clause of its operation

#### Scenario: No tool named outside a profile

- **WHEN** the seams', core's, fakes' and suite's sources are searched for `wt`, `worktrunk`, `openspec`, `claude`, `herdr` and `sluice` as words
- **THEN** none is found

### Requirement: What the seams carry beyond Part B's list

The seams SHALL also answer what Part A and the rules need that Part B
lists under no operation: the merge queue where the base's current
revision lives, or a refusal naming why (9, core.7); the change's path
in a place, for a prompt's change part; and the store the place's own
folder beside its held state, for a report kept there (123).

#### Scenario: The base where the merge queue says it lives

- **WHEN** the fake merge queue says the base lives at a revision other than the local base's
- **THEN** the place is cut at that revision

#### Scenario: No base revision readable

- **WHEN** the fake merge queue refuses to give the base's revision, naming a failed fetch
- **THEN** the launch is refused naming the fetch, and nothing is made

### Requirement: The types the seams pass

The seams SHALL pass one type for each of: a launch (2, 3, 7, 8); a
place and its held state (62, 73 to 76, 102 to 104); a kind with its
stages as read-kinds returns them (18, 23); a deliverable in one of the
six forms (24); a prompt and its parts (27, 28); an agent's identity,
its start's secret and a stop-short (29, 30, 33, 41, 92); and a
submission with its outcomes (99, 111 to 113, 117, 125). Each SHALL
serialize to JSON and back unchanged.

#### Scenario: A type round-trips

- **WHEN** a value of each type is written to JSON and read back
- **THEN** it equals the value written

#### Scenario: A kind with two agents or two deliverables

- **WHEN** a kind's stage carries two agents or two deliverables as read-kinds returns them
- **THEN** the type keeps both, so the core can refuse the kind (4, 23)

### Requirement: Git's history and the host's facts beside the seams

The core SHALL read git's history and the host's facts through two
interfaces of their own, which are no seam of Part B (section 9): a
branch's head, whether a head holds commits another does not, the
newest commit at a head touching a path, whether a branch name is
valid; and the host's clock, a new secret, a file's write time, and
whether a command of flume's that waits is still live.

#### Scenario: A commit since the stage started

- **WHEN** the fake history holds a head that reaches a commit the stage's start head does not
- **THEN** the history answers that the head holds a commit since the start (25)

#### Scenario: An agent reset to an older commit

- **WHEN** the head is moved back to an ancestor of the start head
- **THEN** the history answers that the head holds no commit since the start
