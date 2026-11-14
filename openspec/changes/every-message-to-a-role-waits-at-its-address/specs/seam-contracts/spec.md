## MODIFIED Requirements

### Requirement: The terminal reaches a recipient's session by its name

The terminal seam SHALL send a text to, and read the state of, a
recipient's session named as the terminal names sessions, the same
guarantee as a send to an agent: delivered as input or refused, never
left typed; a session that reads working SHALL be refused with nothing
typed; and a recipient the terminal cannot find SHALL be answered as
not found, never guessed (43, 54, 95, 96, 98, 214).

#### Scenario: A session by its name

- **WHEN** the terminal scenarios send a notice to a recipient's session by its name, then to a name no session holds
- **THEN** the first is sent and the session's state is read, and the second is answered not found with nothing sent

#### Scenario: A session at work

- **WHEN** the terminal scenarios send a notice to a recipient's session that reads working
- **THEN** it is refused naming that the session works, and nothing is typed into it (54, 214)

### Requirement: The types of an answer taken in a session

The held state SHALL hold an end's `answer`, with its words, the act it
came by, when it was taken and, where it was handed to a session, when
it was sent and received; each start's count of the answers it took in
its session; and an end's count of the answers its start took before
it, each read with a default so state written before reads with none.
An end's key SHALL count the answers only where there are some. An
answer's act written `tell` before SHALL read as `message` (42, 43, 53,
75).

#### Scenario: Two answers

- **WHEN** held state with a start whose two ended-short ends each hold an answer, one by `retry` and one by `message`, is written and read back
- **THEN** it reads back equal, each field as written, and the two ends key apart (43, 75)

#### Scenario: State written before

- **WHEN** held state written before this change is read
- **THEN** it reads with no agent kept open, no close, no answers and no `keep_open`, and each of its ends keys as it did (75)

#### Scenario: An answer by tell

- **WHEN** held state whose end holds an answer by `tell` is read
- **THEN** the answer reads by `message`, and is written back as `message` (53)

## ADDED Requirements

### Requirement: The agent seam's deliver part

The agent seam SHALL say whether the program takes words as one prompt
the next time its agent waits by its own channel, leaving its input as
a person left it, not offered where its profile says nothing; and a
start's held state SHALL hold the messages waiting for that start's
take, each with its words, when, who sent it and when it was taken,
read with none in state written before (54, 214, 215).

#### Scenario: The fake agent's deliver

- **WHEN** the fake agent is set to offer deliver, and held state with a start holding one message taken and one waiting is written and read back
- **THEN** `deliver` answers offered, and the state reads back equal; a start written before reads with no message waiting (75, 215)

#### Scenario: A profile that names no deliver

- **WHEN** `deliver` is asked of an agent profile that says nothing of it, as the common profile will not
- **THEN** it answers not offered (171, 215)
