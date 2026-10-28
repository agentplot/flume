## ADDED Requirements

### Requirement: One document per place in the host's state directory

The store profile SHALL keep each place's held state as one document
under `$XDG_STATE_HOME/flume/places/`, with its version, written whole
by rename under a lock and refused when the version moved, a new place's
created only where none exists, and its own folder beside it, gone with
the document (62, 102 to 104, 128, C.7).

#### Scenario: Seam scenarios over the profile

- **WHEN** the store seam's scenarios run against the profile over a scratch state directory
- **THEN** each passes, two processes writing at one version leaving one write

#### Scenario: A restart loses nothing

- **WHEN** a second engine is built over the same state directory while a stage works
- **THEN** it reads the place whole (S12, 77)

### Requirement: The host's configuration

The host's configuration SHALL be read from
`$XDG_CONFIG_HOME/flume/config.toml`: `[host] agents`, the bound (70),
and `[host] terminal`, the terminal bound for the host (105); a file or a
word left out SHALL be no bound and the printed command.

#### Scenario: A bound stated

- **WHEN** the configuration states `[host] agents = 2`
- **THEN** the host's bound reads two
