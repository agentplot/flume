## ADDED Requirements

### Requirement: The terminal says whether it hands a running agent words

The terminal seam SHALL say whether it can type words into a running
agent's input: the herdr profile SHALL say it can, the printed command
SHALL say it cannot, and the core SHALL keep no agent open for an
answer under a terminal that cannot (53, 95, 198, C.5).

#### Scenario: The printed command

- **WHEN** the printed command's profile is asked whether it hands a running agent words
- **THEN** it says it cannot, and its send is refused with nothing typed (53, C.5)

#### Scenario: herdr

- **WHEN** the herdr profile is asked whether it hands a running agent words
- **THEN** it says it can (53, C.5)

### Requirement: The types of an agent kept open

The held state SHALL hold a stage end's `open_until` and `closed`, with
when, why, one of `time`, `stopped`, `agent_ended` and `account_spent`,
when its agent was ended and whether a bare agent was left in its pane,
and a setting's `keep_open` as written, each read with a default so
state written before reads with none (56, 75, 77, 198).

#### Scenario: An agent let go

- **WHEN** held state with an agent let go by `account_spent` and ended is written and read back
- **THEN** it reads back equal, each field as written (75)

### Requirement: The types of an answer taken in a session

The held state SHALL hold an end's `answer`, with its words, the act it
came by, when it was taken and, where it was handed to a session, when
it was sent and received; each start's count of the answers it took in
its session; and an end's count of the answers its start took before
it, each read with a default so state written before reads with none.
An end's key SHALL count the answers only where there are some (42, 43,
75).

#### Scenario: Two answers

- **WHEN** held state with a start whose two ended-short ends each hold an answer, one by `retry` and one by `tell`, is written and read back
- **THEN** it reads back equal, each field as written, and the two ends key apart (43, 75)

#### Scenario: State written before

- **WHEN** held state written before this change is read
- **THEN** it reads with no agent kept open, no close, no answers and no `keep_open`, and each of its ends keys as it did (75)
