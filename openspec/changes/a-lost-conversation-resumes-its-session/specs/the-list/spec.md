## ADDED Requirements

### Requirement: The list says a place waits on its session

A place's line SHALL add `waits on its session` where its state is
`waiting on its session`, in the place of `waits on the host`, read
from the stage's wait as the last settle left it (49, 200, 207).

#### Scenario: A talk waiting on its session

- **WHEN** the list is asked of a place whose lost talk waits on its session
- **THEN** its line ends `waiting on its session · waits on its session`, and the list asks no agent program whether the session runs (49, 200, 207)
