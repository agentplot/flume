## ADDED Requirements

### Requirement: Status gives an agent's own settings

Status SHALL give the current agent's own settings, its program's entry
alone, each key with its value as its layer wrote it and its layer, as
`programs.<program>.<key>`, with how its profile passed them and that
they were not read back; an agent given none SHALL be given no own
settings, in the JSON form and the person's (56, 175).

#### Scenario: Keys from three layers

- **WHEN** status is asked of a fix in `work` whose agent took `line` from the host's `[agents.streams.work.programs.agent]`, `tips` from `[agents.programs.agent]` and `style` from the repository
- **THEN** its own settings give `programs.agent.line` from `host-stream`, `programs.agent.tips` from `host` and `programs.agent.style` from `repository`, how they were passed, and that they were not read back (S76, 56)

#### Scenario: Another program's entry

- **WHEN** the host also gives `[agents.programs.other] line = "b"`
- **THEN** the agent's own settings name nothing of `other` (175)
