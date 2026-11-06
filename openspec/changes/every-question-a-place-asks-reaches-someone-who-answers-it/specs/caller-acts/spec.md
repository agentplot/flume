## ADDED Requirements

### Requirement: An answer names the stop-short it answers by its key

A retry, or a tell that answers an agent kept open (198), naming a
stop-short's key by `--answers <key>` SHALL be taken only while the
place waits on that stop-short, and SHALL record on its end the
answer's words, who gave them, as the answering command's recipient
names it, and when. Otherwise it SHALL be refused, naming the answer
taken, who gave it and when, or the move the place waits on now, and
SHALL end, start and type nothing. An answer with no key SHALL be taken
as before (42, 49, 53).

#### Scenario: One answer taken by a fresh agent

- **WHEN** a construct ended short needing an answer under the key `stage/construct/1/ended-short`, naming `east/ops`, on a host that keeps no agent open, and the run of `east/ops` retries construct with that key and its words
- **THEN** a fresh agent starts whose prompt carries the question, then the words, and status gives the end answered by `east/ops`, with when

#### Scenario: One answer taken by the agent kept open

- **WHEN** the same stop-short's agent is kept open for its answer, and the run of `east/ops` retries construct with that key and its words within its time
- **THEN** the answer reaches that agent's session, no agent is ended or started, the stage carries on under its first start, and status gives the end answered by `east/ops`, with when (S93)

#### Scenario: A second answer to the same question

- **WHEN** a retry from the pane `ops` names the same key a minute later
- **THEN** it is refused naming `east/ops`, its words and when, and no agent is ended or started (S93)

#### Scenario: A tell naming an answered key

- **WHEN** a tell from the pane `ops` names the same key with its words, while the agent is still kept open
- **THEN** it is refused naming `east/ops`, its words and when, and nothing is typed

#### Scenario: An answer after the place moved on

- **WHEN** construct, retried with an answer, delivers, code starts, and a late retry names construct with the first stop-short's key
- **THEN** it is refused naming the move the place waits on now, none, and code goes on

#### Scenario: A second stop-short of one start

- **WHEN** construct's agent, kept open, takes an answer in its session and then stops short again
- **THEN** the second stop-short's key is `stage/construct/1/ended-short/1`, a retry naming it is taken, and one naming `stage/construct/1/ended-short` is refused as answered

#### Scenario: An answer with no key

- **WHEN** a caller retries a stage that ended short with words and no key
- **THEN** it is taken as before, and the end records the answer with no key named
