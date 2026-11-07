# place-profile Specification

## Purpose
The place seam, Part B.1, through plain `git worktree`: a place made
inside no working tree of its repository on its own branch cut from the
base, its folder, and its worktree and branch removed at its tidy.

## Requirements

### Requirement: A place is a plain git worktree

The place profile SHALL make a place by `git worktree add -b
<prefix>/<name> <location> <revision>` at the location it gives the
place with no hook run, read it by `git status --porcelain=v2` and the
worktree's lock, remove it by the teardown it was handed and `git
worktree remove` with no force, and remove its branch by `git branch -D`
only when `git cherry` against the base names no commit the base lacks;
every operation all or nothing (78 to 83, C.1).

#### Scenario: Seam scenarios over the profile

- **WHEN** the place seam's scenarios run against the profile over scratch repositories
- **THEN** each passes: make touches no branch or location that exists, and each operation is all or nothing (83)

#### Scenario: An untracked file and a failed teardown

- **WHEN** one place holds a file neither committed nor ignored and another's teardown step fails
- **THEN** both are kept, naming the file and the step (S10)

#### Scenario: A branch with a commit its base lacks

- **WHEN** a dropped place's branch holds a commit whose change its base lacks, and another's commit was cherry-picked onto the base
- **THEN** the first branch is kept naming the commit, and the second deleted (67, S9)

### Requirement: Every place is made under flume's own data directory

The place profile SHALL give every place's location as
`$XDG_DATA_HOME/flume/places/<id>/<kind>--<name>`, the repository's id,
whether the repository's directory is a clone's main worktree or holds
its bare repository beside its worktrees; it SHALL read and remove a
place where it was made, never moving it (3, 78, C.1, C.4; record 0027;
core.40).

#### Scenario: A bare repository beside its worktrees

- **WHEN** a place is made of a directory holding `project.git` and its worktree `main`, and its location asked from the directory and from `main`
- **THEN** both give `<data>/places/<id>/fix--<name>`, the id the bare repository's, nothing is made under the directory, and the main worktree's status shows nothing

#### Scenario: A clone

- **WHEN** a place is made of a clone, and its location asked from a directory inside the clone
- **THEN** both give `<data>/places/<id>/fix--<name>`, outside the clone, the clone's status shows nothing, and another clone's place of the same name has another location

#### Scenario: A place made inside a clone's main worktree

- **WHEN** a place stands at `<clone>/places/fix--<name>`, as an earlier flume made it
- **THEN** it is read with its branch and head and removed there, its branch kept while it holds a commit the base lacks

#### Scenario: A place made beside a bare repository's worktrees

- **WHEN** a place stands at `<directory>/places/fix--<name>` beside a bare repository's worktrees, as an earlier flume made it
- **THEN** it is read with its branch and head and removed there, nothing written under flume's data directory, its branch kept while it holds a commit the base lacks

### Requirement: Readiness and teardown as the merge queue declares

The place profile SHALL run the readiness step and the teardown the
merge queue's profile declares, handed by the binary's resolver for the
revision the place's launch read, in the place, with an environment
cleared to the host user's login environment; a step's reading SHALL
give the readiness's steps; nothing of readiness SHALL be kept in the
place's git directory; with no merge queue bound nothing SHALL run (11,
12, 20, 66, 79, C.1, C.7, core.2).

#### Scenario: Readiness that fails at its toolchain

- **WHEN** a scratch repository's readiness step answers its toolchain failed with its words
- **THEN** ready gives the toolchain step and its words and no step after it, and nothing of the caller's environment reached the step (S16)

#### Scenario: No merge queue bound

- **WHEN** a place is made in a repository that binds no merge queue
- **THEN** ready runs nothing and remove runs no teardown step

#### Scenario: Readied at the revision the launch read

- **WHEN** a place was cut from main at R, main has since moved on in the repository, and the place is readied again for a retry
- **THEN** its readiness step is asked at R, not at main's revision then (12, 20)

#### Scenario: Nothing kept in the place's git directory

- **WHEN** a place's readiness passes
- **THEN** the place's git directory holds no `flume/toolchain.sh` and nothing else of the readiness's (C.7)

#### Scenario: A fresh clone's pins installed by the pinned sluice

- **WHEN** a scratch moon repository on main, whose `.prototools` pins a stand-in moon nothing has installed and whose toolchain puts a stand-in proto on the path and installs nothing, is cloned fresh, and a place of the clone is readied under the sluice the tree pins
- **THEN** ready gives `toolchain`, `proto install`, `moon --version` and `moon setup`, each passed, and the stand-in moon is installed where the stand-in proto installs (S89, 11, 79, C.1, C.6)

### Requirement: A base place's location is given and nothing made there

The git worktree profile SHALL give a base place the location it gives
any place of its kind and name, and SHALL make, read and remove nothing
for it: no `git worktree` command, no hook, no readiness and no
teardown runs for a base place, and no branch is made or moved by the
profile (3, 78, 202, C.1; core.40).

#### Scenario: Over a scratch repository

- **WHEN** a base place is launched and later tidied in a scratch repository whose fake merge queue opens and closes its branch
- **THEN** its location is under flume's data directory as any place's, nothing stands there at any moment, `git worktree list` names no worktree for it, and the repository's teardown never ran (3, 66, 78, 202)
