## MODIFIED Requirements

### Requirement: Retry

A retry SHALL end the place's agent, one that is not quiet only when
forced, then start the stage it names, the current one where it names
none, with a fresh agent, its prompt's words carrying the question of
the stop-short the stage ended with before the retry's words, the stages
after it then running again in order, the host's room allowing, but
where it answers an agent kept open. A retry naming the submission
SHALL submit the place's head again and start no agent (42, 49, 50, 70,
198).

#### Scenario: A retry with words

- **WHEN** a construct ended short needing an answer, its agent let go and ended, and its caller retries it with the answer as its words
- **THEN** construct's second start is a fresh agent whose prompt carries the question, then the answer (S4, 42, 49)

#### Scenario: A retry with no words while the agent is kept open

- **WHEN** a construct ended short needing an answer, its agent kept open and quiet, and its caller retries it with no words
- **THEN** the asking agent is ended, and a fresh agent starts whose prompt carries the question alone (49, 50)

#### Scenario: A retry of the first stage after readiness failed

- **WHEN** a change's readiness failed and its caller retries construct
- **THEN** readiness runs again as its launch read it, and construct starts once it passes (12)

### Requirement: Tell

A tell SHALL hand its words to the current stage's agent as a prompt
once it is waiting, received or sent again as a prompt is, and SHALL be
refused with nothing typed when the place has no live agent, its agent
is asking, its stage ended short needing an answer and its agent is not
kept open, or it is not delivered within its bound, ten minutes where
the caller gives none. A tell SHALL never start, end, hold or move a
stage, but as an answer to an agent kept open (53 to 55, 198).

#### Scenario: A tell into a question, then into work

- **WHEN** a caller tells an agent that is asking, then tells it again while it works
- **THEN** the first tell is refused with nothing sent, the second reaches the agent once it waits, and the stage is unchanged (S6)

#### Scenario: A tell once the agent is let go

- **WHEN** a caller tells a place whose code ended short needing an answer and whose agent was let go
- **THEN** the tell is refused with nothing typed, naming the retry that answers the stage (42, 53, S104)

## ADDED Requirements

### Requirement: An answer reaches the agent kept open

A retry with words naming a stage ended short needing an answer, or a
tell to its place, while its agent is kept open, SHALL be its answer:
the core SHALL hand that agent the question and the answer as a tell
is handed, record the answer on the stop-short it answers with its act
and when it was sent and received, end each hold on the stage, and
carry the stage on as begun under the same start, starting and ending
no agent and counting no start (42, 47 to 50, 53 to 55, 198).

#### Scenario: A retry's answer

- **WHEN** a code agent ended short needing an answer, "Which file?", its agent kept open, and its caller retries code with the words "notes.md"
- **THEN** the same agent receives "The question: Which file?" then "The answer: notes.md" as one prompt, code reads begun under its first start and started once, and the stop-short's end holds the answer "notes.md" with `retry` and when it was sent and received (S4, 42, 49, 50)

#### Scenario: A tell's answer

- **WHEN** a caller tells "notes.md" to a place whose code ended short needing an answer, its agent kept open
- **THEN** the same agent receives the question then the answer, code reads begun under its first start, and the stop-short's end holds the answer with `tell` (42, 53, 55)

#### Scenario: A repeating stage's answer

- **WHEN** the third repetition of change@2's code stops short needing an answer, its agent kept open, and its caller retries code with words
- **THEN** the same agent receives the question and the answer, and code carries on its third group under the same start, never its first group with a task unticked (142, 198)

#### Scenario: An answer the agent does not take

- **WHEN** an answer is given while the asking agent is asking a person in its pane, until the tell's bound passes
- **THEN** it is refused with nothing typed, nothing is recorded, and the stage still waits on its answer (53, 54)

#### Scenario: An answer after the time passed

- **WHEN** a retry with words comes once the agent's time kept open has passed, before any settle read it let go
- **THEN** the agent reads let go, it is ended once quiet, and a fresh agent starts whose prompt carries the question and the answer (42, 49, 198)
