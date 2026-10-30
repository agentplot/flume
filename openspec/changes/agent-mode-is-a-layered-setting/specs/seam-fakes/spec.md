## ADDED Requirements

### Requirement: The fake agent scripts a map of modes

The fake agent SHALL script its map of flume's four modes and the own
values it names, whether it reads the mode back, and a start that
began in another mode than its record names, each passing the agent
seam's scenarios; its command SHALL pass the value mapped, so a test
reads what the profile was given (106, 161, 169).

#### Scenario: A scripted start in another mode

- **WHEN** a test scripts the fake agent to map `full` to `all` and its code start to begin in `ask-each`
- **THEN** the core records the start's mode `all`, reads it started without its settings naming both modes, and the fake's log shows the command passing `all` (161, 169)
