## ADDED Requirements

### Requirement: Claude Code tells its agent what a found is

Claude Code's way to stop short SHALL be the same for every start, and
SHALL say to run `flume needs "<words>"` when the agent cannot go on
without an answer, saying what it needs, and `flume needs --found
"<words>"` when it found work a stage before its own must do again,
saying what it found, and then to stop (28, 41, 131; core.31).

#### Scenario: The words a fix's code is told

- **WHEN** the profile's way to stop short is asked for the agent of a fix's code stage and for a change's verify stage
- **THEN** each is the same text, which names `flume needs --found` for work a stage before this one must do again, and holds neither "cannot deliver past" nor anything of either agent's identity (28, 41; core.31)
