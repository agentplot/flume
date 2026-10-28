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
none, with a fresh agent and the retry's words, the stages after it
then running again in order. A retry naming the submission SHALL submit
the place's head again and start no agent. A retry SHALL be refused,
naming the place, once the place's work is over (49, 50).

#### Scenario: A retry with words

- **WHEN** a construct ended short and its caller retries it with words
- **THEN** a fresh agent starts with the words in its prompt (S4)

#### Scenario: A retry of a landed place

- **WHEN** a retry names a stage of a place whose work landed
- **THEN** it is refused naming the place (S21)

#### Scenario: A retry of the first stage after readiness failed

- **WHEN** a change's readiness failed and its caller retries construct
- **THEN** readiness runs again as its launch read it, and construct starts once it passes (12)

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
is asking, or it is not delivered within its bound, ten minutes where
the caller gives none. A tell SHALL never start, end, hold or move a
stage (53 to 55).

#### Scenario: A tell into a question, then into work

- **WHEN** a caller tells an agent that is asking, then tells it again while it works
- **THEN** the first tell is refused with nothing sent, the second reaches the agent once it waits, and the stage is unchanged (S6)
