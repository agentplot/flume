---
number: 51
title: Every branch that lands is a place's, a caller keeps watch over the host, and every place waiting on the merge queue is settled at every depth
status: proposed
date: 2026-11-06
decision-makers:
- Chuck Swanberg
---

# Every branch that lands is a place's, a caller keeps watch over the host, and every place waiting on the merge queue is settled at every depth

Status: proposed on 2026-11-06 by the construct stage of the change `every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`. The direction is the user's: on Pending You, 2026-11-06, they approved the plan to close the gaps between flume's merge-queue design and deneb, with the note quoted below; and, reading the design the same day, they ruled that a base place has no worktree and that the runner, not herdr's plugin, settles every waiting place between commands, in the words quoted below. How a caller states its watch and how flume reads it lapsed, the terminal's nudge, the base place's shape, the refusals that keep the depths apart and shipping no kind of that shape are the stage's choices. The record is accepted on the user's yes to it and to the statement's amendment.

Deciders: the user, by their note and their two answers; the construct stage, on clauses 10, 46, 111, 119, 122 and C.6 and rules core.4, core.5, core.6 and core.13.
Sources: `agentplot/blueprints:design/flume/requirements.md` on the branch `design/flume-every-place-on-the-queue-is-settled-at-every-depth` at c2e3170, cut from main at c94c496, with weir's 51, 52 and 57 and sluice's 124 beside it; the research note `research-queues-driven.md` of 2026-11-06 (the research place `research--how-a-host-drives-the-queues-of-the-repositories-it-keeps`); flume's log on deneb, 00:54Z to 02:34Z on 2026-11-06; crew's `plugin/lib/plan.py` at 23b3b3dc (`bolt_give`, `BOLT_CREATE`, `SUBMIT`, `land_delete`) and its rules plan.8, plan.16, plan.18 and plan.19; Boomdrive at 2937b09: `crates/boomdrive/src/sluice.rs` and `carry.rs`, its rules core.1, core.13, core.14, core.15 and core.16, its records 0002 and 0003, and its running spec's passes; `plugins/herdr/bin/watch.sh`, `crates/flume/src/commands/mod.rs`, `crates/flume-core/src/settle.rs`, `submit.rs` and `terminals.rs`.

## Context and problem statement

The user's note, whole:

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

Their answers to the design, 2026-11-06. On the outer branch's place: "No worktree: a branch and a record". On a terminal's watch settling every place: "I do not like this as a solution. I do not like relying on the Herder plugin to make this work. I'd rather it be the Boomdrive process."

Two gaps stood in the way. What moved a submitted place between flume's commands was herdr's plugin: its watch runs `flume settle` naming no place on its bound, which settles every place the store holds, and each settle of a submitted place steps its base. flume's log on deneb shows the runner's swancloud chores each settled 43 times between 00:54Z and 02:34Z on 2026-11-06 with no agent in them. flume read whether it ran attended from whether each terminal kept a watch, so under any other terminal, or with the plugin's lease stale, a submitted place moved only when a caller acted. The rounds those settles ran found no room, since the host's sluice keeps no slot for candidates and a round asks for room only after its checkout: those are sluice's and the host's.

And a crew bolt's branch had no place. crew or the runner made it with `sluice branch create`. ops submitted it with `wt submit main` from a worktree crew made, crew waited on it with a background `sluice wait`, and crew removed it with `sluice branch delete`. The unit places on the bolt were flume's, and landed through flume's settles. One depth was flume's and the other crew's and the runner's.

## Considered options

For who settles every waiting place between commands:

1. **A caller keeps watch over the host.** `flume settle --every <bound>` settles every place and records in the host's state when it began and its bound; a caller keeps watch while that settle began less than twice its bound ago, and flume runs attended while none does. The runner's passes keep it on a host that keeps the runner. A terminal's settle at an agent's change stays as a nudge that decides nothing.
2. **The terminal's watch settles every place,** as herdr's plugin does on its bound.
3. **The host's configuration declares a watch,** `[host] watch = "2m"`, beside the runner's own intervals.
4. **Every acting command settles every place, always,** with no watch read.
5. **The runner or crew steps the outer branch's queue,** a `sluice step` for each bolt submitted.

For what the outer branch is:

6. **A base place: a branch and a record with no worktree.** A place whose kind lands and has no stages, `stages = []` in weir's format. flume asks the merge queue to make its branch and later to close it, makes no worktree, starts no agent, names it by the location its place profile gives where nothing is made, submits its branch by name from the repository on its caller's retry once no place on its branch is live, and reads its outcome from the merge queue's record of the branch.
7. **A base place whose worktree checks out its branch,** as crew's landing worktree does for the instant of its submit.
8. **A base place with a detached worktree,** at the revision the merge queue made its branch at.
9. **A base place whose branch flume makes itself,** by `git worktree add -b`.
10. **A field of flume's in the kind,** `base = true`, or a stage that waits for ever.
11. **A kind of that shape of flume's own,** such as `gather@1`.

## Decision outcome

Options 1 and 6: the statement's 202 to 206 and 208, with the vocabulary, 9, 10, 14, 18, 46, 47, 67, 78, 80, 82, 111, 112, 117, 119, 120, 122, 133, 165, 176, C.1, C.5, C.6, C.7, section 9, I8, S27, S28, S91 and S109 to S112 and S118 amended or added beside them, weir's 51, 52 and 57, and sluice's 124.

