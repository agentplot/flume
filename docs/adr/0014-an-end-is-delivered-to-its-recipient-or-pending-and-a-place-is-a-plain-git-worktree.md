---
number: 14
title: An end is delivered to its recipient or pending, collected by flume inbox, and a place is a plain git worktree
status: accepted
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# An end is delivered to its recipient or pending, collected by flume inbox, and a place is a plain git worktree

Status: accepted by the user on 2026-10-28, in their design session ("words: table as is"; on the worktrunk place profile, "drop").

Deciders: the user, choosing the words of a table put to them in the design review of crew, flume and sluice of 2026-10-28, and dropping the worktrunk place profile on the same review's question; the shape is the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` section 3, clauses 41 to 43, 56, 111, 124, 130 to 133, C.1, C.2, C.4, C.6, S5, S34, S35, S38, as amended at 9ae2125; records 0008, 0009, 0012, 0013; `agentplot/sluice` records 0013 (a place is a plain git worktree) and 0070 (the same words for sluice); rules core.3, core.24, core.25, core.27, core.28.

## Context and problem statement

Records 0008, 0009 and 0013 named how an end reaches whoever waits on a place with the words told, untold, waiter and left, and a stop-short as one that asks or finds. Sluice and crew used the same words. The user asked for cleaner verbs before any of it is built under those names.

The statement also kept two place profiles, worktrunk and plain `git worktree`. Worktrunk's create hooks run the place's code with the host user's full permissions outside any executor, the reason sluice's record 0013 gave for dropping worktrunk from its own places, and record 0012 has flume run no `wt submit`. Nothing of a place still needs worktrunk.

## Considered options

1. Keep the words, and both place profiles.
2. Delivered and pending for an end's state, recipient for whoever it goes to, collect by `flume inbox` for a caller's taking of what is pending, notice for what a delivery carries into a pane, stalled for a place waiting on its caller under no hold's words; a stop-short needs an answer or found work; one place profile, plain `git worktree`, readied as sluice readies its own places.

## Decision outcome

Option 2. The mechanisms do not change: an end is delivered once to each recipient, recorded with whom and when, pending until then; a caller that starts collects what is pending for it; a stalled place is said once; `flume needs` and `flume needs --found` stay the commands. The B.6 operation `untold` is `inbox`, run under sluice as `sluice inbox --json`. A repository that binds `place = "worktrunk"` is refused at the launch, naming the one profile.

### Consequences

- The statement says it, at 9ae2125; rules core.24, core.25, core.27 and core.28 are rewritten in these words, their sense unchanged, and core.3 rules out a place profile but plain `git worktree`.
- Records 0008, 0009, 0012 and 0013 carry an amendment line naming this record.
- The core's code, landed from `bolt/flume-the-core`, and its specs take the words where they name these things, in the OpenSpec changes that build records 0007 to 0013.
