---
number: 10
title: Flume keeps a host's queue of launched work: a stage starts only while the host has room, by its callers' priorities, and status says the agents running, the room and the queue
status: proposed
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# Flume keeps a host's queue of launched work: a stage starts only while the host has room, by its callers' priorities, and status says the agents running, the room and the queue

Status: proposed to the user on 2026-10-28, as `briefs.15` has it, and revised the same day on their note; accepted only on their yes.

Deciders: the user, on this record's first card (2026-10-28): "This might be right but I want to make sure it's consistent with the recent design work I was doing, in which we remove teams from crew. Instead crew starts to manage bolts using flume so we no longer get bottlenecked by teams being attached to a single bolt at a time. In that world if there's work to be done in a bolt, it would appear to flume as a queue of work that needs to be processed concurrently. I think flume would be managing that queue. There would be some sort of priority system so the operator has control over making sure that as priorities change, the work that needs to get scheduled gets scheduled", which settles that flume keeps the queue of a host's launched work and starts it by a priority the operator controls; the user, accepting crew record 0045 on 2026-10-28 (`afterthought/crew:docs/adr/0045-a-team-is-bound-to-one-bolt-and-launches-work-while-its-host-has-room.md`): a fix, a unit and a chore are three Flume kinds, not slots; a team is bound to one bolt and to nothing smaller; room is the host's, counted in agents running, a waiting piece holding none; and, from 2026-10-27, "a fix, a unit and a chore are all just different Flume kinds, and once we integrate with Flume we need to decide whether a team is bound to one unit of work or can work on multiple" (relayed by madswan-planner, in `_open.md`); the shape is madswan-design's, on swancloud-design's asking of 2026-10-28, and the user's yes is asked on the record's card.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 54, 56, 58, 70 and section 9's list of what is not flume's, the C.8 crew row and S7; C.7, the host's state directory and the family's storage rule; records 0004, 0006; rules core.14, core.16; `afterthought/crew:docs/adr/0045`; sluice's host configuration (`~/.config/sluice/config.toml`, `[runs] max`), the precedent for a bound the host states.

## Context and problem statement

flume holds no count or cap of its own on places or agents, plans nothing across places and orders none (70, 58); how many places a caller runs at once is the caller's (54). crew, Flume's first caller, has counted in slots: a team's four or six, each holding a unit, a fix needing one lent. The user ended that: crew record 0045 binds a team to one bolt and to nothing smaller, makes room the host's, counted in agents running, with a waiting piece holding none, and has crew start a piece whenever Flume says the host has room. The statement's open item on a team's bound waited on this integration. What Flume lacked was the fact crew now wants from it: how many agents run on this host, and whether there is room for one more, in the terms a caller launches with.

The seat's limit and the CPU's are the host's facts, known to whoever administers the host and to nobody inside flume or crew; sluice's run bound is sluice's own and counts runs, not agents.

The first draft of this record had flume report the host's room and crew launch a piece whenever there was some. The user read it against the design they were making the same day: teams leave crew (afterthought/crew record 0047; flume record 0011), crew manages bolts through flume so no team is tied to one bolt, and a bolt's work then appears to flume as a queue to be worked concurrently, which flume manages under a priority system the operator controls. A caller that launches only when there is room is a scheduler outside flume, one per caller, and the user asked that there be one, inside.

## Considered options

1. crew keeps its slots and counts for itself, as today.
2. flume enforces a cap: a launch past the host's bound is refused.
3. flume reports: status says, for the host, how many agents of flume's places are running, every caller's counted together, and, where the host's own configuration states how many agents it runs at once, how many more it has room for; a place waiting with no agent counts nothing; flume refuses no launch for want of room, and a caller that launches past the bound is told so once.
4. flume reads sluice's run bound as the agent bound.
5. flume keeps the host's queue: the bound is the host's fact, stated in its configuration; every launch is taken; a stage that may start starts only while the agents running are under the bound and otherwise waits for room; a settle starts the waiting stages in priority order, as many as there is room for; a place's priority is its caller's, given at the launch and changed after; status says the agents running, the bound, the room and the queue; a host that states no bound has no queue (chosen).

## Decision outcome

Option 5.

**The bound is the host's fact, and flume keeps to it.** How many agents a host runs at once is stated by whoever administers the host, in the host's own configuration under the family's storage rule (C.7, `[host] agents`), as sluice's run bound is: the seat's concurrency and the machine's size are facts of the host, counted by nobody inside flume or a caller. flume knows every agent in every place on the host, across every caller (56), so it is the one that can keep to the bound: a stage that may start, a launch's first or a running place's next, starts only while the agents running are under it, and otherwise waits for room (70). A place whose stage waits on its caller's word has no agent and holds nothing (0045).

**Every launch is taken, and the host's launched work is one queue.** A caller launches what it wants worked, a bolt's units whole among them, and flume refuses none for want of room: the places wait their turn. At each settle, which runs whenever an agent ends, a submission decides or the terminal's watch ticks (46, 120), flume starts the stages waiting for room in priority order, the highest first and equals in the order they came to wait, as many as there is room for, so a host runs at its bound and no more, with no process of flume's and no scheduler in any caller. An agent running is never ended for a higher priority (31): priority orders starts, never ends.

**Priority is the caller's word, and the operator's control.** A launch may give its place a priority, equal for all where none is given, and a caller may change a place's priority after the launch, which reorders only what has not started (7, 70). crew carries the user's priorities from the rail to flume this way, so as priorities change the work that must run next runs next, and nothing else of planning enters flume: what to launch, what each place holds, what matters most and what comes after what stay the caller's (I6).

**Status shows the queue.** For the host, status gives the agents running, the bound, the room, and the places waiting for room in the order they would start, each with its priority and since when (56, 70). A stage waiting for room waits on the host and not on its caller, so it is never left (130). A host whose configuration states no bound has no queue: every stage that may start starts, and status gives the agents running and judges no room.

**Why not the others.** Option 1 is what the user ended: slots count what is not running and lend what is not theirs. Option 2 refuses launches by a number, which leaves a caller to retry and know when; a queue takes them and starts them in turn. Option 3, this record's first draft, puts the scheduling in each caller, one per caller and none across them, which is the bottleneck the user named. Option 4 counts the wrong thing: a run of sluice's gate and an agent in a place take different shares of the host.

### Consequences

- Clause 70 of the statement is rewritten: the bound is the host's fact, a stage starts only while the agents running are under it and otherwise waits for room, a settle starts the waiting stages by priority, a place's priority is its caller's at the launch and after, every launch is taken, status shows the queue, and a host with no bound has no queue. The purpose, the actors, the vocabulary (room, priority, waiting for room), clauses 7, 40, 46, 56, 130 and 131, invariant I6, the non-goals, C.7, the callers table's crew row, S7 and new scenario S37 follow it. Rule core.26 is drawn from them.
- The host's configuration gains one word, `[host] agents`, for how many agents the host runs at once; a host that states none has agents running reported, no room judged and no queue.
- Record 0011's kind that runs through starts each next stage as the host's room allows (131).
- afterthought/crew records 0045 and 0047 are read under this: crew launches a bolt's work whole and keeps no slots and no count; the room and the queue are flume's; the user's priorities on the rail reach flume as each place's priority.
- Units on the yes, with record 0011's: the host's bound read from its configuration and kept to at each settle, with the queue by priority and status showing it; a place's priority at the launch and changed after.
- `_open.md`'s item on a team's bound is closed: crew record 0045 decides it from crew's side, this record from flume's.
- crew's slot count, and the lending of an idle slot to a fix, end when crew launches through flume (crew record 0045); nothing of flume's is built for slots.
