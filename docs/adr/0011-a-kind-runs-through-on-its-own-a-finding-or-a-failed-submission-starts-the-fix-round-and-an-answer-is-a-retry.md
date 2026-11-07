---
number: 11
title: A kind runs through on its own; a finding or a failed submission starts the fix round, a stop-short that asks waits on its answer, and a review note's fix is a fix launched with the note
status: accepted
date: 2026-10-28
decision-makers:
- Chuck Swanberg
---

# A kind runs through on its own; a finding or a failed submission starts the fix round, a stop-short that asks waits on its answer, and a review note's fix is a fix launched with the note

Status: accepted by the user on 2026-10-28, on their card ("Approved").

Deciders: the user, in swancloud-design's pane on 2026-10-28, relayed by madswan-operator: "yes. i want to simplify crew that way. so the priority i think becomes to fix the fires with cpu usage in sluice/crew. then build flume, then refactor teams from crew", which settles that Flume, not a conductor, runs a unit's stages, and that Flume is built right after the fires; swancloud-design's four asks of the same night, in its words: a kind's stage sequence launched and ended by Flume with no conductor's word; a failed or conflicting submission launching the kind's fix stage on its own with sluice's blame in its prompt; a stage that stops short with a question shown in status with its words and started again by the answer; a review batch's notes marked fix starting the fix round the same way. The shape is madswan-design's; the user's yes is asked on this record's card. The crew side is afterthought/crew record 0047 (the end state: no conductor, no team ops, no team, no dispatcher; per partition a planner, one ops, design and the operator).
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 18, 22, 23, 39 to 49, 53 to 56, 110 to 112, 124, 130, C.2, C.4, the callers table's crew row, S4 and S7, and section 10's first question; records 0003 (a stage ends on its deliverable or a stop-short), 0008 (a hold is the caller's words), 0009 (an end told once), 0010 (room is the host's, flume holds no cap); afterthought/crew records 0045 and 0047; sluice clauses 20 and 51 (what a failed or conflicting submission's words carry).

## Context and problem statement

Under crew today a conductor stands between every two steps of a unit: it reads the construct's end, starts the code stage, reads the code's end, starts verify, merges, reads sluice's blame, charges a fix to a slot, reads the review batch's notes and launches the fixes. Each of those readings is a judgement that is in fact a state machine, and each conductor is an agent whose tokens and context the partition pays for. The user has decided to take teams out of crew: once Flume runs a unit's stages, there is no conductor, no team ops, no team and no dispatcher, and capacity is Flume's room per host (record 0010). So Flume's first build must carry a unit's whole course, or a conductor comes back by hand.

The statement already has most of it. A stage's delivery starts the next and the last stage's delivery submits the place (40, 111); a failed submission sends the place back to the stage its kind names with the merge queue's words and a fresh agent (112); a stop-short ends the stage and its words reach whoever waits and status (41, 42); a retry starts a stage with words (49); fix and chore are kinds of flume's own (22). What it left to a caller's word, or open, were four points: the kind change@1 sent a failed submission back to verify, a stage whose agent checks and cannot fix; a verify stage that found work to fix stopped short for a caller's retry (section 10's first question, open); a stop-short did not say whether it asked a question or reported a finding, so status could not tell a place waiting on an answer from one whose work must be redone; and the callers table had crew hold every stage and every submission for its word (S7).

## Considered options

1. A conductor's word between the stages, as crew runs today: every stage after the first and the submission on hold, started by a retry; the conductor reads the merge queue's blame and the review's notes and charges fixes.
2. The kind runs through on its own: the fix round is the stage the kind names for a failed submission, started by a failed or conflicting submission with the merge queue's words and by the last stage's finding with the findings as its words, up to the kind's bound; a stop-short that asks waits on its answer, which is a retry carrying question and answer; a review note marked fix is a fix kind launched with the note as its brief; the caller decides what to launch, what to answer and what to do with a place past its bound (chosen).
3. A conductor inside Flume: a conversation agent per place or per host that reads ends and decides the next step.
4. The merge queue retries or fixes: sluice re-runs or amends a blamed submission itself.

## Decision outcome

Option 2.

**The kind runs through.** A stage's delivery starts the next stage and the last stage's delivery submits the place, with no word from any caller unless the launch or a hold after it asks for one (40, 47, 111, new clause 131). crew launches a unit, chore or fix with nothing on hold, so the unit's course from construct to landing runs inside Flume's settle, which already runs wherever Flume may act (46, 119, 120, record 0009). What a caller still decides is what to launch and on what base, what to answer, and what to do with a place that went back as often as its kind allows (70).

**The fix round is the kind's send-back stage, and two things start it.** A failed submission, one the merge queue blamed or found in conflict alike, starts it with the merge queue's words, the checks blamed and their logs or the files contested (112; sluice 20, 51). A stop-short that finds, from the kind's last stage, starts it with the findings as its words, as a failed submission does, which answers section 10's first question: a verify agent that finds work short of the change sends the place back by itself (131). For change@1 the send-back stage is code, not verify: verify checks and cannot fix, and a place sent to it would only stop short again for a word nobody gives; the code prompt, sent again with words, fixes what they name and records the fix as a ticked task of the change, so its deliverable, tasks ticked, is made since the stage started (24, C.2). Goings-back by submission and by finding count together toward the kind's bound (23); past it the place waits on its caller, said in status and told to whoever waits (130, 43).

**A stop-short asks or finds, and an answer is a retry.** An agent that stops short says which it does (41; C.4: `flume needs "<words>"` asks, `flume needs --found "<words>"` finds). A stop-short that asks leaves the place waiting on an answer, which status shows with the question (56, 130) and whoever waits is told (43). The answer, from the planner, the design agent or ops, through a tell to crew or a card the user answered, reaches Flume as a retry naming the stage with the answer as its words (49), and the fresh agent's prompt carries the question with the answer (42, `{{words}}` in C.2). Flume reads no tell, no card and no pane for an answer (72, record 0008); a tell to a place whose agent has ended stays refused (53, 55). A finding from a stage that is not the kind's last is taken as an ask, since no stage before it is the fix round.

**A review note's fix is a fix kind launched with the note.** A review batch and its notes are crew's (non-goals). A note the user marks fix becomes a place of flume's fix kind on the bolt's branch with the note as its brief (22, C.2, callers table), which runs its code stage, is submitted and lands, and goes back to code on a failed submission like any fix. Nothing new is needed of Flume for it; the callers table says so, and scenario S36 shows it.

**Why not the others.** Option 1 is the conductor the user is taking out, and its judgement is this record's state machine. Option 3 puts a conductor back inside Flume, against I6 (flume decides nothing its callers decide), 72 (flume tells no person anything) and record 0005 (no process of flume's own): a conversation agent reading ends is a second crew. Option 4 asks the merge queue to fix code; sluice tests and lands and never amends a submission's work (sluice core.4, 14).

### Consequences

- Clauses 22, 41, 42, 49, 56, 112 and 130 are amended and clause 131 added in the statement, with the crew row of the callers table, the non-goals, S4, S7, new scenarios S35 and S36, the `{{words}}` part and the change@1 row in C.2, and the stop-short row in C.4; section 10's first question is ruled. Rule core.27 is drawn from it, marked proposed.
- `_open.md`'s first item names this record for the verify finding; the item leaves when the record is accepted.
- Units on the yes, Flume's first builds, each citing this record: a kind that runs through under the settle, from launch to landing, with the fix round started by a failed or conflicting submission carrying the merge queue's words; a stop-short that asks or finds, status saying which with the words, a finding from the last stage sending the place back and an ask answered by a retry whose prompt carries question and answer; and flume's own kinds as C.2 now reads, change@1 sending back to code and the code prompt taking a failed submission's or a finding's words. A review note's fix needs no unit of Flume's.
- afterthought/crew record 0047 cites this record for its end state; crew's launches hold nothing, and its word to a place is a retry answering a stop-short or acting on a place past its bound.

## Amendment, 2026-11-06: a found at its kind's own fix round waits for an answer

Status: decided on 2026-11-06 by the design agent, on crew's place `fix--a-landing-in-another-hosts-queue-reads-as-in-the-queue`. Nothing in it is the user's alone: it holds to clauses 41 and 42 as they stand, narrows 131 to what this record's own reason for it says, and is cheap to change.

Deciders: the design agent.
Sources: `agentplot/blueprints:design/flume/requirements.md` on main at 734d60c, clauses 23, 41, 42, 48, 49, 112, 131 and 156 and C.2's table of flume's own kinds; `crates/flume-core/src/submit.rs` `sent_back_by` and `fix_round`, `crates/flume-core/src/status.rs` `waiting_on` and `crates/flume/src/profiles/agent/claude/mod.rs` `stop_short`, on main at 376c17c; crew's place `fix--a-landing-in-another-hosts-queue-reads-as-in-the-queue`, whose runs 1 to 3 each found a blocker outside the place and sent the place back to its own code stage, at flume main 376c17ca.

### Context and problem statement

This record has a found from the kind's last stage start the kind's fix round, since a verify agent checks and cannot fix, and code can. That reason holds where the fix round is a stage before the one that found, as change's code is before verify. fix@1 and chore@1 are one code stage, which is both their last stage and their fix round (C.2), and a kind whose last stage names no `on_fail` is sent back to that stage itself (23). So a found from fix's code started fix's code again, a fresh agent with the findings as its words and one going-back counted, until the kind's three ran out and the place waited past its bound. No stage before code exists to do that work: each fresh agent met the same thing and found it again. On crew's fix place, runs 1 to 3 each found a blocker outside the place and sent the place back to itself. Three agents' work bought nothing, and the caller heard of it only past the bound, as a going-back too many rather than as a question.

What every agent is told adds to it. Claude Code's way to stop short offers `flume needs --found` for "work this stage cannot deliver past", which a blocker outside the place is, while clause 41 says a found is work a stage before the agent's own must do again.

### Considered options

1. **A found sends the place back only where the fix round is a stage before the one that found; from a last stage that is its kind's own fix round it is taken as needing an answer, as from any other stage; and every agent is told that a found is work a stage before its own must do again** (chosen).
2. Keep sending a fix round's found back to that stage, and let the kind's bound stop it.
3. Refuse `flume needs --found` from a stage with no fix round before it, so its agent asks instead.
4. New versions of fix and chore whose code prompt tells the agent to ask rather than find.
5. Send a fix round's found back to that stage once, then wait on an answer.

### Decision outcome

Option 1.

**A found goes back only to a stage before it.** A found from the kind's last stage starts the fix round, as this record decided, where the fix round is a stage before the last. Where the last stage is the fix round, as fix's and chore's code, a repository's kind whose last stage names itself, or one whose last stage names none, the found is taken as needing an answer (42, 131). No stage starts, and the place waits on its caller's retry. Status and the end say the agent found work, with its words (core.27). The retry's prompt carries the findings with the answer (49). It counts no going-back, since only flume's own starts are goings-back (48, 112). A found from any other stage is taken so already.

**Every agent is told what a found is.** The one way to stop short (28) says `flume needs --found` is for work a stage before the agent's own must do again, in the same words for every start (core.31). An agent of fix's code, with no stage before its own, reads that anything it cannot go on without is an answer it needs.

**Why not the others.** Option 2 is what ran on crew's fix place: three fresh agents meet the same blocker, and the caller hears only past the bound. Option 3 refuses a stop-short whose words the caller needs, and leaves the agent to stop with nothing said (110). Option 4 changes nothing for a repository's kind whose last stage is its own fix round, and a place launched at fix@1 keeps fix@1 (21). A version of flume's own kinds keeps its content in every release (156), so fix@1's and chore@1's prompts stay as they are; the way to stop short is the agent profile's and fills their `{{needs}}`. Option 5 spends one fresh agent on the same blocker and still answers the caller late.

### Consequences

- Good: a fix that meets a blocker outside its place asks at once, its words reaching its recipients and status as a question's do, and no agent's work is spent finding it again.
- Good: one reading of a kind's fix round decides the send-back, the move status gives and the bound alike.
- Bad: a fix round's agent that found work its own stage could redo waits on its caller, where a fresh agent would have been started; the caller's retry starts that agent, with the findings.
- The statement's 41, 48 and 131 say so (blueprints 734d60c). Rule core.27 takes it.
- Unit: the change `a-found-at-its-kinds-own-fix-round-waits-for-an-answer`.
