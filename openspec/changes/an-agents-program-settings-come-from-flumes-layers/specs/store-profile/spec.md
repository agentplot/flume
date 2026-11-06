## ADDED Requirements

### Requirement: The host's tables for every agent and for a stream

The host's configuration SHALL read `[agents]`, settings for every
agent, and `[agents.streams.<stream>]`, for every agent in one stream,
each giving `model`, `mode`, `instructions` and `programs` as the kind
tables do. A `program`, `env`, `beside` or `stages` there, or a stream
the host does not declare, SHALL refuse the configuration naming the
table and the key (32, 143, 195, C.7).

#### Scenario: Settings for every agent and for a stream

- **WHEN** the configuration gives `[agents] model = "sonnet"`, `[agents.programs.claude] file = "~/.config/agent-launcher/claude-settings.json"` with `settings = { statusLine = { type = "command", command = "line" } }`, `[streams.work]`, `[agents.streams.work] model = "opus"` and `[agents.streams.work.programs.claude.settings] tui = "default"`
- **THEN** the host's settings read `sonnet`, the status line and the file's keys, the file under the host user's home, at the `host` layer, and `opus` and `tui` at the `host-stream` layer for `work` (32, C.7)

#### Scenario: A table these two cannot hold

- **WHEN** the configuration gives `[agents] program = "claude"`, or `[agents.stages.code] model = "opus"`, or `[agents.streams.missing] model = "opus"` with no `[streams.missing]`
- **THEN** every launch and start is refused naming the table and the key, `stages` naming `[kinds.<kind>.agents.stages.<stage>]` as where a stage's settings go (C.7)

### Requirement: Own settings in the host's agents tables

Every agents table of the host's configuration SHALL read `programs`, a
table of entries each with `file` and `settings`, kept as JSON values,
and any other shape SHALL refuse the configuration naming the table and
`programs` (195, C.7).

#### Scenario: Own settings for a kind and for a repository's stage

- **WHEN** the configuration gives `[kinds.fix.agents.programs.claude.settings] tui = "fullscreen"` and `[repos."/r".kinds.fix.agents.stages.code.programs.claude] file = "fix-code.json"`, a file beside the configuration giving a `statusLine`
- **THEN** the host's settings for fix@1 in `/r` read `tui` at `host-kind` and, for code, the file's `statusLine` at `host-repository` with the file's path (32, 195, C.7)

#### Scenario: A programs that is no table of entries

- **WHEN** the configuration gives `[agents] programs = "claude"`, or `[agents.programs] claude = 3`
- **THEN** every launch and start is refused naming the table and `programs` (C.7)

### Requirement: The host's settings files are read on the host at each start

A file a host's agents table names SHALL be read on the host with the
configuration at each start: absolute, under `~/` read as the host
user's home, or relative to the configuration's directory, and read
where it leads when it is a link. One that cannot be read, does not
parse, or is named neither `.json` nor `.toml` SHALL refuse the
configuration naming the file (195, C.7).

#### Scenario: A file flume cannot read

- **WHEN** the configuration gives `[agents.programs.claude] file = "settings.yaml"`, or names a `.json` file that does not parse
- **THEN** every launch and start is refused naming `agents.programs.claude.file` and the file (C.7)

### Requirement: The host's table for every kind of a repository

The host's configuration SHALL read `[repos."<repository>".agents]`,
settings for every agent of every kind in that repository, by its
location as `[repos."<repository>"]` is, giving `model`, `mode`,
`instructions` and `programs` as the host's kind tables do. A
`program`, `env`, `beside`, `stages` or `streams` there SHALL refuse
the configuration naming the table and the key (32, 195, C.7).

#### Scenario: A fork's table for every kind

- **WHEN** the configuration gives `[repos."~/Code/upstream-fork".agents.programs.claude.settings] attribution = { commit = "", pr = "", sessionUrl = false }`
- **THEN** the host's settings for every kind of the repository at `~/Code/upstream-fork`, under the host user's home, read `attribution` as written, at the `host-repository` layer beneath its kinds' tables (S102, 32, C.7)

#### Scenario: A repository's table that names a stage

- **WHEN** the configuration gives `[repos."/r".agents.stages.code] model = "opus"`, or `[repos."/r".agents] program = "claude"`
- **THEN** every launch and start is refused naming `repos."/r".agents` and the key (C.7)
