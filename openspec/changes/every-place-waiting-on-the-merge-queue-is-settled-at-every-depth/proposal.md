# Every place waiting on the merge queue is settled, at every depth of branches

## Why

The user approved on Pending You, 2026-11-06, closing the gaps between
flume's merge-queue design and deneb. Their note asks for one design
that holds at every depth and knows no bolt and no unit:

> We just need to make sure the design in the runner and Flume is
> symmetric, consistent, and has integrity, meaning we could have nested
> branches. […] The code shouldn't know about bolts and units. […] I
> don't want it to be hacky. I want it to be a consistent and obvious
> design in terms of where the merge queue runs.

Reading the design the same day, they answered two things: a bolt's
place has "No worktree: a branch and a record", and of the terminal's
watch settling every place, "I do not like relying on the Herder plugin
to make this work. I'd rather it be the Boomdrive process."

Two things stand in the way.

- **What moves a submitted place between commands is one terminal's
  plugin.** The herdr plugin's watch runs `flume settle` naming no place
  on its bound, which settles every place and steps each open
  submission's base (`submit.rs`, `step_base`). On deneb flume's log
  shows each of the runner's swancloud chores settled 43 times between
  00:54Z and 02:34Z on 2026-11-06 with no agent in them. flume decides
  whether it runs attended by asking each terminal whether it keeps
  watch. So under any other terminal, or with the plugin's lease stale,
  a submitted place moves only when a caller acts.
- **The outer branch has no place.** A crew bolt's branch is made by the
  runner's branch command or `crew bolt give` (`sluice branch create`),
  submitted to main by ops through `crew bolt submit` from a worktree
  crew makes (`wt submit main`), waited on by a background `sluice
  wait` crew starts, and removed by `crew bolt land` (`sluice branch
  delete`). The unit places on it are flume's. So one depth is flume's
  and the other is crew's and the runner's: the asymmetry the user
  named.

