## ADDED Requirements

### Requirement: The status line passed by --settings

The Claude Code profile's command SHALL pass `--settings` with one JSON string whose `statusLine` is of type `command`, its command flume's own binary by its absolute path running `hook status-line`, and its `refreshInterval` ten seconds, holding no other setting, since 2.1.296 takes no `statusLine` from a plugin's `settings.json` (56, 60, C.4; core.30, core.39).

#### Scenario: The command for a code stage

- **WHEN** the profile gives the command for a stage
- **THEN** its arguments hold `--settings` followed by `{"statusLine":{"type":"command","command":"'<flume>' hook status-line","refreshInterval":10}}`, `<flume>` the binary's absolute path, and the settings JSON holds nothing else (C.4)

### Requirement: The status line's reading pinned to the release the tree holds

A `check-tools` test SHALL require each fragment the status line rests on to stand in the pinned Claude Code's program byte for byte: a plugin's settings keys, `--settings`, the `statusLine` schema with `refreshInterval`, the order of the settings' sources, and how a status line's command is run and given its input (C.4; core.22, core.39).

#### Scenario: A release that moves the status line

- **WHEN** the pinned program lacks one of the status line's fragments
- **THEN** `flume:tools` fails naming the fragment and the release (core.22)
