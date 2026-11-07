# stage-runs Specification

## Purpose
How one stage runs: one fresh agent under an identity of its own, its
prompt filled and delivered, and its end read from its deliverable, its
agent's stop-short or a conversation's finish, and never from a quiet
terminal (I1, I2, I5; 24 to 45, 107 to 110, 118).

## Requirements

### Requirement: One fresh agent per stage

Each start of a stage SHALL start exactly one fresh agent through the
agent seam's command and the terminal seam's open, under an identity
naming its repository, place, stage and start, and with a secret of
that start's alone. No agent SHALL start while another of the place's
is live (30, 33, 41).

#### Scenario: A second start while an agent is live

- **WHEN** a settle and a retry both try to start a stage of one place at once over one fake store
- **THEN** one agent is opened, and the fake terminal's log holds one open (30, 75)

#### Scenario: Identities never shared

- **WHEN** a stage is started three times
- **THEN** each start's identity names its start number and none equals another (33)

### Requirement: The prompt is filled before it is sent

A stage's prompt SHALL be rendered from the launch, the place and any
retry's words, with every part it asks for filled and the agent seam's
one way to stop short in it (27, 28). A part with nothing to fill it
SHALL leave the stage refused, naming the part, with no agent started.
Rendering SHALL start nothing (29).

#### Scenario: A retry's words asked for and missing

- **WHEN** a stage whose prompt asks for a retry's words is reached with none
- **THEN** it stands refused naming the part and starts no agent, and a retry with words starts it with them in its prompt (S32)

#### Scenario: The prompt rendered

- **WHEN** a stage's prompt is rendered for a place
- **THEN** the text holds the brief whole as the caller wrote it and the way to stop short, and no fake records a call (5, 29)

### Requirement: A prompt is received or reported never begun

The core SHALL send a prompt only once its agent is waiting and not
asking, SHALL read within 60 seconds of each sending whether it was
received, and SHALL send once more where the terminal can send, and
report the stage never begun when no sending is received (37, 38).

#### Scenario: A slow start and a second sending

- **WHEN** the fake agent takes four minutes to come up, then misses the first sending
- **THEN** its start reads unknown until it waits, the prompt is sent once more, it is received, and the stage reads begun (S5, 35)

#### Scenario: A terminal that cannot send

- **WHEN** the fake terminal refuses every send and the agent never records the prompt
- **THEN** the start carries the one sending and the stage reads never begun (38)

### Requirement: A stage ends at its deliverable, read at one head

A stage that is not a conversation SHALL end only when its deliverable
exists in its form, made since the stage started, every fact of it read
at one head of the place's branch, or when its agent stops short (24,
25, 39). A delivered stage SHALL end once its agent is quiet (40).

#### Scenario: Waiting on own work

- **WHEN** a code stage's agent ticks the last task, then waits twelve fake minutes on own work it started
- **THEN** the stage is neither ended nor sent back nor stuck until own work ends and the agent is quiet (S3)

#### Scenario: Tasks ticked over four commits

- **WHEN** a code stage's agent ticks two of five tasks, then all five, over four commits
- **THEN** status shows two of five, then five of five, and the stage delivers only at the head that ticks the fifth (S2, 24)

#### Scenario: Facts never paired across heads

- **WHEN** the head moves between the change's read and the newest commit's read in one settle
- **THEN** the settle reads every fact again at the new head and never pairs the two (25)

### Requirement: A stop-short ends the stage short

The core SHALL take a stop-short only under the identity of the place's
current stage's live agent and that start's secret, saying whether it
needs an answer or found work, SHALL end the stage short with the
agent's words whole and its kind, and SHALL start no next stage, except
that found work from the kind's last stage starts the kind's fix round
(41, 42, 131).

#### Scenario: A construct stops short

- **WHEN** a construct's agent stops short needing an answer, with its words
- **THEN** the stage ends short, no code stage starts, and status gives the words whole and that it needs an answer (S4)

#### Scenario: A stop-short with another start's secret

- **WHEN** a stop-short carries the place's identity and the secret of an earlier start
- **THEN** it is refused and the stage goes on (41)

### Requirement: The stop check

When the agent seam asks whether an agent of a stage that is not a
conversation may go quiet with its deliverable missing, nothing said,
no own work and no question open, the core SHALL answer with words
naming the deliverable and the way to stop short (110, 115).

#### Scenario: An agent that would stop with nothing made

- **WHEN** a code stage's fake agent would go quiet with no commit and nothing said
- **THEN** it is sent back with words naming commits and the way to stop short, it commits, and the stage ends at its deliverable (S24)

### Requirement: Stuck, lost and working past its limit

A stage SHALL read stuck once its agent has been quiet past its limit,
30 minutes where none is declared, with nothing delivered or said; lost
once its agent stopped with nothing delivered or said; and its agent
working past its limit when it works that long with nothing delivered.
The core SHALL end no agent on any of these accounts (44, 45, 118).

