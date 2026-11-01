## ADDED Requirements

### Requirement: The host's tables for every agent and for a stream

The host's configuration SHALL read `[agents]`, settings for every
agent, and `[agents.streams.<stream>]`, for every agent in one stream,
each giving `model`, `mode`, `instructions` and `programs` as the kind
tables do. A `program`, `env`, `beside` or `stages` there, or a stream
the host does not declare, SHALL refuse the configuration naming the
table and the key (32, 143, 175, C.7).

#### Scenario: Settings for every agent and for a stream

- **WHEN** the configuration gives `[agents] model = "sonnet"`, `[agents.programs.claude] statusLine = { type = "command", command = "line" }` and `[streams.work]` with `[agents.streams.work] model = "opus"`
- **THEN** the host's settings read `sonnet` and the status line at the `host` layer and `opus` at the `host-stream` layer for `work` (32, C.7)

#### Scenario: A table these two cannot hold

- **WHEN** the configuration gives `[agents] program = "claude"`, or `[agents.stages.code] model = "opus"`, or `[agents.streams.missing] model = "opus"` with no `[streams.missing]`
- **THEN** every launch and start is refused naming the table and the key, `stages` naming `[kinds.<kind>.agents.stages.<stage>]` as where a stage's settings go (C.7)

### Requirement: Own settings in the host's agents tables

Every agents table of the host's configuration SHALL read `programs`, a
table of programs each a table, kept as JSON values, and any other
shape SHALL refuse the configuration naming the table and `programs`
(175, C.7).

#### Scenario: Own settings for a kind and for a repository's stage

- **WHEN** the configuration gives `[kinds.fix.agents.programs.claude] outputStyle = "Explanatory"` and `[repos."/r".kinds.fix.agents.stages.code.programs.claude] spinnerTipsEnabled = false`
- **THEN** the host's settings for fix@1 in `/r` read the output style at `host-kind` and the boolean for code at `host-repository` (32, 175, C.7)

#### Scenario: A programs that is no table of tables

- **WHEN** the configuration gives `[agents] programs = "claude"`, or `[agents.programs] claude = 3`
- **THEN** every launch and start is refused naming the table and `programs` (C.7)
