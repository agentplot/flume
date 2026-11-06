---
number: 50
title: A status reads each repository's merge queue once and its places side by side, keeping nothing between asks
status: proposed
date: 2026-11-05
decision-makers:
- Chuck Swanberg
---

# A status reads each repository's merge queue once and its places side by side, keeping nothing between asks

Status: proposed on 2026-11-05 by the code stage of the fix `a-hosts-full-status-read-takes-seconds`. The direction is the statement's: C.6 reads read-submission "once for each repository at each ask", and 58 reads places side by side and keeps no fact between asks. The seam's shape, the bound of eight reads at once and reading each agent once are the stage's choices.

Deciders: the code stage, on clauses 58, 99, 139 and C.6 and rule core.15; the user, at the fix's review.
Sources: `agentplot/blueprints:design/flume/requirements.md` on main at c188bf3, clauses 56, 58, 99, 139, 200 and C.6's two sluice tables; `design/sluice/requirements.md` on main, clause 129, and sluice's rules core.25 and core.41 at 0a77608b; sluice 0.67.0's `status` and `overview`; the fix's brief, from deneb on 2026-11-05; rules core.14 and core.15; `crates/flume-core/src/status.rs`, `clues.rs` and `line.rs`, and `crates/flume/src/profiles/merge_queue/sluice/mod.rs`.

## Context and problem statement

On deneb, with 58 places, `flume status --json` took 4 min 50 s on 2026-11-05 while five of herdr's watches read status too. A board that reads every host's places within 30 s never showed deneb's.

A status asked the merge queue once for each place. The sluice profile's read-submission ran `sluice overview --json`, then `sluice status <base> --all --json` for each base the overview named. Each run read the repository's live records and index again (sluice core.41). On a fixture of 58 places in four repositories, each with two bases, one status ran sluice 174 times and git 522 times. It took 14.6 to 14.9 s alone and 17 to 21 s beside five watches. The places were read one after another. Each live agent was read twice, once for its place and once for its base's count: two herdr calls for its state each time, and its transcript for its own work.

C.6 already says how read-submission reads sluice: "read once for each repository at each ask". The code read it once for each place.

## Considered options

1. **The merge-queue seam answers read-submission for several branches of one repository from one read, and a status asks each repository once for all its places; the places, the repositories and the agents a status did not read for a place are read side by side, at most eight at once; each live agent is read once for its place and its base's count.**
2. **The sluice profile keeps what it last read for each repository, and answers later reads from it.**
3. **flume follows the document sluice's own reader writes for its watchers (sluice 129).**

## Decision outcome

Option 1. The seam's `read_submissions(repository, branches)` gives each branch's newest submission in order, from one `sluice overview` and one `sluice status <base> --all` for each base. read-submission of one branch, as a settle, a launch or a tidy asks it, is that read for the one branch (99). A status groups its places by repository and merge-queue profile, and asks each group once (C.6). Each fact is still read where it lives when asked, and nothing is kept between asks (58; core.15). The places are read on up to eight threads, as 58 says status reads them side by side, and `status --line` reads its lines the same way. The base counts take the state status read for each place's agent, and read the agents of the places it was not asked of side by side (139). The answer is the same document: on the fixture, the before and after binaries gave equal JSON.

On the fixture, one status now runs sluice 12 times and git 198 times. It takes 0.63 to 0.70 s alone, 2.1 s for six at once, and 1.2 to 2.1 s beside five watches each looping on a full settle and a full status.

Option 2 answers a later ask from an earlier read, which 58 and core.15 forbid. Within one command it would also give a settle that submits a read from before its submit. Option 3 needs a live `sluice overview --watch` for each repository, which no command of flume's may keep (core.13). Its document is the overview's, which gives no branch's whole history, and read-submission needs the newest entry of a branch whoever made it, decided or not (99). sluice's own one-reader design is what serves its watchers. flume's one-off reads go through sluice's status and overview, which read the live records and the index alone (sluice core.41).

Eight is a bound of the stage's choosing, cheap to change. It keeps one status's processes few beside the host's other readers, and it reads 58 places in about eight rounds.

### Consequences

- Good: a full status takes seconds on a host of 58 places with five others reading, and a board that reads it within 30 s shows the host's places.
- Good: one place's status reads its own agent once, and the other agents of the host's bases side by side.
- Good: the merge-queue seam's one read for several branches is what C.6 already says of sluice.
- Bad: a merge-queue profile now answers several branches at once, and a profile that read each branch alone would still pass its seam's scenarios while reading per place. The conformance scenario counts the fake's reads to hold the core to one per repository.
- Bad: a full settle still reads the merge queue once for each place at each step, and each place's clues read every live agent on its base. That is a settle's cost, outside this record, and is said to the operator with the fix's delivery.
