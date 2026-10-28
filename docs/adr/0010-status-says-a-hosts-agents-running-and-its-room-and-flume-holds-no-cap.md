---
number: 10
title: Status says a host's agents running and its room against the bound the host states, and flume holds no cap
status: proposed
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# Status says a host's agents running and its room against the bound the host states, and flume holds no cap

Status: proposed to the user on 2026-10-28, as `briefs.15` has it; accepted only on their yes.

Deciders: the user, accepting crew record 0045 on 2026-10-28 (`afterthought/crew:docs/adr/0045-a-team-is-bound-to-one-bolt-and-launches-work-while-its-host-has-room.md`): a fix, a unit and a chore are three Flume kinds, not slots; a team is bound to one bolt and to nothing smaller; room is the host's, counted in agents running, a waiting piece holding none; and, from 2026-10-27, "a fix, a unit and a chore are all just different Flume kinds, and once we integrate with Flume we need to decide whether a team is bound to one unit of work or can work on multiple" (relayed by madswan-planner, in `_open.md`); the shape is madswan-design's, on swancloud-design's asking of 2026-10-28, and the user's yes is asked on the record's card.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 54, 56, 58, 70 and section 9's list of what is not flume's, the C.8 crew row and S7; C.7, the host's state directory and the family's storage rule; records 0004, 0006; rules core.14, core.16; `afterthought/crew:docs/adr/0045`; sluice's host configuration (`~/.config/sluice/config.toml`, `[runs] max`), the precedent for a bound the host states.

## Context and problem statement

flume holds no count or cap of its own on places or agents, plans nothing across places and orders none (70, 58); how many places a caller runs at once is the caller's (54). crew, Flume's first caller, has counted in slots: a team's four or six, each holding a unit, a fix needing one lent. The user ended that: crew record 0045 binds a team to one bolt and to nothing smaller, makes room the host's, counted in agents running, with a waiting piece holding none, and has crew start a piece whenever Flume says the host has room. The statement's open item on a team's bound waited on this integration. What Flume lacked was the fact crew now wants from it: how many agents run on this host, and whether there is room for one more, in the terms a caller launches with.

The seat's limit and the CPU's are the host's facts, known to whoever administers the host and to nobody inside flume or crew; sluice's run bound is sluice's own and counts runs, not agents.

## Considered options

1. crew keeps its slots and counts for itself, as today.
2. flume enforces a cap: a launch past the host's bound is refused.
3. flume reports: status says, for the host, how many agents of flume's places are running, every caller's counted together, and, where the host's own configuration states how many agents it runs at once, how many more it has room for; a place waiting with no agent counts nothing; flume refuses no launch for want of room, and a caller that launches past the bound is told so once.
4. flume reads sluice's run bound as the agent bound.

## Decision outcome

Option 3.

**flume reports the host's fact and decides nothing with it.** A caller launches on a host, and the one thing it needs to know before it does is whether the host can take another agent. flume knows every agent in every place on the host (56), across every caller, so the count is flume's to give, and the bound is the host's to state, in the host's own configuration under the family's storage rule (C.7), as sluice's run bound is: the seat's concurrency and the machine's size are facts of the host, set by whoever administers it, counted by nobody. Status gives both, and the room between them. A place whose stage waits on its caller's word has no agent and holds nothing (0045). flume holds no cap (70): a launch past the room runs, and its caller is told once, so a caller that knows better, or an operator by hand, is not refused by a number.

**Why not the others.** Option 1 is what the user ended: slots count what is not running and lend what is not theirs. Option 2 puts a cap into flume, which 70 and 58 refuse, and would make a host's misconfigured number stop every caller. Option 4 counts the wrong thing: a run of sluice's gate and an agent in a place take different shares of the host.

### Consequences

- Clause 70 of the statement says status reports the host's agents running and its room against the bound the host's configuration states, that a waiting place counts nothing, and that flume refuses no launch for want of room. The C.8 crew row and S7 no longer speak of slots and a cap: crew launches a piece whenever flume says its host has room. Rule core.26 is drawn from them.
- The host's configuration gains one word for how many agents the host runs at once; a host that states none has agents running reported and no room judged.
- `_open.md`'s item on a team's bound is closed: crew record 0045 decides it from crew's side, this record from flume's.
- crew's slot count, and the lending of an idle slot to a fix, end when crew launches through flume (crew record 0045); nothing of flume's is built for slots.
