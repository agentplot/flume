## RENAMED Requirements

- FROM: `### Requirement: A spent agent's stage starts afresh on another account`
- TO: `### Requirement: A spent agent's stage starts again on another account`

## MODIFIED Requirements

### Requirement: A spent agent's stage starts again on another account

An agent stopped at its account's usage limit SHALL be ended once
quiet and its stage started on a usable account of the pool, or left
waiting for one, carrying the agent's conversation where its program
can carry it there and afresh otherwise, with flume's own words; the
start SHALL count toward no bound of the kind's and wait on no caller;
agents still running on that account SHALL not be ended for it (31,
112, 146, 175).

#### Scenario: The stage moves

- **WHEN** a code agent under work-1 stops at its usage limit, and work-2 is usable and does not reach its conversation
- **THEN** once quiet the agent is ended, code starts afresh under work-2 with flume's words, status says code moved from work-1 to work-2 because work-1 was spent, that its conversation stayed and why, and that the fresh agent starts without work-1's cache, and the kind's bound is unchanged (S55)

#### Scenario: The stage moves with its conversation

- **WHEN** a code agent under work-1 stops at its usage limit, and work-2 is usable and reaches its conversation
- **THEN** once quiet the agent is ended, code starts under work-2 resuming that conversation, status says code moved from work-1 to work-2 with its conversation and without work-1's cache, and the kind's bound is unchanged (S76)

#### Scenario: Another agent on the spent account

- **WHEN** a second agent works under work-1 when work-1 is read spent
- **THEN** it works on, and moves only if its own turn fails at the limit (31, 146)

#### Scenario: A stage under no account

- **WHEN** an agent with no account stops at a usage limit
- **THEN** its stage is read as before, stuck with the agent's last words, and no account is recorded spent (43, 146)
