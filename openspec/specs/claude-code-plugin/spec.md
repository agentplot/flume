# claude-code-plugin Specification

## Purpose
flume's plugin for Claude Code, written by the binary and loaded into
every agent it starts and installable in any session: the hooks that
acknowledge a start and each prompt, mark a question asked, keep an
agent from submitting or landing, and settle the place once its agent is
quiet, and the status tool any session may ask.

## Requirements

### Requirement: The plugin's hooks

flume's Claude Code plugin SHALL hold SessionStart, UserPromptSubmit,
PreToolUse, PostToolUse, Stop and SessionEnd hooks, and StopFailure and
the permission-request or permission-prompt notification hook where the
pinned Claude Code has them, each running `flume hook <event>` by the
binary's absolute path; every hook SHALL exit 0 and never hold up a
start or a turn, and each SHALL do nothing in a session flume did not
start (C.4, section 9; record 0016).

#### Scenario: A session the operator started

- **WHEN** each hook runs with no `FLUME_AGENT` in its environment
- **THEN** each allows what it was asked and writes nothing

### Requirement: The acks and the asking mark

SessionStart SHALL run the started ack and UserPromptSubmit the received
ack, each skipping a subagent's input; the asking mark SHALL cover a
permission prompt and clear at UserPromptSubmit and SessionStart as
well as on the answer; StopFailure SHALL record its reason as the
agent's last words (35, 38, 43, 90; record 0016).

#### Scenario: A question dismissed

- **WHEN** an agent opens `AskUserQuestion`, the question is dismissed with no PostToolUse, and the next prompt is submitted
- **THEN** the UserPromptSubmit hook clears the mark, and asking reads no (90)

#### Scenario: A usage limit ends the turn

- **WHEN** the pinned Claude Code's StopFailure hook runs with a usage limit's reason
- **THEN** the reason is recorded as the agent's last words, which status gives with the agent (43)

### Requirement: The plugin passes validation

The plugin SHALL pass the pinned Claude Code's strict validation.

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

### Requirement: SessionStart compares the start's record before it acks

The SessionStart hook SHALL compare its program, read from its parent
process, and the digest of each variable the start's record names, in
its own environment and keyed by `FLUME_SECRET`, with the start's
record before it acks; on any difference it SHALL ack nothing, record
the start's variables that differ by name, start a settle in the
background and exit 0 (35, 121, 161, C.4).

#### Scenario: A variable with another value

- **WHEN** the hook runs for a start whose record names `CLAUDE_CONFIG_DIR` and its environment holds another value
- **THEN** no started ack is written, the agent's record names `CLAUDE_CONFIG_DIR` as different with no value, a settle of the place is started, and the hook exits 0 (161)

#### Scenario: Another program

- **WHEN** the hook's parent runs a program other than the one the start's record names
- **THEN** no started ack is written and the record says the program differed (161)

#### Scenario: A session flume did not start

- **WHEN** the hook runs with no `FLUME_AGENT` in its environment
- **THEN** it compares nothing, writes nothing and exits 0 (C.4)

### Requirement: StopFailure records its error and when it ran

The StopFailure hook SHALL record, beside the agent's last words, the
failure's `error` and the host's time when it ran, so the profile reads
a usage limit and its reset from them (43, 151, C.4).

#### Scenario: A usage limit's failure

- **WHEN** the hook runs with `error` `rate_limit` and a usage limit's words
- **THEN** the agent's record holds the words as its last words, the error and the time the hook ran (43, 151)
