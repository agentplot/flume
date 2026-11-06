# Every place waiting on the merge queue is settled, at every depth of branches

## Why

The user approved on Pending You, 2026-11-06, closing the gaps between
flume's merge-queue design and deneb by "Flume's terminal watch also
settles places waiting on the queue". Their note asks for one design
that holds at every depth and knows no bolt and no unit:

> We just need to make sure the design in the runner and Flume is
> symmetric, consistent, and has integrity, meaning we could have nested
> branches. […] The code shouldn't know about bolts and units. […] I
> don't want it to be hacky. I want it to be a consistent and obvious
> design in terms of where the merge queue runs.

Two things stand in the way.

- **The statement says less than the build does.** Clause 122 says the
  watch settles "each place with an agent in it", and the vocabulary
  says it settles "the places whose agents run in it". The herdr
  plugin's watch runs `flume settle` with no place named, and that
  settles every place on the host (`crates/flume/src/commands/mod.rs`,
  `Engine::settle`). Each settle of a submitted place steps its base
  (`submit.rs`, `step_base`). On deneb, flume's log shows each of the
  runner's swancloud chores settled 43 times between 00:54Z and 02:34Z
  on 2026-11-06, about one settle every 2.3 minutes, with no agent in
  them. Nothing holds that behaviour: no clause, spec or check, so a
  later change could narrow the watch to the statement's words.
- **The outer branch has no place.** A crew bolt's branch is made by the
  runner's branch command or `crew bolt give` (`sluice branch create`),
  submitted to main by ops through `crew bolt submit` from a worktree
  crew makes (`wt submit main`), waited on by a background `sluice
  wait` crew starts, and removed by `crew bolt land` (`sluice branch
  delete`). The unit places on it are flume's and land through flume's
  settles. So one depth is flume's and the other is crew's and the
  runner's: the asymmetry the user named.

The research note
`/Users/chuck/.claude-sub-ia/jobs/148214aa/tmp/research-queues-driven.md`
holds deneb's readings. Its own finding explains the stall the brief
describes: the rounds flume's settles ran found no room, since the
host's sluice keeps no run slot for candidates (`[runs] max = 2,
reserved = 0`) and a round counts room before its checkout and asks for
it after. Those are sluice's and the host's, named below.

## What Changes

- **The watch settles every place (46, 122, C.5, I8, S109).** The
  statement, the specs and the checks say what the build does: a
  terminal's watch settles every place on the host, on its bound,
  whether or not an agent is in it, so a submitted place's base is
  stepped at each bound. No code of the core changes for it.
- **A branch that lands and is landed on is a base place's (18, 202 to
  206).** A base place is a place whose kind lands and has no stages,
  `stages = []` in weir's format. Its launch names its base as any
  launch does. flume asks the merge queue to make its branch (sluice
  104, through the merge-queue seam's new **open**), and makes its
  worktree detached at that revision, so it checks out no branch the
  merge queue lands on (10). It runs no agent, has nothing made ready,
  and takes no room or points. Places launched with its branch as their
  base are cut there and land there through the merge queue, at any
  depth.
- **A base place is submitted on its caller's word (47, 111, 112, 117,
  203).** A retry naming its submission submits its branch by name, as
  the merge queue holds it, and is refused while a place on its branch
  is live. A failed or conflicting submission waits on its caller with
  the merge queue's words; the caller lands the fix through a place on
  its branch and retries.
- **The launch, the drop and the tidy keep the depths apart (10, 52,
  67, 204).** A launch on a live place's branch is refused unless that
  place is a base place whose submission is not open and whose work is
  not over. A drop of a base place is refused while a place on its
  branch is live. Its tidy removes its worktree as any place's, then
  asks the merge queue to close its branch into its base (sluice 105,
  the seam's new **close**), never `git branch -D`.
- **Status says what a base place waits on (56, 130).** The places on
  its branch while any is live, its caller once none is and it is not
  submitted, and its head as the merge queue reads its branch.
- **Nothing names a bolt or a unit.** A base place is a kind's shape.
  flume ships no kind of that shape: a repository, a host's source or a
  launch declares one.
- **A person's plain use stays whole.** A place submitted by hand lands
  as before (63, S30). A host whose `sluice run` drives a repository
  sees the settles' steps do nothing (sluice 135). A base no place
  holds, a person's own branch or a bolt made before this change, takes
  launches as today.
- **The rest of the family, as named follow-ups.** weir lets a kind that
  lands declare no stages (weir 51, 52, 57). sluice takes a submission
  that names its head (sluice 124), and keeps the room a round counted.
  The runner retires its branch command, and crew retires `crew bolt
  submit`'s worktree, `wt submit` and background wait, and `crew bolt
  land`'s branch delete: a bolt becomes a base place of crew's own kind,
  submitted by a retry. None of it is built here.

## Capabilities

### New Capabilities

- `base-places`: a place whose kind lands and has no stages, whose
  branch other places land on: its launch, its submission, the launches
  and drops it refuses, its tidy and what status says of it.

### Modified Capabilities

- `settling`: the watch's settle moves every place, an agent in it or
  none, a requirement of its own.
- `herdr-plugin`: the watch settles on its bound with no flume agent in
  any pane, a requirement of its own.
- `seam-contracts`: the merge queue's open, close and a submit by name;
  the place seam's detached make; requirements of their own.
- `seam-fakes`: the fakes script open, close, a submit by name and a
  detached place, a requirement of their own.
- `merge-queue-profile`: sluice's branch create and delete, and a
  submit naming its head, a requirement of their own.
- `place-profile`: a base place's worktree is detached and its head read
  through the merge queue, a requirement of its own.
- `kinds-profile`: a kind that lands with `stages = []` read through
  weir's crate, a requirement of its own.
- `clause-coverage`: the suite reads the statement's revision holding
  202 to 206 and S109 to S112.

## Impact

- `crates/flume-seams`: `MergeQueue::open` and `close`, `SubmitHead`
  for `submit`, `Opened` and `Closed`; `Place::make` given no branch;
  `BranchRemoved::Refused`; `WaitsOn::Places`; round trips.
- `crates/flume-core`: the launch of a base place and the launches it
  refuses, the retry and drop it refuses, its submission, its failure,
  its tidy and its undo, its wait and status. The settle and the watch
  are unchanged.
- `crates/flume-fakes`: open, close, a submit by name and a detached
  place.
- `crates/flume-conformance`: settling's S109, base places' scenarios
  S110 to S112, the seam scenarios, and the clause list.
- `crates/flume`: sluice's `branch create`, `branch delete` and `submit
  --head`; `git worktree add --detach`; weir-kinds' pin; tests over
  scratch repositories and against the pinned sluice.
- `plugins/herdr`: a stand-in test of the watch's bound with no flume
  agent. No script changes.
- `site/`: the merge-queue guide's base places, the watch, and the
  status reference.
- `docs/`: record 0051, proposed; rule core.53, and core.5's and core.6's
  details.
- agentplot/blueprints: flume's vocabulary, 14, 18, 46, 47, 67, 78, 80,
  82, 111, 112, 117, 122, 165, 202 to 206, C.1, C.5's herdr watch, C.6,
  I8, the callers table's crew row and S109 to S112; weir 51, 52 and
  57; sluice 124; on the branch
  `design/flume-every-place-on-the-queue-is-settled-at-every-depth` at
  fb15fac, unpushed (core.19).
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool. weir-kinds' pin moves, and the pinned sluice moves
  to the release that submits a named head.
