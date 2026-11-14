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

### Requirement: Nothing of a removed place stands at its location

Once git holds no worktree at a place's location, after a `git worktree
remove` that passed, at a later remove after one that failed, or for a
place read gone, the place profile's remove SHALL take what still
stands there, never through a link, and then the place's group folder,
`places/<id>` under flume's own data directory, once that holds no
other place; it SHALL refuse where a `.git` file there names a git
directory that still stands (62, 81, C.1; record 0057).

#### Scenario: What git could not delete

- **WHEN** a place holds a devenv shell's ignored `.devenv` and `.moon`, a link among them to a folder outside it, and a folder of ignored state it may not write in, and is removed, then removed again
- **THEN** `git worktree remove` fails having dropped its entry and the first remove is refused, the folder still standing; the second remove passes, nothing stands at the location, the empty group folder is gone, and the folder the link pointed at keeps what it holds (62; `what_git_left_of_a_failed_removal_goes_at_the_next_remove`)

#### Scenario: What a process wrote at a gone place's location

- **WHEN** a place's worktree is removed and a devenv shell's `.devenv` and `.moon` are written at its location afterwards
- **THEN** the place reads gone, its remove passes and takes the folder and the empty group folder, never what the link points at, and a second remove passes (62, 74)

#### Scenario: A group folder holding another place

- **WHEN** one of two places of a repository is removed
- **THEN** its folder is gone and the group folder stays, the other place read live in it

#### Scenario: Another repository's worktree at a place's location

- **WHEN** another repository holds a worktree at a place's location
- **THEN** the place reads gone and its remove is refused, naming the git directory that holds it, the worktree left whole (64)

### Requirement: What git left of a failed removal goes at a later remove

Where `git worktree remove` fails and git then lists no worktree at the
place's location, the place profile's remove SHALL be refused at that
step with git's words, saying git no longer lists the place and what is
left goes at the next settle, and SHALL remove nothing more in that
call; the place SHALL then read gone, and a later remove SHALL take
what is left with no teardown run again (66, 76, 80, 81).

#### Scenario: A failed removal refused

- **WHEN** a place whose teardown marks each run holds a folder of ignored state it may not write in, and is removed, then removed twice more
- **THEN** the first remove is refused naming git's words and that git no longer lists the place, its folder still stands and it reads gone; the second and third removes pass, and the teardown ran once (74; `what_git_left_of_a_failed_removal_goes_at_the_next_remove` in `crates/flume/tests/profiles/place_git_worktree.rs`)

### Requirement: Only a folder flume made goes from a place's location

The place profile's remove SHALL take what stands at a place's location
only where the location is directly in a group folder under flume's own
`places/`, or in its repository's `places/` where an earlier flume made
it, and where it holds no `.git` folder and no `.git` file but one
naming the repository's own worktree entry that no longer stands;
otherwise it SHALL be refused naming why, what stands left whole (64;
core.17, core.18).

#### Scenario: A folder flume did not make

- **WHEN** a place's location is outside every `places/` folder and holds a file; another's holds a worktree of another repository whose entry that repository dropped; another's holds a repository of its own
- **THEN** each reads gone, its remove is refused naming why, and what stands at each keeps what it holds (64; `only_a_folder_flume_made_goes_from_a_places_location` in `crates/flume/tests/profiles/place_git_worktree.rs`)

#### Scenario: The place's own dropped worktree

- **WHEN** git's entry for a place's worktree is gone while the place's folder still holds its files and its `.git` file
- **THEN** the place reads gone and its remove passes, nothing standing at the location (62; `only_a_folder_flume_made_goes_from_a_places_location`)
