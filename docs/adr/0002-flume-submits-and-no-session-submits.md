---
number: 2
title: flume submits a landing kind's place, and no session submits
status: proposed
date: 2026-10-22
decision-makers:
- Chuck Swanberg
---

# flume submits a landing kind's place, and no session submits

Status: proposed to the user on 2026-10-22, as `briefs.15` has it; accepted only on their yes.

Deciders: the user, in the capture design's ruling 12 (`design/capture/plan.md`, D2) and in flume's statement.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 14, 15, 63, 71, 108, 111 to 113, 124, I3, I7, C.6; `agentplot/blueprints:design/sluice/requirements.md` clauses 1, 2, 4; crew's `teams.5`.

## Context and problem statement

A place's work must reach its base. Under crew today the stage's agent runs the merge queue's submit itself, and an agent that submits can also merge, push or land, and can misread the outcome from its own terminal. Who submits a place, when, and who reads whether it landed?

## Considered options

1. The stage's agent submits as part of its last stage, as crew's merge stage does today.
2. flume submits the place's branch after its last stage delivers and its agent has ended, waits on the merge queue's verdict, and sends the place back to a stage the kind names when the submission fails; an agent never submits, merges or pushes, and flume refuses each such act where the agent program lets it.
3. The caller submits, reading flume's status to learn when the last stage delivered.

## Decision outcome

Option 2. The merge queue is a seam of flume's (B.6), reached only from the settle that follows the last stage, and its outcome is read from the merge queue's own record, never from an agent's words or terminal. A failed submission is a return to the stage the kind names, with the merge queue's words as that stage's words, bounded by the kind's limit on going back. A kind that does not land never reaches the merge queue.

Option 1 leaves the one act that moves shared history to the agent, which is what a merge queue exists to take from it. Option 3 makes every caller re-implement the wait and the return.

### Consequences

- flume never commits, merges, rebases or lands; it moves no branch but a place's own at its launch and its tidy (14, 67).
- Under sluice, submit is `wt submit <base>` or `sluice submit <base>` run by flume in the place, waited on; `--no-wait`, `--check` and `--no-remove` are never passed (C.6).
- The guard in flume's Claude Code plugin refuses `sluice submit`, `wt submit`, `wt merge` and `git push` from an agent and leaves the mark status reports (108, C.4).
