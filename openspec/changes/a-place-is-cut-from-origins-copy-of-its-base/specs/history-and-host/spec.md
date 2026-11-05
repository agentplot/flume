## MODIFIED Requirements

### Requirement: Git's history read through git

The core's reading of git's history SHALL go through `git` in the
repository: a branch's head, a branch at `origin` after `git fetch
origin +refs/heads/<branch>:refs/remotes/origin/<branch>`, whether a
head holds a commit a start does not, the newest commit at a head
touching a path and whether it is since a start, whether a path is in a
commit since a start, and whether a branch name is valid. A branch at
origin SHALL be said lacking only where the fetch fails and `git
ls-remote --exit-code` exits 2, a repository with no `origin` said so,
and any other failed fetch answered with git's words; a tracking ref an
earlier fetch left SHALL never be read in its place (9, 24, 25, C.1).

#### Scenario: The history scenarios over git

- **WHEN** the history scenarios run against the reader over scratch repositories
- **THEN** each passes, a head reset to an ancestor of the start holding nothing since it

#### Scenario: A branch at origin

- **WHEN** a scratch repository's branch is read at origin with no `origin`, then once a bare remote beside it holds the branch two commits past the repository's own, then for a branch the remote lacks, then with the remote out of reach
- **THEN** the first says no origin, the second gives the remote's head and leaves the repository's own branch where it was, the third says origin lacks it, and the fourth gives git's words (9, C.1)
