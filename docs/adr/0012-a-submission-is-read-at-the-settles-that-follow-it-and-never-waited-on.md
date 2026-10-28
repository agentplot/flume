---
number: 12
title: A submission is read at the settles that follow it and never waited on, and the merge queue removes no place
status: accepted
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# A submission is read at the settles that follow it and never waited on, and the merge queue removes no place

Status: accepted by the user on 2026-10-28, in their design session ("i agree with these changes"), on the recommendations drawn from reading crew records 0045 to 0048, sluice records 0056, 0065 and 0068, and flume records 0002 to 0011 together.

Deciders: the user, accepting recommendations R2 and R4 of the design review of 2026-10-28; the shape is the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 43, 46, 66, 111, 113, 120, 124, 125, 133, I8, C.6 and S34, as amended with this record; records 0002, 0005, 0009; `agentplot/sluice:docs/adr/0065` (the driver's wait removes a landed worktree) and `0068` (an outcome pushed to its submitter and kept until told; an entry submitted outside a multiplexer stands untold until asked); sluice clauses 51 and 135; `afterthought/crew:docs/adr/0048`, pattern 5; rules core.6, core.13, core.18, core.25.

## Context and problem statement

Records 0002 and 0005 had the settle that submits a place wait on the merge queue's verdict, `wt submit` or `sluice submit` with `--no-wait` refused, so the submitting command was the one that settled the place again. Record 0009 then said the outcome "reaches flume's settle as sluice record 0068 pushes it". The two cannot both hold. Sluice 0068 pushes an outcome into the submitter's session pane, and a settle is not a session: it runs in the agent's Stop hook or in the herdr plugin's watch. A verdict takes from a minute to most of an hour, and no record said which process holds that wait, or whether a watch tick blocked on it holds up every other place's settle.

A second question rode on the first. Under worktrunk the waiting `wt submit` removes the place's worktree on landing (sluice 0065), while flume's tidy removes its own places through the repository's teardown (clause 66). Two removers of one worktree means the tidy must accept one already gone, and the records did not say whose hooks run.

## Considered options

1. Keep the waiting submit, and say in 0009 that flume learns the outcome from its own wait, marking it told as a reporting wait does (sluice 51).
2. Submit with no wait, recorded as the place's, and have every settle that follows ask the merge queue for the place's untold outcomes by the one command sluice gives a submitter for them; drive the queue one round where nothing else does; flume's tidy removes every landed place.
3. Have sluice push the outcome to flume by a carrier of its own.

## Decision outcome

Option 2.

**No wait, and nothing lost.** flume submits through `sluice submit <base> --no-wait --json`, with no multiplexer's session in its environment, so sluice records the place's worktree as the submitter and no session, and pushes nothing into any pane. Every settle that follows, whatever started it, asks sluice for the outcomes untold to the place (B.6 `untold`, clause 133), which tells each decided outcome once and marks it told to the place (sluice 51). An outcome decided while flume's host was down stands untold in sluice's records until the first settle asks, so a restart loses nothing, the case record 0009 asked for, reached the same way a restarted session reaches its own. Where no `sluice run` is live on the host, a settle drives the base's queue one round (`sluice step`), so the queue moves with nothing standing (sluice 135).

**One remover.** flume never runs `wt submit`, whose waiting half removes the place, and sluice removes nothing on landing. flume's tidy removes every landed place through the place profile's teardown (C.1), so under worktrunk the repository's pre-remove and post-remove hooks run as for `wt merge`, and its branch as 67 says.

**What it costs.** A verdict reaches the place at the next settle rather than the moment sluice decides it: within the watch's bound under herdr, at the next command under the printed command. A unit's course is minutes to hours, and nothing waits on a place but its caller, who is told inside that settle (43).

**Why not the others.** Option 1 keeps an hour-long wait inside a hook or a watch tick, and a second remover. Option 3 has sluice name flume, which sluice's core.1 forbids, and adds a carrier where a read already answers.

### Consequences

- Clauses 43, 46, 66, 111, 113, 120, 124, I8, C.6 and S34 of the statement are amended, B.6's `submit` holds no wait, `wait` becomes `step`, and `untold` (133) is added; rules core.6, core.13 and core.18 are rewritten from them.
- Records 0002, 0005 and 0009 carry an amendment line naming this record.
- The merge-queue profile's units: submit with `--no-wait --json` and a cleaned environment; `untold` at every settle for each place with an open submission; `step` where no `sluice run` is live; the tidy for every landed place. They wait on sluice's untold command (sluice record 0068's units), which flume calls by the name sluice gives it.
- Rule core.25's details drop "crew's session-start hook": a caller's start asks, crew's by a role run's start (record 0013).
