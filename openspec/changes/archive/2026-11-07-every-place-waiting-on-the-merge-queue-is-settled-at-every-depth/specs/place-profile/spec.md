## ADDED Requirements

### Requirement: A base place's location is given and nothing made there

The git worktree profile SHALL give a base place the location it gives
any place of its kind and name, and SHALL make, read and remove nothing
for it: no `git worktree` command, no hook, no readiness and no
teardown runs for a base place, and no branch is made or moved by the
profile (3, 78, 202, C.1; core.40).

#### Scenario: Over a scratch repository

- **WHEN** a base place is launched and later tidied in a scratch repository whose fake merge queue opens and closes its branch
- **THEN** its location is under flume's data directory as any place's, nothing stands there at any moment, `git worktree list` names no worktree for it, and the repository's teardown never ran (3, 66, 78, 202)
