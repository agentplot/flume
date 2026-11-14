# A stage delivers what an earlier start committed

## Why

A deliverable counts only when made since its stage started (25), and
flume reads "started" as the stage's current start: each start keeps
the head, the time and the length of `made.jsonl` it began at, and its
deliverable is read from those alone. An agent that commits its stage's
work and then stops short to ask leaves the work on the branch. The
caller's answer is a retry (49), whose fresh agent finds the work done
and has nothing to commit, so the stage can never deliver. On
2026-11-11 boomdrive's `worktrunk-comes-with-the-flake` asked again at
each retry of its code stage, and on 2026-11-12 sluice's
`fix--main--a-passed-candidate-lands-within-the-hold` was dropped and
launched again for the same reason.

Clause 25, on blueprints main since 41bd0a2, now says when a stage
started for this: at the first of its starts since it last delivered,
and a repetition at the first start given its group. A fix round, and a
retry of a stage that has delivered, start it afresh. Record 0065
decides it and S131 walks it.

## What Changes

- **A stage counts from the first of its starts since it last delivered
  (25).** flume keeps, beside each stage, the start its deliverable
  counts from: the head that start began at, when it began, and how
  many lines `made.jsonl` held then. Each start of a stage that has not
  delivered since that start carries it, a retry's, a spent account's
  next start's and a lost conversation's retry's among them (49, 146).
  The stage's delivery lets it go.
- **A fix round and a retry of a delivered stage start afresh (25, 112,
  131).** A send-back, by a failed submission or by a finding, lets go
  of the count of the stage it starts, and a stage that has delivered
  has none, so each counts only what is made after it (S52).
- **A repetition counts from the first start given its group (25,
  140).** A start given the group its stage's last start was given,
  with no delivery of the stage between, carries that start's count. A
  start with no group delivers as its stage does, from the stage's
  count.
- **Every fact of the deliverable is read from the count.** Whether the
  head holds a commit since, the newest commit touching the change, a
  committed report, a report's write time and the lines of what the
  stage made are each read from where the count began, at one head
  (24, 25, 129). A stuck, lost or never-begun end's commits, and every
  timing of a start, stay the start's own (43, 44, 118).
- **A going-back past a stage leaves its count.** A stage reset by a
  fix round before it or a retry of a stage before it keeps the count
  of the first of its starts since it last delivered, as clause 25
  reads.
- **A document written before counts as it did.** A stage whose held
  state carries no count starts its next count at that start.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `stage-runs`: a stage's deliverable counts from the first of its
  starts since it last delivered; a fix round and a retry of a
  delivered stage start afresh.
- `repeating-stages`: a repetition counts from the first start given
  its group, and a start with no group from its stage's count.
- `made-things`: a declared thing's line counts from the stage's count,
  and the stage's end carries every line written since it.

## Impact

- `crates/flume-seams`: `Counted`, the start a stage's deliverable
  counts from; `StageHeld::counts` and `StartHeld::counts`, each read
  as none in a document written before.
- `crates/flume-core`: a start's claim carrying or setting the count;
  `fix_round` letting go of the fix round's; a delivered or finished
  end letting go of the stage's; `delivered`, `made_since` and `unmade`
  reading from the count; the stage's end carrying its lines from the
  count, a document written before's `made_from` read as before.
- `crates/flume-conformance`: the stage-runs, repeating-stages and
  made-things scenarios of the specs below, S131 among them.
- `docs/`: rule core.9's details, citing 25, S131 and record 0065,
  which is accepted; the site's guide to kinds says when a stage
  started for its deliverable.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool. No seam's interface changes, and no field of the
  JSON form goes (core.14). The statement already says this: 25 and
  S131 on blueprints main 41bd0a2.
