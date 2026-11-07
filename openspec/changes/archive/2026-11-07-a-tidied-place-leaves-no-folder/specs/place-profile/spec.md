## ADDED Requirements

### Requirement: Nothing of a removed place stands at its location

Once git holds no worktree at a place's location, after `git worktree
remove` whatever it exited or for a place read gone, the place
profile's remove SHALL take what still stands there, never through a
link, and then the place's group folder, `places/<id>` under flume's
own data directory, once that holds no other place; it SHALL refuse
where a `.git` file there names a git directory that still stands (62,
81, C.1; record 0057).

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
