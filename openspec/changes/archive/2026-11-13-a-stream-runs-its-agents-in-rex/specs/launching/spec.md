## MODIFIED Requirements

### Requirement: A launch whose binding or settings flume cannot run is refused whole

A launch SHALL be refused, naming the program, field, stage or stream
and its layer, and SHALL leave nothing made, when the binding names a
place profile flume does not hold, when a stage's agent resolves to a
program no agent profile binds or its stream's terminal cannot run,
when its stream's multiplexer names a terminal profile flume does not
carry, when a declaration gives an agent an `env`, or when settings
name a stage the kind does not hold (4, 32, 158, 190, C.1, C.2).

#### Scenario: A program no agent profile binds

- **WHEN** a change is launched with `--agent code.program=nothing-here`
- **THEN** the launch is refused naming the program, the stage `code` and the layer `launch`, and nothing is made (4, 32)

#### Scenario: An environment in a repository's file

- **WHEN** a repository's `.config/flume.toml` gives `[kinds.change.agents] env = { A = "b" }` and a change is launched
- **THEN** the launch is refused naming the field, and nothing is made (C.2, core.30)

#### Scenario: A place profile flume does not hold

- **WHEN** a repository's binding names a place profile the resolver does not hold
- **THEN** the launch is refused naming the place profiles the resolver holds, and nothing is made (C.1, core.3)

#### Scenario: A stream's multiplexer flume does not carry

- **WHEN** the host's `[streams.trial]` gives `multiplexer = "cmux"` and a fix is launched on `trial`
- **THEN** the launch is refused naming `trial`, `cmux` and the terminal profiles the resolver carries, `rex` among them, and nothing is made (4, 190; core.8)
