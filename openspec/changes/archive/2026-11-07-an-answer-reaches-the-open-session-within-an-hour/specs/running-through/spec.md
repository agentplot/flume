## MODIFIED Requirements

### Requirement: Found work from another stage needs an answer

Found work from any stage but the kind's last SHALL be taken as needing
an answer: the stage ends short and the place waits on its answer, its
agent kept open for it as for a question, status giving the kind the
agent said and the words (42, 131, 198).

#### Scenario: Construct says it found work

- **WHEN** a construct agent stops short with `found work`
- **THEN** no stage starts, the place waits on an answer with its agent kept open for it, and status says the agent found work with its words

#### Scenario: Found work from the last stage keeps no agent open

- **WHEN** a verify agent, the kind's last stage, stops short having found work
- **THEN** its agent is kept open for nothing, it is ended once quiet, and the kind's fix round starts with the findings (131)
