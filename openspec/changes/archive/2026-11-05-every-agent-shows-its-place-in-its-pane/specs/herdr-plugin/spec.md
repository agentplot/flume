## MODIFIED Requirements

### Requirement: The sidebar shows what status gives

The sidebar row on each agent's entry SHALL be the short form of its place's line, as `flume status --json` gives it, reported as the pane's `flume` token, and the plugin SHALL compute no stage's end, status, submission, tidy or word of a row of its own (56, 60, 61, core.14).

#### Scenario: A row from status

- **WHEN** a stand-in flume's status gives a change place's line whose short form is `code 1/2 · run 1 · tasks 2/5 · working`
- **THEN** the watch reports that pane's token `flume=code 1/2 · run 1 · tasks 2/5 · working`, and the plugin read nothing but that answer

## ADDED Requirements

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
