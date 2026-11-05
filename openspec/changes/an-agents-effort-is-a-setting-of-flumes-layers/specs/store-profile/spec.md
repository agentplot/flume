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
