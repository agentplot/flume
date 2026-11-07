# caller-acts Specification

## Purpose
What a caller does to a place after its launch: retry a stage or the
submission, stop its agent, drop it, declare it done, finish a
conversation, tell its agent, and release what its launch held, each as
Part A says and none ending an agent that is not quiet unless forced
(I2, I6; 47 to 55, 109, 114).

## Requirements

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

### Requirement: A retry of a place whose work is over

A retry SHALL be refused, naming the place, once the place's work is
over (49, 50).

#### Scenario: A retry of a landed place

- **WHEN** a retry names a stage of a place whose work landed
- **THEN** it is refused naming the place (S21)

### Requirement: The caller's hold

A stage or a submission its launch holds SHALL start only on a retry
that names it, and flume SHALL start nothing on hold of its own accord
(47, 70).

#### Scenario: Two held places

- **WHEN** a caller launches two places with every stage after construct and each submission on hold, and later retries one place's code stage, then its submission
- **THEN** only that place's code stage starts, then its submission is made, and no third place is made (S7)

### Requirement: Stop

A stop SHALL end the place's agent and nothing else: the stage neither
ends nor moves on, and waits for a retry or a drop. An agent that is not
quiet SHALL be stopped only when forced, and an end the agent did not
take SHALL be reported as a failure (31, 51).

#### Scenario: A working agent stopped

- **WHEN** a caller stops a working agent unforced, then forced
- **THEN** the first stop is refused, the forced one ends the agent, and the stage waits for a retry neither ended nor moved on (S20)

### Requirement: Drop and done

A drop SHALL make a place's work over, end its agent, one not quiet only
when forced, and start nothing more (52). A done SHALL do the same for
a place whose kind does not land, and SHALL be refused, naming the
place, for one whose kind lands (114).

#### Scenario: Done on a fix

- **WHEN** a caller declares a fix place done
- **THEN** it is refused naming the place, and a drop of it is taken (S26)

#### Scenario: A drop that keeps the branch

- **WHEN** a caller drops a fix whose branch holds a commit its base lacks, its agent quiet
- **THEN** the agent is ended, the place removed through the place seam, and the branch kept and named in the answer (S9)

### Requirement: Finish

A finish SHALL end a conversation's agent once it is quiet, one not
quiet only when forced, and the conversation SHALL then count as
delivered (109).

#### Scenario: Finish while the agent works

- **WHEN** a caller finishes a conversation whose agent is working, unforced
- **THEN** the finish is refused and the stage goes on

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

### Requirement: A retry of a lost conversation waits while its session runs

A retry of a conversation's stage reported lost SHALL be taken at once
and SHALL end no agent of that stage: an agent standing in its lost
start's pane, and any process running its session, are left as they
are. The retry SHALL mark the stage's start to carry the lost start's
conversation and SHALL answer that the stage waits for its conversation
to end, naming each process, where one runs it (49, 50, 77, 207).

#### Scenario: A bare agent at work in a lost talk's pane

- **WHEN** the fake terminal brought a lost talk's agent back bare and reads it working, its session scripted live, and the caller retries talk unforced
- **THEN** the retry is taken, its answer says talk waits for its conversation to end naming the process, and the fake terminal's log shows no close and no send (49, 77, 207)

#### Scenario: A session run in a pane flume did not make

- **WHEN** a lost talk's session runs in a pane of a caller's own, scripted live, and the caller retries talk forced
- **THEN** the retry is taken, talk waits, and flume closes nothing, forced or not (49, 207; core.18)

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

#### Scenario: An answer taken before its session ends

- **WHEN** a code agent kept open takes a tell's answer, commits and ends its session, and a settle reads it let go, `agent_ended`, before the tell reads the answer received
- **THEN** the stop-short's end holds the answer with `tell`, and code, carried on under its first start, reads delivered from its commit (42, 96, 121)

#### Scenario: A repeating stage's answer

- **WHEN** the third repetition of change@2's code stops short needing an answer, its agent kept open, and its caller retries code with words
- **THEN** the same agent receives the question and the answer, and code carries on its third group under the same start, never its first group with a task unticked (142, 198)

#### Scenario: An answer the agent does not take

- **WHEN** an answer is given while the asking agent is asking a person in its pane, until the tell's bound passes
- **THEN** it is refused with nothing typed, nothing is recorded, and the stage still waits on its answer (53, 54)

#### Scenario: An answer after the time passed

- **WHEN** a retry with words comes once the agent's time kept open has passed, before any settle read it let go
- **THEN** the agent reads let go, it is ended once quiet, and a fresh agent starts whose prompt carries the question and the answer (42, 49, 198)
