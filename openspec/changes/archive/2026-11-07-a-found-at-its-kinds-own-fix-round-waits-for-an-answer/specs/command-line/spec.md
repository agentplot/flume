## ADDED Requirements

### Requirement: needs' help says what a found is

`flume needs --help` SHALL say that `--found` is for work a stage before
the agent's own must do again (41, C.4).

#### Scenario: needs' help

- **WHEN** `flume needs --help` runs
- **THEN** its words for `--found` name work a stage before its own must do again, and none says "cannot deliver past"
