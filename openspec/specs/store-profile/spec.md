# store-profile Specification

## Purpose
The store seam, Part B.7, in the host's state directory: each place's
held state written whole under the store's lock, and the host's
configuration read from its file with each setting's layer.

## Requirements

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
`[host] terminal`, the terminal bound for the host (105), and the
agents' settings by kind, `[kinds.<kind>.agents]`, and by repository and
kind, `[repos."<repository>".kinds.<kind>.agents]`, each with
`.stages.<stage>` tables, the repository keyed by its location, read at
every start (32, C.7); a file or a word left out SHALL be no bound, the
printed command and no host settings.

#### Scenario: A bound stated

- **WHEN** the configuration states `[host] agents = 2`
- **THEN** the host's bound reads two

#### Scenario: Settings for a kind and for a repository's kind

- **WHEN** the configuration gives `[kinds.change.agents] model = "opus"` and `[repos."/r".kinds.change.agents.stages.verify] model = "sonnet"`
- **THEN** the host's settings for change@1 in `/r` read `opus` for the kind at the host-kind layer and `sonnet` for verify at the host-repository layer, and for change@1 elsewhere only `opus` (32, C.7)
