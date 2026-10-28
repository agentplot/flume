# settling Specification

## Purpose
What moves a place on with no process of flume's own: a settle run when
its agent goes quiet, when the merge queue decides and when the
terminal watches, or before every acting command where nothing watches;
and a landing kind's submission, waited on and sent back when it fails
(I7, I8; 40, 46, 111 to 113, 119 to 122, 124).

## Requirements

### Requirement: A settle takes every step the place allows

A settle of a place SHALL read every fact of it again and take each
step Part A then allows: read its current stage's end, end a delivered
stage's quiet agent and start its next stage unless that stage waits on
its caller, submit a landing kind's place whose last stage delivered,
and tidy a place whose work is over. A settle SHALL be safe to repeat
(40, 46, 74).

#### Scenario: A commit and a quiet agent

- **WHEN** a code stage's fake agent commits, goes quiet, and the agent seam's on-quiet settles the place
- **THEN** the agent is ended and the next stage started in that settle, before any watch runs (S29, 121)

#### Scenario: A held next stage

- **WHEN** a construct delivers and the launch holds every later stage and the submission
- **THEN** the settle ends the agent and starts nothing more (S7, 47)

### Requirement: Attended and watched

Where the terminal seam keeps a watch, its watch SHALL settle each
place on its stated bound. Where it keeps none, every command of
flume's that acts on places SHALL settle every place on the host before
it acts, and status and a prompt's rendering SHALL settle nothing (119,
122).

#### Scenario: Killed with nobody watching

- **WHEN** under a fake terminal with no watch a code stage's agent stops with nothing delivered, and later a caller drops another place
- **THEN** nothing reads the stage until the drop, which settles every place first, and status then reports the stage lost (S27)

#### Scenario: Crashed under a watch

- **WHEN** under a fake terminal with a watch a code stage's agent stops with nothing delivered and its on-quiet never runs
- **THEN** the watch's settle reads the stage lost within its bound, with no caller asking, and starts no agent (S28)

### Requirement: A landing kind's place is submitted at its head

Once a landing kind's last stage has delivered and its agent has ended,
a settle SHALL submit the place's own branch at its head to its base
through the merge queue seam and wait on the verdict, unless the launch
holds the submission or the place's work is over. A kind that does not
land SHALL never reach the merge queue (111, 113).

#### Scenario: A fix lands and the merge queue removes it

- **WHEN** a fix's code stage delivers, its agent goes quiet, and the fake merge queue lands the submission and removes the place
- **THEN** the core records the place gone, removes the branch when it holds nothing its base lacks, and keeps no held state of it (S1, 66, 67)

#### Scenario: A landing before flume submits

- **WHEN** a held submission's place is landed by a hand's submission of its own branch
- **THEN** the core reads the work over, never makes the held submission, and tidies the place (S30, 63)

#### Scenario: Another place's landing

- **WHEN** a place's work reaches its base inside another place's landing
- **THEN** the place stays and its next stage waits on its caller (S8, 63)

### Requirement: A failed submission sends the place back

A failed submission SHALL send the place back to the stage its kind
names, started with a fresh agent and the merge queue's words as its
words, the stages after it run again and the place submitted again, up
to the kind's count of send-backs, after which the place SHALL wait for
its caller. A refused or withdrawn submission SHALL leave the place
waiting, status naming why (112).

#### Scenario: A failure, then a landing

- **WHEN** the fake merge queue fails a change's submission with its words, then lands the second
- **THEN** verify starts with a fresh agent and those words, it commits, the place is submitted again, it lands, and its work is over (S19)

#### Scenario: The count reached

- **WHEN** a kind allows one send-back and the fake merge queue fails both submissions
- **THEN** after the second failure the place waits for its caller's retry or drop and starts nothing (112)

### Requirement: An open submission nobody waits on is waited on again

A settle that finds the place's submission open, with no live command
of flume's waiting on it, SHALL wait on that submission through the
merge queue seam and settle the place when it decides, and SHALL never
submit the place anew for it (124, 125).

#### Scenario: A restart while under test

- **WHEN** the core is rebuilt over the same fakes while a change's submission is under test and the waiting command is gone
- **THEN** the next settle waits on the same submission, never submits anew, and when it lands tidies the place (S34)
