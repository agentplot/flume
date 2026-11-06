## ADDED Requirements

### Requirement: A place waiting on its session

Where a place's stage waits for room while a process runs the
conversation its start would carry, or flume cannot read whether one
does, the line's state SHALL be `waiting on its session` in the place
of `waiting for room` (49, 139, 207).

#### Scenario: A lost talk retried while its session runs

- **WHEN** a lost talk is retried while its session is scripted live
- **THEN** its line's state is `waiting on its session`, and `waiting for room` once the session ends while the host has no room (49, 70, 207)
