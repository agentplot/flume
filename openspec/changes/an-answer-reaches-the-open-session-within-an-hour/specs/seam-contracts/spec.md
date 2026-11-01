## ADDED Requirements

### Requirement: The terminal says whether it hands a running agent words

The terminal seam SHALL say whether it can type words into a running
agent's input: the herdr profile SHALL say it can, the printed command
SHALL say it cannot, and the core SHALL hold no session open for an
answer under a terminal that cannot (41, 53, C.5).

#### Scenario: The printed command

- **WHEN** the printed command's profile is asked whether it hands a running agent words
- **THEN** it says it cannot, and its send is refused with nothing typed (53, C.5)

#### Scenario: herdr

- **WHEN** the herdr profile is asked whether it hands a running agent words
- **THEN** it says it can (53, C.5)

### Requirement: The types of an open session and its answers

The held state SHALL hold a stage end's `open_until` and `closed`, with
when, why and when its agent was ended, each start's `answers`, a
setting's `keep_open` as written, and an end's count of the answers its
start took before it, each read with a default so state written before
reads with none; an end's key SHALL count the answers only where there
are some (42, 43, 56, 75).

#### Scenario: A session closed and two answers

- **WHEN** held state with a session closed by `account_spent`, its agent ended, and a start with two answers, one by `retry` and one by `tell`, is written and read back
- **THEN** it reads back equal, each field as written (75)

#### Scenario: State written before

- **WHEN** held state written before this change is read
- **THEN** it reads with no session open, no close, no answers and no `keep_open`, and each of its ends keys as it did (75)
