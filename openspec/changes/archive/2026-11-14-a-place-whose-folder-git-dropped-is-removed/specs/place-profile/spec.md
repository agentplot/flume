## ADDED Requirements

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

## MODIFIED Requirements

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
