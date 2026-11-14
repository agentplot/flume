## MODIFIED Requirements

### Requirement: A declared thing with no line is not yet made

A stage SHALL NOT be read delivered while a thing its declaration says
it makes has no line, and the stop check SHALL send the agent back
naming each such thing beside the deliverable (44, 110, 129). A line
SHALL count when written since the stage started as its deliverable's
facts count, at the first of its starts since it last delivered, and
the stage's end SHALL carry every line written since then (25, 129).

#### Scenario: Commits and no line

- **WHEN** a code stage that declares `preview` commits and goes quiet with no line for `preview`
- **THEN** the stage is not delivered, and the stop check's words name `preview` and the way to stop short

#### Scenario: The line written

- **WHEN** the agent then writes the line for `preview` and goes quiet
- **THEN** the stage is delivered at its head

#### Scenario: A line an earlier start wrote

- **WHEN** a code stage that declares `preview` has its agent commit and write the line for `preview`, then stop short needing an answer, and the retry's agent makes no commit, writes no line and goes quiet
- **THEN** the stage is delivered, and its end carries the line the first start wrote (25, 49, 129)
