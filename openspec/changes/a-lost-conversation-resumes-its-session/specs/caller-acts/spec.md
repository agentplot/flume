## ADDED Requirements

### Requirement: A retry of a lost conversation ends an agent in its pane first

A retry of a conversation's stage reported lost SHALL first read its
lost start's pane again through the terminal that start recorded, live
or not, and end an agent standing there as it ends the place's agent,
once quiet or when forced, and SHALL then carry the lost start's
conversation as carried-conversations says (49, 50, 77).

#### Scenario: A bare agent at work in a lost talk's pane

- **WHEN** the fake terminal brought a lost talk's agent back bare and reads it working, and the caller retries talk unforced, then forced
- **THEN** the first retry is refused naming the agent and that it is not quiet, and starts nothing; the second closes it and starts talk carrying its conversation (31, 49, 50, 77)

#### Scenario: A bare agent brought back after the stage was read lost

- **WHEN** a talk's agent stopped and the settle read talk lost with its start no longer live, and the fake terminal then brings the agent back bare in the same pane, quiet, before the caller retries talk
- **THEN** the retry closes that agent before talk's carried start opens, and the fake terminal's log shows the close before the open (49, 50, 77)
