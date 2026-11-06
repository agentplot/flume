# Design

## Context

### The user's ruling

The user approved on Pending You, 2026-11-06, the plan to close the
gaps between flume's merge-queue design (the guide
`site/src/content/docs/guides/merge-queue.md`, statement C.6) and
deneb, choosing "Flume's terminal watch also settles places waiting on
the queue". Their note, whole:

> We just need to make sure the design in the runner and Flume is
> symmetric, consistent, and has integrity, meaning we could have nested
> branches. That's the intention with bolts and units.
>
> The code shouldn't know about bolts and units. First we make a bolt
> branch and we don't have any places for it in Flume. Then we make units
> as places in Flume for each of the construction tasks. Each of those
> needs to get merged back with sluice to the bolt branch. Once all of
> the units are done, we go to land the bolt branch. The bolt branch
> itself needs to be merged back to main with sluice.
>
> I don't want it to be hacky. I want it to be a consistent and obvious
> design in terms of where the merge queue runs. As I describe that, it
> seems like Flume needs to be driving the merge queue on the nested
> branches or the nested work trees. It seems like maybe the runner has
> to be driving the merge queue on the outer branch, the bolt branch.
>
> To me, that isn't symmetric and I don't know if that limits us from
> having other flexibility in terms of how people might want to do
> branching. On the one hand, I want to be opinionated, like I wanted to
> really just work that way with bolts and units, but I also want a clean
> design.

### What the build does today

The watch already settles every place. The herdr plugin's watch
(`plugins/herdr/bin/watch.sh`) runs `flume settle` with no place named,
on its bound and at each agent's state change. `flume settle` with no
place calls `Engine::settle(None)` (`crates/flume/src/commands/mod.rs`),
which settles every document the store holds, an agent in it or none
(`crates/flume-core/src/settle.rs`). A settle of a live place whose
submission is open steps the submission's base once per command
(`submit.rs`, `step_base`). So a submitted place with no agent has its
base stepped at each of the watch's bounds.

flume's log on deneb shows it. Each settle of a place logs its clues
when they fail to show, and they failed for every one of the runner's
swancloud chores between 00:54Z and 02:34Z on 2026-11-06. Each was
settled 43 times in those 100 minutes, about once every 2.3 minutes:
the watch's thirty-second bound, plus a `flume settle` and a
`flume status --json` that took about 80 seconds on that host. The
chores had no agent then. One of them, `chore--the-fleets-moon-is-2-6-5`,
landed on `bolt/swanclouds-bumps-of-the-5th` at 05:30Z.

The statement says less. Clause 122 has the watch settle "each place
with an agent in it", the vocabulary "the places whose agents run in
it", C.5's herdr row "the session's places", and I8 "the terminal's
watch settle a place". No spec or check holds a settle of a place with
no agent, so nothing would stop a later change narrowing the watch to
those words.

### Why deneb's queues stood still

