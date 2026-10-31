## MODIFIED Requirements

### Requirement: The host's configuration

The host's configuration SHALL be read from
`$XDG_CONFIG_HOME/flume/config.toml` at every start: `[host]` `agents`,
`terminal` and `pool`; `[streams.<stream>]`; `[pools.<pool>]`; agents'
model, mode and instructions by kind and by repository and kind, each
with `.stages.<stage>`; `stream` where C.7 names it; and `[roles]`. A
key left out SHALL be no bound, the printed command, no stream, pool or
settings (32, 70, 105, 143, 144, 162, C.7).

#### Scenario: A bound stated

- **WHEN** the configuration states `[host] agents = 2`
- **THEN** the host's bound reads two

#### Scenario: Settings for a kind and for a repository's kind

- **WHEN** the configuration gives `[kinds.change.agents] model = "opus"` and `[repos."/r".kinds.change.agents.stages.verify] model = "sonnet"`
- **THEN** the host's settings for change@1 in `/r` read `opus` for the kind at the host-kind layer and `sonnet` for verify at the host-repository layer, and for change@1 elsewhere only `opus` (32, C.7)

#### Scenario: C.7's example

- **WHEN** the configuration is C.7's example, with streams `work` and `personal`, pools `work` and `personal`, and `[repos."~/Code/acme"] stream = "work"`
- **THEN** acme's places read the stream `work` from `host-repository` and the pool `work` from the stream, whose accounts read work-1, bounded at four, then work-2, each with its `CLAUDE_CONFIG_DIR` under the host user's home (143, 144, C.7)

#### Scenario: A configuration that refuses

- **WHEN** the configuration gives a stream `pool = "missing"` that no `[pools]` table declares, or a pool `strategy = "round-robin"`
- **THEN** every launch and start is refused naming the key, and status says the host's configuration cannot be read (4, C.7)

## ADDED Requirements

### Requirement: A mode in the host's agents tables

A mode in an agents table of the host's configuration SHALL be read as
one of flume's four or a table of programs whose entries are strings,
and any other SHALL refuse the configuration naming the table and
`mode` (162, 169, C.7).

#### Scenario: A mode for a kind and a table for a repository's stage

- **WHEN** the configuration gives `[kinds.fix.agents] mode = "edits"` and `[repos."/r".kinds.fix.agents.stages.code] mode = { claude = "auto" }`
- **THEN** the host's settings for fix@1 in `/r` read `edits` for the kind at the host-kind layer and the table for code at the host-repository layer (32, 162, C.7)

#### Scenario: A mode that is neither

- **WHEN** the configuration gives `[kinds.fix.agents] mode = "auto"`, or `mode = 3`, or `mode = { claude = 3 }`
- **THEN** every launch and start is refused naming `kinds.fix.agents` and `mode` (162, 169, C.7)
