## ADDED Requirements

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

## MODIFIED Requirements

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
