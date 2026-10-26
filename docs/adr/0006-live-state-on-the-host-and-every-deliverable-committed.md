---
number: 6
title: Live state on the host, and every deliverable committed
status: accepted
date: 2026-10-22
decision-makers:
- Chuck Swanberg
---

# Live state on the host, and every deliverable committed

Status: accepted by the user on 2026-10-26, on their card.

Deciders: the user, in the family's storage ruling (`design/capture/plan.md`, D4) and in flume's statement (a committed report; a `{{state}}` report only where the caller keeps it lasting).
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 24, 58, 62, 73 to 77, 102 to 104, 123, C.7; `agentplot/blueprints:design/capture/design.md`, "Where state lives".

## Context and problem statement

flume holds the live state of each place: its stage, its agent, its starts, its submission. Where does it live, who else may read it, and what of a stage's work must outlive the place?

## Considered options

1. A record of flume's inside the repository, on the place's branch or a state branch.
2. Live state as host state, one document per place under `$XDG_STATE_HOME/flume/`, written by compare-and-set on its version and gone when the place is tidied; what readiness makes for a place kept in the place's own git directory; every deliverable a later reader needs committed with the work on the place's branch, a report under `{{state}}` allowed only where the caller keeps what it holds once the place is gone.
3. A shared branch of flume's, so every host reads every place.

## Decision outcome

Option 2, by the family's rule that a piece of state lives in the lowest layer that holds every reader it has: a place's live state has one reader, flume on its host. Once a place is tidied flume keeps nothing of it; whatever outlives a place is its caller's to keep from what flume reported. Status reads deliverables, changes and submissions where they live when asked, never from a record of flume's.

Option 1 puts a tool's bookkeeping into a repository's content. Option 3 shares what nobody off the host reads, and flume coordinates no hosts.

### Consequences

- The verify stage's report is committed in the change's directory; any other report a reader needs after the place is gone is a committed report (22, 24, 123).
- Two acts on one place at once leave it as one or the other would have, and no act leaves a worktree or branch of flume's that its held state does not name (75, 76).
- flume stopping loses nothing: on its return it reads every place again from its held state, its repository, its terminal and the merge queue (77).
