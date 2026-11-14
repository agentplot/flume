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
start or a turn, and each but SessionStart SHALL do nothing in a session
flume did not start (C.4, section 9; record 0016).

#### Scenario: A session the operator started

- **WHEN** each hook but SessionStart runs with no `FLUME_AGENT` in its environment
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
- **THEN** it compares nothing, acks nothing and records nothing of a start, runs `flume inbox` as "SessionStart gives a caller's session what waits on it" says, and exits 0 (43, C.4)

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

### Requirement: SessionStart compares the session a start resumes

Where the start's record names the session it resumes, the SessionStart
hook SHALL compare its input's `session_id` with it before it acks; on
a difference it SHALL ack nothing, record that the agent came up in
another session, naming none, start a settle in the background and exit
0 (35, 121, 161, 179, C.4).

#### Scenario: A carried start in another session

- **WHEN** the hook runs for a start whose record names the session `s-1`, its input naming `s-2`
- **THEN** no started ack is written, the agent's record says it came up in another session and names neither, a settle of the place is started, and the hook exits 0 (161, 179)

#### Scenario: A start that resumes nothing

- **WHEN** the hook runs for a start whose record names no session
- **THEN** it compares no session, and acks on the program and the variables as before (161)

### Requirement: The started ack records the transcript's length once

The started ack SHALL record the length of the session's transcript
when the start's first ack came, none written reading as 0, and a later
SessionStart of the start, a compaction's or a clear's, SHALL keep the
length the first recorded (91, 179, C.4).

#### Scenario: A carried start's length

- **WHEN** the hook acks a carried start whose transcript holds 4,096 bytes
- **THEN** the ack records 4,096 (179)

#### Scenario: A compaction after the ack

- **WHEN** a start acked at a length of 4,096 compacts, and SessionStart runs again with `source` `compact`
- **THEN** the ack still records 4,096 (91, 179)

### Requirement: The plugin pins the place's line under the prompt

In a session flume started, its environment naming `FLUME_PLACE` and `FLUME_AGENT`, flume's Claude Code plugin SHALL pin the line `flume hook status-line` prints as the plugin's own status line with `$.ui.status`, from a hooks module, at the session's start, after each tool call, at each turn's end and every ten seconds while the session lives, removing it where the hook prints nothing; outside such a session it SHALL pin nothing and run nothing (56, 60, 72, C.4; core.13).

#### Scenario: A code agent's session

- **WHEN** the module runs at `session.start` under the pinned engine's `plugin test`, the session's environment naming a place and an agent, and the hook prints `fix--one · fix@1 · code · run 1 · working`
- **THEN** the plugin's pinned status line is `fix--one · fix@1 · code · run 1 · working`

#### Scenario: Redrawn while the session is idle

- **WHEN** ten seconds pass after the session's start and the hook then prints `fix--one · fix@1 · code · run 1 · held`
- **THEN** the pinned status line is `fix--one · fix@1 · code · run 1 · held`, with no tool call or turn between

#### Scenario: A place the core no longer reads

- **WHEN** the hook prints nothing at a tick
- **THEN** the plugin's pinned status line is removed

#### Scenario: Outside a session flume started

- **WHEN** the module runs at `session.start` in a session whose environment names no `FLUME_PLACE`
- **THEN** it pins nothing, runs no process and registers no clock

### Requirement: hook status-line prints the place's line

In a session flume started, `flume hook status-line` SHALL print, on one line, the line of the place `FLUME_PLACE` names, asked alone with the identity `FLUME_AGENT` names, as the core gives it, and SHALL exit 0, printing nothing where the core cannot read the place or outside a session flume started (56, 57, 60; core.14).

#### Scenario: A code agent's line

- **WHEN** the hook runs in the environment of a fix place's code agent flume launched under the printed command
- **THEN** it prints `fix--one · fix@1 · code · run 1 · starting`, the same as `flume status fix--one --line`, and exits 0 (S14)

#### Scenario: Outside a session flume started

- **WHEN** the hook runs with no `FLUME_PLACE` and no `FLUME_AGENT`
- **THEN** it prints nothing, runs nothing, and exits 0

### Requirement: The status line writes nothing

`flume hook status-line` SHALL write nothing under flume's state, the place or the account's configuration, settle nothing and start no agent (57; core.16).

#### Scenario: Every file unchanged

- **WHEN** the hook runs ten times in a code agent's environment
- **THEN** every file under the scratch state directory has the digest it had before, and the place's held state is at the same version (57)

### Requirement: The engine the module rests on pinned to the release the tree holds

The plugin SHALL keep the engine types the pinned Claude Code writes, a
`check-fast` task SHALL fail where they differ from the pinned
release's, and its modules and tests SHALL be typed against them, so
`$.ui.status`, `$.clock.every`, `$.process.run` with its `stdin`,
`$.env.get`, `session.measure` with its `changed` and `rateLimits`, and
the other events the modules register on are the pinned release's (C.4;
core.22, core.39).

