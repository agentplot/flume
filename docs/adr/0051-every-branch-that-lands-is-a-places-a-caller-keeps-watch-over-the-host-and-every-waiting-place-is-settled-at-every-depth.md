---
number: 51
title: Every branch that lands is a place's, a caller keeps watch over the host, and every place waiting on the merge queue is settled at every depth
status: accepted
date: 2026-11-06
decision-makers:
- Chuck Swanberg
---

# Every branch that lands is a place's, a caller keeps watch over the host, and every place waiting on the merge queue is settled at every depth

Status: accepted by the user on 2026-11-06, on their card ("Approve: push the statement, start code, launch the follow-ups in order"), its statement on boomdrivedev/blueprints main at 51b6316. Amended the same day on their ruling on Pending You that no terminal settles, quoted below, its statement on boomdrivedev/blueprints main at 6dc9399; the amendment is proposed until they see it as a record. Proposed by the construct stage of the change `every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`. The direction is the user's: on Pending You, 2026-11-06, they approved the plan to close the gaps between flume's merge-queue design and deneb, with the note quoted below; reading the design the same day, they ruled that a base place has no worktree and that the runner, not herdr's plugin, settles every waiting place between commands; and asked whether a herdr server's nudge should settle only its own server's places, they ruled that no nudge settles at all, each in the words quoted below. How a caller states its watch and how flume reads it lapsed, what herdr's plugin still does, a pane's row shown as one of flume's clues, the base place's shape, the refusals that keep the depths apart and shipping no kind of that shape are the stage's choices.

Amended in part by record 0056, accepted by the user on 2026-11-07: a base place forecast or found to conflict with its base, or whose submission failed, has the fix its kind names launched on its branch by flume at a settle, and is still submitted only on its caller's retry.

Amended in part by sluice record 0093, accepted by the user on 2026-11-06 ("I approved 93."), once flume pins a sluice that runs its host process: sluice's queues move in that process, which a submit starts and a settle's read of the submission starts again where work waits and none runs, so the sluice profile's step is nothing (flume C.6's step row, boomdrivedev/blueprints main at 88a4cf0; rule core.6). A caller's watch still settles every place waiting on the merge queue at every depth, and its settles read each open submission; where this record says those settles step a queue through sluice, that holds while the pinned sluice, 0.71.0 among them, runs no host process.

