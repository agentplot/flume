---
number: 3
title: A stage is one fresh agent, and ends on its deliverable or its agent's stop-short
status: proposed
date: 2026-10-22
decision-makers:
- Chuck Swanberg
---

# A stage is one fresh agent, and ends on its deliverable or its agent's stop-short

Status: proposed to the user on 2026-10-22, as `briefs.15` has it; accepted only on their yes.

Deciders: the user, in flume's statement, taking over crew's `teams.2` to `teams.4`, `teams.11` and `teams.15`.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 24, 25, 30 to 36, 39 to 45, 109, 110, 118, I1, I2; crew's `teams.2`, `teams.3`, `teams.4`, `teams.11`, `teams.15`.

## Context and problem statement

An agent's terminal goes quiet for many reasons: a turn ended, own work runs, a question waits on a person, the agent crashed. crew learned to read none of them as a stage's end. What ends a stage, what starts the next, and how many agents does a place hold?

## Considered options

1. A stage ends when its agent's terminal is quiet past a limit, as a watchdog reads it.
2. A stage ends only when its deliverable exists, read where it lives at one head of the place's branch, or when its agent stops short by telling flume so with the start's own secret; a conversation ends on its caller's finish. Each stage is exactly one fresh agent, a place holds at most one agent, and no agent that is not quiet is ended unless forced.
3. A stage ends when its agent says it is done, through the agent program's own exit.

## Decision outcome

Option 2. The deliverable is one of six forms and counts only when made since the stage started; a stop-short is taken only from the current stage's agent, or a process it started, carrying the secret flume gave that start alone. A quiet agent with its deliverable missing is sent back once with words naming the deliverable, and otherwise reported stuck past the stage's limit, never ended by flume. Each agent carries an identity naming its repository, place, stage and start, which no other live agent on the host carries, so a terminal never runs two under one name.

Option 1 ends stages that are waiting on own work or on a person. Option 3 trusts the agent's word for a fact flume can read.

### Consequences

- flume reads every fact of one deliverable at one head and never pairs facts read at two heads (25).
- `flume needs "<words>"` is the one way an agent changes what flume holds; it is refused without the agent's identity and its start's secret (41, C.4).
- The stop check and the stuck limit are the Claude Code plugin's Stop hook and flume's reading of the agent's state; neither ends an agent (110, 44, C.4).
