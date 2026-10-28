## ADDED Requirements

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
kind's last stage SHALL start it with the findings as its words. The
stages after it SHALL run again in order and the place SHALL be
submitted again (112, 131).

#### Scenario: A blamed test, then a landing

- **WHEN** the fake merge queue fails a change's first submission with its words and lands its second
- **THEN** code starts with the words, code and verify deliver again, the place is submitted again and its work is over (S35)

#### Scenario: Verify finds work

- **WHEN** a change's verify agent stops short having found work
- **THEN** code starts at once with the findings as its words, and verify runs again after it (S35, 131)

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

### Requirement: Found work from another stage needs an answer

Found work from any stage but the kind's last SHALL be taken as needing
an answer: the stage ends short and the place waits for a retry, status
giving the kind the agent said and the words (42, 131).

#### Scenario: Construct says it found work

- **WHEN** a construct agent stops short with `found work`
- **THEN** no stage starts, the place waits on an answer, and status says the agent found work with its words

### Requirement: flume's own kinds send a change back to code

flume's own declaration SHALL hold fix@1, chore@1 and change@1 as C.2
gives them, change@1's last stage naming `code` for a failed submission
(22, C.2).

#### Scenario: A change's submission fails

- **WHEN** a place of flume's own change@1 has its submission failed
- **THEN** the stage started is code, never verify (22, S35)
