---
number: 13
title: A waiter may be a role's address, and an end untold to an address with no live run wakes the role
status: accepted
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# A waiter may be a role's address, and an end untold to an address with no live run wakes the role

Status: accepted by the user on 2026-10-28, in their design session ("i agree with these changes"), on the recommendations drawn from reading crew records 0045 to 0048, sluice records 0056, 0065 and 0068, and flume records 0002 to 0011 together.

Amended 2026-10-28 by record 0014, accepted by the user: a waiter is a recipient, and an end untold to an address is pending for it, collected by the run's start.

Deciders: the user, accepting recommendation R1 of the design review of 2026-10-28; the shape is the reviewing agent's.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 3, 43, 69, 70, 72, 132, C.7 and S38, as amended with this record, and the callers table's crew row; records 0005, 0009, 0010, 0011; `afterthought/crew:docs/adr/0047` move 6 (no crew role stands; each is a kind Flume launches on a nudge naming a record) and `0048` patterns 3 and 4; rules core.13, core.25, core.26, core.27.

## Context and problem statement

Crew record 0047's move 6 has no crew role stand: the planner, ops, design, the operator and the reviewer each run on demand, launched by Flume on a nudge, reading the records, acting and ending with their turn. Every push in the design lands in a session that is already running: flume tells the waiter's own pane (record 0009), sluice pushes into the submitter's session (sluice 0068), and Pending You's wake mod wakes the card owner's session. When the planner run that launched a place has ended, the place's end, a stop-short's question, a left place and a card's answer all stand untold, and no record says what starts the next planner run. flume cannot decide to start it (rule core.27: no agent of flume's decides the next step), and crew runs no service (crew `state.4`). Something must turn "untold for this role" into "launch this role", or move 6 never wakes.

## Considered options

1. A role is addressed, not a session: a waiter is named by `<partition>/<role>`, a run of the role takes that address as its session's name, and an end untold to an address with no live run, or a carrier's wake, launches the kind the host's configuration names for it, one run per address at a time.
2. A host job of crew's or swancloud's that polls each tool's untold lists and launches roles.
3. Pending You, sluice and flume each launch the role themselves when their push finds no pane.

## Decision outcome

Option 1.

**The address is the waiter.** A caller names the waiter of a place by a role's address rather than by a session. A run of the role is a place of the kind the host's configuration names for the address (`[roles."<partition>/<role>"]`, C.7), named by the role, so clause 3 already keeps one run of an address live at once; its agent takes the address as its session's name, so the terminal finds it by the address as it finds any session.

**Flume wakes on its own facts, and on a carrier's word.** Where a settle finds an end standing untold to an address with no run live or waiting for room, it launches the address's kind at the priority the configuration gives it, inside the settle, so no process of flume's is added (core.13). A carrier of another tool, the card wake among them, may ask flume to wake an address (`flume wake <address>`); a wake of an address whose run is live or waiting does nothing. The run's start asks every tool for what is untold to the address (43; crew 0048 pattern 4), so a burst of events is one run that reads them all.

**Flume decides nothing.** Which address waits on a place, what the role's kind does and what its run decides are the caller's (69, 70, 72). flume delivers to an address as it delivers to a session, and launching the address's kind when nobody is there to be told is how the delivery is made. An address the configuration names no kind for is woken by nothing, and status names its untold ends.

**Why not the others.** Option 2 is a standing poller, the daemon the family refuses, and a second reader of every tool's untold state. Option 3 has sluice name flume (sluice core.1) and puts three launchers on one role.

### Consequences

- Clause 132 is added to the statement, 43 names the role's address, C.7 gains `[roles]`, the callers table's crew row names its waiters by address, and S38 shows a burst of ends woken into one run. Rule core.28 is drawn from it.
- Rule core.25's details name a role run's start as the caller's ask, in place of "crew's session-start hook".
- Units: the address as a waiter; the wake at a settle; `flume wake`; `[roles]` read from the host's configuration; status naming ends untold to an address with no kind.
- crew's side, the role kinds and the waiter named by address, is crew record 0049.
