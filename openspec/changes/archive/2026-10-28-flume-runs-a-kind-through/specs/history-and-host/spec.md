## ADDED Requirements

### Requirement: Git's history read through git

The core's reading of git's history SHALL go through `git` in the
repository: a branch's head, whether a head holds a commit a start does
not, the newest commit at a head touching a path and whether it is since
a start, whether a path is in a commit since a start, and whether a
branch name is valid (24, 25, C.1).

#### Scenario: The history scenarios over git

- **WHEN** the history scenarios run against the reader over scratch repositories
- **THEN** each passes, a head reset to an ancestor of the start holding nothing since it

### Requirement: The host's facts

The host's facts SHALL be its clock, a sleep, a secret of 32 random
bytes, a file's write time, a command of flume's named by process and
start time with whether it lives, the bound its configuration states,
and a host-wide lock over the room (41, 70, 75).

#### Scenario: Secrets never repeat

- **WHEN** a thousand secrets are taken
- **THEN** no two are equal

#### Scenario: The room lock is exclusive

- **WHEN** two processes take the room lock at once
- **THEN** the second takes it only after the first lets it go
