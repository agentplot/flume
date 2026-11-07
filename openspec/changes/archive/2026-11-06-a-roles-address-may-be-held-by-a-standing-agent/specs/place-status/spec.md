## ADDED Requirements

### Requirement: Status says where each standing address lives

The host's status SHALL give every address the configuration gives a
standing agent, pending or not, with its agent, stream, session and
where that came from, the stream's terminal, and what the terminal
found at the ask: one agent with its state, none, more than one, or why
it could not read; each terminal and session read once per status, and
nothing kept (56, 132, 190; core.14, core.15).

#### Scenario: An agent found

- **WHEN** the host's status is asked while `madswan/planner`'s standing agent waits in `madswan` and two ends are pending for it
- **THEN** the address gives `madswan-planner`, the session `madswan` from `role`, the agent found waiting, and the two pending ends (56, 132)

#### Scenario: None of the name

- **WHEN** the fake terminal holds no `madswan-planner` in `madswan`
- **THEN** the address's standing says none found, and its pending ends stand beneath (132)

#### Scenario: More than one

- **WHEN** the fake terminal holds two `madswan-planner` in `madswan`
- **THEN** the address's standing says two found (132)

#### Scenario: A part not read

- **WHEN** the fake terminal refuses to read `madswan`'s agents with its words
- **THEN** the address's standing says it could not read, with those words (132)

#### Scenario: Nothing pending

- **WHEN** the configuration gives `madswan/ops` a standing agent and no end is pending for it
- **THEN** the host's status still gives `madswan/ops` with its standing (56)

#### Scenario: Two addresses in one session

- **WHEN** `madswan/planner` and `madswan/ops` each name an agent in `madswan`, and the host's status is asked
- **THEN** the fake terminal is asked for `madswan`'s agents once (core.15)

#### Scenario: The fields kept

- **WHEN** the suite's field list is read after this change
- **THEN** it holds each address's standing with its agent, stream, session and its `from`, terminal and what was found, and the wake's held answer, and an answer lacking one fails the suite (61; core.14)
