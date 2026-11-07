# A found that waits on an answer keeps its agent open

## Why

Two branches of flume each carried a decision of clause 42 and met when
the bolt `a-fix-round-that-finds-work-asks` took main in on 2026-11-07.
The bolt's change `a-found-at-its-kinds-own-fix-round-waits-for-an-answer`
takes a found from a last stage that is its kind's own fix round, fix's
and chore's code among them, as needing an answer (131), and words its
answer as a retry whose fresh agent's prompt carries the findings with
the answer. Main's change `an-answer-reaches-the-open-session-within-an-hour`
keeps open every agent that stops short needing an answer, where its
terminal hands a running agent words, and hands an answer that comes
while it is kept open to its session (42, 198; record 0046).

On agentplot/blueprints main at b0df70b the statement holds both: 131
takes such a found as needing an answer, and 198 keeps open an agent
that stops short needing one. The merge reads them together in code,
one reading of whether a found goes back (`submit::found_goes_back`)
taken by the stop-short, the settle and the open session alike. The
running-through spec's "The retry that answers a found" still says the
retry starts a fresh agent whatever the agent's keeping open, which
198 does not allow, and its scenario of a fix's code finding work says
nothing of the agent kept open.

## What Changes

- "The retry that answers a found" says the retry is the answer and
  counts no going-back: while the stage's agent is kept open, it hands
  that agent the findings with the answer in its session, the stage
  carrying on under the same start; otherwise a fresh agent's prompt
  carries them (42, 48, 49, 198).
- "Found work no stage before it can do again needs an answer" says, in
  its scenario of a fix's code finding work, that code's agent is kept
  open for the answer (131, 198).

No code changes: the merge commit of 2026-11-07 built and tested both,
the conformance scenarios `found_work_at_the_kinds_own_fix_round`,
`a_fixs_code_finds_work`, `the_retry_that_answers_a_fixs_found_in_its_session`
and `the_retry_that_answers_a_fixs_found` among them.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `running-through`: "Found work no stage before it can do again needs
  an answer" and "The retry that answers a found".

## Impact

The running-through spec. Rule core.27, records 0011 and 0046 and the
site already read so at the merge commit.
