## ADDED Requirements

### Requirement: Status gives a start's conversation

Status SHALL give, with the current agent of a start after a spent one,
the stopped start's number and whether the agent carries its
conversation or, where it does not, why; with a stage's move, whether
its conversation went with it; and each carried start that did not
begin, with why; and never a configuration home's path (56, 146, 179;
core.30).

#### Scenario: A carried agent

- **WHEN** status is asked of a place whose code start 2 carries the conversation of start 1 under work-2
- **THEN** the agent's conversation gives start 1 and carried, and the move of code from work-1 to work-2 gives carried, in the JSON and the person's form (56, 146, S78)

#### Scenario: A conversation that stayed

- **WHEN** status is asked of a place whose code start 2 started afresh because work-2's `CLAUDE_CONFIG_DIR` does not reach start 1's conversation
- **THEN** the agent's conversation gives start 1, not carried, and why, naming `CLAUDE_CONFIG_DIR` and no path, and the move gives not carried (56, S55; core.30)

#### Scenario: A carried start that did not begin

- **WHEN** status is asked of a place whose carried start 2 ended before it said it started and whose start 3 started afresh
- **THEN** the place lists start 2 as carried and not begun with why, and the agent of start 3 gives start 1, not carried, and why (56, S79)
