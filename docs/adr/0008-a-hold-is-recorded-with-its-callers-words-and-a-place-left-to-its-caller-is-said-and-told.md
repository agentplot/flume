---
number: 8
title: A hold is recorded with its caller's words, and a place left to its caller is said and told
status: accepted
date: 2026-10-27
decision-makers:
- Chuck Swanberg
---

# A hold is recorded with its caller's words, and a place left to its caller is said and told

Status: accepted by the user on 2026-10-27, on their card.

Deciders: the user, accepting crew record 0044 on 2026-10-27 with the note "I'd rather spend the tokens to build out the correct functionality in Flume, sluice, and Tailrace instead of continuing to make crew more kludgy", which hands Flume two of that record's outcomes; the shape is madswan-design's, on swancloud-design's and madswan-planner's asking of the same day, and the user's yes is asked on the record's card.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 8, 12, 40, 42, 43, 44, 47, 49, 56, 58, 62, 70, 72, 114, 120 and the open question on how long a stage may wait on its caller's word; `afterthought/crew:docs/adr/0044-a-bolts-next-stage-starts-itself-and-a-stalled-team-is-raised.md`, points 1 and 3; `afterthought/crew-state:swancloud/main` signal 2026-10-27-swancloud-operator-f037f753/02-idle-conductor-raises-nothing; records 0003 and 0006; rules core.14, core.15, core.16.

## Context and problem statement

On wldn's swb-2 a conductor held a unit for seventeen hours on a note to itself, waiting for another bolt's landing that came at 00:59Z with nothing telling it. crew's record 0044 answered for crew: a hold is a recorded wait that ends at its event and says what for and since when, never a note the holder keeps, and a team holding startable work with nothing running for five minutes is said and told. The user accepted it and asked that the function be built in Flume, which runs stages and their waits, not added to crew.

What Flume already holds: a stage or submission on hold waits on its caller's word and starts only on a retry that names it (47, 49); which holds there are is fixed at the launch, the words of a retry being the only thing a caller adds after (8); a stage that ends short waits for a retry or a drop, its agent's words reaching whoever waits on the stage and status whole (42, 43); flume starts every next stage that waits on no word itself (40), so no stage is left unrun by anyone's forgetting; a stage whose agent is quiet past its limit is reported stuck (44); status gives every fact of a place read where it lives (56, 58); flume tells no person anything by itself (72) and runs no process between its commands (120); and when a held stage starts is the caller's (70). The statement left open how long a stage or a submission may wait on its caller's word before flume reports it, if ever.

The gap is in three places. A caller cannot put a hold on a place after its launch, so a wait decided mid-flight, for a card's answer or a reading owed, has nowhere in Flume to live but the caller's memory. Status says a place waits on its caller but not since when, nor what for. And a place whose next move has been its caller's for a long time is said to nobody.

## Considered options

1. flume watches the events holds wait for, a card's answer, a tell, another place's end or landing, and ends each hold itself.
2. A hold is a recorded wait with its caller's words and flume's clock, put at the launch or after it, shown by status with what for and since when, and ended by the caller's retry; a place whose next move has been its caller's for five minutes under no hold's words is left: status says so and whoever waits on the place is told once. flume watches no event.
3. Nothing in flume: the caller keeps its own waits and reminders, as crew does today.

## Decision outcome

Option 2.

**The hold is flume's to keep; the event is the caller's to see.** A hold's reason and clock belong where the place lives, in flume's held state, so that no conductor holds a unit in its memory and status shows every waiting place with what it waits for and since when (56, 62). But the events a hold waits for are the caller's world: a card answered on the user's phone, a reading an agent owes, a landing on another branch. flume tells no person anything and runs nothing between its commands (72, 120), and a flume that polled cards and tells would be a second crew. So the caller ends the hold, by the retry that already starts a held stage (47, 49), when its own wait on the event ends; what flume adds is that the hold exists, is written, and is read.

**A hold after the launch, with its words.** Clause 8 fixed every hold at the launch, which fit a plan known whole at the start and not a construct that finds mid-flight it must wait. A caller may now put a place's next stage, or its submission, on hold after the launch, saying what for; the words are the second thing a caller adds to a place, beside a retry's (8, 49). A stage that ended short is already such a wait with the agent's words as its reason (42).

**Left is said, once, to whoever waits.** A place whose next move has been its caller's for five minutes, under no hold's words, is left: status says so with since when, and whoever waits on the place is told once that it is left and which move is theirs, through the same channel that carries a stage's end (43). A held place is not left, however long it waits, since its words say why; a place ended short is left when nobody has acted on the agent's words for five minutes. The bound is the launch's to set and five minutes when it sets none. This closes the statement's open question: flume reports the wait at five minutes, as a fact of the place, to whoever waits and to status, and never to a person on its own.

**Why not the others.** Option 1 makes flume a watcher of cards, tells and other places, which is the caller's job (70) and a daemon flume does not run (120); it would also decide for the caller when its wait has ended. Option 3 is the note swb-2 held for seventeen hours, and the user has refused it.

### Consequences

- Clauses 8 and 47 of the statement are amended, clause 56 gains the wait's facts, and a new clause 130 states the hold, the left place and the bound; the open question on how long a wait may last is answered and removed.
- A caller's hold after the launch is a new ask of flume's, named like a retry, carrying words; status gives for every waiting place the move it waits on, since when, a hold's words and whether it is left.
- crew's conductor, as a caller, records its holds in flume and keeps none in its memory; its wait on the event stays crew's, which wakes on the card or the tell and ends the hold by a retry.
- Rule core.24 is drawn from this record.
