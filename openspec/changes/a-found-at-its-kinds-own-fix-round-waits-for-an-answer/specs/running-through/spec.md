## RENAMED Requirements

- FROM: `### Requirement: Found work from another stage needs an answer`
- TO: `### Requirement: Found work no stage before it can do again needs an answer`

## MODIFIED Requirements

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

### Requirement: Found work no stage before it can do again needs an answer

Found work from any stage but the kind's last, and from a last stage
that is the stage the kind names for a failed submission, or names none,
SHALL be taken as needing an answer: the stage SHALL end short with the
agent's words and the kind it said, no stage SHALL start, no going-back
SHALL be counted, and the place SHALL wait on a retry of that stage,
status saying it waits on an answer and that the agent found work, with
its words (41, 42, 131).

#### Scenario: Construct says it found work

- **WHEN** a construct agent stops short with `found work`
- **THEN** no stage starts, the place waits on an answer, and status says the agent found work with its words

#### Scenario: A fix's code finds work

- **WHEN** a fix's code agent, its stage naming `code` for a failed submission, stops short having found work, and the place is settled after its agent goes quiet
- **THEN** no agent starts, code stands ended short having found work, the place's goings-back are none, status gives the move as a retry of code waiting on an answer with the findings, the place's line reads `waiting on an answer`, and the end delivered is `code found work` with the findings (131, 42, 56)

#### Scenario: A last stage that names no stage for a failed submission

- **WHEN** a repository's kind of one stage, `review`, naming no stage for a failed submission, has its agent stop short having found work
- **THEN** no agent starts, the place waits on a retry of review for an answer with the words, and its goings-back are none (23, 131)

## ADDED Requirements

### Requirement: The retry that answers a found

A caller's retry of a stage that ended short having found work and
waits on an answer SHALL start it with a fresh agent whose prompt
carries the findings with the answer, and SHALL count no going-back
(42, 48, 49).

#### Scenario: The retry that answers a fix's found

- **WHEN** a fix's code agent stopped short having found work, and the caller retries code with an answer
- **THEN** code starts once with a fresh agent whose prompt holds the findings and then the answer, the goings-back stay none, and once code delivers the place is submitted and lands (42, 48, 49)
