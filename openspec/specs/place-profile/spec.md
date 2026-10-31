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

The place profile SHALL run the readiness steps and the teardown the
merge queue's profile declares for the repository, handed to it by the
binary's resolver, in the place, in order, with an environment cleared
to the host user's login environment; what the toolchain step makes
SHALL be kept in the place's own git directory; with no merge queue
bound nothing SHALL run (11, 12, 66, 79, C.1, C.7, core.2).

#### Scenario: Readiness that fails at its toolchain

- **WHEN** a scratch repository's toolchain step exits non-zero
- **THEN** ready gives the step and its words, the steps after it are not run, and nothing of the caller's environment reached the step (S16)

#### Scenario: No merge queue bound

- **WHEN** a place is made in a repository that binds no merge queue
- **THEN** ready runs nothing and remove runs no teardown step
