## REMOVED Requirements

### Requirement: Tell

**Reason**: A message replaces a tell, to a place's agent or to a role's
address, and reaches a place's agent by its agent profile's deliver
part where its program offers one (53 to 55, 214, 215; record 0069).

**Migration**: "A message to a place's agent", "A message to a role's
address" and "A message's target" below; `flume tell` runs as `flume
message`.

## ADDED Requirements

### Requirement: A message to a place's agent

A message naming a place SHALL hand its words to the current stage's
agent as a prompt once it waits, received or sent again as a prompt is,
and SHALL be refused with nothing typed when the place has no live
agent, its agent asks, its stage waits on an answer its agent is not
kept open for, or the bound passes, ten minutes where none is given. It
SHALL move no stage, but as an answer to an agent kept open (53 to 55,
198).

#### Scenario: A message into a question, then into work

- **WHEN** a caller messages an agent that is asking, then messages it again while it works
- **THEN** the first is refused with nothing sent, the second reaches the agent once it waits, and the stage is unchanged (S6)

#### Scenario: A message once the agent is let go

- **WHEN** a caller messages a place whose code ended short needing an answer and whose agent was let go
- **THEN** the message is refused with nothing typed, naming the retry that answers the stage (42, 53, S104)

### Requirement: A message reaches a place's agent by its deliver part

A message to a place whose agent's profile offers deliver SHALL wait on
the agent's start for its take, within its bound, and SHALL send
nothing through the terminal; one not taken within the bound SHALL be
taken back with nothing typed, and one taken and not received SHALL be
put back for one more take (38, 54, 215).

#### Scenario: A message taken by the deliver part

- **WHEN** the fake agent's profile offers deliver, and a caller messages its place while it works
- **THEN** nothing is sent through the fake terminal, the message waits on the start untaken, and once the agent waits and takes it, it is read received and the message answers acted (54, 215)

#### Scenario: A message the deliver part never takes

- **WHEN** the fake agent's profile offers deliver, a caller messages its place with a bound of a minute, and the agent works past it
- **THEN** the message is refused with nothing typed, the start holds no message waiting, and a take after the bound gets nothing (54)

#### Scenario: A message taken and not received

- **WHEN** the fake agent's profile offers deliver, takes a message and reads it not received within 60 seconds, then takes it again and reads it not received again
- **THEN** the message is refused naming that the agent did not receive the words, and it was taken twice (38, 54)

### Requirement: A message to a role's address

A message naming a role's address that one source of the host declares,
with a run, a standing agent or carried, SHALL be recorded at once as a
delivery to that address, keyed `message/<stream>/<role>/<token>`, its
words after a first line naming who sent it; SHALL wake the address's
run where its source gives it a run and none is live or waiting for
room; and SHALL answer with its key, the address and what it found,
waiting on no bound (53, 132, 193, 214, C.7).

#### Scenario: A message to a role with no run live

- **WHEN** the pane `ops` messages `east/planner`, whose source gives it a run and none is live, "the nightly job failed"
- **THEN** the answer gives a key `message/east/planner/<token>`, the address and its run woken; the host's deliveries hold one delivery under that key whose one reader is `east/planner` and whose message begins `ops messages east/planner:` with the words after it; and one run of the address is launched (132, 214)

### Requirement: A message no address can take is refused

A message naming an address no source declares, two declare, or whose
stream the host does not declare SHALL be refused naming why, recording
nothing; and one naming an address with `--within` or `--answers` SHALL
be refused naming that a bound and an answer are a place's (42, 53,
132, 193).

#### Scenario: A message to an address no source declares

- **WHEN** a caller messages `east/nobody`, which no source declares
- **THEN** it is refused naming that no source declares it, and the host's deliveries are unchanged (132)

#### Scenario: A bound with an address

- **WHEN** a caller messages `east/planner` with `--within 1m`
- **THEN** it is refused naming that a bound is how long a message waits for a place's agent, and nothing is recorded

### Requirement: A message's target

A message's target SHALL be a role's address where it is
`<stream>/<role>`, each part a word and neither `.` nor `..`, and
otherwise SHALL name a place as every command names one (53, 132).

#### Scenario: An address and a place

- **WHEN** `flume message east/planner "words"` and `flume message fix--one "words"` run against the fakes, and `./east/planner` names a place's location
- **THEN** the first is a delivery at `east/planner`, the second a message to `fix--one`'s agent, and `flume message ./east/planner "words"` messages that place

## MODIFIED Requirements

### Requirement: An answer reaches the agent kept open

A retry with words naming a stage ended short needing an answer, or a
message to its place, while its agent is kept open, SHALL be its
answer: the core SHALL hand that agent the question and the answer as a
message is handed, record the answer on the stop-short it answers with
its act and when it was sent and received, end each hold on the stage,
and carry the stage on as begun under the same start, starting and
ending no agent and counting no start (42, 47 to 50, 53 to 55, 198).

#### Scenario: A retry's answer

- **WHEN** a code agent ended short needing an answer, "Which file?", its agent kept open, and its caller retries code with the words "notes.md"
- **THEN** the same agent receives "The question: Which file?" then "The answer: notes.md" as one prompt, code reads begun under its first start and started once, and the stop-short's end holds the answer "notes.md" with `retry` and when it was sent and received (S4, 42, 49, 50)

#### Scenario: A tell's answer

- **WHEN** a caller runs `flume tell`, message's alias, with "notes.md" to a place whose code ended short needing an answer, its agent kept open
- **THEN** the same agent receives the question then the answer, code reads begun under its first start, and the stop-short's end holds the answer with `message` (42, 53, 55)

#### Scenario: A message's answer

- **WHEN** a caller messages "notes.md" to a place whose code ended short needing an answer, its agent kept open
- **THEN** the same agent receives the question then the answer, code reads begun under its first start, and the stop-short's end holds the answer with `message` (42, 53, 55)

#### Scenario: An answer taken before its session ends

- **WHEN** a code agent kept open takes a message's answer, commits and ends its session, and a settle reads it let go, `agent_ended`, before the message reads the answer received
- **THEN** the stop-short's end holds the answer with `message`, and code, carried on under its first start, reads delivered from its commit (42, 96, 121)

#### Scenario: A repeating stage's answer

- **WHEN** the third repetition of change@2's code stops short needing an answer, its agent kept open, and its caller retries code with words
- **THEN** the same agent receives the question and the answer, and code carries on its third group under the same start, never its first group with a task unticked (142, 198)

#### Scenario: An answer the agent does not take

- **WHEN** an answer is given while the asking agent is asking a person in its pane, until the message's bound passes
- **THEN** it is refused with nothing typed, nothing is recorded, and the stage still waits on its answer (53, 54)

#### Scenario: An answer after the time passed

- **WHEN** a retry with words comes once the agent's time kept open has passed, before any settle read it let go
- **THEN** the agent reads let go, it is ended once quiet, and a fresh agent starts whose prompt carries the question and the answer (42, 49, 198)

### Requirement: An answer names the stop-short it answers by its key

A retry, or a message that answers an agent kept open (198), naming a
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

- **WHEN** a message from the pane `ops`, by `flume tell` or `flume message`, names the same key with its words, while the agent is still kept open
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
