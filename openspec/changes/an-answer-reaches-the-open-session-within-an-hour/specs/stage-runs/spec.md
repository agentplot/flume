## MODIFIED Requirements

### Requirement: A stop-short ends the stage short

The core SHALL take a stop-short only under the identity of the place's
current stage's live agent and that start's secret, saying whether it
needs an answer or found work, SHALL end the stage short with the
agent's words whole and its kind, and SHALL start no next stage, except
that found work from the kind's last stage starts the kind's fix round.
A stop-short after an answer SHALL be an end of its own (41, 42, 131).

#### Scenario: A construct stops short

- **WHEN** a construct's agent stops short needing an answer, with its words
- **THEN** the stage ends short, no code stage starts, and status gives the words whole and that it needs an answer (S4)

#### Scenario: A stop-short with another start's secret

- **WHEN** a stop-short carries the place's identity and the secret of an earlier start
- **THEN** it is refused and the stage goes on (41)

#### Scenario: A second stop-short in one session

- **WHEN** a code agent answered in its session stops short again with other words
- **THEN** a second end of code's first start is recorded and delivered once to each recipient, the first staying delivered, and its session is open for `keep_open` from the second stop-short (41, 43, 74)

### Requirement: Stuck, lost and working past its limit

A stage SHALL read stuck once its agent has been quiet past its limit,
30 minutes where none is declared, with nothing delivered or said; lost
once its agent stopped with nothing delivered or said; and its agent
working past its limit when it works that long with nothing delivered,
from the latest of the stage's beginning, its last answer's receipt and
the place's last head move. The core SHALL end no agent on any of these
accounts (44, 45, 118).

#### Scenario: Stuck under a program with no send-back

- **WHEN** the fake agent gives no way to send back and stays quiet past the limit with nothing delivered
- **THEN** status reports the stage stuck naming it and the agent, the agent is not ended, and after a tell wakes it and it ticks the last task the stage ends at its deliverable (S17)

#### Scenario: Limits after an answer

- **WHEN** a code stage began two hours before its agent received an answer in its session, and the agent then works 20 minutes with nothing delivered under a 30-minute limit
- **THEN** it is not read working past its limit, and it is read so once it has worked 30 minutes since the answer (44, 118)

## ADDED Requirements

### Requirement: An asking agent is kept open for its answer

Where the host's terminal can hand a running agent words, the core
SHALL keep the agent of a stage ended short needing an answer live,
its session open for the answer until the stop-short's time plus its
settings' `keep_open`, one hour where none gives one, and SHALL record
that time with the stage's end; where the terminal cannot, no session
SHALL be open (32, 41, 42, 53).

#### Scenario: An hour by default

- **WHEN** a code agent stops short needing an answer at a time t, on a terminal that hands a running agent words, with no layer giving `keep_open`
- **THEN** its agent stays live, and its end's session is open until t plus one hour (41)

#### Scenario: A host's time

- **WHEN** the host gives `[kinds.fix.agents] keep_open = "10m"`, and a fix's code agent stops short needing an answer
- **THEN** its session is open until ten minutes after the stop-short (32, 41)

#### Scenario: No session under a terminal that hands no words

- **WHEN** the fake terminal hands no words to a running agent, and a code agent stops short needing an answer
- **THEN** its end holds no session open, and its agent is left as before the stop-short (41, C.5)

#### Scenario: Found work from a stage before the last

- **WHEN** a construct agent stops short having found work, construct not the kind's last stage
- **THEN** its session is open for an answer, as for a question (41, 131)

### Requirement: A closed session's agent is ended

The core SHALL read an open session closed at a settle once its time
has passed, its agent is read stopped or its session ended, or its
account is spent, recording when and why; SHALL then end the agent once
it is quiet, never forced, freeing its room and its account; and the
stage SHALL keep waiting on its answer, a later answer being a retry
with a fresh agent (31, 41, 42, 70, 96, 134, 146).

#### Scenario: The time passing

- **WHEN** an hour passes on the fake clock after a code agent stopped short needing an answer, the agent quiet, with another place's stage waiting for room on a host at its bound, and a settle runs
- **THEN** the session reads closed by its time, the agent is ended through the terminal, code still reads ended short waiting on its answer, and the other place's stage starts (41, 70, S76)

#### Scenario: Not quiet once its time has passed

- **WHEN** the asking agent works on once its time has passed
- **THEN** the session reads closed, and the agent is not ended until a settle reads it quiet (31, 41)

#### Scenario: The agent dying within its time

- **WHEN** the asking agent's pane is gone within its hour
- **THEN** the next settle records the session closed with its agent ended, and the stage waits on its answer with no agent (41, 96, S77)

#### Scenario: The account spent within its time

- **WHEN** another agent of the asking agent's account reaches the account's usage limit within the hour
- **THEN** the next settle records the session closed by its account spent, and ends the asking agent once it is quiet (41, 146, S77)

#### Scenario: A late answer

- **WHEN** a code agent's session closed by its time and the agent was ended, and its caller retries code with "notes.md"
- **THEN** code's second start is a fresh agent whose prompt carries the question, then the answer (42, 49, S76)

#### Scenario: Two settles at once

- **WHEN** two settles read a session closed at once
- **THEN** it is recorded closed once, and its agent is closed once (74, 75)

### Requirement: A fresh start carries what its stage was told

A fresh start the core makes on its own of a stage, after a spent
account or a start begun without its settings, SHALL carry after
flume's own words the words its earlier start was given and each answer
that start took in its session, in order (42, 146, 161).

#### Scenario: Spent after an answer

- **WHEN** a code agent answered "notes.md" in its session then stops at its account's usage limit
- **THEN** code's fresh start on the pool's next account carries flume's words, then the question and the answer (42, 146)