Deciders: the user, by their note, their two answers and their ruling on the nudge; the construct stage, on clauses 10, 46, 111, 119, 122, 139 and C.6 and rules core.4, core.5, core.6, core.13 and core.14.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` on main at 51b6316, with weir's 51, 52 and 57 and sluice's 124 beside it, rebased onto 6d322be, whose standing agent touched the vocabulary, 43, 132 and C.7, of which this record amends the vocabulary, 43's wait and C.7's host state, each read whole with both; the research note `research-queues-driven.md` of 2026-11-06 (the research place `research--how-a-host-drives-the-queues-of-the-repositories-it-keeps`); flume's log on deneb, 00:54Z to 02:34Z on 2026-11-06; crew's `plugin/lib/plan.py` at 23b3b3dc (`bolt_give`, `BOLT_CREATE`, `SUBMIT`, `land_delete`) and its rules plan.8, plan.16, plan.18 and plan.19; Boomdrive at 2937b09: `crates/boomdrive/src/sluice.rs` and `carry.rs`, its rules core.1, core.13, core.14, core.15 and core.16, its records 0002 and 0003, and its running spec's passes; `plugins/herdr/bin/watch.sh` and `bin/places.sh`, `crates/flume/src/commands/mod.rs`, `crates/flume/src/profiles/terminal/herdr/mod.rs` (`show`, `setup`), `crates/flume-core/src/settle.rs` (`show`), `line.rs`, `submit.rs` and `terminals.rs`; deneb on 2026-11-06, twelve herdr servers each running `flume status --json` every 30 seconds at 57% kernel time, as the user's ruling reports; the statement's amendment on main at 6dc9399, of 46, 119, 120, 122, 139, 176, the vocabulary, the common profile's list, C.4's row, C.5, section 9, I8, S27, S28, S109 and S118.

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

Their ruling the same day, asked whether a herdr server's nudge should settle only its own server's places: "I thought we agreed that only the runner was doing the settling. I don't know what this nudge by the Herdr server is all about. I don't think we want it." The brief that carried it added deneb's reading of that day: twelve herdr servers each running `flume status --json` every 30 seconds saturated the host, at 57% kernel time, and timed out a crew gate.

Two gaps stood in the way. What moved a submitted place between flume's commands was herdr's plugin: its watch runs `flume settle` naming no place on its bound, which settles every place the store holds, and each settle of a submitted place steps its base. flume's log on deneb shows the runner's swancloud chores each settled 43 times between 00:54Z and 02:34Z on 2026-11-06 with no agent in them. flume read whether it ran attended from whether each terminal kept a watch, so under any other terminal, or with the plugin's lease stale, a submitted place moved only when a caller acted. The rounds those settles ran found no room, since the host's sluice keeps no slot for candidates and a round asks for room only after its checkout: those are sluice's and the host's.

And a crew bolt's branch had no place. crew or the runner made it with `sluice branch create`. ops submitted it with `wt submit main` from a worktree crew made, crew waited on it with a background `sluice wait`, and crew removed it with `sluice branch delete`. The unit places on the bolt were flume's, and landed through flume's settles. One depth was flume's and the other crew's and the runner's.

## Considered options

For who settles every waiting place between commands:

1. **A caller keeps watch over the host.** `flume settle --every <bound>` settles every place and records in the host's state when it began and its bound; a caller keeps watch while that settle began less than twice its bound ago, and flume runs attended while none does. The runner's passes keep it on a host that keeps the runner.
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

For whether a terminal settles beside the watch:

12. **No terminal settles.** The terminal seam offers no operation that settles, keeps watch or nudges, and no plugin of a terminal runs `flume settle`.
13. **A terminal's nudge beside the watch:** herdr's plugin settling every place at each agent's change, keeping no bound and deciding nothing of attended.

For what herdr's plugin does once it settles nothing:

14. **The plugin keeps no process, and a pane's row is one of flume's clues.** The popup reads status when a person opens it and after each key that acts; the startup writes what herdr's configuration lacks; show's clues carry the place's line's short form, which herdr's show reports as the pane's row.
15. **The plugin reads its own server's places' lines** at each change herdr's listing shows in its panes and on a long tick.
16. **The plugin reads every place's status on its tick,** as its watch does for the sidebar and the popup.

## Decision outcome

Options 1, 6, 12 and 14: the statement's 202 to 206 and 208, with the vocabulary, 9, 10, 14, 18, 46, 47, 67, 78, 80, 82, 111, 112, 117, 119, 120, 122, 133, 139, 165, 176, C.1, C.4, C.5, C.6, C.7, section 9, I8, S27, S28, S91 and S109 to S112 and S118 amended or added beside them, weir's 51, 52 and 57, and sluice's 124.

A caller keeps watch over the host by settling every place on a bound it states, `flume settle --every <bound>`. flume keeps, as a host document written by compare-and-set, when the newest such settle began and its bound. A caller keeps watch while that settle began less than twice its bound ago, so a settle that runs as long as its bound keeps it; flume runs attended otherwise, every acting command settling every place first and a wait driving its settles. No terminal decides it. Status gives the watch, its bound and when it last settled, or since when none is kept. On a host that keeps the runner, each of the runner's passes runs `flume settle --every` once and awaits it, the runner's follow-up amending its core.13 and its record 0003 on the user's word. A host with no runner runs attended, a person's plain use, its places moving at each caller's command and at an agent's own quiet hook, and a timer of the host's own may keep the watch as the runner does. No terminal settles: the terminal seam offers no operation that settles, keeps watch or nudges, and herdr's plugin runs no `flume settle`. What no hook reports, a stage of a program with no hook, an agent that crashed, a start held at the folder-trust question and a lost conversation's process ending in its pane, is read at the watch's next pass, or on a host with no runner at a person's next command that acts.

herdr's plugin keeps no process. Its popup reads `flume status --json` once when a person opens it and again after each key that acts, and its keys act through flume's commands as before. Its startup, at herdr's start of a server, writes what herdr's configuration lacks into `setup.json` with the server's socket, which flume's herdr profile gives in status for each server still running, reading no lease, and ends a watch an earlier release left running by the pid and identity its lease names. A pane's sidebar row, the short form of its place's line, becomes one of show's clues: the core computes it from what `flume status --line` reads, shows the clues again when it changes, and herdr's show reports it as the pane's `flume` token beside the title and the base's count. So every clue herdr shows has one owner, flume's settle, and nothing reads flume on a tick.

A base place is a place whose kind lands and has no stages, known by that shape alone. It has no worktree: it is its branch, which flume asks the merge queue's open to make at its base, and the state flume holds of it, named by the location its place profile gives where nothing is made. It runs no readiness, starts no agent and counts no room or points. A launch on a live place's branch is refused where that place is no base place, or where the base place's submission is open or its work is over. A base no place holds is launched on as today. A retry naming a base place's submission is refused while a place on its branch is live, and otherwise submits its branch by name from the repository, with no worktree as its submitter. Its outcome is read from the merge queue's record of its branch, since it has no worktree for an inbox, and delivered to its recipients. Its failure waits on its caller, who lands a fix through a place on its branch. A drop of it is refused while a place on its branch is live. Its tidy removes no worktree and runs no teardown, and asks the merge queue's close for its branch into its base. Status gives the places on its branch, that it has no worktree, and whom it waits on.

So where the merge queue runs has one answer at every depth. sluice's rounds run in the repository, moved by the host's `sluice run` where the host keeps one and otherwise by the settles of the places submitted to it, whether their base is `main` or a base place's branch. Those settles come between flume's commands from the host's watch, which the runner keeps. Neither the runner nor crew drives a queue, and no terminal is relied on. The runner's follow-up keeps the watch at each pass, retires its branch command and launches a bolt as a base place of its caller's kind. crew's retires `BOLT_CREATE`, `crew bolt submit`'s worktree, `wt submit` and background wait, and `crew bolt land`'s branch delete, and declares its own kind with no stages. weir's lets a kind that lands declare `stages = []`. sluice's takes a submission naming a branch it lands on as its head from no worktree, its outcome read from its records, and keeps the room a round counted. A person's own use stays as it was: a place submitted by hand lands as before, and a host whose `sluice run` drives the repository sees the settles' steps do nothing.

Option 13 keeps a second settler beside the runner, each of its settles reading every place from every herdr server, and the user ruled it out. Option 2 rests the guarantee on one terminal's plugin and its lease, which the user ruled out. Option 3 keeps two settings in step on every host, and a runner that stops leaves flume believing its places move; a caller stating its bound at each settle keeps one setting, and its lapse is read from the host's own record. Option 4 makes every `flume needs`, retry and launch on a host of fifty places wait on a settle of all of them, and still moves a submitted place only when someone acts. Option 5 makes a second driver beside flume's settle, which the runner's core.1 and its record 0003 rule out, and keeps one depth outside flume: the asymmetry the user named. Option 7 breaks clause 10: the merge queue levels the host's copy of the branch under the worktree after every landing (sluice 124), git then reads the worktree as holding a change not committed, and its tidy keeps it for ever (65). Option 8 keeps clause 10 but shows a person the base as it stood at the launch, runs readiness and teardown for nothing, and asks the place seam for a worktree on no branch; the user chose no worktree. Option 9 makes a branch sluice lands on behind sluice's back, which its protection refuses (sluice 104 to 106), and a branch kept on a remote must be pushed, which flume never does (core.5). Option 10 defines a field of weir's format in flume (core.4), or starts an agent for nothing. Option 11 puts a kind's name and prefix on the site every host's operator reads (core.35) when nothing here needs one; it stays a package away. Option 15 keeps a process on every herdr server, needs the plugin to find which places its panes hold with no read of status, and copies a clue flume's settle already reads. Option 16 is what saturated deneb.

### Consequences

- Good: one design at every depth. Every branch that lands is submitted by flume through the merge queue, and every queue no driver moves is stepped by flume's settles, nested as deep as a caller nests base places.
- Good: the guarantee that waiting places move between commands is the host's watch, kept by a caller, read by flume from its own record, and lapsing to attended when the caller stops. No terminal and no plugin settles or is relied on.
- Good: herdr's plugin reads flume only at a person's asking, so a host's load no longer grows with its herdr servers, and every clue herdr shows comes from flume's settle.
- Good: the runner and crew stop making, submitting, waiting on and removing branches. Their follow-ups cite flume's clauses and drive nothing.
- Good: flume moves no more branches than before, and a base place has nothing on the host to tidy but its record. Its branch is the merge queue's to make and remove, at flume's asking.
- Bad: a host that keeps the runner runs attended until the runner's follow-up lands, its acting commands settling every place first.
- Bad: a move no hook reports waits for the watch's next pass, or on a host with no runner for a person's next command, where herdr's tick read it in seconds; and where herdr's restart drops a pane's metadata, its title and row return at the next settle that changes one.
- Bad: the build waits on two other follow-ups, weir's `stages = []` for the kinds profile and sluice's submit by name from no worktree for the sluice profile. The core and the conformance suite build over the fakes before either lands.
- Bad: steps do not make room. A host whose sluice keeps no slot for candidates stalls its queues however often flume steps them, until sluice's and the host's follow-ups.
