## ADDED Requirements

### Requirement: Status gives an agent's own settings

Status SHALL give the current agent's own settings, its program's entry
alone, each key with its value as its layer or its file gave it, its
layer and its file where it came from one, as
`programs.<program>.settings.<key>`; each file a layer named for the
program, with the keys left out of it and their owners; how its profile
passed them; and that they were not read back. An agent given none
SHALL be given no own settings, in the JSON form and the person's (56,
175).

#### Scenario: Keys from three layers

- **WHEN** status is asked of a fix in `work` whose agent took `line` from the host's `[agents.streams.work.programs.agent.settings]`, `tips` from the file `[agents.programs.agent]` names and `style` from the repository
- **THEN** its own settings give `programs.agent.settings.line` from `host-stream`, `programs.agent.settings.tips` from `host` with the file, and `programs.agent.settings.style` from `repository`, how they were passed, and that they were not read back (S76, 56)

#### Scenario: Keys left out of a file

- **WHEN** the host's file gave `env` and `perms.mode`, which the fake agent's profile reserves
- **THEN** the agent's own settings name the file with `env` and `perms.mode` left out and their owners, and no value of either (S76, 175)

#### Scenario: Another program's entry

- **WHEN** the host also gives `[agents.programs.other.settings] line = "b"`
- **THEN** the agent's own settings name nothing of `other` (175)
