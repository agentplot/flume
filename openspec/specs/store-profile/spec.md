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
`$XDG_CONFIG_HOME/flume/config.toml` at every start: `[host]` `agents`,
`terminal` and `pool`; `[streams.<stream>]`; `[pools.<pool>]`; agents'
model and instructions by kind and by repository and kind, each with
`.stages.<stage>`; `stream` where C.7 names it; and `[roles]`. A key
left out SHALL be no bound, the printed command, no stream, pool or
settings (32, 70, 105, 143, 144, C.7).

#### Scenario: A bound stated

- **WHEN** the configuration states `[host] agents = 2`
- **THEN** the host's bound reads two

#### Scenario: Settings for a kind and for a repository's kind

- **WHEN** the configuration gives `[kinds.change.agents] model = "opus"` and `[repos."/r".kinds.change.agents.stages.verify] model = "sonnet"`
- **THEN** the host's settings for change@1 in `/r` read `opus` for the kind at the host-kind layer and `sonnet` for verify at the host-repository layer, and for change@1 elsewhere only `opus` (32, C.7)

#### Scenario: C.7's example

- **WHEN** the configuration is C.7's example, with streams `work` and `personal`, pools `work` and `personal`, and `[repos."~/Code/acme"] stream = "work"`
- **THEN** acme's places read the stream `work` from `host-repository` and the pool `work` from the stream, whose accounts read work-1, bounded at four, then work-2, each with its `CLAUDE_CONFIG_DIR` under the host user's home (143, 144, C.7)

#### Scenario: A configuration that refuses

- **WHEN** the configuration gives a stream `pool = "missing"` that no `[pools]` table declares, or a pool `strategy = "round-robin"`
- **THEN** every launch and start is refused naming the key, and status says the host's configuration cannot be read (4, C.7)

### Requirement: The host's own documents

The store profile SHALL keep each account's spent time at
`accounts/<pool>/<account>.json` and each intent's affinity at
`intents/<digest>.json` under `$XDG_STATE_HOME/flume/`, each with its
version, written whole by compare-and-set under the store's lock and
readable by the host's user alone, and reached by every flume process
on the host (146, 147, 153, C.7).

#### Scenario: Two processes record one account spent

- **WHEN** two engines over one state directory each write an account's spent time at the version both read
- **THEN** one write lands and the other reads the version moved and writes again on what it reads (153)

#### Scenario: A spent time seen by another process

- **WHEN** one engine records work-1 spent until 15:00
- **THEN** a second engine built over the same state directory reads work-1 spent until 15:00, and the file's mode is 0600 (146, 153)
