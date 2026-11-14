# running-through Specification

## Purpose
A kind's course from launch to landing with no caller's word: each
delivery starting the next stage as the host's room allows, the last
stage's delivery submitting a landing kind's place, the fix round a
failed submission or found work sends the place to and its bound, and
the two stop-shorts, one needing an answer and one finding work.

## Requirements

### Requirement: Delivery starts the next stage and the last submits

A stage's delivery SHALL start the place's next stage as the host's room
allows, and the last stage's delivery SHALL submit a place whose kind
lands, with no caller's word unless the launch holds the stage or the
submission (40, 47, 111, 131).

#### Scenario: A change runs through

- **WHEN** a change is launched with nothing on hold over the fakes, and construct, code and verify each deliver and go quiet
- **THEN** each next stage starts in the settle its agent's going quiet runs, the place is submitted after verify, and no caller acts between (S35, S2)

### Requirement: The fix round

A failed or conflicting submission SHALL start the stage the kind's last
stage names for a failed submission, that last stage where it names
none, with a fresh agent and the outcome's words; found work from the
kind's last stage SHALL start it with the findings as its words where it
is a stage before that last stage. The stages after it SHALL run again
in order and the place SHALL be submitted again (23, 48, 112, 131).

#### Scenario: A blamed test, then a landing

- **WHEN** the fake merge queue fails a change's first submission with its words and lands its second
- **THEN** code starts with the words, code and verify deliver again, the place is submitted again and its work is over (S35)

#### Scenario: Verify finds work

- **WHEN** a change's verify agent stops short having found work
- **THEN** code starts at once with the findings as its words, and verify runs again after it (S35, 48, 131)

#### Scenario: A fix's failed submission

- **WHEN** the fake merge queue fails a fix's submission with its words, the fix's code naming `code` for a failed submission
- **THEN** code starts again with a fresh agent and the words, the place's goings-back are one, and once code delivers the place is submitted again (23, 112, 131)

### Requirement: A conflict's fix takes the base in

A conflict's words SHALL carry the base's revision the merge queue
tested against, so the fix round's agent merges it into the place's
branch and commits before anything else; flume SHALL merge, rebase or
commit nothing itself (14, 112, core.5).

#### Scenario: A conflict

- **WHEN** the fake merge queue's inbox gives the place's submission in conflict with the files contested
- **THEN** the fix round starts with the outcome's words naming the files and the base's revision tested against, and the fakes record no merge, rebase or commit of flume's (112, core.5)

### Requirement: Goings-back count together toward the kind's bound

Every going-back, by submission and by finding, SHALL count toward one
count against the last stage's `max_send_backs`, three where none is
set; a place at its bound SHALL start nothing more and wait on its
caller, status saying so with the words (23, 112, 130, 131).

#### Scenario: A finding then a failure past the bound

- **WHEN** a kind allows one going-back, verify finds work once, and the next submission fails
- **THEN** the second sends nothing back, the place waits for its caller's retry or drop, and status gives the move and the merge queue's words

### Requirement: Found work no stage before it can do again needs an answer

Found work from any stage but the kind's last, and from a last stage
that is the stage the kind names for a failed submission, or names none,
SHALL be taken as needing an answer: the stage SHALL end short with the
agent's words and the kind it said, no stage SHALL start, no going-back
SHALL be counted, and the place SHALL wait on its answer, its agent kept
open for it as for a question, status saying it waits on an answer and
that the agent found work, with its words (41, 42, 131, 198).

#### Scenario: Construct says it found work

- **WHEN** a construct agent stops short with `found work`
- **THEN** no stage starts, the place waits on an answer with its agent kept open for it, and status says the agent found work with its words

#### Scenario: Found work from the last stage keeps no agent open

- **WHEN** a verify agent, the kind's last stage, stops short having found work
- **THEN** its agent is kept open for nothing, it is ended once quiet, and the kind's fix round starts with the findings (131)

#### Scenario: A fix's code finds work

- **WHEN** a fix's code agent, its stage naming `code` for a failed submission, stops short having found work, and the place is settled after its agent goes quiet
- **THEN** no agent starts, code stands ended short having found work with its agent kept open for the answer, the place's goings-back are none, status gives the move as a retry of code waiting on an answer with the findings, the place's line reads `waiting on an answer`, and the end delivered is `code found work` with the findings (131, 42, 56, 198)

#### Scenario: A last stage that names no stage for a failed submission

- **WHEN** a repository's kind of one stage, `review`, naming no stage for a failed submission, has its agent stop short having found work
- **THEN** no agent starts, the place waits on a retry of review for an answer with the words, and its goings-back are none (23, 131)

### Requirement: flume's own kinds send a change back to code

flume's own declaration SHALL hold fix@1, chore@1 and change@1 as C.2
gives them, change@1's last stage naming `code` for a failed submission
(22, C.2).

#### Scenario: A change's submission fails

- **WHEN** a place of flume's own change@1 has its submission failed
- **THEN** the stage started is code, never verify (22, S35)

### Requirement: change@3 codes its change with one agent

A place of flume's own change@3, or of change@4 after it, SHALL run its
construct, then its code stage as one start of one agent for every task
of the change, whatever its task groups, then its verify, and SHALL be
submitted once verify delivers the change archived; status SHALL give
its code stage as `code` with no group (22, 30, 154, S119; record 0064).

#### Scenario: A bare change runs change@3 through

- **WHEN** a repository whose lock records `flume/change@3` launches `change`, its construct writes a change of four task groups, its code agent ticks every task and goes quiet, and its verify archives the change
- **THEN** the place runs `flume/change@3`, its code stage has one start, status gives the stage as `code` with no group count while it runs, and the place is submitted once verify delivers (22, 154, 155, S119)

#### Scenario: A bare change runs change@4 through

- **WHEN** a repository that locks nothing launches `change`, its construct writes a change of four task groups, its code agent ticks every task and goes quiet, and its verify archives the change
- **THEN** the place runs `flume/change@4`, each stage's agent in `full` from `kind`, its code stage has one start, status gives the stage as `code` with no group count while it runs, and the place is submitted once verify delivers (22, 154, 162, S119)

### Requirement: The retry that answers a found

A caller's retry of a stage that ended short having found work and
waits on an answer SHALL be its answer and SHALL count no going-back:
while the stage's agent is kept open for it, the retry SHALL hand that
agent the findings with the answer in its session, the stage carrying
on under the same start; otherwise it SHALL start the stage with a
fresh agent whose prompt carries the findings with the answer (42, 48,
49, 198).

#### Scenario: The retry that answers a fix's found in its session

- **WHEN** a fix's code agent stopped short having found work and is kept open for its answer, and the caller retries code with an answer
- **THEN** that agent is handed the findings and then the answer, code is begun again under its first start with one agent opened, the goings-back stay none, and once code delivers the place is submitted and lands (42, 48, 49, 198)

#### Scenario: The retry that answers a fix's found

- **WHEN** a fix's code agent stopped short having found work and is no longer kept open, and the caller retries code with an answer
- **THEN** code starts once with a fresh agent whose prompt holds the findings and then the answer, the goings-back stay none, and once code delivers the place is submitted and lands (42, 48, 49, 198)
