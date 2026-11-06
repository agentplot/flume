## ADDED Requirements

### Requirement: An agent brought back in a conversation's pane is left for a person

After a restart, an agent the terminal brought back that is not the
stage's SHALL be ended once quiet, but one in a conversation's pane,
which SHALL be left in its pane, sent nothing, and ended only by its
caller's stop, drop or done; its start SHALL go not live once the
terminal reads it stopped (31, 49, 51, 52, 77, 114).

#### Scenario: A bare agent in a conversation's pane

- **WHEN** after a rebuild the fake terminal brings a talk's agent back bare and quiet, and the place is settled three times
- **THEN** talk reads lost after the first, the fake terminal's log shows no close and no send to that agent, and the start stays live; once the fake terminal reads the agent stopped, the next settle reads the start not live (31, 77, S113)

#### Scenario: A bare agent in a code stage's pane

- **WHEN** after a rebuild the fake terminal brings a code stage's agent back bare and quiet
- **THEN** code reads lost and the settle closes that agent (31, 77, S41, S116)

#### Scenario: A stop ends it

- **WHEN** the caller stops a place whose lost talk's pane holds an agent the fake terminal brought back bare, quiet
- **THEN** the stop closes it (51, 77)
