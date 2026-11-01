## ADDED Requirements

### Requirement: How long a session stays open is the launch's or the host's

An agent's `keep_open` SHALL come from the first that gives it of the
launch's settings, the host's for the repository and kind, and the
host's for the kind, a stage's own first in each, else flume's one
hour, resolved at the agent's start; status SHALL give it with its
layer (32, 41; record 0017).

#### Scenario: From the host's kind

- **WHEN** the host gives `[kinds.fix.agents] keep_open = "10m"`
- **THEN** a fix place's code agent starts with `keep_open` `10m` from `host-kind` (32)

#### Scenario: The launch over the host

- **WHEN** a launch gives `--agent code.keep_open=0` over the host's `10m`
- **THEN** code's agent starts with `0` from `launch`, and once it stops short needing an answer it is ended once quiet (32, 41)

#### Scenario: No layer gives one

- **WHEN** no layer gives `keep_open`
- **THEN** the agent starts with `1h` from `default` (32, 41)
