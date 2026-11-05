# A place is cut from origin's copy of its base, merge queue or none

## Why

The runner's record 0019 (agentplot/boomdrive, accepted by the user on
2026-11-05) gives a repository's local default branch one owner. Where
sluice keeps a branch on a remote, the host's `refs/heads/<branch>` is
sluice's copy, moved only by sluice (sluice 124). The runner no longer
moves any branch of its clones and reads each at `origin/<branch>`
after its fetch. crew already cuts places from `origin/bolt/<bolt>`
after a fetch (crew's plan.19), and the fleet's views timer is being
changed to move no default branch either.

flume was the one reader left on the host's own copy. A launch read the
base where the merge queue says it lives, and otherwise the host's
`refs/heads/<base>`, which nothing on the host now moves. Clause 9 says
a place is never cut from a copy that may lag, and C.1 read the local
base wherever no merge queue lands it. No repository the runner keeps
has such a base today, since all of them land through sluice; a host's
own repositories, or a branch outside sluice's patterns, can (record
0041, proposed).

## What Changes

- **A base no merge queue lands is read at origin (9, 126, C.1).** Where
  no merge queue is bound, or its profile lands no such base, a place is
  cut at `origin/<base>` after flume's own fetch. The host's own branch
  is read only where the repository has no `origin`, or where the fetch
  fails and `git ls-remote --exit-code` exits 2, so origin holds no such
  branch. Any other failed fetch refuses the launch.
- **A base a merge queue lands is read where its profile says, as
  before.** sluice's profile answers that it lands no such base where
  the policy names no table for the base, or no revision of it holds a
  policy (C.6).
- **Status says where the base was read (56).** Each place gives
  `base_from`: `merge-queue`, `origin`, `no-origin` or `origin-lacks`,
  and the person's form says it beside the base. A place launched before
  gives none.
- **One fetch of a branch.** The binding's read at origin's default
  branch (C.2), the default branch's own read (20), sluice's base (C.6)
  and origin's base (C.1) make the same fetch through one function.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `launching`: a place is cut where its base's current revision lives,
  at origin wherever no merge queue lands the base.
- `seam-contracts`: the merge queue's base may answer that it lands no
  such base.
- `history-and-host`: git's history reads a branch at origin after a
  fetch, a requirement of its own.
- `merge-queue-profile`: sluice's base answers that it lands no base its
  policy names no table for, a requirement of its own.
- `place-status`: status gives where the base's revision was read, a
  requirement of its own.

## Impact

- `crates/flume-seams`: `BaseRevision::Unlanded`, `OriginBranch` and
  `History::origin_branch`, `BaseFrom` and `PlaceRecord::base_from`, and
  their round trips.
- `crates/flume-core`: the launch's `base_now`, and `PlaceStatus::base_from`.
- `crates/flume-fakes`: the fake git's `origin`, and the fake history's
  read of it.
- `crates/flume-conformance`: three launching scenarios, a history
  scenario and a merge-queue scenario, with their harnesses.
- `crates/flume`: `GitHistory::origin_branch`, `Git::fetch_branch` and
  `Git::remote_holds`, sluice's `base_revision`, the person's status
  line, and tests over scratch repositories with a stand-in origin.
- `site/`: the guide to places and the status reference.
- `docs/`: record 0041, proposed; rule core.7 and its details.
- agentplot/blueprints: 9's reading in C.1, 56, 126, C.6's base row and
  the scenario S92, on the branch
  `design/flume-a-place-is-cut-from-origins-copy-of-its-base` (core.19).
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool.
