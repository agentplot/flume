## ADDED Requirements

### Requirement: The terminal seam gives what its configuration lacks for flume's plugin

The terminal seam SHALL offer `setup`, what the terminal's own configuration lacks for flume's plugin there, each in words a person acts on, none by default, and the fake terminal SHALL script it (122).

#### Scenario: A terminal with no plugin

- **WHEN** a terminal profile that has no plugin, as the printed command, is asked its setup
- **THEN** it gives none

#### Scenario: The fake's scripted setup

- **WHEN** the fake terminal is scripted with one missing entry and status is asked
- **THEN** status gives that entry, and the fake recorded no open, send or close (57)
