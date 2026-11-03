## ADDED Requirements

### Requirement: The fake agent scripts carry and a carried start

The fake agent SHALL script, for each account by its environment,
whether it carries a stopped start's conversation, with the session or
the words; the session a start's word names; and a carried start that
ends before its word that it started; its log naming each `carry` it
answers (106, 179).

#### Scenario: A scripted carry

- **WHEN** a test scripts the fake agent to carry to work-2 with the session `s-1`, and a start stops at its account's limit with work-2 chosen next
- **THEN** the core starts the stage resuming `s-1`, and the fake's log shows the `carry` reading for work-2 (146, 179)

#### Scenario: A scripted carried start that ends first

- **WHEN** a test scripts the fake agent's carried start to end before it says it started
- **THEN** the core starts the stage afresh carrying nothing, and the fake's log shows the second start's command with no session (146, S79)
