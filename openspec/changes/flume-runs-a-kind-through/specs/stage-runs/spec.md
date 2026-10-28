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

#### Scenario: Held at the bypass warning

- **WHEN** a code stage's agent stops at the warning about bypassing permissions and nobody answers it
- **THEN** no prompt is sent, status gives the start starting with the warning's lines, and at the stage's limit it is reported never begun with the screen (S40)

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
