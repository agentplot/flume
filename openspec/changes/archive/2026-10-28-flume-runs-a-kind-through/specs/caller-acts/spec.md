## MODIFIED Requirements

### Requirement: Retry

A retry SHALL end the place's agent, one that is not quiet only when
forced, then start the stage it names, the current one where it names
none, with a fresh agent, its prompt's words carrying the question of
the stop-short the stage ended with before the retry's words, the stages
after it then running again in order, the host's room allowing. A retry
naming the submission SHALL submit the place's head again and start no
agent. A retry SHALL be refused, naming the place, once the place's work
is over (42, 49, 50, 70).

#### Scenario: A retry with words

- **WHEN** a construct ended short needing an answer and its caller retries it with the answer as its words
- **THEN** a fresh agent starts whose prompt carries the question, then the answer (S4, 42, 49)

#### Scenario: A retry of a landed place

- **WHEN** a retry names a stage of a place whose work landed
- **THEN** it is refused naming the place (S21)

#### Scenario: A retry of the first stage after readiness failed

- **WHEN** a change's readiness failed and its caller retries construct
- **THEN** readiness runs again as its launch read it, and construct starts once it passes (12)
