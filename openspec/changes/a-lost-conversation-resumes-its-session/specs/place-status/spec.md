## MODIFIED Requirements

### Requirement: Status gives a start's conversation

Status SHALL give, with the current agent of a start after a spent one
or of a lost conversation's retry, the start whose conversation it
carries with whether that start stopped at its account's limit or was
lost, and whether the agent carries its conversation or, where it does
not, why; with a stage's move, whether its conversation went with it;
and each carried start that did not begin, with why; and never a
configuration home's path (49, 56, 146, 179; core.30).

#### Scenario: A carried agent

- **WHEN** status is asked of a place whose code start 2 carries the conversation of start 1 under work-2
- **THEN** the agent's conversation gives start 1, spent, and carried, and the move of code from work-1 to work-2 gives carried, in the JSON and the person's form (56, 146, S78)

#### Scenario: A conversation that stayed

- **WHEN** status is asked of a place whose code start 2 started afresh because work-2's `CLAUDE_CONFIG_DIR` does not reach start 1's conversation
- **THEN** the agent's conversation gives start 1, not carried, and why, naming `CLAUDE_CONFIG_DIR` and no path, and the move gives not carried (56, S55; core.30)

#### Scenario: A carried start that did not begin

- **WHEN** status is asked of a place whose carried start 2 ended before it said it started and whose start 3 started afresh
- **THEN** the place lists start 2 as carried and not begun with why, and the agent of start 3 gives start 1, not carried, and why (56, S79)

#### Scenario: A lost talk carried

- **WHEN** status is asked of a place whose talk start 2 carries the conversation of start 1, lost, under work-1
- **THEN** the agent's conversation gives start 1, lost, and carried, and the person's form reads that it carries the conversation of start 1, lost (49, 56, S113)

## ADDED Requirements

### Requirement: Status gives an account taken for its conversation

Status SHALL give the account of a start that took its lost start's
account for the conversation it carries as from the conversation,
beside the affinity and the strategy, in the JSON and the person's form
(56, 145).

#### Scenario: The conversation's account

- **WHEN** status is asked of a place whose talk start 2 took work-2, the account its lost start 1 ran under, while the intent keeps work-1
- **THEN** the agent's account reads work-2 from the conversation, and the person's form says it is its conversation's (56, 145, S113)

### Requirement: Status names an agent left in a lost conversation's pane

Status SHALL say, with a conversation's stage reported lost, whether an
agent the terminal brought back stands in its last start's pane, read
through the terminal that start recorded whether or not its start is
live, as `left_in_pane` in the JSON, and in the person's form that the
terminal brought it back, that flume left it and that a retry ends it
once quiet and carries its conversation (56, 77).

#### Scenario: A bare agent left

- **WHEN** status is asked of a place whose talk reads lost while the fake terminal holds its agent brought back bare
- **THEN** the agent gives `left_in_pane`, and the person's form says the terminal brought it back and flume left it in its pane (56, 77, S113)

#### Scenario: Brought back after the stage was read lost

- **WHEN** status is asked of a place whose talk was read lost with its start not live, and the fake terminal has since brought its agent back bare in the same pane
- **THEN** status gives `left_in_pane` for it (56, 77)
