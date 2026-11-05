## ADDED Requirements

### Requirement: A branch read at origin through git

The core's reading of git's history SHALL read a branch at `origin`
after `git fetch origin +refs/heads/<branch>:refs/remotes/origin/<branch>`,
and never a tracking ref an earlier fetch left in its place. It SHALL
say origin lacks the branch only where that fetch fails and `git
ls-remote --exit-code` exits 2, say so of a repository with no
`origin`, and answer any other failed fetch with git's words (9, 126,
C.1).

#### Scenario: A branch at origin

- **WHEN** a scratch repository's branch is read at origin with no `origin`, then once a bare remote beside it holds the branch two commits past the repository's own, then for a branch the remote lacks, then with the remote out of reach
- **THEN** the first says no origin, the second gives the remote's head and leaves the repository's own branch where it was, the third says origin lacks it, and the fourth gives git's words (9, C.1)
