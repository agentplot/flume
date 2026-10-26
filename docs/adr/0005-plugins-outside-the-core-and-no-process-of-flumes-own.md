---
number: 5
title: Plugins outside the core, and no process of flume's own
status: accepted
date: 2026-10-22
decision-makers:
- Chuck Swanberg
---

# Plugins outside the core, and no process of flume's own

Status: accepted by the user on 2026-10-26, on their card.

Deciders: the user, in flume's statement (what moves stages on; herdr required for unattended use; no daemon).
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 46, 60, 61, 119 to 122, I8, C.4, C.5, section 9; sluice's ADR 0023 (plugins kept outside the core).

## Context and problem statement

Something must notice that an agent went quiet, that a submission was decided, that a pane was killed by hand, and move the place on, while flume is a command-line tool with no process of its own. And flume has three surfaces, a command line, a Claude Code plugin and a herdr plugin, which must never disagree about a place.

## Considered options

1. A flume daemon per host that watches every place.
2. No process of flume's own: a place settles inside a command of flume's, started by its agent going quiet (the Claude Code plugin's Stop hook runs `flume settle`), by the merge queue's verdict (the submitting command waits on it) and by the terminal's watch (the herdr plugin, on herdr's server, runs `flume settle` on a bound it states and at each agent state change). Every surface answers from one core, each kept outside the core as sluice keeps its plugins, and no surface computes a stage's end, a status, a submission or a tidy of its own.
3. The callers poll `flume status` and drive every settle themselves.

## Decision outcome

Option 2. Under a terminal that keeps no watch, flume runs attended: every command that acts settles every place on its host first, and an agent that died with no word is read only then. Unattended use needs herdr with flume's plugin. Every answer has one form a program reads, the same through every surface, which only ever gains fields.

Option 1 is a second process to keep alive, deploy and doctor, which the family's storage and fleet rules already avoid for sluice. Option 3 makes the callers' polling the clock, and crew is the only caller that has one.

### Consequences

- `flume settle` is the one entry point every mover runs; a settle is safe to repeat (74).
- The Claude Code plugin holds the asking mark, the stop check, the guard and the on-quiet settle; the herdr plugin holds the watch and the sidebar, and curates nothing (C.4, C.5).
- A printed-command terminal profile has no watch, so flume runs attended under it and every tell is refused (C.5).
