## ADDED Requirements

### Requirement: How long an agent is kept open is the launch's or the host's

An agent's `keep_open` SHALL be resolved at its start as clause 32
resolves every setting, taken whole from the first that gives it of the
launch's settings, the host's for the repository and kind, then for the
repository's every kind, the host's for the kind, the host's for the
place's stream, and the host's for every agent, a stage's own first in
each, else flume's own one hour; a repository's settings and a kind's
declaration SHALL give none (32, 158, 198).

#### Scenario: From the host's kind

- **WHEN** the host gives `[kinds.fix.agents] keep_open = "10m"`
- **THEN** a fix place's code agent starts with `keep_open` `10m` from `host-kind` (32)

#### Scenario: The host's kind over its stream

- **WHEN** the host gives `[kinds.fix.agents] keep_open = "10m"` and `[agents.streams.work] keep_open = "2h"`, and a fix runs in the stream `work`
- **THEN** its code agent starts with `10m` from `host-kind` (32)

#### Scenario: The launch over the host

- **WHEN** a launch gives `--agent code.keep_open=0` over the host's `10m`
- **THEN** code's agent starts with `0` from `launch`, and once it stops short needing an answer it is ended once quiet (32, 198)

#### Scenario: No layer gives one

- **WHEN** no layer gives `keep_open`
- **THEN** the agent starts with `1h` from `default` (32, 198)

#### Scenario: A host change after the launch

- **WHEN** the host's `[agents] keep_open` changes from `1h` to `15m` while a place's first stage runs
- **THEN** that stage's agent keeps `1h`, and the place's next stage's agent starts with `15m` (8, 32)
