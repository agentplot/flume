## MODIFIED Requirements

### Requirement: Retry

A retry SHALL end the place's agent, one that is not quiet only when
forced, then start the stage it names, the current one where it names
none, with a fresh agent whose prompt carries the question of the
stop-short the stage ended with before the retry's words, the stages
after it then running again in order as the host's room allows, but
where it answers an open session. A retry naming the submission SHALL
submit the place's head again and start no agent (42, 49, 50, 70).

#### Scenario: A retry with words

- **WHEN** a construct ended short needing an answer, its agent's session closed and its agent ended, and its caller retries it with the answer as its words
- **THEN** construct's second start is a fresh agent whose prompt carries the question, then the answer (S4, 42, 49)

#### Scenario: A retry with no words while the session is open

- **WHEN** a construct ended short needing an answer, its agent's session open and the agent quiet, and its caller retries it with no words
- **THEN** the asking agent is ended, and a fresh agent starts whose prompt carries the question alone (49, 50)

#### Scenario: A retry of the first stage after readiness failed

- **WHEN** a change's readiness failed and its caller retries construct
- **THEN** readiness runs again as its launch read it, and construct starts once it passes (12)

### Requirement: Tell

A tell SHALL hand its words to the current stage's agent as a prompt
once it is waiting, received or sent again as a prompt is, and SHALL be
refused with nothing typed when the place has no live agent, its agent
is asking, its stage ended short needing an answer and its agent's
session is closed, or it is not delivered within its bound, ten minutes
where the caller gives none. A tell SHALL never start, end, hold or
move a stage but as an answer (53 to 55).

#### Scenario: A tell into a question, then into work

- **WHEN** a caller tells an agent that is asking, then tells it again while it works
- **THEN** the first tell is refused with nothing sent, the second reaches the agent once it waits, and the stage is unchanged (S6)

#### Scenario: A tell once the session has closed

- **WHEN** a caller tells a place whose code ended short needing an answer and whose agent's session has closed
- **THEN** the tell is refused with nothing typed, naming the retry that answers the stage (42, 53)

## ADDED Requirements

### Requirement: An answer reaches the open session

A retry with words naming a stage ended short needing an answer, or a
tell to its place, while that stage's agent's session is open, SHALL be
its answer: the core SHALL hand the question and the answer to that
agent as a tell is handed, SHALL record both with the stage's start,
and SHALL carry the stage on as begun under the same start, starting
and ending no agent and counting no start (42, 48 to 50, 53 to 55).

#### Scenario: A retry's answer

- **WHEN** a code agent ended short needing an answer, "Which file?", its session open, and its caller retries code with the words "notes.md"
- **THEN** the same agent receives "The question: Which file?" then "The answer: notes.md" as one prompt, code reads begun under its first start and started once, and that start's answers hold the question, the answer, `retry`, and when it was sent and received (S4, 42, 49, 50)

#### Scenario: A tell's answer

- **WHEN** a caller tells "notes.md" to a place whose code ended short needing an answer, its session open
- **THEN** the same agent receives the question then the answer, code reads begun under its first start, and that start's answers hold them with `tell` (42, 53, 55)

#### Scenario: An answer the agent does not take

- **WHEN** an answer is given while the asking agent is asking a person in its pane, until the tell's bound passes
- **THEN** it is refused with nothing typed, nothing is recorded, and the stage still waits on its answer (53, 54)

#### Scenario: An answer after the time passed

- **WHEN** a retry with words comes once the session's time has passed, before any settle read the session closed
- **THEN** the session reads closed, the asking agent is ended once quiet, and a fresh agent starts whose prompt carries the question and the answer (41, 42, 49)
