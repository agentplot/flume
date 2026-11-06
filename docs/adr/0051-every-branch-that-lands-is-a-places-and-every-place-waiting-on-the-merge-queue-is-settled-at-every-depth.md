---
number: 51
title: Every branch that lands is a place's, and every place waiting on the merge queue is settled at every depth
status: proposed
date: 2026-11-06
decision-makers:
- Chuck Swanberg
---

# Every branch that lands is a place's, and every place waiting on the merge queue is settled at every depth

Status: proposed on 2026-11-06 by the construct stage of the change `every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`. The direction is the user's: on Pending You, 2026-11-06, they approved the plan to close the gaps between flume's merge-queue design and deneb, choosing "Flume's terminal watch also settles places waiting on the queue", with the note quoted below. The base place's shape, its detached worktree, the refusals that keep the depths apart and shipping no kind of that shape are the stage's choices. The record is accepted on the user's yes to it and to the statement's amendment.

Deciders: the user, by their note; the construct stage, on clauses 10, 46, 111, 122 and C.6 and rules core.4, core.5 and core.6.
Sources: `agentplot/blueprints:design/flume/requirements.md` on the branch `design/flume-every-place-on-the-queue-is-settled-at-every-depth` at fb15fac, cut from main at 12866a8, with weir's 51, 52 and 57 and sluice's 124 beside it; the research note `research-queues-driven.md` of 2026-11-06 (the research place `research--how-a-host-drives-the-queues-of-the-repositories-it-keeps`); flume's log on deneb, 00:54Z to 02:34Z on 2026-11-06; crew's `plugin/lib/plan.py` at 23b3b3dc (`bolt_give`, `BOLT_CREATE`, `SUBMIT`, `land_delete`) and its rules plan.8, plan.16, plan.18 and plan.19; Boomdrive's `crates/boomdrive/src/sluice.rs` and `carry.rs` at e3aa0f8, its rules core.13 and core.14 and its records 0002 and 0003; `plugins/herdr/bin/watch.sh`, `crates/flume/src/commands/mod.rs`, `crates/flume-core/src/settle.rs` and `submit.rs`.

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

Two gaps stood in the way. The statement had the watch settle "each place with an agent in it" (122). The herdr plugin's watch runs `flume settle` naming no place, which settles every place the store holds, and each settle of a submitted place steps its base. flume's log on deneb shows the runner's swancloud chores each settled 43 times between 00:54Z and 02:34Z on 2026-11-06 with no agent in them. No clause, spec or check held that, so a later change could narrow the watch to the statement's words. The rounds those settles ran found no room, since the host's sluice keeps no slot for candidates and a round asks for room only after its checkout: those are sluice's and the host's.

And a crew bolt's branch had no place. crew or the runner made it with `sluice branch create`. ops submitted it with `wt submit main` from a worktree crew made, crew waited on it with a background `sluice wait`, and crew removed it with `sluice branch delete`. The unit places on the bolt were flume's, and landed through flume's settles. One depth was flume's and the other crew's and the runner's.

## Considered options

1. **Every branch that lands is a place's.** A branch that work lands on, and that itself lands, is a base place's: a place whose kind lands and has no stages, `stages = []` in weir's format. flume asks the merge queue to make its branch and later to close it, makes its worktree detached at the revision the merge queue made the branch at, starts no agent in it, and submits it by name on its caller's retry once no place on its branch is live. The watch's settle of every place steps every open submission's base at every depth. Nothing names a bolt, a unit or a depth.
2. **The runner or crew steps the outer branch's queue.** The runner's pass, or crew's waiter, runs `sluice step` for each bolt submitted.
3. **A base place whose worktree checks out its branch,** as crew's landing worktree does for the instant of its submit.
4. **A base place whose worktree takes its branch only at its submission,** by `git switch`.
5. **A base place with no worktree,** a branch and a held document alone.
6. **A base place whose branch flume makes itself,** by `git worktree add -b`.
7. **A field of flume's in the kind,** `base = true`, or a stage that waits for ever.
8. **A kind of that shape of flume's own,** such as `gather@1`.