#### Scenario: Stuck under a program with no send-back

- **WHEN** the fake agent gives no way to send back and stays quiet past the limit with nothing delivered
- **THEN** status reports the stage stuck naming it and the agent, the agent is not ended, and after a tell wakes it and it ticks the last task the stage ends at its deliverable (S17)

### Requirement: A conversation ends on its caller's finish

A conversation SHALL have no quiet limit and no stop check, SHALL never
read stuck, and SHALL end only on its caller's finish, which ends its
agent once quiet and counts the stage delivered (109).

#### Scenario: An hour beside the operator

- **WHEN** a conversation's fake agent goes quiet many times over an hour, then its caller finishes it
- **THEN** it is never sent back nor reported stuck, and the finish ends the quiet agent and the stage reads delivered (S23)

### Requirement: The guard's refusals reach status

Each act the agent seam's guard reports refused SHALL appear in status
for its place, and the stage SHALL go on (108, 116).

#### Scenario: An agent tries to submit

- **WHEN** the fake agent's guard reports a refused submission
- **THEN** status reports the refusal, the fake merge queue records no submission, and the stage goes on (S25)

### Requirement: A start is starting until its agent says it started

The core SHALL hold a start starting until the agent seam's `started`
reads yes, send no prompt meanwhile, and give the terminal's screen with
it in status; it SHALL read the start never begun at once when its
agent stops with no started word, at the stage's limit while it stays
starting, or on its caller's stop or retry (35, 37, 44, 135, 136;
record 0016).

#### Scenario: The trust question, then the started word

- **WHEN** a code stage's agent shows the folder-trust question, flume answers it, and the agent's started word comes
- **THEN** only then is the prompt sent, and the stage begins once it is received (S39)

#### Scenario: Held at a dialog a terminal cannot name

- **WHEN** a code stage's agent stops at a dialog under a terminal whose open cannot read a start held, and nobody answers it
- **THEN** no prompt is sent, status gives the start starting with the dialog's lines, and at the stage's limit it is reported never begun with the screen (35, 44)

### Requirement: A start the terminal reads held or never come up

The core SHALL read a start never begun at once when the terminal's
open answers it held at a question its program asks before it starts,
or its program not come up, delivered with the screen the open gave,
and SHALL send it no prompt (35, 43, 94; record 0018).

#### Scenario: Held at the bypass warning

- **WHEN** a code stage's agent stops at the warning about bypassing permissions, and the terminal's open answers the start held with the warning's lines
- **THEN** no prompt is sent, and the start is reported never begun at once with those lines (S40)

#### Scenario: A program that never comes up

- **WHEN** the terminal's open answers that the stage's program did not come up within its bound
- **THEN** no prompt is sent, and the start is reported never begun at once with the screen the open gave (S45)

### Requirement: The terminal's clues follow each stage

The core SHALL call the terminal's `show` at each settle that changes
the place's current stage's state, or the count of the places of its
repository on its base, with the stage, its state and that count of
places running, asking, waiting on their caller and stalled; a show
that fails SHALL be logged on the host and fail nothing, and nothing
the core decides SHALL read a clue back (139; record 0018).

#### Scenario: Titles follow a stage

- **WHEN** a code stage's agent starts, waits, works and asks, a settle between each
- **THEN** show is called once at each, its state starting, waiting, working and asking, and a show that fails is logged while the settle goes on (S49)

### Requirement: The terminal tidies with the place

The core SHALL call the terminal's `tidy` when a place goes, landed or
dropped, and give what the terminal closed and kept in the tidy's
answer; status SHALL name each thing the terminal keeps for what it did
not make, with what holds it (56, 137, 138; record 0018).

#### Scenario: The last place on a base

- **WHEN** the last place on a repository's base is done and tidied
- **THEN** the terminal closes its tab, then the base's workspace (S47)

#### Scenario: A person's pane

- **WHEN** a person's pane stands in a base's workspace the terminal made and the last place on that base is dropped
- **THEN** the place's tab is closed, the workspace kept, and status's host part names it with the person's pane (S48)

### Requirement: An agent with no started word is no agent of the stage's

The core SHALL take an agent the terminal brought back with no started
word for its start as no agent of the stage's, read the stage lost, and
end that agent once it is quiet (31, 77; record 0016).

#### Scenario: herdr resumes the agent bare

- **WHEN** herdr's server restarts and resumes a working code stage's session with no started word for its start
- **THEN** the stage is read lost, delivered with the agent's last words and the commits since the start, and the agent is ended once quiet (S41)

### Requirement: An end carries what was read

The core SHALL deliver a stuck, lost or never-begun end with the
agent's last words, the commits on the place's branch since the start
and, for never begun, the terminal's screen (43; record 0016).

#### Scenario: A stuck stage's notice

- **WHEN** a code stage is reported stuck after two commits, the agent's last message asking nothing
- **THEN** the notice carries that message and both commits
