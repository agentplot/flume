## ADDED Requirements

### Requirement: The terminal seam's runs

The terminal seam SHALL offer runs: given a part by its name or the
terminal's own, that it runs or does not, with the name the terminal
knows it by; or that the terminal holds no part a person can be shown,
or that it could not read the part, in the terminal's words; starting
nothing, within a bound the profile states. No terminal SHALL take a
default answer for it (98, 200, 201; core.2).

#### Scenario: Runs over the fake

- **WHEN** the fake terminal is scripted to read `work` running and `desk` unread with `no listing`, and is asked runs of `work`, `desk`, `office` and its own part
- **THEN** it answers `work` running, `desk` unread with `no listing`, `office` not running and its own part not running as `own`, logs `runs work`, `runs desk`, `runs office` and `runs own`, and opens nothing (201)

### Requirement: The type of a session's running

The seams SHALL pass one type for runs' answer, `SessionRuns`, each of
its four answers serializing to JSON and back unchanged (61, 201).

#### Scenario: Each answer round-trips

- **WHEN** `runs`, `not_running`, `no_part` and `unread`, each with its name or its reason, are written to JSON and read back
- **THEN** each equals the value written (201)