#### Scenario: A release that moves the engine

- **WHEN** the vendored engine types differ from those the pinned Claude Code writes
- **THEN** `plugin-claude:engine-types` fails naming the file and the release (core.22)

### Requirement: SessionStart gives a caller's session what waits on it

In a session flume did not start, SessionStart SHALL run `flume inbox`
as the session the terminal names, never as a role's address,
confirming on receipt each end it is handed, and SHALL give the agent,
as added context, each end and each waiting place it answers with the
key, whether its agent is kept open and until when, and how to answer,
hold or leave it; or nothing where it answers nothing, no session is
named, or flume does not answer (43, C.4; weir 103).

#### Scenario: A restarted pane

- **WHEN** a session flume did not start begins in the pane `ops`, a place whose recipient is `ops` waits on a retry of construct with a stop-short handed over before, and SessionStart runs
- **THEN** the hook's output gives the agent the place, the stop-short's words, its key and the three ways on, the stop-short reads handed to `ops` by `session_start` and confirmed by it, and the hook exits 0 (S94)

#### Scenario: Nothing waits

- **WHEN** the same hook runs for a session nothing is pending for and nothing waits on
- **THEN** it gives no output and exits 0

#### Scenario: An address in the environment

- **WHEN** SessionStart runs in a session flume did not start with `FLUME_RECIPIENT=east/planner`, and an end stands pending for `east/planner`
- **THEN** the hook collects as the session the terminal names, and the end stays pending for `east/planner`, handed to it by nothing

#### Scenario: flume unreachable

- **WHEN** the configuration cannot be read and SessionStart runs in a session flume did not start
- **THEN** the hook gives no output and exits 0

### Requirement: The plugin passes each report of the windows to hook usage

In a session whose environment names `FLUME_PLACE` and `FLUME_AGENT`,
a hooks module of flume's plugin SHALL, at each `session.measure` whose
`changed` names `rateLimits` and whose `rateLimits` holds a window, run
`flume hook usage` with `{"rateLimits": …}` on its standard input,
waiting on no run; at any other measure, and in any other session, it
SHALL run nothing (184, 211, C.4; core.13).

#### Scenario: A report passed

- **WHEN** the module runs at `session.measure` under the pinned engine's `plugin test`, the session's environment naming a place and an agent, `changed` naming `context` and `rateLimits`, and `rateLimits` giving `five_hour` 42 resetting at `2026-11-05T17:00:00.000Z` and `seven_day` 18
- **THEN** it runs `flume hook usage` once, its standard input `{"rateLimits": …}` holding those two windows as given (211, C.4)

#### Scenario: A measure of the context alone

- **WHEN** `changed` names `context` alone
- **THEN** the module runs nothing (C.4)

#### Scenario: No window to pass

- **WHEN** `changed` names `rateLimits` and `rateLimits` is empty
- **THEN** the module runs nothing (184)

#### Scenario: A run that fails

- **WHEN** the hook exits 1 at one measure, and cannot be found at the next
- **THEN** the module throws nothing into the engine, and runs the hook again at the measure after (C.4)

#### Scenario: Outside a session flume started

- **WHEN** the module runs at `session.measure` in a session whose environment names no `FLUME_PLACE`, or no `FLUME_AGENT`
- **THEN** it runs no process (C.4)

### Requirement: hook usage keeps the windows and has the core take them

In a session flume started, `flume hook usage` SHALL keep the windows
on its input beside the start's record with the time it ran, a later
report kept there standing, and ask the core to take the start's
reading with `FLUME_AGENT` and `FLUME_SECRET`; a report with a window
in no form the profile reads SHALL keep nothing. It SHALL print nothing
and exit 0, and in any other session read and write nothing (184, 211,
C.4).

#### Scenario: A report under an account

- **WHEN** a fix's code agent flume launched under work-1 with the printed command runs the hook with `five_hour` 42 resetting at 17:00 and `seven_day` 107.5
- **THEN** `accounts/work/work-1.json` holds a reading taken at the hook's time from the code agent's identity and the session its started ack names, `five_hour` 42 resetting at 17:00 and `seven_day` 100, `flume status --json` gives it fresh, and the hook printed nothing and exited 0 (184, 185, 211)

#### Scenario: A window in no such form

- **WHEN** the hook runs with a window naming no `kind`, then one whose `percentUsed` is `"42%"`, then one whose `resetsAt` is `at five`
- **THEN** each keeps nothing, the account's reading stands as it was, and the hook exits 0 each time (184)

#### Scenario: A start under no account

- **WHEN** a code agent flume launched on a host with no pool runs the hook with one window
- **THEN** the report is kept beside its start's record, no account's document is written, and the hook exits 0 (184)

#### Scenario: Outside a session flume started

- **WHEN** the hook runs with no `FLUME_AGENT` and no `FLUME_SECRET`
- **THEN** it prints nothing, every file under the scratch state directory has the digest it had before, and it exits 0 (C.4)
