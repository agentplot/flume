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

The construct stage of `a-stage-delivers-what-an-earlier-start-committed` read it into the code's terms on 2026-11-14, below.

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

### The construct's reading, 2026-11-14

Status: decided on 2026-11-14 by the construct stage of flume's change
`a-stage-delivers-what-an-earlier-start-committed`. Each choice is cheap
to change and none is the user's alone: clause 25 and the rules decide
it or leave it open, and the change's design gives each in full.

Deciders: the construct stage of `a-stage-delivers-what-an-earlier-start-committed`.
Sources: clause 25 and S131 above; on the bolt
`bolt/flumes-places-need-no-hand`, `crates/flume-core/src/reading.rs`
`delivered` and `made_since`, `crates/flume-core/src/start.rs` `claim`,
`crates/flume-core/src/end.rs` `ended`, `crates/flume-core/src/submit.rs`
`fix_round` and `crates/flume-seams/src/held.rs` `StageHeld` and
`StartHeld`, read 2026-11-14; rules core.9, core.16 and core.19.

- **The count is kept in held state.** A stage holds only its last
  start, so the head, the time and the length of `made.jsonl` its count
  began at are kept beside the stage and copied onto each start that
  carries them. Not taken: keeping every start of a stage, a start's
  whole record at each retry to keep four fields of one.
- **A going-back past a stage leaves its count.** A fix round or a
  retry of a stage before it resets a later stage to run again, and the
  later stage keeps the count of the first of its starts since it last
  delivered, as 25 says: only its own delivery and a fix round of the
  stage itself let it go. Not taken: starting every stage so reset
  afresh, which reads more into 25 than it says and would be a change to
  the statement first (core.19).
- **A document written before counts from its next start.** A stage
  whose held state holds no count begins one at its next start, as flume
  counted it. Not taken: deriving a count from the stage's last start
  where its state shows no delivery, which guesses at starts the
  document no longer holds.
- **The suite cites S131 in words alone.** Its clause list reads the
  statement at 31f9f1c, before S131, so its scenarios cite 25, 49, 112
  and S52 and name S131 in their words. Not taken: moving the list to
  main's revision here, which brings clauses 211 to 224 and S121 to S131
  with it and is its own change.
