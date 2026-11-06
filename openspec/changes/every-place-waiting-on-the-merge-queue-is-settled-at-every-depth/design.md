# Design

## Context

### The user's ruling

The user approved on Pending You, 2026-11-06, the plan to close the
gaps between flume's merge-queue design (the guide
`site/src/content/docs/guides/merge-queue.md`, statement C.6) and
deneb. Their note, whole:

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

They read the design on 2026-11-06 and answered two things. On the
outer branch's place: "No worktree: a branch and a record". On who
settles every waiting place between commands: "I do not like this as a
solution. I do not like relying on the Herder plugin to make this work.
I'd rather it be the Boomdrive process."

### What moves a submitted place today

A settle runs when an agent goes quiet (121), when the terminal's watch
settles (122), or when a caller runs a command (119, 120). The herdr
plugin's watch (`plugins/herdr/bin/watch.sh`) runs `flume settle` with
no place named on its bound and at each agent's state change. That
settles every document the store holds, an agent in it or none
(`crates/flume-core/src/settle.rs`), and a settle of a place whose
submission is open steps the submission's base once per command
(`submit.rs`, `step_base`). flume's log on deneb shows each of the
runner's swancloud chores settled 43 times between 00:54Z and 02:34Z on
2026-11-06, about once every 2.3 minutes, with no agent in them.

