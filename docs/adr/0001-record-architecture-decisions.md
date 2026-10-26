---
number: 1
title: Record architecture decisions
status: accepted
date: 2026-10-22
decision-makers:
- Chuck Swanberg
---

# Record architecture decisions

Status: accepted by the user on 2026-10-26, on their card.

Deciders: the user, in flume's statement (plan step L1, ruled and amended through 2026-10-20).
Sources: `agentplot/blueprints:design/flume/requirements.md`; `agentplot/blueprints:design/capture/plan.md` (L2); `agentplot/blueprints:design/requirements-style.md`; sluice's `docs/adr/0001` as the model for the form.

## Context and problem statement

flume starts with a statement of invariants and no code. The decisions that shaped the statement were made in design talk and in the operator's rulings on a reconstruction, and a decision made in a conversation alone is not recorded. Where do the decisions live, how do the rules cite them, and what is kept out of this public repository?

## Considered options

1. Decisions as prose in the README and the site.
2. One MADR record per decision under `docs/adr/`, with the rules in `docs/architecture/` naming them as sources.
3. Decisions only in the statement and its reconstruction in blueprints.

## Decision outcome

Option 2. The statement in blueprints says what is always true of flume; the records here say why a choice was made where the statement admits several; the rules in `docs/architecture/core.md` say what a diff must keep and cite both. A record is proposed when written and accepted only on the operator's yes (`briefs.15`). Nothing commercial appears in this repository; commercial thinking belongs in agentplot/strategy.

### Consequences

- Every rule in `core.md` names a clause or a record as its source.
- A change that departs from a rule names it and says why, in the commit or the design.
- A clause that no longer holds is changed in the statement first, never worked around here.
