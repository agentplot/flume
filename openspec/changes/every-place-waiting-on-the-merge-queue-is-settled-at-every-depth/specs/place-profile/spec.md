## ADDED Requirements

### Requirement: A base place's worktree is detached

The git worktree profile SHALL make a place given no branch by `git
worktree add --detach <location> <revision>`, at the location it gives
any place, with no hook run and no branch made or moved; its read SHALL
give what the worktree holds for the tidy as for any place, and its
teardown SHALL be any place's (65, 66, 78, 202, C.1).

#### Scenario: Over a scratch repository

- **WHEN** the profile makes a place with no branch at `main`'s revision in a scratch repository, then `main` moves on by a commit
- **THEN** the worktree stands detached at the first revision, `git status --porcelain=v2` in it reads clean, the place's read names no change, lock or unignored file, and its removal goes through the repository's teardown (65, 78)
