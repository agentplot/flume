---
number: 9
title: A caller is told an end once, inside the settle that reads it, and resumes its wait by place from the host's state
status: accepted
date: 2026-10-27
decision-makers:
- Chuck Swanberg
---

# A caller is told an end once, inside the settle that reads it, and resumes its wait by place from the host's state

Status: accepted by the user on 2026-10-28, on their card ("Approved").

Amended 2026-10-28 by record 0012, accepted by the user: a submission's outcome reaches flume's settle by the settle asking sluice for the place's untold outcomes (clause 133), not by sluice's push; and a caller's start that asks for its untold ends is, for crew, a role run's start (record 0013), not a session-start hook.

Deciders: the user, in swancloud-ops's pane on 2026-10-28 (afterthought/crew-state, swancloud/main, signal 2026-10-28-swancloud-ops-4f509aa5/01-waits-are-pushed-and-reconnect-after-restart): "If our records already know about sluice submissions, crew units, and chores, then if we restarted a Claude Code session, it should be able to come back up and reconnect with those watchers (either through the mod or through some instruction)", which settles that an end reaches its waiter by a push the waiter picks up again from the records after a restart; the shape is madswan-design's, on swancloud-design's asking of the same night for Flume's answer to the statement's open question 43/46, and the user's yes is asked on the record's card.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 43, 46, 56, 58, 72, 95, 119 to 122, 125, 129 and the section 10 question "Does a caller learn a stage's or a submission's end by asking, or does flume tell it, and through what"; records 0003, 0005, 0006, 0008; rules core.9, core.10, core.11, core.14, core.24; `agentplot/sluice:docs/adr/0068` (a submission's outcome pushed to its submitter and kept until told); `afterthought/crew:docs/adr/0046` (waits are pushed into the session and reconnect from the records after a restart), part 6.

## Context and problem statement

The statement says whoever waits on a stage learns its end with what was delivered, and whoever waits on a submission learns its outcome with the commit it landed as, and that no caller needs to read a terminal for either (43). It says a place moves on at the settle that reads its end (46), that flume runs no process between its commands (120) and tells no person anything by itself (72). It did not say how the caller learns: by asking, or told, and through what. crew, Flume's first caller, polls: a conductor's wait asks every ten seconds, and on deneb on 2026-10-28 the polling waits of every agent, with sluice's, held the host at the load that stopped every team. A conductor whose session restarts loses the wait it was in and learns nothing until it asks again.

## Considered options

1. The caller asks: status polled by the caller's own loop, as crew does today.
2. flume keeps a process that pushes ends to callers as they happen.
3. The end is told once, inside the settle that reads it, through the terminal into each waiter's pane, and recorded on the place as told; a wait is by place, answering an untold end at once and otherwise following the host's state under a timeout; a caller that starts asks flume for its untold ends.
4. The caller reads its agent's pane for the end.

## Decision outcome

Option 3.

**Told, inside the settle.** The settle that reads a stage's end or a submission's outcome (46) tells each waiter on the place once, through the terminal's send into the waiter's own pane (95, core.11), what 43 names: the head or the report and the addresses of what was made (129), or the agent's words; the commit the place landed as, or the merge queue's words. It records on the place, in the host's state (record 0006), that the end was told, to whom and when. The settle already runs where flume is allowed to act, inside the terminal's watch or a caller's command (119, 120, 122), so no process of flume's is added, and a person is told by no push of flume's (72): the printed-command terminal has no pane to tell, and there the end is read from status.

**A wait is by place, and a place outlives a process.** A caller waits on a place by its name, naming the stage or the submission: the wait answers at once with any end of that place that stands untold to this waiter, and otherwise follows the host's state, exiting by the outcome, or still waiting when its timeout passes, as sluice's wait does (sluice record 0041). It drives nothing where the terminal keeps watch, and settles as 119 says where it does not. Because the end and whether it was told live on the host and not in the waiting process, a caller whose session restarted runs the same wait and gets the same answer; nothing was lost with the process.

**A caller that starts asks once for what it was not told.** flume answers, for a caller's identity, every end of the places it launched or waits on that stands untold to it, with what 43 names; reading the list marks nothing told, since only the tell does. That list is what a caller's own start reads, crew's session-start hook among them, so no caller needs a wait to survive its restart.

**Why not the others.** Option 1 is the load the user measured and a wait that dies with its process. Option 2 breaks 120 and makes flume a second crew, polling or pushing between its commands. Option 4 is what 43 forbids: a caller reading a terminal to learn how work ended, from a pane whose words are not the record's (core.9).

### Consequences

- Clause 43 of the statement says how a caller learns an end: told once inside the settle, through the terminal into its pane, recorded as told; a wait by place; the untold list at a start. The section 10 question is answered, and `_open.md` says so. Rule core.25 is drawn from it.
- The place's record in the host's state gains, per end, who was told and when; status (56) shows an end untold as such.
- The command line gains a wait by place with a timeout and an exit per outcome, and a listing of a caller's untold ends.
- The herdr plugin's settle carries the tells; the printed-command terminal tells nothing and relies on status.
- What crew does with a told end is crew's, as record 0046 has it; sluice's outcome for the place's submission reaches flume's settle as sluice record 0068 pushes it, and flume tells the caller once, in its own words.
