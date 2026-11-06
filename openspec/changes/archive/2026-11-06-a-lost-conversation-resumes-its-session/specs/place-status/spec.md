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

### Requirement: Status names each process running a waiting conversation

Status SHALL give, with a place whose stage waits to carry a
conversation a process runs, each process as live last read it, by its
id and when it started, or what could not be read, and since when the
stage waits; and, with a conversation's stage reported lost whose last
start's pane holds an agent the terminal brought back, that flume left
it there (49, 56, 77, 207; core.30).

#### Scenario: A talk waiting on its session

- **WHEN** status is asked of a place whose lost talk waits while the fake agent reads its session live as process 4242
- **THEN** it gives process 4242 with when it started and since when talk waits, in the JSON and the person's form (49, 56, 207, S117)

#### Scenario: A bare agent left

- **WHEN** status is asked of a place whose talk reads lost while the fake terminal holds its agent brought back bare
- **THEN** the agent gives `left_in_pane`, and the person's form says the terminal brought it back and flume left it in its pane (56, 77, S113)
