## MODIFIED Requirements

### Requirement: A place is a plain git worktree

The place profile SHALL make a place by `git worktree add -b
<prefix>/<name> <location> <revision>` at the location it gives the
place with no hook run, read it by `git status --porcelain=v2` and the
worktree's lock, remove it by the teardown it was handed and `git
worktree remove` with no force, and remove its branch by `git branch -D`
only when `git cherry` against the base names no commit the base lacks;
every operation all or nothing (78 to 83, C.1). Once git holds no
worktree at a place's location, after `git worktree remove` whatever it
exited or for a place read gone, its remove SHALL take what still
stands there, never through a link, and then the place's group folder,
`places/<id>` under flume's own data directory, once that holds no other
place; it SHALL refuse where a `.git` file there names a git directory
that still stands (62, 81; record 0057).

#### Scenario: Seam scenarios over the profile

- **WHEN** the place seam's scenarios run against the profile over scratch repositories
- **THEN** each passes: make touches no branch or location that exists, and each operation is all or nothing (83)

#### Scenario: An untracked file and a failed teardown

- **WHEN** one place holds a file neither committed nor ignored and another's teardown step fails
- **THEN** both are kept, naming the file and the step (S10)

#### Scenario: A branch with a commit its base lacks

- **WHEN** a dropped place's branch holds a commit whose change its base lacks, and another's commit was cherry-picked onto the base
- **THEN** the first branch is kept naming the commit, and the second deleted (67, S9)

#### Scenario: What git could not delete

- **WHEN** a place holds a devenv shell's ignored `.devenv` and `.moon`, a link among them to a folder outside it, and a folder of ignored state it may not write in, and is removed
- **THEN** `git worktree remove` fails having dropped its entry, the remove passes, nothing stands at the location, the empty group folder is gone, and the folder the link pointed at keeps what it holds (62)

#### Scenario: What a process wrote at a gone place's location

- **WHEN** a place's worktree is removed and a devenv shell's `.devenv` and `.moon` are written at its location afterwards
- **THEN** the place reads gone, its remove passes and takes the folder and the empty group folder, never what the link points at, and a second remove passes (62, 74)

#### Scenario: A group folder holding another place

- **WHEN** one of two places of a repository is removed
- **THEN** its folder is gone and the group folder stays, the other place read live in it

#### Scenario: Another repository's worktree at a place's location

- **WHEN** another repository holds a worktree at a place's location
- **THEN** the place reads gone and its remove is refused, naming the git directory that holds it, the worktree left whole (64)
