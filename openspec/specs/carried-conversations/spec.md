# carried-conversations Specification

## Purpose
An agent stopped at its account's usage limit goes on with its own
conversation on the account its pool chooses, where its program can
carry it there, and its stage starts afresh otherwise, status saying
which (146, 179).

## Requirements

### Requirement: The start after a spent agent carries its conversation

Where an agent stopped at its account's usage limit, the stage's next
start SHALL carry that agent's own conversation where its profile
answers that its program can carry it to the account chosen for that
start, read under the room lock after the account is chosen (145, 146,
179).

#### Scenario: A conversation carried to the pool's other account

- **WHEN** a conversation's agent under work-1 stops at work-1's usage limit, and the fake agent answers that work-2, the account chosen, can carry its conversation
- **THEN** once quiet the agent is ended, and the stage starts under work-2 resuming that conversation, with no caller's word and no bound of the kind's counted (S78)

#### Scenario: A carried start waits for a usable account

- **WHEN** a code agent stops at its account's limit and no account of the pool is usable until 15:00
- **THEN** the stage waits for room, and the start that takes an account after 15:00 carries the conversation where the profile answers it can there (145, 146)

#### Scenario: A pool of one account

- **WHEN** a pool's only account is spent until 15:00 and its agent's stage waits for it
- **THEN** the start after 15:00 takes the same account and carries the conversation (146, 179)

### Requirement: Only the start after a spent agent carries a conversation

No start but the one that follows an agent stopped at its account's
usage limit SHALL carry a conversation, and it SHALL carry only that
agent's: a retry, a fix round, a start after one begun without its
settings and every start of another stage SHALL carry none (30, 146).

#### Scenario: A retry carries nothing

- **WHEN** a caller retries a stage whose agent stopped at its account's limit, before its stage starts again
- **THEN** the start the retry makes resumes no conversation, and its prompt is the stage's with the retry's words (30, 49)

#### Scenario: The next stage carries nothing

- **WHEN** a carried code start delivers and verify starts
- **THEN** verify's agent resumes no conversation (30)

### Requirement: A carried start's prompt is flume's words alone

A start that carries a conversation SHALL be sent flume's own words
alone, saying that the agent's last turn stopped at its account's usage
limit and that it goes on under an account of its pool, and then the
way to stop short, with nothing of the stage's prompt (28, 146).

#### Scenario: The prompt of a carried start

- **WHEN** a carried code start's agent says it started and waits
- **THEN** its prompt is flume's words and the way to stop short, and holds nothing of the code stage's template (28, 146)

### Requirement: Where a conversation cannot be carried the stage starts afresh

Where the profile answers that its program cannot carry the
conversation to the account chosen, or offers no carry, the stage SHALL
start afresh with flume's words, as before, and status SHALL say why the
conversation stayed, naming any variable by its name alone (146, 179;
core.30).

#### Scenario: Accounts that share nothing

- **WHEN** a code agent under work-1 stops at its limit, and the profile answers that work-2's `CLAUDE_CONFIG_DIR` does not reach the conversation
- **THEN** code starts afresh under work-2 with flume's words, and status says the conversation stayed and why, naming `CLAUDE_CONFIG_DIR` and no path (S55)

#### Scenario: A profile that carries nothing

- **WHEN** a stage's agent stops at its limit under a profile that offers no carry
- **THEN** the stage starts afresh, and status says its agent profile carries no conversation (146, 171)

### Requirement: A carried start that does not begin starts the stage afresh

A carried start read never begun, or whose agent came up otherwise than
its record, SHALL not be delivered: its agent SHALL be ended once quiet
and the stage started afresh at once, carrying nothing, counting no
bound and waiting on no caller, status saying why (35, 112, 146, 161).

#### Scenario: No such conversation

- **WHEN** a carried code start's program finds no such conversation and ends before it says it started
- **THEN** no end is delivered, code starts afresh at once with flume's words, status says the conversation could not be carried and why, and the kind's bound is not counted (S79)

#### Scenario: Another session

- **WHEN** a carried start's agent says it started in another session than its record names
- **THEN** its started ack is not taken, its agent is ended once quiet, and the stage starts afresh carrying nothing (161, 179)

#### Scenario: The fresh start after it begins otherwise

- **WHEN** the fresh start after a carried start that ended before it said it started comes up without its account's `CLAUDE_CONFIG_DIR`
- **THEN** it is ended once quiet and the stage starts afresh once more, and a third start that comes up so is never begun (161)

### Requirement: flume changes nothing of an account to carry a conversation

flume SHALL copy, link and write nothing in any account's configuration
home, and SHALL only read whether the host made the conversation
reachable from the account chosen (146, 179, C.4; core.32).

#### Scenario: Two scratch configuration homes

- **WHEN** a conversation is carried between two scratch configuration homes, the second's `projects` a link to the first's, and when one is not carried between two that share nothing
- **THEN** each home holds what it held before the launch and the transcripts the agent program wrote, and no other file, link or folder, and the credentials of each are as they were (C.4)

### Requirement: Claude Code carries over a shared projects folder

The Claude Code profile SHALL answer that it can carry where the
stopped start's transcript, as its started ack named it, is the same
file by device and inode as the chosen account's
`<home>/projects/<folder>/<file>`, `<home>` its `CLAUDE_CONFIG_DIR`, else
`~/.claude`, and resume it by `--resume <session>` (179, C.4).

#### Scenario: The second account's projects a link to the first's

- **WHEN** the stopped start's transcript is in work-1's home, and work-2's `projects` is a symbolic link to work-1's
- **THEN** carry answers yes with the ack's session, and the next start's command holds `--resume` and that session (179, C.4)

#### Scenario: Two homes of their own

- **WHEN** work-2's `projects` is a folder of its own holding no such transcript
- **THEN** carry answers no, naming `CLAUDE_CONFIG_DIR` and no path (179; core.30)

#### Scenario: A start that never said it started

- **WHEN** the stopped start has no started ack
- **THEN** carry answers no, saying its conversation is not known (179)
