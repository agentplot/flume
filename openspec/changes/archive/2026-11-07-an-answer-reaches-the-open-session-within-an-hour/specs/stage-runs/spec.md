## MODIFIED Requirements

### Requirement: A stop-short ends the stage short

The core SHALL take a stop-short only under the identity of the place's
current stage's live agent and that start's secret, saying whether it
needs an answer or found work, SHALL end the stage short with the
agent's words whole and its kind, and SHALL start no next stage, except
that found work from the kind's last stage starts the kind's fix round.
A stop-short after an answer its start took in its session SHALL be an
end of its own, keyed apart from the one answered (41, 42, 43, 131,
198).

#### Scenario: A construct stops short

- **WHEN** a construct's agent stops short needing an answer, with its words
- **THEN** the stage ends short, no code stage starts, and status gives the words whole and that it needs an answer (S4)

#### Scenario: A stop-short with another start's secret

- **WHEN** a stop-short carries the place's identity and the secret of an earlier start
- **THEN** it is refused and the stage goes on (41)

#### Scenario: A second stop-short in one session

- **WHEN** a code agent answered in its session stops short again with other words
- **THEN** a second end of code's first start is recorded under its own key and delivered once to each recipient, the first staying delivered, and the agent is kept open for `keep_open` from the second stop-short (43, 74, 198)

### Requirement: Stuck, lost and working past its limit

A stage SHALL read stuck once its agent has been quiet past its limit,
30 minutes where none is declared, with nothing delivered or said; lost
once its agent stopped with nothing delivered or said; and its agent
working past its limit when it works that long with nothing delivered,
from the latest of the stage's beginning, its last answer's receipt in
its session and the place's last head move. The core SHALL end no agent
on any of these accounts (44, 45, 118, 198).

#### Scenario: Stuck under a program with no send-back

- **WHEN** the fake agent gives no way to send back and stays quiet past the limit with nothing delivered
- **THEN** status reports the stage stuck naming it and the agent, the agent is not ended, and after a tell wakes it and it ticks the last task the stage ends at its deliverable (S17)

#### Scenario: Limits after an answer

- **WHEN** a code stage began two hours before its agent received an answer in its session, and the agent then works 20 minutes with nothing delivered under a 30-minute limit
- **THEN** it is not read working past its limit, and it is read so once it has worked 30 minutes since the answer (44, 118)

## ADDED Requirements

### Requirement: An asking agent is kept open for its answer

Where the terminal of its start hands a running agent words, the core
SHALL keep the agent of a stage ended short needing an answer live and
kept open for that answer until the stop-short's time plus its
settings' `keep_open`, one hour where no layer gives one, and SHALL
record that time on the stage's end; where the terminal does not, the
core SHALL keep no agent open (32, 42, 53, 95, 198).

#### Scenario: An hour by default

- **WHEN** a code agent stops short needing an answer at a time t, on a terminal that hands a running agent words, with no layer giving `keep_open`
- **THEN** its agent stays live, and its end records it kept open until t plus one hour (198)

#### Scenario: A host's time for a stream

- **WHEN** the host gives `[agents.streams.work] keep_open = "10m"`, and a code agent of a place in the stream `work` stops short needing an answer
- **THEN** its agent is kept open until ten minutes after the stop-short (32, 198, S104)

#### Scenario: Kept open for no time

- **WHEN** the launch gives `--agent keep_open=0`, and code's agent stops short needing an answer and goes quiet
- **THEN** the settle its going quiet runs lets it go by its time and ends it, and code waits on its answer (198)

#### Scenario: Not kept open under a terminal that hands no words

- **WHEN** the fake terminal hands no words to a running agent, and a code agent stops short needing an answer
- **THEN** its end holds no time kept open, and its agent is left as a stop-short leaves it under that terminal today (198, C.5)

#### Scenario: Found work from a stage before the last

- **WHEN** a construct agent stops short having found work, construct not the kind's last stage
- **THEN** its agent is kept open for an answer, as for a question (131, 198)

### Requirement: An agent let go is ended

The core SHALL read an agent kept open let go at a settle once its time
has passed, it is read stopped or its session ended, or its account is
spent, recording when and why; SHALL then end the agent once it is
quiet, never forced, freeing its room and its account; and the stage
SHALL keep waiting on its answer under the same move and since, a later
answer being a retry with a fresh agent (31, 42, 70, 96, 130, 134, 146,
198).

