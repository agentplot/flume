# history-and-host Specification

## Purpose
Git's history and the host's facts read for real beside the seams: the
commits of a place since its stage started and the repository's common
git directory; the host's clock, its secrets, the commands of flume's
that live, the bound its configuration states and the lock over its
room.

## Requirements

### Requirement: Git's history read through git

The core's reading of git's history SHALL go through `git` in the
repository: a branch's head, whether a head holds a commit a start does
not, the newest commit at a head touching a path and whether it is since
a start, whether a path is in a commit since a start, and whether a
branch name is valid (24, 25, C.1).

#### Scenario: The history scenarios over git

- **WHEN** the history scenarios run against the reader over scratch repositories
- **THEN** each passes, a head reset to an ancestor of the start holding nothing since it

### Requirement: The default branch read through git

The repository's default branch SHALL be read through `git` as the one
`origin/HEAD` names, set from `origin`'s own `HEAD` where the repository
lacks it, and as the one its common git directory's `HEAD` names only
where it has no `origin`, never a branch that names no commit (20, 132,
C.7).

#### Scenario: The default branch is the remote's

- **WHEN** a repository made by `git init --bare`, its `HEAD` naming `master`, a branch never made, is given `origin`, whose default branch is `main`, and is fetched with no `origin/HEAD` set, and its default branch is read twice, then again with `origin` removed, then again with its `HEAD` naming `main`
- **THEN** the first two reads give `main` and `origin/HEAD` names `origin/main` after the first, the third gives none, and the fourth gives `main` (20, 132, C.7)

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

### Requirement: The host's streams and pools

The host's facts SHALL give, as its configuration stands when asked,
its declared streams, the stream it names for a repository and kind
and the pool for a stream, each with the key it came from, and each
pool by name; none SHALL be kept between asks (32, 143, 144, C.7).

#### Scenario: A stream's pool and the host's

- **WHEN** the configuration gives `[streams.work] pool = "work"` and `[host] pool = "personal"`, and declares `personal` with no pool
- **THEN** the host names `work` for `work` from the stream, and `personal` for `personal` and `default` from the host (144)
