---
number: 23
title: A fix of a base whose own checks failed after a landing is a fix launched on that base with the failure's words, held for the word the base asks
status: accepted
date: 2026-10-29
decision-makers:
- Chuck Swanberg
---

# A fix of a base whose own checks failed after a landing is a fix launched on that base with the failure's words, held for the word the base asks

Status: accepted by the user on 2026-10-30, on their card (req_4e228f0e220a4c92beef, "Approved").

Deciders: swancloud-design, on 2026-10-29 through madswan-operator, asking a fifth requirement for Flume's first build beside afterthought/crew record 0047 move 5's four: "when a gated branch's own checks go red after a landing, Flume launches the kind's fix stage in a place cut from that branch, with sluice's blame in its prompt, and its submission goes to that branch under the branch's policy, the user's word asked as the branch's policy asks it, with no bolt to ride and no unit moved. Crew adds nothing for it (roles.5)." The shape is madswan-design's; the user's yes was given on this record's card.
Sources: `agentplot/blueprints:design/flume/requirements.md` clauses 1, 5, 7, 22, 47, 49, 69, 70, 72, 111, 112, 124, 130, 131, 132, I6, the callers table's crew row, S1, S7 and S64, C.2's fix@1 row and C.6's submit row (blueprints 22bb880); records 0005, 0010, 0011, 0013; afterthought/crew records 0047 (move 5) and 0049, rules `state.3` and `roles.5`; `agentplot/blueprints:design/sluice/requirements.md` clauses 17, 19, 20, 70, 127 and the C.5 pull request row; the kits' `.config/sluice.toml` on their mains on 2026-10-29 (sluice, flume, tailrace, weir, agentplot: `[branches.main.pull_requests] reviews = 0`); wldn signal 2026-10-29-swb-4-conductor-ad4cb5f9/01, wldn-design's answer to it, and swancloud-design's reading of 2026-10-30 (WilldanGroup/switchboard-kit main 29f5db4; workflow runs 40974243331 and 40974243310).

## Context and problem statement

On 2026-10-29 in wldn a bolt landed on a kit's main and a test of its own then failed there, on both fleets. crew's one path for a fix of main is a bolt of one chore: the planner proposes it, the user approves it on a card, a team takes it and main-level ops lands it on the user's word (crew `state.3`, record 0032). The team held a bolt already, so it rode the fix on that bolt and dropped a unit still in construct to make room: the fix reached main mixed with unrelated work, a unit was lost, and the card the rule asks for was never raised. swancloud-design asks that the fix of a red base be Flume's from its first build, since record 0047 move 5 takes teams, conductors and the fix slot out of crew and leaves a conductor to come back by hand wherever a kind's course needs a word between its steps.

Flume's statement already holds the pieces. A fix is a kind of flume's own, one code stage with a brief, whose place is cut from its base's current revision and submitted to that base (22, C.2; S1 shows headwater's fix key launching one on a release's base). A launch may hold the submission for its caller's word, which a retry gives (7, 47, 49, S7). flume submits through the merge queue, so the base's policy governs what lands and when (111, 124). And flume launches only when a caller asks, and decides nothing its callers decide (1, 69, I6). What the statement did not say is who launches a fix of a red base, with what brief, and how the user's word is asked where a base's landings wait on one.

The word differs by base. Every kit's main takes pull requests with `reviews = 0`, so sluice's policy asks no word of its own there, and main's landings wait on the user's send through main-level ops (crew `state.3`; record 0047 move 5: "a landing is main-level ops's on the user's send, as now"). A base whose policy asks a review would ask the word itself: sluice's submit opens the request and takes it in once the review holds (sluice 127, C.5). A bolt's branch asks none.

## Considered options

