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
