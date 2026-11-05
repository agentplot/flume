---
number: 41
title: A place is cut from origin's copy of its base wherever no merge queue lands it, and from the host's own only where origin holds none
status: accepted
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A place is cut from origin's copy of its base wherever no merge queue lands it, and from the host's own only where origin holds none

Status: accepted by the user on 2026-11-05, at the fix's review: "Accept and release it, and push the amendment". Its statement amendment is on agentplot/blueprints main at 1b0275f (9, 56, 126, C.1, C.6, S92). Proposed earlier that day by the code stage of the fix `a-place-is-cut-from-origins-copy-of-its-base`; the direction is the orchestrator's, in its brief's words below. How a base origin lacks is told from a fetch that failed, what status says, and that a base a merge queue lands stays where the merge queue's profile says, are the stage's choices, made on the statement and these rules.

Deciders: the orchestrator's brief, for the direction; the code stage, for the rest; the user, at the fix's review.
Sources: the orchestrator's brief of the fix `a-place-is-cut-from-origins-copy-of-its-base`, 2026-11-05; `agentplot/blueprints:design/flume/requirements.md` at a465ba7, clauses 9, 56, 126, C.1, C.2 and C.6, as amended with this record on the branch `design/flume-a-place-is-cut-from-origins-copy-of-its-base` at 347520c; the runner's record 0019 (agentplot/boomdrive, accepted by the user on 2026-11-05); sluice's statement, clause 124, its scenario S58 and sluice record 0069; crew's rule plan.19; swancloud's rule herdr.13 and its fix `the-views-timer-moves-anchors-only`, in flight; records 0004 and 0027; rules core.1, core.2, core.5, core.7 and core.14; `crates/flume-core/src/launch.rs`, `crates/flume/src/host/history.rs`, `crates/flume/src/git.rs`, `crates/flume/src/profiles/merge_queue/sluice/mod.rs` and `crates/flume/src/profiles/kinds/weir/mod.rs`.

## Context and problem statement

The runner's record 0019 gives a repository's local default branch one owner. Where sluice keeps a branch on a remote, the host's `refs/heads/<branch>` is sluice's copy, levelled by sluice to each landing it reads and moved by no other hand (sluice 124, S58). The runner no longer moves any branch of its clones, and reads every branch at `origin/<branch>` after its fetch. That record names flume as the one reader left on the host's own copy.

flume read a place's base where the merge queue's profile says the base lives where one is bound, and the host's own `refs/heads/<base>` where none is. Its statement disagreed with itself. Clause 9 says a place is cut from the base's current revision where work lands on it, "never from a copy of the base that may lag it". C.1 read the local `<base>` wherever no merge queue is bound, and 126 had the seam answer the local branch there. Under sluice a base the policy names no table for was read at the local branch too. With the runner no longer levelling, such a copy stays where the clone was made, or where levelling last left it.

No repository the runner keeps has such a base today, since all of them land through sluice. A host's own repositories, or a branch outside sluice's patterns, can.

The orchestrator's brief, 2026-11-05: "Where the repository has `origin` and holds the base there, a place is cut from `origin/<base>` after Flume's own fetch, whether or not a merge queue lands it. The host's own ref is read only where origin has no such branch, and status says so."

## How the family reads a branch on a remote

- **sluice** reads a branch whose policy names a remote at the remote's revision, fetched, and reads nothing from the host's own ref of it, its copy (124). flume relies on that: where sluice lands the base on a remote, the base's current revision is `<remote>/<base>` after one fetch, as C.6 has read it since before this record.
- **The runner** reads every branch at `origin/<branch>` after its own fetch and moves none (record 0019). flume relies on that: nothing on a host the runner keeps moves a branch sluice does not keep, so the host's own copy of one may lag origin indefinitely.
- **crew** cuts every place from `origin/bolt/<bolt>` after a fetch, never from the local `bolt/<bolt>` (plan.19). flume does the same for any base no merge queue lands.
- **swancloud's views timer** is being changed to move no default branch either (its fix `the-views-timer-moves-anchors-only`). flume relies on nothing it moves.
- **flume itself** already read the binding at origin's default branch after one fetch (C.2), and the default branch at `origin/HEAD` (20). The base now follows the same rule.

