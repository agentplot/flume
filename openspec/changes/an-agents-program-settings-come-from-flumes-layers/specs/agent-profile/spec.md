## ADDED Requirements

### Requirement: Claude Code's own settings passed by --settings

Where the agent's resolved keys for `claude` are not empty, the agent
profile SHALL write them as one JSON object to `own-settings.json` in
the start's folder and start `claude` with `--settings` naming that
file; where they are empty it SHALL pass no `--settings` and write no
file. It SHALL never pass a file a layer named. The release lays them
over the user's, the project's and the local settings (88, 175, 176,
C.4).

#### Scenario: The command for a stage with own settings

- **WHEN** the profile gives the command for a stage whose resolved keys for `claude` give `tui = "fullscreen"` from a named file and `statusLine` from a table's `settings`
- **THEN** it runs `claude` with `--settings <folder>/claude/<identity>/own-settings.json` after `--plugin-dir`, its files hold that path with both keys as JSON, and its line names no path of the named file (S76, 176)

#### Scenario: The command for a stage with none

- **WHEN** the profile gives the command for a stage whose settings give `claude` no key
- **THEN** it runs `claude` with no `--settings`, and its files are empty (175)

### Requirement: Claude Code's reserved keys named

The agent profile SHALL name as reserved, each with what owns it,
Claude Code's `env`, `model` and `permissions.defaultMode`, its
credential helpers `apiKeyHelper`, `awsAuthRefresh`,
`awsCredentialExport`, `gcpAuthRefresh` and `otelHeadersHelper`, its
sign-in keys `forceLoginMethod` and `forceLoginOrgUUID`,
`disableAllHooks`, and `skipDangerousModePermissionPrompt` and
`skipAutoPermissionPrompt`; and take every other key as written (34,
144, 162, 172, 175; core.12, core.30).

#### Scenario: Each reserved key

- **WHEN** the profile's reserved keys are read
- **THEN** they hold each of those keys, `permissions.defaultMode` by its path, each with its owner, the account, the model, the mode, the account's sign-in, flume's plugin or a person (175)

#### Scenario: A people's settings file

- **WHEN** a host's named file gives `tui = "fullscreen"`, `statusLine`, `hooks`, `env`, `permissions = { allow = ["Bash(git log:*)"], defaultMode = "bypassPermissions" }` and `skipDangerousModePermissionPrompt = true`
- **THEN** the command's file holds `tui`, `statusLine`, `hooks` and `permissions = { allow = ["Bash(git log:*)"] }`, and no `env`, `defaultMode` or `skipDangerousModePermissionPrompt` (S76, 175)

#### Scenario: Keys taken as written

- **WHEN** the profile is asked to take `statusLine`, `tui`, `hooks`, `permissions = { allow = ["Bash(git log:*)"] }` and `enableAllProjectMcpServers = true`
- **THEN** it takes them, and its command's file holds each as written (175)

### Requirement: Own settings pinned to the release the tree holds

A check-tools test SHALL find in the pinned Claude Code, byte for byte,
the `--settings <file-or-json>` option and its words, its sources'
order with `flagSettings` after the user's, the project's and the local
settings, the merge that joins a source's lists with those beneath it,
and each reserved key as its settings schema names it, naming any it
does not find (175, C.4; core.22).

#### Scenario: The pinned release read

- **WHEN** the test reads the program the `claude` wrapper from devenv.nix execs
- **THEN** every fragment of the profile's settings stands in it (C.4)