The research note
`/Users/chuck/.claude-sub-ia/jobs/148214aa/tmp/research-queues-driven.md`
holds deneb's readings. Its own finding explains the stall the brief
describes: the rounds flume's settles ran found no room, since the
host's sluice keeps no run slot for candidates (`[runs] max = 2,
reserved = 0`) and a round counts room before its checkout and asks for
it after. Those are sluice's and the host's, named below.

## What Changes

- **A caller keeps watch over the host (46, 119, 120, 207, I8, S109,
  S113).** `flume settle --every <bound>` settles every place and
  records in the host's state when it began and the bound it stated. A
  caller keeps watch while its newest such settle began less than twice
  its bound ago, and flume runs attended while none does: every acting
  command settles every place first. On a host that keeps the runner,
  each of its passes runs it, so every waiting place, a submitted one
  with no agent among them, is settled and its base stepped within the
  runner's interval. Status gives the watch, its bound and when it last
  settled, or that none is kept and since when.
- **A terminal's watch becomes its nudge (122, C.5, S28).** The herdr
  plugin's watch still settles when an agent's state changes, a quicker
  settle nothing waits on, and drops its bound. The terminal seam's
  `watch` becomes `nudge`, and no terminal decides whether flume runs
  attended (190, S91).
- **A host with no runner runs attended.** Its places move at their
  agents' quiet, at a nudge, and at each command a caller runs, which
  settles every place first; its queues by those settles' steps, or by
  `sluice run` where the host runs it. A timer running `flume settle
  --every` keeps watch as the runner does.
- **A branch that lands and is landed on is a base place's (9, 10, 18,
  202 to 206).** A base place is a place whose kind lands and has no
  stages, `stages = []` in weir's format. It has no worktree: it is its
  branch, which flume asks the merge queue to make (sluice 104, through
  the merge-queue seam's new **open**), and the state flume holds. It is
  named by the location its place profile gives, where nothing is made.
  It runs no agent, has nothing made ready, and takes no room or points.
  Places launched with its branch as their base are cut there and land
  there through the merge queue, at any depth.
- **A base place is submitted on its caller's word (47, 111, 112, 117,
  133, 203).** A retry naming its submission submits its branch by name
  from the repository, as the merge queue holds it, and is refused while
  a place on its branch is live. Its outcome is read from the merge
  queue's record of its branch, since it has no worktree for an inbox,
  and delivered to its recipients. A failed or conflicting submission
  waits on its caller with the merge queue's words; the caller lands the
  fix through a place on its branch and retries.
- **The launch, the drop and the tidy keep the depths apart (10, 52,
  67, 204).** A launch on a live place's branch is refused unless that
  place is a base place whose submission is not open and whose work is
  not over. A drop of a base place is refused while a place on its
  branch is live. Its tidy removes no worktree and runs no teardown: it
  asks the merge queue to close its branch into its base (sluice 105,
  the seam's new **close**), never `git branch -D`.
- **Status says what a base place waits on (56, 130).** The places on
  its branch while any is live, its caller once none is and it is not
  submitted, and its head as the merge queue reads its branch; and that
  it has no worktree.
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
  naming a branch it lands on as its head from no worktree, its outcome
  read from its records (sluice 124), and keeps the room a round
  counted. The runner keeps the host's watch at each pass, amending its
  core.13 and record 0003 on the user's word, and retires its branch
  command. crew retires `crew bolt submit`'s worktree, `wt submit` and
  background wait, and `crew bolt land`'s branch delete: a bolt becomes
  a base place of crew's own kind, submitted by a retry. None of it is
  built here.

## Capabilities

### New Capabilities

- `base-places`: a place whose kind lands and has no stages, a branch
  and a record with no worktree, whose branch other places land on: its
  launch, its submission and outcome, the launches and drops it refuses,
  its tidy and what status says of it.

### Modified Capabilities

- `settling`: a caller keeps watch over the host and flume runs
  attended while none does; a settle of every place moves every place,
  an agent in it or none.
- `command-line`: `flume settle --every <bound>`.
- `place-status`: status gives the host's watch.
- `end-delivery`: a wait drives no settle while a caller keeps watch.
- `stream-terminals`: no terminal decides whether flume runs attended.
- `herdr-plugin`: the watch nudges at each agent's change and keeps no
  bound; it decides nothing of attended.
- `terminal-profiles`: herdr answers its nudge from the plugin's lease;
  the printed command keeps none.
- `seam-contracts`: the merge queue's open, close and a submit by name;
  the terminal's nudge; the host's watch held by the store.
- `seam-fakes`: the fakes script open, close and a submit by name, the
  fake terminal's nudge, and the host's watch.
- `merge-queue-profile`: sluice's branch create and delete, a submit
  naming its head from the repository, and a submitted place landing by
  a watch's settles.
- `place-profile`: a base place's location is given and nothing made
  there.
- `kinds-profile`: a kind that lands with `stages = []` read through
  weir's crate.
- `clause-coverage`: the suite reads the statement's revision holding
  202 to 207 and S109 to S113.

## Impact

- `crates/flume-seams`: `MergeQueue::open` and `close`, `SubmitHead`
  for `submit`, `Opened` and `Closed`; `Terminal::nudge` and `Nudge` in
  place of `watch` and `Watch`; `HostKey::Watch` and `Watched`;
  `BranchRemoved::Refused`; `WaitsOn::Places`; round trips.
- `crates/flume-core`: `settle_every` and the host's `attended`; status's
  watch; the launch of a base place and the launches it refuses, the
  retry and drop it refuses, its submission and outcome from the record,
  its failure, its tidy and its undo, its wait and status.
- `crates/flume-fakes`: open, close, a submit by name, the nudge and the
  watch.
- `crates/flume-conformance`: the watch's and attended's scenarios,
  every scenario that set a terminal's watch moved to the host's,
  settling's S109 and S113, base places' S110 to S112, the seam
  scenarios, and the clause list.
- `crates/flume`: `flume settle --every`; status's watch in the person's
  form; sluice's `branch create`, `branch delete` and `submit --head`
  from the repository; the herdr profile's nudge; weir-kinds' pin;
  tests over scratch repositories and against the pinned sluice.
- `plugins/herdr`: the watch drops its bound; its tests and README.
- `site/`: the merge-queue guide's base places and who moves a submitted
  place, the places-and-herdr guide's watch and nudge, the commands and
  state references.
- `docs/`: record 0051, proposed; rule core.53, core.13's rule, and
  core.5's, core.6's, core.13's, core.25's and core.49's details.
- agentplot/blueprints: flume's vocabulary, preamble, 9, 10, 14, 18, 46,
  47, 67, 78, 80, 82, 111, 112, 117, 119, 120, 122, 133, 165, 176, 202
  to 207, C.1, C.5, C.6, C.7, section 9, I8, section 12, S27, S28, S91 and
  S109 to S113; weir 51, 52 and 57; sluice 124; on the branch
  `design/flume-every-place-on-the-queue-is-settled-at-every-depth` at
  a43f811, cut from main at e4fab7b, unpushed (core.19).
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool. weir-kinds' pin moves, and the pinned sluice moves
  to the release that submits a named head from no worktree.
