## MODIFIED Requirements

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

## ADDED Requirements

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
