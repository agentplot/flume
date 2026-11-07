---
number: 56
title: A base place forecast or found to conflict with its base, or whose submission failed, has its fix launched by flume
status: accepted
date: 2026-11-07
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# A base place forecast or found to conflict with its base, or whose submission failed, has its fix launched by flume

Status: accepted by the user on 2026-11-07, on their card. Proposed the same day from their own words, relayed by agentplot-operator.

Amends record 0051 in part, where a base place's failed or conflicting submission starts nothing and its caller launches the fix (203), and clause 1 and I6, where flume launches only when a caller asks.

Deciders: the user, 2026-11-07, in agentplot-operator's pane, relayed by it: "Why couldn't the fix have been launched? That is my fucking point and you know that"; "After we rework Boomdrive and Flume, the fixes should be automatic because Flume will settle. At least according to the diagrams, it should all be in the Rust code. It should detect and launch a new fix"; and "Just make sure it's in the fucking design because it should be." The design's words are agentplot-design's.

Sources: flume's statement, clauses 1, 23, 112, 131, 202, 203 and I6 (agentplot/blueprints `design/flume/requirements.md`); sluice's clause 64, a forecast of a head not yet submitted (agentplot/blueprints `design/sluice/requirements.md`); records 0011, 0023 and 0051; crew record 0056, by which crew lands a bolt as flume's base place; captures 2026-11-07-agentplot-operator-d03e1696 and 2026-11-07-agentplot-operator-7514cde8 on afterthought/crew-state agentplot/main.

## Context and problem statement

On 2026-11-07 two of this repository's proven bolts, `a-fix-round-that-finds-work-asks` and `flume-codes-a-change-with-one-agent`, were forecast to conflict with main for 9h 46m and 2h 14m while nothing launched the fix that takes main in. Under crew that fix waits on the planner, and the planner started both only when the user asked.

The rework as designed doesn't close the gap; it moves it. Under record 0051 a bolt is a base place, and a base place's failed or conflicting submission starts nothing: it waits on its caller, who lands the fix on its branch and retries (203). Nothing forecasts a waiting base place against its base at all, so a bolt that main has moved past is found only when its caller submits it. A place with stages already goes back to its fix round on its own (112, 131); a base place has no stage to go back to.

## Considered options

1. **flume launches a base place's fix at a settle.** The base place's kind names the kind its fixes run as. A settle forecasts a waiting base place's branch against its base through the merge queue, and a forecast conflict, or a failed or conflicting submission, launches that fix on the branch with the merge queue's words.
2. **The caller forecasts and launches the fix**, as 203 stands: crew today, Boomdrive's runner later. Clause 1 stays whole, but every caller builds the same watch, and a caller that doesn't leaves the bolt waiting, as crew did.
3. **flume submits a proven base place on its own**, so the merge queue finds the conflict. The submission to main waits on the user's send (record 0023), so this answers nothing before the send and moves who sends.

## Decision outcome

Option 1, the user's decision.

- **The kind names its fix.** A base place's kind names the kind its fixes run as, as a kind with stages names its fix round (23). A base place whose kind names none waits on its caller, as before.
- **A settle forecasts a waiting base place.** A base place with no live place on its branch and no submission open is forecast at each settle: the merge queue's forecast of its branch's head against its base (sluice 64), placed after every entry already submitted. Where the host's reader is live, the forecast fetches nothing.
- **A conflict or a failure launches the fix.** A forecast conflict, or a failed or conflicting submission (112), launches a place of the fix kind on the base place's branch, cut there and landing there (202), with the merge queue's words as its brief: for a conflict, the base's revision and the files contested, which the fix takes in by a merge (112); for a failure, the checks blamed and their logs. The settle that launches it needs no caller, so the fix starts within the host's watch (0051) of the forecast that found it.
- **One fix at a time.** While the fix is live, the base place waits on it (203), and no second fix is launched. Once it lands, the base place is forecast again at the next settle, and waits as before when that forecast is clean.
- **The submission stays the caller's.** A base place is still submitted only on its caller's retry (203), so a send to main still waits on the user.
- **A bound.** Each fix counts toward the kind's bound (23). Past it, the base place waits on its caller, said and delivered (130, 43).
- **Clause 1 and I6.** flume launches when a caller asks, and launches the fixes a base place's kind names, which its caller asked for in launching the base place. flume still decides nothing a caller decides: the kind decides which fix, and the merge queue's words decide what it does.

### Consequences

- Good: a bolt that main moves past is taken back to main within the host's watch, with no agent or person deciding it, at every depth of base places.
- Good: callers build nothing for it. Once crew's bolts are base places (crew record 0056), crew's own bolt-against-main forecast and the planner's start of a take-main fix have nothing left to do; Boomdrive's runner does nothing for it.
- Bad: each settle of a waiting base place costs a forecast, a merge formed in the repository, while its branch stays proven and unsent.
- Bad: clause 1's flat "only when a caller asks" gains one exception, which status must show: a fix status names as launched by flume for its base place.
- Statement clauses 1, 203 and I6 are amended with this record, marked proposed.
