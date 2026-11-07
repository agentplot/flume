## ADDED Requirements

### Requirement: An effort in the host's agents tables

An effort in an agents table of the host's configuration SHALL be read
as a string, kept as written, in every agents table the host's
configuration holds, and as a key the local layer may set over the
managed one; any other value SHALL refuse the configuration naming the
table and `effort` (32, 197, C.7).

#### Scenario: An effort for a kind and one for a repository's stage

- **WHEN** the configuration gives `[kinds.fix.agents] effort = "high"` and `[repos."/r".kinds.fix.agents.stages.code] effort = "xhigh"`
- **THEN** the host's settings for fix@1 in `/r` read `high` for the kind at the host-kind layer and `xhigh` for code at the host-repository layer (32, 197, C.7)

#### Scenario: An effort for every agent from the local layer

- **WHEN** the managed layer gives `[agents] effort = "medium"` and the local layer gives `[agents] effort = "high"`
- **THEN** the host's settings for every agent read `high` at the host layer, and status names the local layer as where it came from (32, 197, C.7)

#### Scenario: An effort that is no word

- **WHEN** the configuration gives `[kinds.fix.agents] effort = 3`
- **THEN** every launch and start is refused naming `kinds.fix.agents` and `effort` (197, C.7)

## MODIFIED Requirements

### Requirement: The host's tables for every agent and for a stream

The host's configuration SHALL read `[agents]`, settings for every
agent, and `[agents.streams.<stream>]`, for every agent in one stream,
each giving `model`, `effort`, `mode`, `instructions` and `programs`
as the kind tables do. A `program`, `env`, `beside` or `stages` there, or a stream
the host does not declare, SHALL refuse the configuration naming the
table and the key (32, 143, 195, 197, C.7).

#### Scenario: Settings for every agent and for a stream

- **WHEN** the configuration gives `[agents] model = "sonnet"`, `[agents.programs.claude] file = "~/.config/agent-launcher/claude-settings.json"` with `settings = { statusLine = { type = "command", command = "line" } }`, `[streams.work]`, `[agents.streams.work] model = "opus"` and `[agents.streams.work.programs.claude.settings] tui = "default"`
- **THEN** the host's settings read `sonnet`, the status line and the file's keys, the file under the host user's home, at the `host` layer, and `opus` and `tui` at the `host-stream` layer for `work` (32, C.7)

#### Scenario: A table these two cannot hold

- **WHEN** the configuration gives `[agents] program = "claude"`, or `[agents.stages.code] model = "opus"`, or `[agents.streams.missing] model = "opus"` with no `[streams.missing]`
- **THEN** every launch and start is refused naming the table and the key, `stages` naming `[kinds.<kind>.agents.stages.<stage>]` as where a stage's settings go (C.7)

### Requirement: The host's table for every kind of a repository

The host's configuration SHALL read `[repos."<repository>".agents]`,
settings for every agent of every kind in that repository, by its
location as `[repos."<repository>"]` is, giving `model`, `effort`,
`mode`, `instructions` and `programs` as the host's kind tables do. A
`program`, `env`, `beside`, `stages` or `streams` there SHALL refuse
the configuration naming the table and the key (32, 195, 197, C.7).

#### Scenario: A fork's table for every kind

- **WHEN** the configuration gives `[repos."~/Code/upstream-fork".agents.programs.claude.settings] attribution = { commit = "", pr = "", sessionUrl = false }`
- **THEN** the host's settings for every kind of the repository at `~/Code/upstream-fork`, under the host user's home, read `attribution` as written, at the `host-repository` layer beneath its kinds' tables (S102, 32, C.7)

#### Scenario: A repository's table that names a stage

- **WHEN** the configuration gives `[repos."/r".agents.stages.code] model = "opus"`, or `[repos."/r".agents] program = "claude"`
- **THEN** every launch and start is refused naming `repos."/r".agents` and the key (C.7)
