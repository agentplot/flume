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

### Requirement: One watch per herdr server

The plugin SHALL keep one watch per herdr server, started at herdr's
startup and held by a lease, so starting it again while it runs does
nothing, and a server started in a stopped one's place starts its own
(122).

#### Scenario: Started twice

- **WHEN** the plugin's startup runs twice against a stand-in herdr
- **THEN** one watch runs, and the second start exits leaving it

#### Scenario: A lease whose pid was reused

- **WHEN** the lease names a pid now held by another process with a different start time
- **THEN** the lease is read stale and a new watch takes it (record 0016)

### Requirement: The watch refreshes the display clues

After writing status at each tick, the watch SHALL report each flume
pane's title and each base's workspace's count line, copied from status, by
`herdr pane report-metadata` and `herdr workspace report-metadata` with
`--source flume`, each under the bound, computing nothing, and a report
that fails SHALL be logged while the tick goes on (139, C.5; record
0018; core.14).

#### Scenario: Clues copied from status

- **WHEN** a tick writes status in which a place's stage is asking
- **THEN** the watch reports that pane's title `<stage> · asking` and its base's workspace's count line as status gives them, and sends nothing to any pane

#### Scenario: A clue that fails

- **WHEN** herdr refuses a metadata report
- **THEN** the failure is logged and the tick settles and writes status as before

### Requirement: flume runs unattended only while the watch is kept

The herdr terminal SHALL answer that it keeps watch, with its bound,
only while the plugin's watch is running, and no watch otherwise, so
flume runs attended on a host where the plugin's watch is not kept
(119, 122, C.5).

#### Scenario: The plugin not running

- **WHEN** the herdr terminal profile is bound and the plugin's watch lease is stale
- **THEN** the terminal answers no watch, and the next acting command settles every place first

### Requirement: The watch settles on its bound and at each state change

The watch SHALL run `flume settle` on the bound the herdr terminal
profile states, which it reads from flume, and again whenever herdr
reads a flume agent's state change, and SHALL run nothing of flume's
but `flume settle` and `flume status` (46, 120, 122, C.5).

#### Scenario: A state change

- **WHEN** a stand-in herdr reports a flume agent going from working to idle between two bounds
- **THEN** the watch runs `flume settle` at once, before its bound passes

#### Scenario: One listing a tick

- **WHEN** a tick reads the states of five flume panes
- **THEN** the stand-in herdr is asked one listing, and no call per pane (core.29)

#### Scenario: Nothing changes

- **WHEN** no agent's state changes for three bounds
- **THEN** the watch runs `flume settle` once per bound and nothing else of flume's

### Requirement: The watch reads a dead agent within its bound

An agent that crashed, hung or was killed, and a prompt never received,
SHALL be read by the settle the watch runs within its bound, with no
caller asking (38, 45, 119, S28).

#### Scenario: A pane killed

- **WHEN** a stand-in herdr's pane holding a flume agent is closed by hand
- **THEN** within one bound the watch runs `flume settle`, and the stand-in flume records that settle

### Requirement: The sidebar shows what status gives

The sidebar row on each agent's entry SHALL be the short form of its place's line, as `flume status --json` gives it, reported as the pane's `flume` token, and the plugin SHALL compute no stage's end, status, submission, tidy or word of a row of its own (56, 60, 61, core.14).

#### Scenario: A row from status

- **WHEN** a stand-in flume's status gives a change place's line whose short form is `code 1/2 · run 1 · tasks 2/5 · working`
- **THEN** the watch reports that pane's token `flume=code 1/2 · run 1 · tasks 2/5 · working`, and the plugin read nothing but that answer

### Requirement: Keys act through flume's commands

The plugin's keys SHALL open a place's pane, tell its agent, stop it and
retry its stage, each by herdr's focus or by one flume command, `flume
tell`, `flume stop` or `flume retry`, and SHALL never send to, open or
close an agent's pane itself (53, 107, section 9).

#### Scenario: Retry from the sidebar

- **WHEN** the retry key is pressed on a place whose construct ended short
- **THEN** the stand-in flume records one `flume retry` naming the place and its stage, and the stand-in herdr records no send to any pane

### Requirement: The popup shows each place's line

The popup SHALL show, for every place in the status the watch last wrote, its number and its line's full form, the status line's words, with its location beneath, computing none of it (56, 60; S14).

#### Scenario: Two places in the popup

- **WHEN** a stand-in flume's status gives two places, one whose line is `fix--one · fix@1 · code · run 1 · working` and one held
- **THEN** the popup's rows are `▶ 1  fix--one · fix@1 · code · run 1 · working` and `  2  ` with the second's line, each with its location beneath (S14)

### Requirement: The watch records what herdr's configuration lacks

At its start and after each settle the watch SHALL read herdr's configuration file from its server's environment and write `setup.json` in its server's state directory: what is missing, the popup's `[[keys.command]]` entry and the sidebar rows' `$flume`, each in the plugin's words naming the file, or none; it SHALL log them at its start and tell no person (72, 122).

#### Scenario: A configuration with neither

- **WHEN** the watch starts with `HERDR_CONFIG_PATH` naming a scratch file that binds no key to `agentplot.flume.places` and names no `$flume`
- **THEN** `setup.json` holds two entries, one giving the README's `[[keys.command]]` entry and naming the file, `watch.log` holds them, and the stand-in herdr recorded no notification

#### Scenario: A configuration with both

- **WHEN** the scratch file holds the README's entry and its rows, and the watch settles
- **THEN** `setup.json` is an empty array

#### Scenario: No configuration file

- **WHEN** `HERDR_CONFIG_PATH` names no file
- **THEN** `setup.json` holds both entries, naming that path

### Requirement: The popup's key is the host's entry

The manifest SHALL declare no key, since herdr 0.9.5's action has none, and SHALL describe the entry that binds one; the README SHALL give that `[[keys.command]]` entry apart from the sidebar's rows, the same as the watch's words; and the entry and the rows SHALL pass the pinned herdr's `config check` (122, C.5; core.29).

#### Scenario: The manifest and the README

- **WHEN** the manifest's check reads `herdr-plugin.toml` and the README
- **THEN** the `places` action holds a description naming `plugin_action` and `agentplot.flume.places`, and the README's entry is the one `lib/common.sh` gives

#### Scenario: The entry against the pinned herdr

- **WHEN** `herdr config check` of the pinned herdr reads a scratch configuration holding the README's entry and rows
- **THEN** it answers `config: ok`
