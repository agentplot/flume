## MODIFIED Requirements

### Requirement: A settle takes every step the place allows

A settle of a place SHALL read every fact of it again and take each
step Part A then allows: collect the outcomes of its submission, read
its current stage's end, end a delivered stage's quiet agent and start
its next stage unless that stage waits on its caller, submit a landing
kind's place whose last stage delivered, and tidy a place whose work is
over; and then start the stages waiting for room on the host, as many
as there is room for. A settle SHALL be safe to repeat (40, 46, 70,
74).

#### Scenario: A commit and a quiet agent

- **WHEN** a code stage's fake agent commits, goes quiet, and the agent seam's on-quiet settles the place
- **THEN** the agent is ended and the next stage started in that settle, before any watch runs (S29, 121)

#### Scenario: A held next stage

- **WHEN** a construct delivers and the launch holds every later stage and the submission
- **THEN** the settle ends the agent and starts nothing more (S7, 47)

#### Scenario: Room freed by a settle

- **WHEN** a settle ends one place's agent on a full host while another place waits for room
- **THEN** the same settle starts the waiting place's stage (46, 70)

### Requirement: A landing kind's place is submitted at its head

Once a landing kind's last stage has delivered and its agent has ended,
a settle SHALL submit the place's own branch at its head to its base
through the merge queue seam, holding no wait on the verdict, unless the
launch holds the submission or the place's work is over. A landed place
SHALL be tidied by flume, the merge queue removing nothing of it. A kind
that does not land SHALL never reach the merge queue (66, 111, 113).

#### Scenario: A fix lands and the merge queue removes it

- **WHEN** a fix's code stage delivers, its agent goes quiet, the place is submitted, and the fake merge queue lands it
- **THEN** the merge queue removes nothing; the next settle collects the landing, and flume removes the place through the place seam and its branch when it holds nothing its base lacks, and keeps no held state of it (S1, 66, 67, 113)

#### Scenario: A landing before flume submits

- **WHEN** a held submission's place is landed by a hand's submission of its own branch
- **THEN** the core reads the work over, never makes the held submission, and tidies the place (S30, 63)

#### Scenario: Another place's landing

- **WHEN** a place's work reaches its base inside another place's landing
- **THEN** the place stays and its next stage waits on its caller (S8, 63)

### Requirement: A failed submission sends the place back

A failed or conflicting submission SHALL send the place back to the
stage its kind's last stage names, that last stage where it names none,
started with a fresh agent and the merge queue's words as its words, the
stages after it run again and the place submitted again, while the
place's goings-back, by submission and by finding together, are fewer
than the kind allows, after which the place SHALL wait for its caller.
A withdrawn submission SHALL leave the place waiting, and a submission
refused at submit likewise, status naming why (112, 131).

#### Scenario: A failure, then a landing

- **WHEN** the fake merge queue fails a change's submission with its words, then lands the second
- **THEN** code starts with a fresh agent and those words, code and verify deliver again, the place is submitted again, it lands, and its work is over (S19, S35)

#### Scenario: The count reached

- **WHEN** a kind allows one send-back and the fake merge queue fails both submissions
- **THEN** after the second failure the place waits for its caller's retry or drop and starts nothing (112)

## REMOVED Requirements

### Requirement: An open submission nobody waits on is waited on again

**Reason**: No command of flume's waits on a submission (120, 124;
record 0012); its outcome is collected at the settles that follow.

**Migration**: "A submission's outcome is collected at the settles that
follow" below.

## ADDED Requirements

### Requirement: A submission's outcome is collected at the settles that follow

Every settle of a place flume submitted SHALL collect the outcomes
pending for it through the merge queue seam's inbox and act on each
once, and where its submission is open SHALL drive its base one round
through step, once per repository and base in one command; a settle
SHALL never submit the place anew for an open submission (111, 124,
125, 133).

#### Scenario: A restart while under test

- **WHEN** the core is rebuilt over the same fakes while a change's submission is under test, and the fake merge queue lands it before the next settle
- **THEN** that settle collects the landing, never submits anew, and tidies the place (S34)

#### Scenario: Two places on one base

- **WHEN** one settle of every place finds two places' submissions open on one base
- **THEN** the fake merge queue's log holds one step for that base
