## ADDED Requirements

### Requirement: A carried start's record names the session it resumes

At a start that carries a conversation flume SHALL record, in the
start's record, the session the start resumes, and SHALL record none at
any other start (146, 161, 175; core.10).

#### Scenario: The session recorded

- **WHEN** a start carries the conversation of session `s-1`, and another start of the place carries none
- **THEN** the first start's record names `s-1` and the second's names no session (161, 175)

### Requirement: The started ack is taken only in the session the record names

Where a start's record names a session, the agent's word that it
started SHALL be taken only where it names that session; otherwise the
start SHALL read started without its settings, saying it came up in
another conversation than the one it resumes, and naming no session
(35, 135, 161, 175).

#### Scenario: Another session

- **WHEN** a start's record names the session `s-1`, and its agent says it started with every variable as recorded in the session `s-2`
- **THEN** no started ack is recorded, and the start reads started without its settings, saying it came up in another conversation, naming neither session (161, 175)

#### Scenario: The session resumed

- **WHEN** a start's record names the session `s-1`, and its agent says it started in `s-1` with every variable as recorded
- **THEN** the started ack is taken and its prompt is sent (35, 37, 175)