## Decision outcome

Option 1, the statement's 202 to 206, with 10, 14, 18, 46, 47, 67, 78, 80, 82, 111, 112, 117, 122, 165, C.1, C.5, C.6, I8 and S109 to S112 amended or added beside them, weir's 51, 52 and 57, and sluice's 124. The core knows a base place by its kind's shape alone. Its launch writes its held state, asks the merge queue's open for its branch at its base, makes its worktree detached at that revision, and runs no readiness, starts no agent and counts no room or points. A launch on a live place's branch is refused where that place is no base place, or where the base place's submission is open or its work is over. A base no place holds is launched on as today. A retry naming a base place's submission is refused while a place on its branch is live, and otherwise submits its branch by name at the revision the merge queue reads for it. Its failure waits on its caller, who lands a fix through a place on its branch. A drop of it is refused while a place on its branch is live. Its tidy removes its worktree as any place's and asks the merge queue's close for its branch into its base. Status gives the places on its branch and whom it waits on.

So where the merge queue runs has one answer at every depth. sluice's rounds run in the repository, moved by the host's `sluice run` where the host keeps one and otherwise by the settles of the places submitted to it, whether their base is `main` or a base place's branch. Neither the runner nor crew drives a queue. The runner's follow-up retires its branch command and launches a bolt as a base place of its caller's kind. crew's retires `BOLT_CREATE`, `crew bolt submit`'s worktree, `wt submit` and background wait, and `crew bolt land`'s branch delete, and declares its own kind with no stages. weir's lets a kind that lands declare `stages = []`. sluice's takes a submission that names its head, and keeps the room a round counted. A person's own use stays as it was: a place submitted by hand lands as before, and a host whose `sluice run` drives the repository sees the settles' steps do nothing.

Option 2 makes a second driver beside flume's settle, which the runner's core.1 and its record 0003 rule out, steps at most once a pass, and keeps one depth outside flume: the asymmetry the user named. Option 3 breaks clause 10. The merge queue levels the host's copy of the branch under the worktree after every landing (sluice 124), and git then reads the worktree as holding a change not committed, so its tidy keeps it for ever (65). A scratch repository showed it: a worktree on `g`, `git update-ref refs/heads/g` to a later commit, and `git status --porcelain=v2` reading `1 M.` for the file. Option 4 still checks out a branch the merge queue lands on, stale again wherever the copy is levelled after the switch. Option 5 leaves every seam that reads a place as a worktree, and sluice's submitter and inbox, which are keyed by the worktree a command runs in. Option 6 makes a branch sluice lands on behind sluice's back, which its protection refuses (sluice 104 to 106), and a branch kept on a remote must be pushed, which flume never does (core.5). Option 7 defines a field of weir's format in flume (core.4), or starts an agent for nothing. Option 8 puts a kind's name and prefix on the site every host's operator reads (core.35) when nothing here needs one; it stays a package away.

### Consequences

- Good: one design at every depth. Every branch that lands is submitted by flume through the merge queue, and every queue no driver moves is stepped by flume's settles, nested as deep as a caller nests base places.
- Good: the watch's settle of every place is held by a clause, a spec and three checks, so no later change narrows it.
- Good: the runner and crew stop making, submitting, waiting on and removing branches. Their follow-ups cite flume's clauses and drive nothing.
- Good: flume moves no more branches than before. A base place's branch is the merge queue's to make and remove, at flume's asking.
- Bad: the build waits on two follow-ups, weir's `stages = []` for the kinds profile and sluice's submit by name for the sluice profile. The core and the conformance suite build over the fakes before either lands.
- Bad: a base place's worktree shows the revision its branch was made at, never its branch's head. Status and the list give the head from the merge queue, and the guide says so.
- Bad: steps do not make room. A host whose sluice keeps no slot for candidates stalls its queues however often flume steps them, until sluice's and the host's follow-ups.