So the one owner of a base's current revision is the merge queue that lands it, and where none does, origin. Each tool reads origin's branch after its own fetch and no tool reads a host's copy while origin holds the branch.

## Considered options

1. **Where a merge queue lands the base, where its profile says; otherwise `origin/<base>` after flume's own fetch; the host's own branch only where the repository has no `origin` or origin holds no such branch.**
2. **`origin/<base>` wherever origin holds the base, a merge queue that lands it or not.**
3. **The host's own branch, as before, the host asked to keep it level.**
4. **The host's own branch, a launch refused where it lags origin.**

## Decision outcome

Option 1.

1. **A base a merge queue lands** is read where its profile says, as before (126, C.6): under sluice, `<remote>/<base>` after one fetch where the policy lands it on a remote, and the local base where the base's table names none. A queue that lands the base locally moves the local branch with each landing, so the local branch is where work lands (9), and origin's may lag it.
2. **A base no merge queue lands**, under no merge queue or one whose profile lands no such base, is read at `origin/<base>` after `git fetch origin +refs/heads/<base>:refs/remotes/origin/<base>`. The merge queue seam's base says the merge queue lands no such base (`BaseRevision::Unlanded`); sluice's profile says so where the policy names no table for the base, or no revision of it holds a policy (C.6).
3. **The host's own branch** is read only where the repository has no remote `origin`, or origin holds no such branch. A branch origin lacks is told from a fetch that failed by `git ls-remote --exit-code origin refs/heads/<base>` exiting 2, asked only once the fetch failed, so a launch whose fetch works costs one round trip. Any other failure refuses the launch naming the fetch (9). A tracking ref an earlier fetch left is never read in place of a fetch, so a branch origin deleted is read as lacking.
4. **Status says where the base was read (56).** The place keeps it, `base_from` in its held state and in the JSON form, one of `merge-queue`, `origin`, `no-origin` and `origin-lacks`, none for a place launched before. The person's form says it beside the base: `branch fix/one from main (at origin)`.
5. **One fetch of a branch.** `Git::fetch_branch` is the fetch every reader of a remote makes: the binding's read at origin's default branch (C.2), the default branch's own read (20), sluice's base (C.6) and origin's base (C.1).

Option 2 is the brief's words read whole. Where a merge queue lands the base on origin, it gives what option 1 gives. Where one lands the base locally, origin's copy lags each landing, and a place cut there is cut from a copy that may lag, which clause 9 rules out. The runner's record 0019 asks the same of flume in its own words: origin "where the repository has origin and no merge queue lands the base locally". Option 3 is what record 0019 ended: sluice's protection refuses a hand that moves its copy (S58), and a host that levels every other branch would be a second owner of each. Option 4 refuses a launch flume could serve by reading origin itself.

### Consequences

- `BaseRevision` gains `Unlanded`, `History` gains `origin_branch`, answering `OriginBranch`, and `PlaceRecord` and `PlaceStatus` gain `base_from`, read with none where a document lacks it. The JSON form only gains a field (core.14).
- The core reads origin through git's history, which section 9 fixes, and names no tool (core.1). The merge queue's profile still decides alone where a base it lands lives (core.2).
- C.1, 56, 126 and C.6's base row say so, and S92 is added (blueprints 347520c, on the branch `design/flume-a-place-is-cut-from-origins-copy-of-its-base`; core.19). Rule core.7 and its details are amended with this record.
- A place is tidied as before. Remove-branch still runs `git cherry` against the merge queue's remote's tracking ref, or the local base (C.1). A base that lags there only keeps a branch, and never deletes one whose work the base lacks (67).
- flume's sluice profile reads which remote a base is on from the local base's policy first, and a remote's tracking copy only where the local base holds none (C.6). sluice reads it from every revision the host holds, a copy that lags deciding nothing against one that names a remote (124; sluice record 0069). This record leaves that reading as it stands, the brief keeping a bound merge queue unchanged; reconciling it is told to the orchestrator.
