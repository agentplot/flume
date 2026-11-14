---
number: 65
title: A stage counts what any of its starts made since it last delivered
status: accepted
date: 2026-11-12
decision-makers:
- agentplot-design
---

# A stage counts what any of its starts made since it last delivered

Status: accepted, agentplot-design's own choice, made on the rules written and cheap to change.

Deciders: agentplot-design, on its challenges of 2026-11-11 and 2026-11-12 to clause 25 from signals `2026-11-11-agentplot-planner-3ce5d093/01` and `2026-11-12-agentplot-ops-af264c2f/01`.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` 24, 25, 49, 112, 131, 140, S52 and S131, on boomdrivedev/blueprints' main since `41bd0a2`; boomdrive's `worktrunk-comes-with-the-flake`, whose retried code stage could not deliver on 2026-11-11; sluice's `fix--main--a-passed-candidate-lands-within-the-hold` (head 69b144b3), dropped and launched again on 2026-11-12 because its answered code stage could not deliver.

## Context and problem statement

A deliverable counts only when made since its stage started (25), and flume read "started" as the stage's current start. An agent that commits its stage's work and then stops short to ask leaves the work on the branch. The caller's answer is a retry, and its fresh agent finds the work done (49). It has nothing to commit, so the stage can never deliver. The place asks again at each retry until someone has an agent make an empty commit, or drops it and launches it again. That happened twice in two days.

## Considered options

1. **From the first start since the stage last delivered (chosen).** A retry or a carried start of a stage that has not delivered goes on from that stage's first start, so what an earlier start made counts. A repetition counts from the first start given its group. A fix round, and a retry of a stage that has delivered, start afresh.
2. From the stage's first start ever. A fix round's code stage would deliver with no fix, since the commits before the failed submission count. The place would be submitted again unchanged, the fix round would record no fix (S52), and a failing head would go round until the kind's bound.
3. Unchanged. Every retry after a stop-short needs an empty commit made by hand, or a drop and a new launch.

## Decision outcome

Option 1, written as clause 25 and scenario S131. An agent that finds its stage's work already committed ends its turn, and the stage delivers. A fix round still needs its own commit, since it starts afresh. Where a submission failed for a reason that isn't the branch's, the caller retries the submission (49), and flume never submits an unchanged head by itself.

### Consequences

- Good: an answered stop-short ends delivered with no empty commit and no new launch.
- Good: a fix round still records its fix, so a failing head is never sent back unchanged.
- Bad: an agent that wrongly finds its work done ends the stage delivered. The later stages, the verify stage among them, and the merge queue still check that work.
