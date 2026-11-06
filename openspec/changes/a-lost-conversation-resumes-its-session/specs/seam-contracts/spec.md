## MODIFIED Requirements

### Requirement: The agent seam carries a conversation or says why not

The agent seam SHALL answer, given the start whose conversation a start
carries, one whose agent stopped at its account's usage limit or a
conversation's start reported lost, and the environment of the account
the next start takes, an empty one under no pool, whether its program
can carry that agent's conversation there, with the session to resume,
or why not, or that it offers no carry, the default; and its command
SHALL take the session a start resumes (49, 88, 179).

#### Scenario: A profile that offers no carry

- **WHEN** the seam's scenarios run against a profile that implements no carry
- **THEN** carry answers that it is not offered, for any start and account (179)

#### Scenario: The fake's carry over the seam's scenarios

- **WHEN** the seam's scenarios run against the fake agent scripted to carry to one account and not to another
- **THEN** carry answers yes with the scripted session for the first, and no with the scripted words for the second (179)

### Requirement: The types of a carried start

The seams SHALL pass one type each for a stage's wait to carry a
conversation, naming the start it carries and its cause, spent or
lost; a start's conversation with that cause; a start carried that did
not begin; the session in a start's record; and an unsettled start's
other session. Each SHALL serialize to JSON and back unchanged, and
held state written before them SHALL read with none, and a wait or a
conversation written before with no cause SHALL read as spent (49, 146,
161, 179).

#### Scenario: A held state with a carried start

- **WHEN** a held state whose stage waits after a spent agent, whose start carries a conversation with its record's session, whose move says carried and which lists a start carried that did not begin is written and read
- **THEN** it reads back unchanged (146, 161, 179)

#### Scenario: A held state written before

- **WHEN** a held state written before this change is read
- **THEN** it reads with no wait after a spent agent, no conversation, no session and no start carried that did not begin (146)

#### Scenario: A lost conversation's wait and conversation

- **WHEN** a held state whose talk waits to carry a lost start's conversation, and whose talk start 2 carries it, is written and read, and one whose wait and conversation name no cause is read
- **THEN** the first reads back unchanged with the cause lost, and the second reads with the cause spent (49, 179)

## ADDED Requirements

### Requirement: The types of an account taken for its conversation and an agent left

The seams SHALL pass an account's choice from the conversation beside
the affinity and the strategy, and a lost end's and a status agent's
`left_in_pane`; each SHALL serialize to JSON and back unchanged, and an
end or a status written before SHALL read with no agent left (43, 56,
77, 145).

#### Scenario: An account from the conversation and an agent left

- **WHEN** a held state whose talk start runs under work-2 chosen from its conversation, and whose earlier talk end gives `left_in_pane`, is written and read
- **THEN** it reads back unchanged (43, 145)
