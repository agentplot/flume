## ADDED Requirements

### Requirement: SessionStart compares the session a start resumes

Where the start's record names the session it resumes, the SessionStart
hook SHALL compare its input's `session_id` with it before it acks; on
a difference it SHALL ack nothing, record that the agent came up in
another session, naming none, start a settle in the background and exit
0 (35, 121, 161, 179, C.4).

#### Scenario: A carried start in another session

- **WHEN** the hook runs for a start whose record names the session `s-1`, its input naming `s-2`
- **THEN** no started ack is written, the agent's record says it came up in another session and names neither, a settle of the place is started, and the hook exits 0 (161, 179)

#### Scenario: A start that resumes nothing

- **WHEN** the hook runs for a start whose record names no session
- **THEN** it compares no session, and acks on the program and the variables as before (161)

### Requirement: The started ack records the transcript's length once

The started ack SHALL record the length of the session's transcript
when the start's first ack came, none written reading as 0, and a later
SessionStart of the start, a compaction's or a clear's, SHALL keep the
length the first recorded (91, 179, C.4).

#### Scenario: A carried start's length

- **WHEN** the hook acks a carried start whose transcript holds 4,096 bytes
- **THEN** the ack records 4,096 (179)

#### Scenario: A compaction after the ack

- **WHEN** a start acked at a length of 4,096 compacts, and SessionStart runs again with `source` `compact`
- **THEN** the ack still records 4,096 (91, 179)