1. **A fix bolt of one chore**, as crew `state.3` has it: proposed, approved on a card, given to a team, landed on the user's word. The fix waits on a card, a proposal and a team's turn while main is red, and a team that holds a bolt cannot take it.
2. **Ride the fix on a held bolt**, as the team did: the fix lands mixed with the bolt's work, a unit is dropped to make room, and no card is asked.
3. **flume watches the base and launches the fix itself** when its checks go red. Against 1, 69 and I6 (flume launches only when a caller asks and decides nothing its callers decide), 72 (flume tells no person anything) and record 0005 (no process of flume's own): reading a base's checks and deciding a fix is due is a caller's judgement.
4. **The merge queue amends**: sluice fixes what it blamed. Against sluice's core.4 and core.14 and its non-goals: sluice tests and lands and never changes a submission's work.
5. **A fix place on the base, launched by the caller that reads the failure, with the failure's words as its brief and its submission held where the base's landings wait on a word the merge queue's policy does not ask** (chosen).

## Decision outcome

Option 5.

**The fix is a fix place on the base.** A caller launches flume's fix kind on the base whose checks failed, named for the failure, and its place is cut from the base's current revision (22, core.7); its code stage delivers commits, flume submits the place to that base through the merge queue, and a failed submission sends it back to code like any fix (111, 112, 131). No bolt's branch stands between the fix and the base, the bolt that landed the defect is not reopened, and no other place is moved or dropped for it.

**The caller is whoever reads the failure, never flume.** flume watches no base and launches nothing of its own (1, 69, I6). Under crew the caller is main-level ops, which lands on main and reads the landing's outcome (record 0047 move 5); the user through `/launch`; or a role woken to it (132, record 0013). Which of them, and how the red is read, is crew's and the forge's, not Flume's.

**The brief is the failure's words as the caller has them.** The merge queue's where the failure is a submission's: the check blamed and its log, as 112 gives a place's own. The check's own words where the base's checks ran elsewhere, on the forge's workflow or another host's gate. flume never judges or rewrites a brief (5).

**The word is asked as the base asks it.** Where the base's policy asks the word itself, as a request's review, flume's plain submit opens the request and it waits for the review; nothing is held (C.6, sluice C.5). Where the base's landings wait on a word the policy does not ask, every kit's main today, the launch holds the submission (7, 47) and the caller's word reaches flume as a retry naming the submission (49, S7, S64); under crew that word is the user's send, given as a bolt's landing is given. Where the base asks none, a bolt's branch, the fix lands as tested.

**Nothing new is needed of Flume's clauses.** fix@1, the launch's hold, the retry and the plain submit carry it; the statement gains the callers table's crew line, scenario S64 and C.6's reading of a submit on a base whose policy asks a review, at blueprints 22bb880. Rule core.36 is drawn from it, so no code of flume's ever polls a base's checks or launches a place by itself.

**Why not the others.** Options 1 and 2 keep the fix in crew's hands, which record 0047 move 5 takes out, and option 2 is what went wrong. Option 3 puts a conductor's judgement inside flume. Option 4 asks the merge queue to write code.

### What could go wrong, and what holds it

- **The blame when the base itself is red.** sluice attributes a failed candidate to its newest entry (sluice 17), so a base whose own check failed in sluice's gate would have the next submissions blamed for a failure that is the base's, and a check whose inputs an entry did not touch shares the landing's key, which then holds a pass and a fail and reads flaky (sluice 19). wldn's case was not that. Read on 2026-10-30 by swancloud-design (WilldanGroup/switchboard-kit main 29f5db4, sluice entry #991, PR 99): the failing test, `apps/console:test`'s dev server against production's install, was new in the landing itself and failed in the forge's workflows on main after sluice's push, on wldn's own fleet runners, while sluice's gate had passed the entry and saw no failure, no blame and no flaky key; main's policy there reads no check of the forge's. That is this record's second case, the check's own words where the checks ran elsewhere, and it asks no clause of sluice. A base red in sluice's own gate, should one come, is read then; this record decides nothing of sluice's.
- **A fix landing on main with no word.** Only where the base's policy asks none and its landings wait on none; main's wait on the user's send, held at the launch. A caller that launches a fix of main with nothing held lands it as tested: that is the caller's choice, as a bolt submitted with nothing held is today.
- **The build's sluice profile reading a request.** A submit on a base whose policy asks a review answers with the request and no entry (C.6); a profile that reads only `{ahead, branch, id}` would read that as a refusal. Checked at record 0011's build; a unit only if it does.

### Consequences

- The statement's callers table, S64 and C.6 are amended at blueprints 22bb880; rule core.36 is added; `_open.md`'s item for this record left at its acceptance.
- No unit of flume's own from this record: fix@1, the hold and the retry are record 0011's build. The sluice profile's reading of a request waiting on its conditions is checked at that build.
- crew's side, a fix of a red main as a fix place and not a bolt of one chore, and who launches it, is afterthought/crew `state.3`'s to amend, swancloud-design's record; crew adds no mechanism for it (`roles.5`).
- sluice's side: wldn's reading of 2026-10-30 shows a failure sluice never saw, so nothing of sluice's follows from this record.