#### Scenario: The time passing

- **WHEN** an hour passes on the fake clock after a code agent stopped short needing an answer, the agent quiet, with another place's stage waiting for room on a host at its bound, and a settle runs
- **THEN** the agent reads let go by its time and is ended through the terminal, code still reads ended short waiting on its answer since the stop-short, and the other place's stage starts (70, 130, 198, S104)

#### Scenario: Not quiet once its time has passed

- **WHEN** the asking agent works on once its time has passed
- **THEN** it reads let go, and it is not ended until a settle reads it quiet (31, 198)

#### Scenario: The agent dying within its time

- **WHEN** the asking agent's pane is gone within its hour
- **THEN** the next settle records it let go, ended, and the stage waits on its answer with no agent (96, 198, S105)

#### Scenario: The account spent within its time

- **WHEN** another agent of the asking agent's account reaches the account's usage limit within the hour
- **THEN** the next settle records the agent let go by its account spent, and ends it once it is quiet (146, 198, S105)

#### Scenario: Past its pool's threshold within its time

- **WHEN** the asking agent's account passes its pool's threshold within the hour, and is not spent
- **THEN** the agent stays kept open (185, 198)

#### Scenario: A late answer

- **WHEN** a code agent was let go by its time and ended, and its caller retries code with "notes.md"
- **THEN** code's second start is a fresh agent whose prompt carries the question, then the answer (42, 49, S104)

#### Scenario: Two settles at once

- **WHEN** two settles read an agent let go at once
- **THEN** it is recorded let go once, and its agent is closed once (74, 75)

### Requirement: A conversation's asking agent lost is carried at its answer

The core SHALL read an agent kept open let go `agent_ended` where the
agent in its pane is not its start's, as a restart leaves one brought
back bare, leaving that agent in a conversation's pane and ending it
once quiet elsewhere, and `stopped` where a caller's stop ended it; and
a retry with words of a conversation's stage so let go `agent_ended`
SHALL carry its conversation as a retry of one reported lost does, the
question and the answer its words (45, 49, 51, 77, 179, 198, 207).

#### Scenario: Brought back bare in a conversation's pane

- **WHEN** a research talk's agent, kept open for its answer, is brought back bare in its pane by the terminal after a restart, and a settle runs
- **THEN** it reads let go, `agent_ended`, the agent left in its pane, sent nothing and not ended, and talk still waits on its answer (77, 198; core.10)

#### Scenario: Brought back bare in another stage

- **WHEN** a code agent kept open for its answer is brought back bare in its pane after a restart, quiet, and a settle runs
- **THEN** it reads let go, `agent_ended`, and the bare agent is ended (77, 198, S116)

#### Scenario: The answer carries the conversation

- **WHEN** the caller retries that talk with "notes.md" while the bare agent runs its conversation, and the bare agent then ends
- **THEN** talk waits while the bare agent runs, status naming its process, and the next settle after it ends starts talk resuming the session on the account its lost start ran under, its prompt flume's words then the question and the answer (49, 179, 198, 207)

#### Scenario: Stopped by its caller

- **WHEN** a caller stops a talk's agent kept open, and then retries talk with "notes.md"
- **THEN** the next settle reads it let go, `stopped`, and the retry starts a fresh agent carrying no conversation, its prompt carrying the question and the answer (49, 51, 198)

### Requirement: A fresh start carries what its stage was told

A fresh start the core makes on its own of a stage, after a spent
account its program carries no conversation from or after a start
begun without its settings, SHALL carry after flume's own words the
words its earlier start was given and each answer that start took in
its session, in order (42, 146, 161).

#### Scenario: Spent after an answer

- **WHEN** a code agent answered "notes.md" in its session then stops at its account's usage limit, its program carrying no conversation
- **THEN** code's fresh start on the pool's next account carries flume's words, then the question and the answer (42, 146)

#### Scenario: Unsettled after a retry

- **WHEN** code's second start, its words a question and its answer, begins without the settings it recorded
- **THEN** the fresh start after it carries flume's words, then the question and the answer (42, 161)
