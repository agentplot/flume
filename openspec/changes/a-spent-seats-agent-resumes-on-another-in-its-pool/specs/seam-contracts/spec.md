## ADDED Requirements

### Requirement: The agent seam carries a conversation or says why not

The agent seam SHALL answer, given the start whose agent stopped at its
account's usage limit and the chosen account's environment, whether its
program can carry that agent's conversation there, with the session to
resume, or why not, or that it offers no carry, the default; and its
command SHALL take the session a start resumes (88, 175).

#### Scenario: A profile that offers no carry

- **WHEN** the seam's scenarios run against a profile that implements no carry
- **THEN** carry answers that it is not offered, for any start and account (175)

#### Scenario: The fake's carry over the seam's scenarios

- **WHEN** the seam's scenarios run against the fake agent scripted to carry to one account and not to another
- **THEN** carry answers yes with the scripted session for the first, and no with the scripted words for the second (175)

### Requirement: The types of a carried start

The seams SHALL pass one type each for a stage's wait after a spent
agent, a start's conversation, a start carried that did not begin, the
session in a start's record and an unsettled start's other session;
each SHALL serialize to JSON and back unchanged, and held state written
before them SHALL read with none (146, 161, 175).

#### Scenario: A held state with a carried start

- **WHEN** a held state whose stage waits after a spent agent, whose start carries a conversation with its record's session, whose move says carried and which lists a start carried that did not begin is written and read
- **THEN** it reads back unchanged (146, 161, 175)

#### Scenario: A held state written before

- **WHEN** a held state written before this change is read
- **THEN** it reads with no wait after a spent agent, no conversation, no session and no start carried that did not begin (146)
