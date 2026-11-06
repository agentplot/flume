## ADDED Requirements

### Requirement: Claude Code reads a session live from its records

The Claude Code profile SHALL answer live runs, naming each process,
where a `sessions/<pid>.json` under a home it is given, each account's
`CLAUDE_CONFIG_DIR` or the login environment's `~/.claude`, names the
session its start's started ack names, and a process of the host runs
with its `pid` started at its `procStart`. A record whose process is
gone or started otherwise SHALL count nothing (207, C.4).

#### Scenario: A session record and its process

- **WHEN** a scratch home's `sessions/<pid>.json` names the session with the `pid` and `procStart` of a process the test runs
- **THEN** live answers runs with that process; once the process ends, it answers none, the record still on disk (207, C.4)

#### Scenario: A reused process id

- **WHEN** the record names a `pid` the host runs, started at another time than its `procStart`
- **THEN** live answers none (207)

### Requirement: Claude Code reads a session live from the host's processes

The Claude Code profile SHALL answer live runs, naming each process,
where a process of the host whose program is `claude` holds the session
after `--resume` or `-r` in its arguments, under whatever home (207,
C.4).

#### Scenario: A resume under a home not given

- **WHEN** a process `claude --resume <session>` runs under a home the profile is not given
- **THEN** live answers runs with that process (207, S117)

### Requirement: Claude Code's live says what it cannot read

The Claude Code profile SHALL answer live unknown where a home's
`sessions` folder exists and cannot be read, naming `CLAUDE_CONFIG_DIR`
and no path, or where the host's processes cannot be listed, naming
them (207; core.30).

#### Scenario: A sessions folder that cannot be read

- **WHEN** a home's `sessions` folder exists and cannot be read
- **THEN** live answers unknown, naming `CLAUDE_CONFIG_DIR` and no path (207; core.30)

### Requirement: Claude Code's live pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source its live rests on to stand: the session record's fields
`sessionId`, `pid` and `procStart`, its `sessions` folder under the
configuration home, and `--resume`'s short form `-r` (core.22, C.4).

#### Scenario: The pinned release's session record read

- **WHEN** the test reads the program devenv.nix pins
- **THEN** each fragment live rests on is found in it, and the test names any that is not (C.4)
