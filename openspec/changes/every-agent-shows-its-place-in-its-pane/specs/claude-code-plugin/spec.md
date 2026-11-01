## ADDED Requirements

### Requirement: hook status-line prints the place's line

In a session flume started, `flume hook status-line` SHALL print, on one line, the line of the place `FLUME_PLACE` names, asked alone with the identity `FLUME_AGENT` names, as the core gives it, and SHALL exit 0, printing nothing of flume's where the core cannot read the place (56, 57, 60; core.14).

#### Scenario: A code agent's status line

- **WHEN** the hook runs with 2.1.296's status line input in the environment of a fix place's code agent flume launched under the printed command
- **THEN** it prints `fix--one · fix@1 · code · run 1 · starting` as its first line, the same as `flume status fix--one --line`, and exits 0 (S14)

#### Scenario: Outside a session flume started

- **WHEN** the hook runs with no `FLUME_PLACE` and no `FLUME_AGENT`, a scratch account's settings naming a status line of its own
- **THEN** it prints nothing, runs nothing, and exits 0

### Requirement: The session's own status line beneath flume's

In a session flume started, the hook SHALL then run the status line of type `command` that the last of the account's, the place's and the place's local Claude Code settings gives, flume's own passed over, with the same input on standard input and `FLUME_STATUS_LINE` set, under a bound of two seconds, printing its output beneath flume's line, and nothing where it fails or passes the bound (core.13).

#### Scenario: The user's own status line

- **WHEN** the hook runs in a code agent's environment whose account's `settings.json` gives a `statusLine` command that prints `mine` and the input's `session_id`
- **THEN** flume's line is printed first and `mine` with the session's id beneath it

#### Scenario: The place's local settings over the account's

- **WHEN** the account's settings give a status line printing `account` and the place's `.claude/settings.local.json` gives one printing `local`
- **THEN** `local` is printed beneath flume's line and `account` is not

#### Scenario: A status line past its bound

- **WHEN** the account's status line command sleeps for ten seconds
- **THEN** the hook prints flume's line alone and exits within three seconds, and no process the command started still runs (core.13)

#### Scenario: A status line that calls the hook

- **WHEN** the account's status line command runs `flume hook status-line` and prints `mine`
- **THEN** flume's line is printed once, with `mine` beneath it

### Requirement: The status line writes nothing

`flume hook status-line` SHALL write nothing under flume's state, the place or the account's configuration, settle nothing and start no agent (57; core.16).

#### Scenario: Every file unchanged

- **WHEN** the hook runs ten times in a code agent's environment
- **THEN** every file under the scratch state directory has the digest it had before, and the place's held state is at the same version (57)
