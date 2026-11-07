## REMOVED Requirements

### Requirement: One watch per herdr server

**Reason**: The plugin keeps no process between herdr's startup and a
person's asking: no terminal settles (122), and the clues herdr shows
are flume's, shown at its settles (139, C.5).

**Migration**: "The plugin keeps no process and runs no settle" below,
whose startup ends a watch an earlier release left running.

### Requirement: The watch refreshes the display clues

**Reason**: flume's settles show every clue herdr shows, the sidebar's
row among them, through the herdr profile's show (139, C.5); the
plugin copies none from status.

**Migration**: terminal-profiles' "herdr shows clues" and stage-runs'
"The terminal's clues follow each stage".

### Requirement: flume runs unattended only while the watch is kept

**Reason**: Whether flume runs attended is read from the host's watch,
which a caller keeps (119, 208), and no terminal or plugin says
anything of it (122).

**Migration**: settling's "A caller keeps watch over the host" and
stream-terminals' "No terminal decides whether flume runs attended".

### Requirement: The watch settles on its bound and at each state change

**Reason**: No terminal and no plugin of one settles (122). The user
ruled on 2026-11-06: "I thought we agreed that only the runner was
doing the settling. I don't know what this nudge by the Herdr server
is all about. I don't think we want it." (record 0051)

**Migration**: settling's "A caller keeps watch over the host": a
caller keeping watch, the runner's passes on a host that keeps one,
settles every place on its bound.

### Requirement: The watch reads a dead agent within its bound

**Reason**: A dead agent is read at the settles of a caller keeping
watch, or at a command that acts where none is kept (119, 208, S27,
S28); the plugin settles nothing (122).

**Migration**: settling's "Attended and watched", its "Crashed under a
watch" and "Killed with nobody watching".

### Requirement: The watch records what herdr's configuration lacks

**Reason**: The plugin keeps no watch; its startup records what herdr's
configuration lacks (C.5).

**Migration**: "The startup records what herdr's configuration lacks"
below.

## ADDED Requirements

### Requirement: The plugin keeps no process and runs no settle

The plugin SHALL keep no process between herdr's startup and a
person's asking, SHALL run no `flume settle`, and SHALL run `flume
status` only for its popup at a person's asking; it SHALL report no
clue, title or row of its own (122, 139, C.5; core.13, core.14).

#### Scenario: herdr's startup

- **WHEN** the plugin's startup runs against a stand-in herdr and a stand-in flume
- **THEN** it writes `setup.json` and exits, no process of the plugin's is left, and the stand-in flume records no command

#### Scenario: Agents changing for ten minutes

- **WHEN** after the startup a stand-in herdr's flume agents change state, come up and go for ten minutes, and the stand-in flume's status names a submitted place with no agent
- **THEN** the stand-in flume records no `flume settle` and no `flume status`, and the stand-in herdr records no `report-metadata` of the plugin's (122, 208)

#### Scenario: A watch an earlier release left

- **WHEN** the startup runs while a watch of an earlier release holds a fresh lease on the same server
- **THEN** that watch is ended by the pid and identity its lease names, never another process holding the pid, its lease is removed, and no process of the plugin's is left (record 0016)

### Requirement: The startup records what herdr's configuration lacks

At herdr's startup the plugin SHALL read herdr's configuration file
from its server's environment and write `setup.json` in its server's
state directory, with its server's socket: what is missing, the
popup's `[[keys.command]]` entry and the sidebar rows' `$flume`, each
in the plugin's words naming the file, or none. It SHALL log them and
tell no person (72, C.5).

#### Scenario: A configuration with neither

- **WHEN** the startup runs with `HERDR_CONFIG_PATH` naming a scratch file that binds no key to `agentplot.flume.places` and names no `$flume`
- **THEN** `setup.json` holds two entries, one giving the README's `[[keys.command]]` entry and naming the file, the plugin's log holds them, and the stand-in herdr recorded no notification

#### Scenario: A configuration with both

- **WHEN** the scratch file holds the README's entry and its rows, and the startup runs
- **THEN** `setup.json` holds an empty array

#### Scenario: No configuration file

- **WHEN** `HERDR_CONFIG_PATH` names no file
- **THEN** `setup.json` holds both entries, naming that path

## MODIFIED Requirements

### Requirement: The sidebar shows what status gives

The sidebar row on each agent's entry SHALL be the short form of its
place's line, which flume's herdr profile reports as the pane's
`flume` token at each settle that changes it (139, C.5); the plugin
SHALL compute and report no stage's end, status, submission, tidy or
word of a row of its own (56, 60, 61; core.14).

#### Scenario: A row from status

- **WHEN** a settle changes a change place's line to one whose short form is `code 1/2 · run 1 · tasks 2/5 · working`
- **THEN** the stand-in herdr records that pane's token `flume=code 1/2 · run 1 · tasks 2/5 · working` reported by flume's settle, and nothing of the plugin's

### Requirement: The popup shows each place's line

The popup SHALL read `flume status --json` once when a person opens
it and again after each key that acts, and never on a timer, and show
for every place that status gives its number and its line's full form,
the status line's words, with its location beneath, computing none of
it (56, 60, 122; S14).

#### Scenario: Two places in the popup

- **WHEN** a stand-in flume's status gives two places, one whose line is `fix--one · fix@1 · code · run 1 · working` and one held, and the popup opens
- **THEN** the popup's rows are `▶ 1  fix--one · fix@1 · code · run 1 · working` and `  2  ` with the second's line, each with its location beneath (S14)

#### Scenario: A popup left open

- **WHEN** the popup stays open for ten minutes with no key, then the retry key is pressed on a place
- **THEN** the stand-in flume records one `flume status --json` at the opening, one `flume retry`, and one `flume status --json` after it, and nothing else

### Requirement: The popup's key is the host's entry

The manifest SHALL declare no key, since herdr 0.9.5's action has none, and SHALL describe the entry that binds one; the README SHALL give that `[[keys.command]]` entry apart from the sidebar's rows, the same as the startup's words; and the entry and the rows SHALL pass the pinned herdr's `config check` (C.5; core.29).

#### Scenario: The manifest and the README

- **WHEN** the manifest's check reads `herdr-plugin.toml` and the README
- **THEN** the `places` action holds a description naming `plugin_action` and `agentplot.flume.places`, and the README's entry is the one `lib/common.sh` gives

#### Scenario: The entry against the pinned herdr

- **WHEN** `herdr config check` of the pinned herdr reads a scratch configuration holding the README's entry and rows
- **THEN** it answers `config: ok`