The research note
(`/Users/chuck/.claude-sub-ia/jobs/148214aa/tmp/research-queues-driven.md`,
sections 2 and 5) read sluice's records beside the process table. The
rounds ran: at 02:01:58 a round, by its timing a flume settle, claimed
a candidate and released it at the bound. Every round found no room.
The host keeps two run slots and none for candidates (`[runs] max = 2,
reserved = 0`, from swancloud's `lib/sluice-runs.nix`). Agents' `sluice
exec` runs retry at every pass and take a freed slot within a second.
And a round counts room when it decides and asks for it only after its
checkout, three to five seconds later. Those are sluice's and the
host's, named below as sluice's follow-up S2.

### The outer branch has no place

A crew bolt is made, submitted, waited on and removed outside flume:

| step | today | where |
|---|---|---|
| make the bolt's branch | `sluice branch create bolt/<b> --from main --json` | crew's `bolt_give`, `BOLT_CREATE` (`plugin/lib/plan.py` 1736 to 1744); the runner's branch command (`crates/boomdrive/src/sluice.rs:105`, its record 0002) |
| launch its units | `flume launch` with `--base bolt/<b>` | crew's `requests.rec`, the runner's launch (Boomdrive's work-into-flume spec) |
| land the units | flume's settles submit and step | flume (111, 124) |
| submit the bolt | `git update-ref` of the host's copy, `git worktree add --no-checkout <kit>/places/land-<b> bolt/<b>`, `wt submit main --no-wait --json --no-remove` | crew's `SUBMIT` (3946 to 3998), run by ops on the user's word |
| wait on it | a background `sluice wait <n> --timeout 9m` looped while it exits 5 | crew's waiter (3981) |
| remove the branch | `sluice branch delete bolt/<b> --into main --json` | crew's `land_delete`, `BOLT_DELETE` (1762 to 1766, 9728) |

So flume carries the inner depth, and crew, ops and the runner the
outer one. That is the asymmetry the user named.

### Rules relied on

This change touches the area core.md binds; docs/architecture has no
area page. It relies on core.1 and core.2 (the core names no tool, one
interface per seam), core.3 (sluice's profile runs sluice's commands
once bound), core.4 (the kind format is weir's), core.5 and core.14's
clause 14 (flume moves no branch but its place's own), core.6 (a place
is submitted only by a settle or its caller's retry, through the seam,
with no wait), core.7 (a place is cut where its base lives), core.8
(a launch refused whole), core.10 (an agent to each start), core.13
(no process of flume's own), core.15 (facts read where they live),
core.16 and core.17 (held state first, safe to repeat), core.18
(removal), core.24 (a stall), core.26 (room), core.27 (running
through), core.36 (a base's fix is a caller's launch), core.38
(points) and core.47 (readiness by the merge queue). It departs from
none. It draws core.53 and amends core.5's and core.6's details
(decision 11).

## Goals / Non-Goals

**Goals:**

- One design at every depth: every branch that lands is a place's,
  submitted by flume through the merge queue, its queue stepped by
  flume's settles where nothing else drives it (46, 111, 124).
- The statement, the specs and the checks hold that the watch settles
  every place, an agent in it or none (122).
- A base place: what a branch that work lands on, and that itself
  lands, is in flume's terms, and what flume needs for a place whose
  base is one (202 to 206).
- Nothing names a bolt, a unit or a depth.
- A person's plain use stays as it is: a place submitted by hand, a host
  whose `sluice run` drives the repository, a base that is no place's
  branch.

**Non-Goals:**

- Building the runner's, crew's, sluice's or weir's parts. "Named
  follow-ups" below gives each to its repository.
- One watch per host. Each herdr server's watch settles every place, so
  a host with several servers settles each place several times a bound.
  Settles are safe to repeat (74), steps are one per base per command,
  and sluice's branch lock makes a step beside another do nothing.
- A host driver for sluice, or one copy of each repository per host.
  Those are the research note's options A and E, sluice's and
  Boomdrive's.
- How fast a settle and a status run on a host with many places. The
  sibling fix `a-hosts-full-status-read-takes-seconds` takes the status
  read.
- A kind of flume's own with no stages (decision 2).

## Decisions

### 1. The watch settles every place

The statement's amendment brings 46, 122, the vocabulary's watch, C.5's
herdr watch and I8 to what the build does. 122 reads: "the terminal
settling every place on the host, on a bound it states and when an
agent's state changes, while flume runs nothing; or no watch. It
settles a place whether or not an agent is in it, so a submitted
place's base is driven (124)." 46 says a settle drives the place's base
where the merge queue moves only so, and that this holds at every depth
of base places alike.

No code of the core or the plugin changes. The change adds what holds
it:

- **The conformance suite**, `engine/settling.rs`,
  `s109_a_submitted_place_with_no_agent_moves_under_the_watch`: under
  the fake terminal's watch, a chore delivers, its agent ends, and it is
  submitted. Each `settle(None)` after the bound logs one `step main`
  of the fake merge queue's, the place having no agent; the fake lands
  it, and the next settle tidies it. Beside it, a fake merge queue that
  moves on its own answers `MovesOnItsOwn`, and the place still lands
  (46, 122, 124, 125, S109).
- **The herdr plugin's tests**, `plugins/herdr/tests/test.sh`: with the
  stand-in herdr listing no pane that holds a flume agent, and the
  stand-in flume's status naming one submitted place, the watch runs
  `flume settle` once per bound for three bounds (122).
- **The tools layer**, `tests/profiles/tools/merge_queue_sluice.rs`: a
  place submitted to the pinned sluice, no agent started for it and no
  `sluice run` live, lands after `flume settle` runs a few times, each
  a step, as the watch would run it (124, 125, C.6).

### 2. A base place is a kind's shape: a kind that lands with no stages

A branch that work lands on, and that itself lands on another, is a
**base place**'s: a place whose kind lands and has no stages (18, 202).
The core knows a base place by `kind.lands && kind.stages.is_empty()`,
and by nothing else: no kind's name, no branch's prefix, no depth.

weir's format today refuses a simple kind with no stage (weir 57). The
amendment lets a kind that lands declare `stages = []` (weir 51, 52,
57), keeps refusing one that does not land, and keeps a table that
leaves `stages` out reading as settings for a kind declared elsewhere.
flume reads it through `weir-kinds` at the revision that carries it
(core.4).

flume ships no kind of this shape. A repository declares one in
`.config/flume.toml`, a host in a source of its own (191), or a launch
in its file:

```toml
[kinds.train]
branch = "train"
lands = true
stages = []
```

crew's follow-up declares its own, under the prefix its bolts already
use. A kind of flume's own would put a name and a branch prefix on the
site that every host's operator reads (core.35), and this change needs
none: the conformance suite and the profile tests declare theirs. The
path not taken is a `gather@1` of flume's own; adding it later is one
kind's package and changes nothing here.

### 3. Its launch

`launch.rs` takes a base place's launch as any launch until the place
is made, and then:

1. **Refusals first (2, 4, 8).** A base place's launch that gives
   points is refused, naming them (165): it has no agents' work. A
   repository whose binding names no merge queue refuses it as it
   refuses any kind that lands.
2. **The base read (9, 126; core.7).** `base_now` reads the base as for
   any place and keeps `base_from`. The launch refuses a base it cannot
   read.
3. **Held state, then the branch (core.17).** The document is written
   `Making`. Then `MergeQueue::open(repository, branch, base)` asks the
   merge queue to make the branch. `Opened::Made { revision }` gives the
   revision it was made at. `Opened::Refused { reason }` refuses the
   launch with the merge queue's words and removes the document, nothing
   else made.
4. **The worktree (78).** `Place::make(repository, revision, None,
   location)` makes the worktree detached at that revision. A refusal
   there closes the branch into the base and removes the document, so
   the launch leaves nothing (2, 8, 17).
5. **Live, with nothing started.** The place goes `Live` with no
   current stage. No readiness runs (11), no start is made, no room or
   points are counted (70, 165), and the launch answers `Launched` with
   the place.

`undo` (`settle.rs`), for a launch whose command died while making,
removes the worktree and closes the branch the same way.

The worktree is detached so the place checks out no branch the merge
queue lands on (10). A worktree on the branch would read as holding
uncommitted work after every landing: the merge queue levels the host's
copy of the branch under it (sluice 124), and git then reads the index
against a head that moved. A scratch repository showed it: a worktree
on `g`, then `git update-ref refs/heads/g` to a later commit, then
`git status --porcelain=v2` reading `1 M. …` for the file. The tidy
would keep the place for ever (65).

### 4. A launch on a place's branch

Before anything is made, `launch.rs` reads the live places of the
repository whose branch is the launch's base (204):

- **None.** The launch goes on as today. A base that is no place's
  branch, a person's own or a bolt made before this change, takes
  launches as it does now.
- **A place that is no base place.** Refused, naming that place. Its
  worktree checks out its branch, and the merge queue landing on that
  branch would move it under the place's agent (10).
- **A base place whose submission is open, or whose work is over.**
  Refused, naming the base place. A place landing on a branch already
  submitted would land after the revision that was submitted, and the
  branch's close would then keep it (206). A failed or withdrawn
  submission is not open, so the caller's fix may launch on it
  (decision 6).
- **A base place otherwise.** The launch goes on. The new place is cut
  at the base place's branch where the merge queue's profile says it
  lives (126, C.1), as on any base, and lands there (15).

A base place may itself be launched on a base place's branch: the same
reads hold at every depth.

### 5. Its submission

A base place has no last stage, so nothing makes its submission due
(111). It is submitted only by its caller's retry naming the submission
(47, 49, 203). `acts.rs`'s retry of a base place's submission:

- is refused, naming them, while a place whose base is its branch is
  live;
- otherwise calls `submit(id, true)`, which reads the branch's revision
  where the merge queue says it lives, `base_revision(repository,
  branch)`, and submits it named:

```rust
/// What a submission sends: the place's worktree head, or a branch
/// the merge queue lands on, named, as the merge queue holds it (117,
/// 203).
pub enum SubmitHead {
    Worktree { head: Revision },
    Branch { branch: String, revision: Revision },
}

fn submit(&self, place: &PlaceRef, head: &SubmitHead) -> Submitted;
```

The outcome is collected as any place's: the inbox run in the place,
the merge queue's record of the branch (99, 133). A landing makes the
place's work over (63, 113). A person's own submission of the branch,
by hand from a checkout of theirs, lands it the same way (63).

### 6. Its failure

`verdict` (`submit.rs`) on a base place's failed or conflicting
submission starts no stage, since its kind names none. It records the
verdict and sets `submission.waiting` to the cause and the merge
queue's words, as a withdrawn submission does, and the settle delivers
it to the place's recipients (43, 112, 203). The place waits on its
caller.

Its caller lands the fix through a place on its branch: a fix launched
with the base place's branch as its base and the words as its brief,
which decision 4 allows since the submission is no longer open. For a
conflict the words carry the revision of the base place's base that the
merge queue tested against, and the fix's agent merges that revision
into the fix's branch first, as 112 has any conflict's fix do. The fix
lands on the base place's branch, bringing the base's revision in, and
the caller retries the base place's submission. flume launches no fix
of its own (36; core.36).

### 7. What the base place waits on

A base place has no stage to hold or stall, so its wait is its
submission's:

- **On its places**, while a place whose base is its branch is live. It
  is not stalled (130). Status gives `waits_on: places` and the places.
- **On its caller**, once none is live and it is not submitted, from
  the settle that read it so. It stalls after five minutes, or the
  launch's bound, under no hold's words, and each recipient is
  delivered that once (130; core.24). A caller that holds its
  submission with words, `flume hold <place> --submission "<words>"`,
  keeps it from stalling, and its retry ends the hold.
- **On the merge queue**, once submitted, as any place (111).

`settle.rs`'s `stall` reads a base place's wait from the store's live
places of its repository whose base is its branch, at each settle.
`answer.rs`'s `WaitsOn` gains `Places`, worded `waits on the places on
its branch`. `PlaceStatus` gains `on_branch: Vec<PlaceId>`, the live
places whose base is the place's branch, given for a base place and
left out otherwise, so the JSON form only gains a field (61). Its head
is read through the merge queue's base on its branch (80, 126), never
from its worktree, which stays at the revision it was made at.

The place's line gives `<kind>--<name> · <n> on its branch · open`
while places on its branch are live, `waiting on its caller` once none
is, and then `submitted`, `landed` and the rest as any place's (56,
139). The list gives the same words (200).

### 8. The drop and the tidy

A drop of a base place is refused, naming them, while a place whose
base is its branch is live (52, 204): those places land on its branch,
and the caller drops them first. A done is refused as on any kind that
lands (114).

`tidy.rs` removes a base place as any place, its worktree through the
repository's teardown, `sluice release` among it (64 to 66). It then
calls `MergeQueue::close(repository, branch, base)` in place of
`Place::remove_branch`:

```rust
pub enum Closed {
    Removed,
    /// The merge queue kept the branch: a submission open on it, or
    /// work of it its base lacks, in the merge queue's words (204, 206).
    Kept { reason: String },
    Gone,
}
```

`Closed::Kept` is recorded as `BranchRemoved::Refused { reason }`, a new
variant, named once in the answer to the act that removed the place, and
a launch that would make the branch again is refused while it stands
(3, 67). flume never runs `git branch -D` on a base place's branch:
sluice's protection would refuse it (sluice 106), and the merge queue
judges what its own landings hold.

A place on the branch that is tidied after the base place's branch went
reads its base as gone, and `remove_branch` keeps its branch, as it
keeps any branch whose base cannot be read (67). Decision 5's refusal
makes this rare: the base place is submitted only once every place on
its branch is over, and a landed place is tidied at the next settle.

### 9. The seams and the profiles

The merge-queue seam (B.6) gains:

```rust
/// open: the branch made at its base's current revision as one the
/// merge queue lands on (205).
fn open(&self, repository: &str, branch: &str, base: &str) -> Opened;
/// close: the branch removed into the branch its work landed in (206).
fn close(&self, repository: &str, branch: &str, into: &str) -> Closed;

pub enum Opened {
    Made { revision: Revision },
    Refused { reason: String },
}
```

and `submit` takes a `SubmitHead`. The place seam's `make` takes the
branch as `Option<&str>`, `None` making a detached worktree (78).

The sluice profile (C.6):

| operation | sluice |
|---|---|
| open | `sluice branch create <branch> --from <base> --json`, run in the repository; its answer's revision; exit 2 a refusal in its `error.message` |
| close | `sluice branch delete <branch> --into <base> --json`, run in the repository; exit 1 gone, exit 2 kept with its words |
| submit, a branch | `sluice submit <base> --head <branch> --no-wait --json` in the base place's worktree; answered as any submit |

Each runs with every multiplexer's session stripped and
`GIT_TERMINAL_PROMPT=0`, as the profile's other commands do (C.6). The
pinned sluice, 0.67.0, takes `--head` only with `--check`. The submit
by name waits on sluice's follow-up S1, and the profile's test of it on
the pin that carries it.

The git worktree place profile (C.1) makes a base place's worktree by
`git worktree add --detach <location> <revision>`, with no hook run.
Its read gives the worktree's own state for the tidy (65) and leaves
the head to the core (decision 7).

The kinds profile reads a kind with `stages = []` through weir-kinds at
the revision weir's follow-up W1 lands, mapping it to a kind with no
stages; a kind that does not land and declares none is refused in
weir's words (4, 18; core.4).

The no-tool profiles answer `open` and `close` as refusals naming that
no merge queue is bound; a launch of a base place under them is refused
before that as any landing kind's is.

### 10. The fakes

The fake merge queue gains `open` and `close`, scripted to make,
refuse, keep or answer gone, each logged as `open <branch> from
<base>` and `close <branch> into <base>`, and records a submit by name
as `submit <branch> named`. A branch it opened is one it lands on:
a submission from a place on it lands there when scripted to. The fake
place makes a detached place, logged `make <location> detached at
<revision>`. The fake kinds hold a kind with no stages as any kind.

### 11. How it is held

- **The statement**, on agentplot/blueprints' branch
  `design/flume-every-place-on-the-queue-is-settled-at-every-depth` at
  fb15fac, cut from main at 12866a8 and unpushed. flume: the
  vocabulary's base and watch entries, 14, 18, 46, 47, 67, 78, 80, 82,
  111, 112, 117, 122, 165, the new 202 to 204 in A.7 and 205 and 206 in
  B.6, C.1, C.5's herdr watch, C.6's step, open and close rows and its
  submit by name, I8, the callers table's crew row, and S109 to S112.
  weir: 51, 52, 57 and the format's example. sluice: 124's submission
  that names its head. They hold on the user's yes, and the orchestrator
  pushes the branch.
- **Record 0051**, proposed: "Every branch that lands is a place's, and
  every place waiting on the merge queue is settled at every depth",
  quoting the user's note.
- **core.53**, under "Branches, submissions and landings": a branch
  other places land on is a base place's; the merge queue alone makes
  and removes it at flume's asking; its worktree checks out no branch;
  it is submitted only on its caller's retry, by its branch's name, once
  no place on its branch is live; and flume names no depth and no use
  of one. core.5's details gain the base place's branch made and closed
  by the merge queue; core.6's gain a base place's submission by name
  and the watch's settle of a place with no agent.
- **The site**: the merge-queue guide gains "Branches that land on
  branches", with the kind above, a place on its branch, its submission
  and its fix; "Reading the outcome" says the watch's settles step a
  submitted place's base with no agent and no caller. The status
  reference gives `on_branch` and `waits_on: places`, and the herdr
  plugin's README says the watch moves every place, an agent in it or
  none (core.35).

## Who owns what, and what each tool stops doing

### One owner for each piece

| piece | owner | cited by |
|---|---|---|
| a queue's rounds, and the host's driver where one runs | sluice (135) | flume, the runner, crew |
| a step of a queue where no driver lives, at every depth | flume's settle (124, 125, C.6) | the runner, crew |
| every settle of every place between commands | the terminal's watch (122) | the runner, crew |
| a branch sluice lands on, made and removed | sluice (104 to 106), asked by flume's base place (202, 204) | the runner, crew |
| a submission of any branch that lands, and its outcome | flume (111, 133, 203) | the runner, crew |
| a kind with no stages | weir's format (51, 57) | flume, crew |
| bolts, units and when a bolt is sent | crew | the user |

Where the merge queue runs is then one answer at every depth: sluice's
rounds run in the repository, moved by the host's `sluice run` where
the host keeps one, and otherwise by the settles of the places
submitted to it, whether the base is `main` or a base place's branch.
Neither the runner nor crew drives a queue.

### Named follow-ups

- **agentplot/weir, W1: a kind that lands may declare no stages.**
  weir 51, 52 and 57 as the statement branch amends them; weir-kinds
  reads `stages = []` on a kind that lands, refuses it on one that does
  not, and reads a table that leaves `stages` out as settings, as
  today. flume's task 3.3 moves its pin to the revision that lands it.
- **agentplot/sluice, S1: a submit names its head.** sluice 124 as
  amended: `sluice submit <target> --head <branch>` without `--check`
  submits a branch sluice lands on as its policy holds it, the remote's
  revision where it names one, from any worktree of the repository, the
  worktree its submitter and the branch the entry's (`from.worktree`),
  so `sluice inbox` in that worktree delivers its outcome. flume's task
  5.3 moves its pin of sluice to the release that carries it.
- **agentplot/sluice, S2: a round keeps the room it counted.** The
  research note's two gaps: a round counts room before its checkout and
  asks for it after, and nothing gives a queued candidate the first free
  slot ahead of a session's run that retries at every pass. They held
  deneb's queues on 2026-11-06 while flume stepped them. No clause of
  flume's changes for them.
- **agentplot/boomdrive, B1: a bolt is a launch.** The runner retires
  its branch command: `sluice branch create` in `crates/boomdrive/src/sluice.rs`
  (line 105) and `carry.rs` (611 to 662), the branch-commands spec's "A
  bolt's branch is made by sluice in the clone", and record 0002's
  leased Command for a branch. A control plane's bolt is a `flume
  launch` of its caller's kind with no stages, passed as given (its
  core.15), its units launched with `--base <its branch>` as they are
  now. A bolt's send is a `flume retry <place> --submission` the control
  plane asks, a tool's own command run once and awaited (its core.13,
  core.16). Its tool-records spec's read of `sluice status <id>` for
  each submission flume names stays: it reads, and drives nothing. Its
  core.13 holds as written, and record 0003's consequence, "Flume being
  the driver of a submission", holds at every depth. It cites flume 202
  to 204, 122 and 124.
- **afterthought/crew, C1: a bolt is a base place.** crew declares its
  kind with no stages, `branch = "bolt"`, `lands = true`, in a package
  its hosts lock (C.2, 191; record 0040). `crew bolt give` asks for the
  bolt's launch through the runner rather than running `BOLT_CREATE`
  (`plugin/lib/plan.py` 1736 to 1744, 1882). `crew bolt submit` asks
  for a retry of the bolt place's submission, retiring `SUBMIT` (3946
  to 3998): the `git update-ref` of the host's copy, which sluice's
  protection refuses (sluice 124), the `places/land-<bolt>` worktree,
  `wt submit main` and the background `sluice wait` (3981, 4053).
  `crew bolt land` reads the bolt's landing from flume's delivery to its
  recipient and writes the plan, retiring `land_delete` and
  `BOLT_DELETE` (1762 to 1766, 9728), since flume's tidy closes the
  branch. Its rules plan.8, plan.16, plan.18, plan.19 and plan.21, and
  `_open.md`'s stopgap waiter ("Flume submits landings (move 6)"),
  follow; teams.5 holds. A bolt's failed landing is a fix crew launches
  on the bolt's branch, as it takes conflicts into a bolt today (sluice
  C.5's crew row, which crew's change amends with sluice). It cites
  flume 202 to 204, 112 and 131.

Until B1 and C1 land, crew's bolts are bases no place holds, and land
by crew's path as today: decision 4's first case.

## Alternatives Considered

- **The runner or crew steps the outer branch's queue.** The research
  note's option C, the runner's half. It makes a second driver beside
  flume's settle (the runner's core.1, record 0003), steps at most once a
  pass, and keeps one depth outside flume. The user called it not
  symmetric.
- **A base place's worktree on its branch.** Simplest to read, and what
  crew's landing worktree does for the instant of its submit. Clause 10
  forbids it, and the scratch repository above shows why: the merge
  queue's levelling of the host's copy leaves the worktree reading as
  holding a change, which keeps the place at its tidy. sluice levelling
  the worktrees that hold its copy, as git's `updateInstead` does, would
  move other tools' worktrees under them.
- **The worktree takes its branch only at its submission.** `git switch`
  in the detached worktree before the submit. Still a checkout of a
  branch the merge queue lands on (10), and stale again wherever the
  copy is levelled after the switch.
- **A place with no worktree.** The base place as a branch and a held
  document alone. Every seam reads a place as a worktree (9, B.1), and
  sluice keys a submitter and its inbox by the worktree it runs in.
- **flume makes the branch itself.** `git worktree add -b`, as for any
  place. sluice makes and removes a branch it lands on alone, and its
  protection refuses anyone else's (sluice 104 to 106); on a branch kept
  on a remote, the branch must also be pushed, which flume never does
  (core.5).
- **A kind field naming a base place.** `base = true`, or a kind's
  stage that waits forever. Either is a field flume would define in
  weir's format (core.4), or an agent started for nothing. A kind with
  no stages says it in the format's own terms.
- **A kind of flume's own.** Decision 2.

## Risks / Trade-offs

- **The build waits on two follow-ups.** The kinds profile's read waits
  on weir's W1, and the sluice profile's submit by name on sluice's S1.
  Groups 1, 2 and 4 need neither: the core's base places run over the
  fakes before either lands. A build that finds either missing stops
  short naming it.
- **A base place's worktree shows its launch's revision.** A person who
  opens it reads the base as it stood when the branch was made. Status
  and the list give the branch's head from the merge queue, and the
  guide says to open a place on the branch to read it.
- **Several watches, many steps.** deneb runs a watch per herdr server,
  each settling every place. Each command steps a base once, and a
  step beside another round finds the branch's lock held and does
  nothing. The cost is a process each, and a step beside a live `sluice
  run` is the same nothing (sluice 135).
- **Steps do not make room.** A host whose sluice keeps no slot for
  candidates stalls its queues however often flume steps them. That is
  sluice's S2 and the host's configuration.
- **A stall between a base place's launch and its first place.** A
  caller that launches a base place and waits over five minutes before
  its first place is delivered a stall. A hold on the submission with
  words keeps it from stalling (130).
- **A base place's kind is the caller's.** A kind declared with no
  stages and a prefix nobody's sluice policy names is refused at its
  launch by the merge queue's open, naming the policy's words.

## Migration Plan

Nothing held changes shape: a document gains no field a base place
needs, and `on_branch` and `BranchRemoved::Refused` only add to what
status and a tidy give (61). Bolts made before this change, by crew or
the runner, are bases no place holds and keep landing by crew's path
until C1. Places already launched on them land there as today.

## Open Questions

None for this change. The follow-ups' own designs decide their
commands' spellings: S1's `--head`, and how B1's control plane carries a
retry.
