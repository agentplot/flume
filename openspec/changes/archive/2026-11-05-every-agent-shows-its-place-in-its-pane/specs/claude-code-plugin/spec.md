## ADDED Requirements

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

The plugin SHALL keep the engine types the pinned Claude Code writes, a `check-fast` task SHALL fail where they differ from the pinned release's, and its module and tests SHALL be typed against them, so `$.ui.status`, `$.clock.every`, `$.process.run`, `$.env.get` and the events the module registers on are the pinned release's (C.4; core.22, core.39).

#### Scenario: A release that moves the engine

- **WHEN** the vendored engine types differ from those the pinned Claude Code writes
- **THEN** `plugin-claude:engine-types` fails naming the file and the release (core.22)
