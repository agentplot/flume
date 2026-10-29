## ADDED Requirements

### Requirement: The terminal reaches a recipient's session by its name

The terminal seam SHALL send a text to, and read the state of, a
recipient's session named as the terminal names sessions, the same
guarantee as a send to an agent: delivered as input or refused, never
left typed; and a recipient the terminal cannot find SHALL be answered
as not found, never guessed (43, 95, 96, 98).

#### Scenario: A session by its name

- **WHEN** the terminal scenarios send a notice to a recipient's session by its name, then to a name no session holds
- **THEN** the first is sent and the session's state is read, and the second is answered not found with nothing sent

### Requirement: An agent opened under a session name

The terminal seam's open SHALL take the session name the agent is to
carry, a role run's address where the place is one, and a send or a
state read by that name SHALL reach that agent (94, 107, 132).

#### Scenario: A role run's agent

- **WHEN** an agent is opened under the session name `madswan/planner`
- **THEN** a send to the recipient `madswan/planner` reaches that agent, and a second open under that name while it is live is refused (98)
