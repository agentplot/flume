## MODIFIED Requirements

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