A caller keeps watch over the host by settling every place on a bound it states, `flume settle --every <bound>`. flume keeps, as a host document written by compare-and-set, when the newest such settle began and its bound. A caller keeps watch while that settle began less than twice its bound ago, so a settle that runs as long as its bound keeps it; flume runs attended otherwise, every acting command settling every place first and a wait driving its settles. No terminal decides it. Status gives the watch, its bound and when it last settled, or since when none is kept. On a host that keeps the runner, each of the runner's passes runs `flume settle --every` once and awaits it, the runner's follow-up amending its core.13 and its record 0003 on the user's word. A host with no runner runs attended, a person's plain use, and a timer of the host's own may keep the watch as the runner does. A terminal's settle at an agent's state change stays as its nudge: herdr's plugin settles when an agent changes and keeps no bound, so an agent whose program has no hook, one that crashed, a start held at the folder-trust question and a lost conversation's process ending in its pane are read within herdr's tick rather than the watch's interval, and nothing waits on it. Retiring it loses no move, each read at the watch's next pass or the next command that acts, but costs that interval at each such move, and on a host with no runner leaves them to a person's next command.

A base place is a place whose kind lands and has no stages, known by that shape alone. It has no worktree: it is its branch, which flume asks the merge queue's open to make at its base, and the state flume holds of it, named by the location its place profile gives where nothing is made. It runs no readiness, starts no agent and counts no room or points. A launch on a live place's branch is refused where that place is no base place, or where the base place's submission is open or its work is over. A base no place holds is launched on as today. A retry naming a base place's submission is refused while a place on its branch is live, and otherwise submits its branch by name from the repository, with no worktree as its submitter. Its outcome is read from the merge queue's record of its branch, since it has no worktree for an inbox, and delivered to its recipients. Its failure waits on its caller, who lands a fix through a place on its branch. A drop of it is refused while a place on its branch is live. Its tidy removes no worktree and runs no teardown, and asks the merge queue's close for its branch into its base. Status gives the places on its branch, that it has no worktree, and whom it waits on.

So where the merge queue runs has one answer at every depth. sluice's rounds run in the repository, moved by the host's `sluice run` where the host keeps one and otherwise by the settles of the places submitted to it, whether their base is `main` or a base place's branch. Those settles come between flume's commands from the host's watch, which the runner keeps. Neither the runner nor crew drives a queue, and no terminal is relied on. The runner's follow-up keeps the watch at each pass, retires its branch command and launches a bolt as a base place of its caller's kind. crew's retires `BOLT_CREATE`, `crew bolt submit`'s worktree, `wt submit` and background wait, and `crew bolt land`'s branch delete, and declares its own kind with no stages. weir's lets a kind that lands declare `stages = []`. sluice's takes a submission naming a branch it lands on as its head from no worktree, its outcome read from its records, and keeps the room a round counted. A person's own use stays as it was: a place submitted by hand lands as before, and a host whose `sluice run` drives the repository sees the settles' steps do nothing.

Option 2 rests the guarantee on one terminal's plugin and its lease, which the user ruled out. Option 3 keeps two settings in step on every host, and a runner that stops leaves flume believing its places move; a caller stating its bound at each settle keeps one setting, and its lapse is read from the host's own record. Option 4 makes every `flume needs`, retry and launch on a host of fifty places wait on a settle of all of them, and still moves a submitted place only when someone acts. Option 5 makes a second driver beside flume's settle, which the runner's core.1 and its record 0003 rule out, and keeps one depth outside flume: the asymmetry the user named. Option 7 breaks clause 10: the merge queue levels the host's copy of the branch under the worktree after every landing (sluice 124), git then reads the worktree as holding a change not committed, and its tidy keeps it for ever (65). Option 8 keeps clause 10 but shows a person the base as it stood at the launch, runs readiness and teardown for nothing, and asks the place seam for a worktree on no branch; the user chose no worktree. Option 9 makes a branch sluice lands on behind sluice's back, which its protection refuses (sluice 104 to 106), and a branch kept on a remote must be pushed, which flume never does (core.5). Option 10 defines a field of weir's format in flume (core.4), or starts an agent for nothing. Option 11 puts a kind's name and prefix on the site every host's operator reads (core.35) when nothing here needs one; it stays a package away.

### Consequences

- Good: one design at every depth. Every branch that lands is submitted by flume through the merge queue, and every queue no driver moves is stepped by flume's settles, nested as deep as a caller nests base places.
- Good: the guarantee that waiting places move between commands is the host's watch, kept by a caller, read by flume from its own record, and lapsing to attended when the caller stops. No terminal and no plugin is relied on.
- Good: the runner and crew stop making, submitting, waiting on and removing branches. Their follow-ups cite flume's clauses and drive nothing.
- Good: flume moves no more branches than before, and a base place has nothing on the host to tidy but its record. Its branch is the merge queue's to make and remove, at flume's asking.
- Bad: a host that keeps the runner runs attended until the runner's follow-up lands, its acting commands settling every place first; herdr's nudge still settles at each agent's change.
- Bad: the build waits on two other follow-ups, weir's `stages = []` for the kinds profile and sluice's submit by name from no worktree for the sluice profile. The core and the conformance suite build over the fakes before either lands.
- Bad: steps do not make room. A host whose sluice keeps no slot for candidates stalls its queues however often flume steps them, until sluice's and the host's follow-ups.
