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

### Requirement: UserPromptSubmit compares the mode at the start's first prompt

At the start's first prompt, the first with no received ack where no
word that the agent began otherwise stands, the UserPromptSubmit hook
SHALL compare its input's `permission_mode`, `default` read as
`manual`, with the start's record before it acks received, where the
record names a mode, since 2.1.296 gives a SessionStart input none
(161, 162, C.4).

#### Scenario: SessionStart acks on the program and the variables

- **WHEN** the SessionStart hook runs for a start whose record names `bypassPermissions`, its input naming no mode, as 2.1.296's never does, and its program and variables as the record names them
- **THEN** the started ack is written (35, 161)

#### Scenario: manual read back as default

- **WHEN** the hook runs at the first prompt of a start whose record names `manual`, its input's `permission_mode` reading `default`
- **THEN** the received ack is written and nothing is blocked (161, C.4)

#### Scenario: No mode in the record

- **WHEN** the hook runs at the first prompt of a start whose record names no mode, its input's `permission_mode` reading `acceptEdits`
- **THEN** the received ack is written, no mode compared (161, 162)

### Requirement: A first prompt in another mode is blocked

Where the mode a start's first prompt names is not its record's, or
the input names none, the UserPromptSubmit hook SHALL answer a block
with words naming both modes, so the prompt is held back and the agent
takes no turn, ack nothing, record the mode asked and the mode begun
with the started ack's session, start a settle in the background and
exit 0 (35, 121, 161, 162, S67).

#### Scenario: Another mode

- **WHEN** the hook runs at the first prompt of a start whose record names `bypassPermissions`, its input's `permission_mode` reading `default`
- **THEN** it answers a block naming `manual` begun where `bypassPermissions` was asked, no received ack is written, the agent's record names the mode asked `bypassPermissions` and the mode begun `manual`, a settle is started, and the hook exits 0 (161, 162, S67)

#### Scenario: No mode in the input

- **WHEN** the hook runs at the first prompt of a start whose record names `acceptEdits` and its input carries no `permission_mode`
- **THEN** the prompt is blocked, no received ack is written, and the agent's record names the mode asked `acceptEdits` and no mode begun (161)

### Requirement: A later prompt or SessionStart of a started session compares no mode

A prompt after the start's first, a tell among them, and a SessionStart
after its session resumed, cleared or compacted, SHALL compare no mode,
and SHALL ack where everything else matches, a SessionStart recording
the session's transcript, since the mode is read at the agent's start
and a started session's mode is its own (161, 162).

#### Scenario: A later prompt and a compaction after the mode moved

- **WHEN** a start's first prompt was taken in `acceptEdits`, a later prompt's input reads `default`, and the session compacts
- **THEN** the later prompt is acked received, the started ack is written again with the session's transcript, and nothing is recorded as unsettled (161, 162)

### Requirement: flume ack received takes the mode the session runs in

`flume ack received` SHALL take `--mode <value>`, the mode the session
runs in as the program names it, and compare it at the start's first
prompt as the hook compares its input's `permission_mode`; `flume ack
started` SHALL compare no mode (135, 161).

#### Scenario: An ack by hand with no mode

- **WHEN** `flume ack received` runs at the first prompt of a start whose record names `acceptEdits`, once with `--mode acceptEdits` and once with no `--mode`
- **THEN** the first is taken, and the second acks nothing, naming the mode asked and no mode begun (135, 161)
