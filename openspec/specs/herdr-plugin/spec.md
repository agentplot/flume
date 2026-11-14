# herdr-plugin Specification

## Purpose
flume's herdr plugin, kept outside the core: the watch on herdr's server
that settles flume's places with no process of flume's own, so flume
runs unattended, and the sidebar that shows every place in flight with
keys that act through flume's commands (46, 60, 119 to 122, C.5,
section 9; record 0005; core.13, core.14).

## Requirements

### Requirement: The plugin installs as herdr reads it

The plugin SHALL be a herdr plugin at `plugins/herdr` whose manifest
herdr reads, naming the least herdr version it needs, every command it
names present in the plugin, and SHALL install by herdr's own install
from flume's repository at a tag (C.5).

#### Scenario: The manifest checked

- **WHEN** the plugin's manifest check runs
- **THEN** it passes only when the manifest parses as herdr reads it, names its least herdr version, and every script a startup, event, action or pane names exists and runs under the system's `sh`

### Requirement: The sidebar shows what status gives

The sidebar row on each agent's entry SHALL be the short form of its
place's line, which flume's herdr profile reports as the pane's
`flume` token at each settle that changes it (139, C.5); the plugin
SHALL compute and report no stage's end, status, submission, tidy or
word of a row of its own (56, 60, 61; core.14).

#### Scenario: A row from status

- **WHEN** a settle changes a change place's line to one whose short form is `code 1/2 · run 1 · tasks 2/5 · working`
- **THEN** the stand-in herdr records that pane's token `flume=code 1/2 · run 1 · tasks 2/5 · working` reported by flume's settle, and nothing of the plugin's

### Requirement: Keys act through flume's commands

The plugin's keys SHALL open a place's pane, message its agent, stop it
and retry its stage, each by herdr's focus or by one flume command,
`flume message`, `flume stop` or `flume retry`, the popup's key for a
message `m`, and SHALL never send to, open or close an agent's pane
itself (53, 107, section 9).

#### Scenario: Retry from the sidebar

- **WHEN** the retry key is pressed on a place whose construct ended short
- **THEN** the stand-in flume records one `flume retry` naming the place and its stage, and the stand-in herdr records no send to any pane

#### Scenario: Message, stop and open, each one key

- **WHEN** `m` is pressed with words, then `s` with the force asked, then `o`, on a working place
- **THEN** the stand-in flume records `flume message` naming the place with the words, then `flume stop` naming it forced, herdr focuses the place's tab, and the stand-in herdr records no send to any pane

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
