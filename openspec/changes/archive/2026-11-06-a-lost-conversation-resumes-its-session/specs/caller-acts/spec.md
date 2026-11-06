## ADDED Requirements

### Requirement: A retry of a lost conversation waits while its session runs

A retry of a conversation's stage reported lost SHALL be taken at once
and SHALL end no agent of that stage: an agent standing in its lost
start's pane, and any process running its session, are left as they
are. The retry SHALL mark the stage's start to carry the lost start's
conversation and SHALL answer that the stage waits for its conversation
to end, naming each process, where one runs it (49, 50, 77, 207).

#### Scenario: A bare agent at work in a lost talk's pane

- **WHEN** the fake terminal brought a lost talk's agent back bare and reads it working, its session scripted live, and the caller retries talk unforced
- **THEN** the retry is taken, its answer says talk waits for its conversation to end naming the process, and the fake terminal's log shows no close and no send (49, 77, 207)

#### Scenario: A session run in a pane flume did not make

- **WHEN** a lost talk's session runs in a pane of a caller's own, scripted live, and the caller retries talk forced
- **THEN** the retry is taken, talk waits, and flume closes nothing, forced or not (49, 207; core.18)