So what moves a submitted place between commands is the herdr plugin.
Where herdr is not the terminal, or its plugin's lease is stale,
nothing does but a caller's command. flume decides whether it runs
attended by asking each terminal the host binds whether it keeps watch
(`terminals.rs`, `attended`), so the whole guarantee rests on one
terminal's plugin. The user ruled that out.

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
once bound), core.4 (the kind format is weir's), core.5 (flume moves no
branch but its place's own), core.6 (a place is submitted only by a
settle or its caller's retry, through the seam, with no wait), core.7
(a place is cut where its base lives), core.8 (a launch refused whole),
core.13 (no process of flume's own), core.14 (the JSON form only gains
fields), core.15 (facts read where they live), core.16 and core.17
(held state first, safe to repeat), core.18 (removal), core.24 (a
stall), core.26 (room), core.27 (running through), core.36 (a base's
fix is a caller's launch), core.38 (points), core.40 (a place's
location under flume's data directory) and core.47 (readiness by the
merge queue). It draws core.53, amends core.13's rule and its details,
and amends core.5's, core.6's, core.25's and core.49's details
(decision 14).

## Goals / Non-Goals

**Goals:**

- One design at every depth: every branch that lands is a place's,
  submitted by flume through the merge queue, its queue stepped by
  flume's settles where nothing else drives it (46, 111, 124).
- The guarantee that every waiting place is settled between flume's
  commands is a caller's, kept over the host: the runner's passes where
  the host keeps the runner. Nothing depends on a terminal or its
  plugin (119, 208).
- A base place: what a branch that work lands on, and that itself
  lands, is in flume's terms, a branch and a record with no worktree,
  and what flume needs for a place whose base is one (202 to 206).
- Nothing names a bolt, a unit or a depth.
- A person's plain use stays whole: a place submitted by hand, a host
  whose `sluice run` drives the repository, a base that is no place's
  branch, and a host with no runner, which runs attended.

**Non-Goals:**

- Building the runner's, crew's, sluice's or weir's parts. "Named
  follow-ups" below gives each to its repository.
- A host driver for sluice, or one copy of each repository per host.
  Those are the research note's options A and E, sluice's and
  Boomdrive's.
- How fast a settle and a status run on a host with many places. The
  sibling fix `a-hosts-full-status-read-takes-seconds` takes the status
  read.
- A kind of flume's own with no stages (decision 4).

## Decisions

### 1. The host's watch is a caller's

A caller keeps watch over the host by settling every place on a bound
it states (208):

```
flume settle --every <bound>
```

`--every` takes a duration as `flume wait --timeout` does, and is
refused beside a place's name, since a watch settles every place.
`Engine::settle_every(bound)` writes the host's watch, then runs
`settle(None)` as today. The watch is a host document of the store's
(B.7, C.7), written by compare-and-set as every host document is:

```rust
pub enum HostKey { Account { .. }, Intent { .. }, Watch }

pub enum HostDocument { Account(AccountSpent), Intent(Affinity), Watch(Watched) }

/// The newest settle of every place a caller asked as its watch: when
/// it began by the host's clock, and the bound it stated (208).
pub struct Watched {
    pub began: Seconds,
    pub bound: Seconds,
}
```

A caller keeps watch while `now - began < 2 * bound`. The doubling lets
a settle that runs as long as its bound keep the watch: on deneb a
settle of every place took about 80 seconds, and a runner whose passes
come every minute starts its next pass once the last ends. A watch that
lapses only makes flume attended, which is safe (74): the commands that
act then settle every place first.

`terminals.rs`'s `attended` becomes the host's: true where no
`Watched` is held or it has lapsed. It reads no terminal. Every caller
of it keeps its meaning: an acting command settles every place first
while attended (119), and a wait drives its settles then (core.25).

Status's host answer gives the watch. Its `watch` field, which status
already gives as `Watch`, a bound or none, now gives the caller's bound,
so the JSON form keeps its field and its shape (61; core.14); `Watch`
stays in `flume-seams` as that form. It gains
`watch_settled`, when the newest such settle began, and `watch_lapsed`,
when a watch lapsed, given while flume runs attended after one. The
person's form prints `a caller keeps watch: every place settles every
<bound>, last at <time>` or `no caller keeps watch since <time>: every
place settles at each command of flume's that acts`.

### 2. The terminal's watch becomes its nudge

A terminal's settle at an agent's state change stays, as a nudge
(122): it quickens a settle a watch or a command would make, keeps no
bound, and decides nothing of attended. The terminal seam's `watch`
operation becomes `nudge`:

```rust
/// nudge: whether the terminal settles when an agent's state changes
/// (122). A nudge is no watch: flume runs attended or not by the
/// host's watch alone (119, 208).
pub enum Nudge { OnChange, None }

fn nudge(&self) -> Nudge;
```

The herdr profile answers `OnChange` while the plugin's watch holds a
fresh lease and `None` otherwise; the printed command answers `None`.
Status's terminal answer keeps its `watch` field, given `none` for
every terminal, and gains `nudge` (61).

The herdr plugin's watch drops its bound. It still reads every agent's
state in one listing per tick, runs `flume settle` naming no place
when a flume agent's state changed or an agent came up or went, writes
`flume status --json` for the sidebar and the popup, and reports the
clues. It no longer settles because time passed. Nothing waits on it.

#### Why the nudge stays beside the watch

The watch's bound is the runner's interval: `fetch_interval`, or the
shorter of it and `poll_interval` (B1), a minute or two as Boomdrive's
README gives them (`2m`, `1m`). The nudge's is herdr's tick, two
seconds (`FLUME_HERDR_TICK`). Most moves need neither. A Claude Code
agent that goes quiet settles its own place at once by its hook (121),
and that settle also starts what waits for room on the whole host
(`settle_place`'s `start_waiting`; 46, 70). The nudge gives seconds,
where the watch gives its interval, to the moves no hook reports:

- **A stage of a program with no hook of its own.** The common profile
  offers no on-quiet (171, C.4's common row), so such a stage's end is
  read, and its next stage started, only at a settle. With the nudge
  that is seconds after herdr reads the agent idle. With the watch
  alone it is the next pass, and a kind of n stages run by such a
  program waits up to n intervals between them, holding its room each
  time.
- **An agent that stopped with no word.** A Claude Code agent killed,
  crashed or closed with its pane before its stop hook ran is read lost
  (45), delivered to its recipients, and its room given to what waits,
  at the nudge (S28).
- **A start held at a question.** A start herdr read blocked past its
  bound, whose screen shows the folder-trust question, is answered at
  the next settle (34; core.12). The nudge's settle at herdr's change is
  that settle.
- **A lost conversation's process ending.** A retry of a conversation
  reported lost waits while a process runs its session, and starts at
  the first settle that reads none (49, 207). Where that process is the
  bare agent herdr brought back in the conversation's pane, the person
  ending it is a pane's change, and the nudge starts the retry.

If the nudge goes, none of these is lost: each is read at the watch's
next pass or the next command that acts. What is lost is time, the
interval instead of seconds at every such move. On a host that keeps no
runner there is no next pass: those moves wait for a person's next
command, so a common-profile stage, an agent closed by hand or a
question at a start sits until the person acts. That host, a person's
plain use under herdr, is the one the nudge serves most.

It is no watch, and what it costs is bounded. Each nudge settles every
place on the host, about 80 seconds on deneb. The plugin awaits each
settle and folds the changes it saw meanwhile into the next, so at most
one nudge's settle runs per herdr server, beside the watch's, and both
are safe to repeat (74; core.17). It decides nothing: flume runs
attended or not by the host's watch alone, nothing waits on a nudge,
and a host without herdr's plugin is as correct, only slower at those
moves.

### 3. A host with no runner

A host where no caller keeps watch runs attended (119). Its places move
at their agents' quiet (121), at a terminal's nudge (122), and at each
command of a caller's that acts, which settles every place first and
so steps every open submission's base (124). A person's own use is
that: they launch, retry, drop and wait, and each act moves every
place. A place submitted by hand lands as before (63, S30). Where the
host runs `sluice run`, its queues move on their own, and the next
settle reads the outcomes (sluice 135). A host may keep its own watch
with no runner: a timer that runs `flume settle --every <bound>` is a
caller like any other (208).

### 4. A base place is a kind's shape: a kind that lands with no stages

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
none: the conformance suite and the profile tests declare theirs.

### 5. A base place is a branch and a record

A base place has no worktree (9, 202). It is its branch, which the
merge queue makes and closes at flume's asking (205, 206), and its held
document. No operation of the place seam is asked of it: no make, ready,
read, remove or remove-branch (78 to 82). So it checks out nothing
(10), and nothing the merge queue lands on its branch moves under it.

It is named as any place is, by the location its place profile gives
a place of its kind and name (C.1; core.40). Nothing is made there.
The location keeps a base place's identity the same shape as every
other's, `<data>/flume/places/<repository id>/<kind>--<name>`, so the
store, status, the list and a caller's acts find it as they find any
place, and a launch is refused where anything stands at it (3).

Its held document is any place's: `location` holds the location,
`branch` and `base` its branch and base, `base_revision` the revision
the merge queue made the branch at, and `binding` the repository's
binding as the launch resolved it (21). A document's shape does not
change: a base place is told by its kind, which the document holds.

### 6. Its launch

`launch.rs` takes a base place's launch as any launch until the place's
location is known, and then:

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
   revision it was made at, kept as `base_revision`. `Opened::Refused {
   reason }` refuses the launch with the merge queue's words and removes
   the document, nothing else made.
4. **Live, with nothing started.** The place goes `Live` with no
   current stage. No worktree is made, no readiness runs (11), no start
   is made, no room or points are counted (70, 165), and the launch
   answers `Launched` with the place.

`undo` (`settle.rs`), for a launch whose command died while making,
closes the branch where open made it and removes the document.

### 7. A launch on a place's branch

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
  (decision 9).
- **A base place otherwise.** The launch goes on. The new place is cut
  at the base place's branch where the merge queue's profile says it
  lives (126, C.1), as on any base, and lands there (15).

A base place may itself be launched on a base place's branch: the same
reads hold at every depth.

### 8. Its submission and its outcome

A base place has no last stage, so nothing makes its submission due
(111). It is submitted only by its caller's retry naming the submission
(47, 49, 203). `acts.rs`'s retry of a base place's submission:

- is refused, naming them, while a place whose base is its branch is
  live;
- otherwise calls `submit(id, true)`, which reads the branch's revision
  where the merge queue says it lives, `base_revision(repository,
  branch)`, and submits it named:

```rust
/// What a submission sends: the place's worktree head, from the place,
/// or a branch the merge queue lands on, named, from the repository,
/// with no worktree as its submitter (117, 203).
pub enum SubmitHead {
    Worktree { head: Revision },
    Branch { branch: String, revision: Revision },
}

fn submit(&self, place: &PlaceRef, head: &SubmitHead) -> Submitted;
```

The outcome has no inbox to come through, since the base place has no
worktree (133). `submit.rs` takes a base place's outcomes from
`read_submission(repository, branch)` alone, as it already takes a
request's entry's outcome from the record where the merge queue
delivered it elsewhere (124, C.6): landed with its commit, failed with
its words, in conflict with its files, its words and the revision it
was tested on, or withdrawn. Each is acted on once, as the held
submission's verdict records it, and delivered to the base place's
recipients as any end is (43). `collect` is never called for a base
place. A landing makes the place's work over (63, 113). A person's own
submission of the branch, by hand from a checkout of theirs, lands it
the same way (63).

### 9. Its failure

`verdict` (`submit.rs`) on a base place's failed or conflicting
submission starts no stage, since its kind names none. It records the
verdict and sets `submission.waiting` to the cause and the merge
queue's words, as a withdrawn submission does, and the settle delivers
it to the place's recipients (43, 112, 203). The place waits on its
caller.

Its caller lands the fix through a place on its branch: a fix launched
with the base place's branch as its base and the words as its brief,
which decision 7 allows since the submission is no longer open. For a
conflict the words carry the revision of the base place's base that the
merge queue tested against, and the fix's agent merges that revision
into the fix's branch first, as 112 has any conflict's fix do. The fix
lands on the base place's branch, bringing the base's revision in, and
the caller retries the base place's submission. flume launches no fix
of its own (36; core.36).

### 10. What the base place waits on, and status

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
its branch`. `PlaceStatus` gains, for a base place alone, `worktree:
false` and `on_branch: Vec<PlaceId>`, the live places whose base is its
branch; both are left out for every other place, so the JSON form only
gains fields (61). Its head is read through the merge queue's base on
its branch (80, 126), and nothing is read at its location.

The place's line gives `<kind>--<name> · <n> on its branch · open`
while places on its branch are live, `waiting on its caller` once none
is, and then `submitted`, `landed` and the rest as any place's (56,
139). The list gives the same words (200). The herdr plugin's sidebar
shows a base place as status gives it; it has no pane to open.

### 11. The drop and the tidy

A drop of a base place is refused, naming them, while a place whose
base is its branch is live (52, 204): those places land on its branch,
and the caller drops them first. A done is refused as on any kind that
lands (114).

`tidy.rs` tidies a base place with no worktree to remove and no
teardown to run (64 to 66, 204): it calls `MergeQueue::close(repository,
branch, base)` in place of `Place::remove` and `Place::remove_branch`,
then removes the held document as any place's once its ends are
delivered (62):

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
keeps any branch whose base cannot be read (67). Decision 8's refusal
makes this rare: the base place is submitted only once every place on
its branch is over, and a landed place is tidied at the next settle.

### 12. The seams and the profiles

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

and `submit` takes a `SubmitHead`. The terminal seam's `watch` becomes
`nudge` (decision 2). The store seam's host documents gain the watch
(decision 1). The place seam is unchanged.

The sluice profile (C.6):

| operation | sluice |
|---|---|
| open | `sluice branch create <branch> --from <base> --json`, run in the repository; its answer's revision; exit 2 a refusal in its `error.message` |
| close | `sluice branch delete <branch> --into <base> --json`, run in the repository; exit 1 gone, exit 2 kept with its words |
| submit, a branch | `sluice submit <base> --head <branch> --no-wait --json`, run in the repository; answered as any submit; its outcome read-submission's, the entry whose `from.head` is the branch |

Each runs with every multiplexer's session stripped and
`GIT_TERMINAL_PROMPT=0`, as the profile's other commands do (C.6). No
`sluice inbox` runs for a base place. The pinned sluice, 0.67.0, takes
`--head` only with `--check`, and records the worktree it runs in as a
submitter. The submit by name waits on sluice's follow-up S1, and the
profile's test of it on the pin that carries it.

The git worktree place profile (C.1) is asked only for a base place's
location, which it gives as for any place; it makes, reads and removes
nothing of one.

The kinds profile reads a kind with `stages = []` through weir-kinds at
the revision weir's follow-up W1 lands, mapping it to a kind with no
stages; a kind that does not land and declares none is refused in
weir's words (4, 18; core.4).

The herdr profile answers its nudge from the plugin's lease (decision
2). The no-tool merge-queue profile answers `open` and `close` as
refusals naming that no merge queue is bound; a launch of a base place
under it is refused before that as any landing kind's is.

### 13. The fakes

The fake merge queue gains `open` and `close`, scripted to make,
refuse, keep or answer gone, each logged as `open <branch> from
<base>` and `close <branch> into <base>`, and records a submit by name
as `submit <branch> named`. A branch it opened is one it lands on: a
submission from a place on it lands there when scripted to, and a
submission by name is decided in its record alone, its inbox giving
nothing for it. The fake terminal's `set_watch` becomes `set_nudge`,
and a test makes flume unattended or not by the host's watch:
`engine.settle_every(seconds)` keeps it, and the fake clock passing
twice its bound lets it lapse. The fake kinds hold a kind with no
stages as any kind. The fake place logs nothing for a base place.

### 14. How it is held

- **The statement**, on agentplot/blueprints' branch
  `design/flume-every-place-on-the-queue-is-settled-at-every-depth` at
  c2e3170, cut from main at c94c496 and unpushed. flume: the
  vocabulary's place, base place, watch, nudge and attended entries, the
  preamble, 9, 10, 14, 18, 46, 47, 67, 78, 80, 82, 111, 112, 117, 119,
  120, 122, 133, 165, 176, the common profile's list and row, a wait's
  wording, the new 202 to 204 and 208 in Part A and 205 and 206 in B.6,
  C.1, C.5's nudge column, C.6's submit, inbox, step, open and close
  rows and its submit by name, C.7's host state, section 9's surfaces, givens, callers
  and the runner's row, I8, section 12, S27, S28, S91 and S109 to S112 and S118.
  Main's lost-conversation design (flume record 0052, d378220 and
  c94c496) took 207, the agent seam's live, and S113 to S117, so the
  watch is 208 and its host with no runner S118; S109 to S112 stay
  after S108, where that design leaves them. sluice's 8bf02d9 on main
  amends 71, 150 and S75, none of them 124. Of flume's clauses this
  change amends, d378220 and c94c496 touched 43, C.4 and C.5, each in
  words this change leaves alone: 43 gains what
  a lost conversation's end says beside this change's wait, C.4 gains
  Claude Code's carry and live beside the common profile's row this
  change rewords, and C.5 gains herdr's restore below the table whose
  column becomes the nudge. Each holds read with the other. The watch
  serves record 0052's wait: a lost conversation's retry, with no agent
  of flume's in its place, starts within the watch's bound once no
  process runs its session, or at a nudge or a command that acts on a
  host with no watch (49, 207).
  weir: 51, 52, 57 and the format's example. sluice: 124's submission
  naming its head from anywhere in the repository, with no worktree as
  its submitter. They hold on the user's yes, and the orchestrator
  pushes the branch.
- **Record 0051**, proposed: "Every branch that lands is a place's, a
  caller keeps watch over the host, and every place waiting on the
  merge queue is settled at every depth", quoting the user's note and
  their two answers.
- **core.53**, under "Branches, submissions and landings": a branch
  other places land on is a base place's, a branch and a record with no
  worktree; the merge queue alone makes and removes its branch at
  flume's asking; it runs no agent; it is submitted only on its caller's
  retry, by its branch's name from the repository, once no place on its
  branch is live; and flume names no depth and no use of one.
  **core.13**'s rule: every settle runs inside a command, started by an
  agent going quiet, by a caller keeping watch, by a terminal's nudge or
  by any other caller. Its details, and core.5's, core.6's, core.25's
  and core.49's, follow the watch and the base place.
- **The site**: the merge-queue guide gains "Branches that land on
  branches" and says who moves a submitted place; the places-and-herdr
  guide says a caller keeps the host's watch and herdr's plugin nudges;
  the commands reference gives `flume settle --every`; the state
  reference gives the host's watch, the nudge, `on_branch`, `worktree`
  and `waits_on: places`; the herdr plugin's README says the watch
  nudges and keeps no bound (core.35).

## Who owns what, and what each tool stops doing

### One owner for each piece

| piece | owner | cited by |
|---|---|---|
| a queue's rounds, and the host's driver where one runs | sluice (135) | flume, the runner, crew |
| a step of a queue where no driver lives, at every depth | flume's settle (124, 125, C.6) | the runner, crew |
| every settle of every place between flume's commands | the caller keeping watch over the host (208): the runner's passes where the host keeps the runner (B1) | flume, crew |
| whether flume runs attended | flume, from the host's watch alone (119, 208) | the runner, the herdr plugin |
| a quicker settle at an agent's change | the terminal's nudge, herdr's plugin (122) | nobody: nothing waits on it |
| a branch sluice lands on, made and removed | sluice (104 to 106), asked by flume's base place (202, 204) | the runner, crew |
| a submission of any branch that lands, and its outcome | flume (111, 133, 203) | the runner, crew |
| a kind with no stages | weir's format (51, 57) | flume, crew |
| bolts, units and when a bolt is sent | crew | the user |

Where the merge queue runs is then one answer at every depth: sluice's
rounds run in the repository, moved by the host's `sluice run` where
the host keeps one, and otherwise by the steps of the settles of the
places submitted to it, whether the base is `main` or a base place's
branch. Those settles come between flume's commands from the host's
watch, which the runner keeps. Neither the runner nor crew drives a
queue, and no terminal is relied on.

### Named follow-ups

- **agentplot/weir, W1: a kind that lands may declare no stages.**
  weir 51, 52 and 57 as the statement branch amends them; weir-kinds
  reads `stages = []` on a kind that lands, refuses it on one that does
  not, and reads a table that leaves `stages` out as settings, as
  today. flume's task 3.4 moves its pin to the revision that lands it.
- **agentplot/sluice, S1: a submit names its head, from no worktree.**
  sluice 124 as amended: `sluice submit <target> --head <branch>
  --no-wait --json` without `--check` submits a branch sluice lands on
  as its policy holds it, the remote's revision where it names one, run
  anywhere in the repository, a bare one's git directory among it. The
  entry records no worktree and no session as its submitter and the
  branch as its head (`from.head`). Its outcome is pending in no inbox:
  `sluice inbox` anywhere answers nothing for it, and `sluice status
  <target> --all --json` gives it with the words an inbox gives, the
  failed checks' notice, the conflict's paths and `tested.on`. flume's
  task 5.2 moves its pin of sluice to the release that carries it.
- **agentplot/sluice, S2: a round keeps the room it counted.** The
  research note's two gaps: a round counts room before its checkout and
  asks for it after, and nothing gives a queued candidate the first free
  slot ahead of a session's run that retries at every pass. They held
  deneb's queues on 2026-11-06 while flume stepped them. No clause of
  flume's changes for them.
- **agentplot/boomdrive, B1: the runner keeps the host's watch, and a
  bolt is a launch.**
  - *The watch.* Each pass of `boomdrive run`, keeping or control,
    ends with one `flume settle --every <bound>`, run once and awaited
    under the pass's own bound, its answer's errors named in the
    runner's log and none stopping the loop. The bound is the interval
    within which its next pass begins: `fetch_interval`, or the shorter
    of it and `poll_interval` where a control plane is named (the
    running spec). `boomdrive run --once` runs it once. So a host that
    keeps the runner is watched while the runner runs, and runs
    attended once it stops (flume 119, 208). This is the user's word of
    2026-11-06, "I'd rather it be the Boomdrive process". It amends
    Boomdrive's core.13, whose details rule out "a `flume settle` loop
    ... spawned, watched or restarted by the runner" and "a tool's
    process started with no control plane's ask behind it": the pass's
    settle is a tool's own command run once and awaited (its core.16),
    no loop of the tool's and no process the runner keeps, and core.13
    names it as the one tool command a pass runs with no ask. It amends
    record 0003's "runs no tool's housekeeping of its own accord" by a
    record of Boomdrive's own quoting the user. core.1 holds: the settle
    decides nothing, and flume decides what each settle does.
  - *A bolt.* The runner retires its branch command: `sluice branch
    create` in `crates/boomdrive/src/sluice.rs` (line 105) and
    `carry.rs` (611 to 662), the branch-commands spec's "A bolt's branch
    is made by sluice in the clone", and record 0002's leased Command
    for a branch. A control plane's bolt is a `flume launch` of its
    caller's kind with no stages, passed as given (its core.15), its
    units launched with `--base <its branch>` as they are now. A bolt's
    send is a `flume retry <place> --submission` the control plane asks,
    a tool's own command run once and awaited (its core.16). Its
    tool-records spec's read of `sluice status <id>` for each submission
    flume names stays: it reads, and drives nothing. Its core.14 holds:
    sluice alone makes and moves the bolt's branch in the clone. It
    cites flume 119, 124, 202 to 204 and 208.
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
by crew's path as today: decision 7's first case. Until B1's watch
lands, a host that keeps the runner runs attended, its submitted places
moved by its callers' commands and by herdr's nudge where herdr is its
terminal.

## Alternatives Considered

- **The terminal's watch keeps every place moving.** What the build
  does today: herdr's plugin settles every place on its bound. It ties
  the guarantee to one terminal's plugin and its lease, and leaves a
  host under any other terminal attended. The user ruled it out.
- **The host's configuration declares a watch.** A key such as `[host]
  watch = "2m"` beside the runner's own intervals. Two settings kept in
  step on every host, and a runner that stops leaves flume believing
  places move. A caller that states its bound at each settle keeps one
  setting, and its lapse is read from the host's own record.
- **Every acting command settles every place, always.** No watch to
  read. On a host of fifty places each `flume needs`, retry and launch
  would wait on a settle of all of them, and a submitted place would
  still move only when someone acts.
- **The runner or crew steps the outer branch's queue.** The research
  note's option C, the runner's half. It makes a second driver beside
  flume's settle (the runner's core.1, record 0003), and keeps one
  depth outside flume. The user called it not symmetric.
- **A base place with a worktree.** A worktree on its branch breaks
  clause 10: the merge queue levels the host's copy under it after
  every landing (sluice 124), git then reads the worktree as holding a
  change, and its tidy keeps it for ever (65). A detached worktree keeps
  10 but gives a person a copy of the base as it stood at the launch,
  runs readiness and teardown for nothing, and needs the place seam to
  make a worktree with no branch. The user chose no worktree.
- **flume makes the branch itself.** `git worktree add -b`, as for any
  place. sluice makes and removes a branch it lands on alone, and its
  protection refuses anyone else's (sluice 104 to 106); on a branch kept
  on a remote, the branch must also be pushed, which flume never does
  (core.5).
- **A kind field naming a base place.** `base = true`, or a kind's
  stage that waits forever. Either is a field flume would define in
  weir's format (core.4), or an agent started for nothing. A kind with
  no stages says it in the format's own terms.
- **A kind of flume's own.** Decision 4.

## Risks / Trade-offs

- **The build waits on three follow-ups.** The kinds profile's read
  waits on weir's W1, the sluice profile's submit by name on sluice's
  S1, and a watched host on the runner's B1. Groups 1, 2 and 4 need
  none of them: the watch and the base places run over the fakes first.
  A build that finds W1 or S1 missing stops short naming it.
- **A host that keeps the runner runs attended until B1.** Its acting
  commands settle every place first, which on a host of many places is
  slow. herdr's nudge still settles at each agent's change. The status
  read's sibling fix takes the cost of a settle of every place.
- **A watch that lapses mid-settle.** A settle of every place that runs
  past twice its bound lets the watch lapse until the next one begins,
  and commands settle every place first meanwhile. That is slower, never
  wrong (74).
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

Nothing held changes shape. A place document gains no field; the host
gains one document, the watch, written by the first `flume settle
--every`. Status keeps its `watch` fields and only gains `watch_settled`,
`watch_lapsed`, `nudge`, `on_branch` and `worktree`; a tidy only gains
`BranchRemoved::Refused` (61). A host whose terminal is herdr runs
attended from this release until a caller keeps watch: its operator
sees `no caller keeps watch` in status, and the herdr plugin keeps
nudging. Bolts made before this change, by crew or the runner, are
bases no place holds and keep landing by crew's path until C1. Places
already launched on them land there as today.

## Open Questions

None for this change. The follow-ups' own designs decide their
commands' spellings: S1's `--head` without `--check`, how B1 states its
bound, and how B1's control plane carries a retry.
