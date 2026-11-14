## RENAMED Requirements

- FROM: `### Requirement: /launch and status over MCP`
- TO: `### Requirement: /launch, and launch, status, message and wake over MCP`

## MODIFIED Requirements

### Requirement: /launch, and launch, status, message and wake over MCP

The plugin SHALL give every session `/launch <brief>`, which launches
through `flume launch --json` and reports the place, and SHALL serve
over MCP by `flume mcp` the tools `status`, answering what `flume status
--json` answers and settling nothing, and `message`, `launch` and
`wake`, each of the last three turned into the command line's arguments
and run by its parser and act, its result the act's JSON and a refusal
marked an error (7, 53, 57, 60, 132, section 9).

#### Scenario: status over MCP

- **WHEN** a scripted MCP session initializes `flume mcp`, lists its tools and calls `status`
- **THEN** the tool's result is the JSON `flume status --json` gives over the same state, and nothing is written

#### Scenario: message over MCP

- **WHEN** a scripted MCP session calls `message` with the target `east/planner` and words, on a host whose source declares the address with a run
- **THEN** the tool's result is the JSON `flume message east/planner <words> --json` gives, its key `message/east/planner/<token>`

#### Scenario: launch over MCP, refused as the command line refuses it

- **WHEN** a scripted MCP session calls `launch` naming a kind no declaration holds
- **THEN** the tool's result is the refusal `flume launch` gives for it, marked an error, and no place is made (4)

### Requirement: A later prompt or SessionStart of a started session compares no mode

A prompt after the start's first, a message among them, and a
SessionStart after its session resumed, cleared or compacted, SHALL
compare no mode, and SHALL ack where everything else matches, a
SessionStart recording the session's transcript, since the mode is read
at the agent's start and a started session's mode is its own (161,
162).

#### Scenario: A later prompt and a compaction after the mode moved

- **WHEN** a start's first prompt was taken in `acceptEdits`, a later prompt's input reads `default`, and the session compacts
- **THEN** the later prompt is acked received, the started ack is written again with the session's transcript, and nothing is recorded as unsettled (161, 162)

## ADDED Requirements

### Requirement: The MCP tools take the command line's words

`flume mcp`'s `message` SHALL take `target`, `words`, `within` and
`answers`; its `launch` SHALL take `kind`, `name`, `brief` and the
command's other words and switches by their long names, `base`,
`recipient`, `hold`, `hold_submission`, `mark`, `priority`,
`stall_after`, `agent`, `stream` and `intent`, arrays where the flag
repeats; its `wake` SHALL take `address`; and `status` alone SHALL be
marked read-only (7, 53, 60, 132, section 9).

#### Scenario: The tools listed

- **WHEN** a scripted MCP session lists `flume mcp`'s tools
- **THEN** it lists `status`, `message`, `launch` and `wake`, each with its input schema, `status` alone read-only

### Requirement: The deliver mod runs the hook at the session's wait

In a session whose environment names `FLUME_PLACE`, `FLUME_AGENT` and
`FLUME_DELIVER`, flume's plugin SHALL run `flume hook deliver` from its
hooks module, `hooks/flume.ts`, at the session's start and at each
turn's end, each after the engine's own, and every ten seconds while no
turn runs, one run at a time, each bounded at ten seconds; outside such
a session it SHALL register no clock and run nothing (54, 214, 215,
C.4; core.13).

#### Scenario: Words at a turn's end

- **WHEN** the module runs over a host whose `flume hook deliver` prints a batch, in a session naming the three variables, and a turn completes
- **THEN** it runs `flume hook deliver` once after the turn and submits the batch once, as the user's words, and the prompt box is not written

### Requirement: The deliver mod submits what the hook prints

The module SHALL submit what a run that exits 0 prints by
`$.prompt.submit` with `asUser`, leaving the prompt box as it stands,
and SHALL submit nothing for a run that fails or prints nothing (54,
215, C.4).

#### Scenario: Nothing to deliver

- **WHEN** the hook prints nothing, or exits 1, at a turn's end
- **THEN** nothing is submitted

#### Scenario: Idle ticks

- **WHEN** the session is idle and the module's clock moves thirty seconds, then a turn starts and the clock moves thirty seconds more
- **THEN** the hook runs three times while idle and none while the turn runs

#### Scenario: A session without the variable

- **WHEN** the module runs in a session naming `FLUME_PLACE` and `FLUME_AGENT` but no `FLUME_DELIVER`, and in one flume did not start
- **THEN** it registers no clock and runs nothing in either

### Requirement: The plugin carries its one hooks module

`hooks/hooks.json` SHALL name `./flume.ts` as its one module, which
pins the place's line and holds the deliver part, since the pinned
Claude Code takes one hooks module per plugin, each event's hook once
in it and `$` into no function of another file; and the binary SHALL
write it into the plugin it gives each agent, naming the binary by its
absolute path, the plugin passing the pinned Claude Code's strict
validation (C.4).

#### Scenario: The plugin written for an agent

- **WHEN** the binary writes the plugin for a start
- **THEN** its `hooks/flume.ts` names the binary's absolute path where `plugins/claude` names `flume`, it holds no `hooks/line.ts`, and `claude plugin validate --strict` passes over it
