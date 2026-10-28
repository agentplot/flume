## ADDED Requirements

### Requirement: The plugin's hooks

flume's Claude Code plugin SHALL hold SessionStart, PreToolUse,
PostToolUse, Stop and SessionEnd hooks, each running `flume hook
<event>`, and SHALL do nothing in a session flume did not start (C.4,
section 9).

#### Scenario: A session the operator started

- **WHEN** each hook runs with no `FLUME_AGENT` in its environment
- **THEN** each allows what it was asked and writes nothing

#### Scenario: The plugin validates

- **WHEN** the pinned Claude Code validates the plugin strictly
- **THEN** it passes

### Requirement: /launch and status over MCP

The plugin SHALL give every session `/launch <brief>`, which launches
through `flume launch --json` and reports the place, and SHALL serve
`status` over MCP by `flume mcp`, answering what `flume status --json`
answers and settling nothing (57, 60, section 9).

#### Scenario: status over MCP

- **WHEN** a scripted MCP session initializes `flume mcp`, lists its tools and calls `status`
- **THEN** the tool's result is the JSON `flume status --json` gives over the same state, and nothing is written

### Requirement: Installed from the repository's marketplace

The repository SHALL hold a marketplace listing the plugin, so a
session installs it, and the binary SHALL write the same plugin for the
agents it starts, once per version of flume.

#### Scenario: The plugin an agent loads

- **WHEN** the binary gives an agent's command twice in one version
- **THEN** both name one plugin directory, written once, whose files equal `plugins/claude`
