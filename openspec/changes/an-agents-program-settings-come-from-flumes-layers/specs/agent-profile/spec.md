## ADDED Requirements

### Requirement: Claude Code's own settings passed by --settings

Where the agent's entry for `claude` holds a key, the agent profile
SHALL write it as one JSON object to `own-settings.json` in the start's
folder and start `claude` with `--settings` naming that file; where it
holds none it SHALL pass no `--settings` and write no file. The
release lays them over the user's, the project's and the local
settings (88, 175, 176, C.4).

#### Scenario: The command for a stage with own settings

- **WHEN** the profile gives the command for a stage whose entry for `claude` gives `statusLine` and `outputStyle`
- **THEN** it runs `claude` with `--settings <folder>/claude/<identity>/own-settings.json` after `--plugin-dir`, and its files hold that path with both keys as JSON (S76, 176)

#### Scenario: The command for a stage with none

- **WHEN** the profile gives the command for a stage whose settings give `claude` no entry
- **THEN** it runs `claude` with no `--settings`, and its files are empty (175)

### Requirement: Claude Code's reserved keys refused

The agent profile SHALL refuse, naming the key and what owns it,
Claude Code's `env`, `model` and `permissions.defaultMode`, its
credential helpers `apiKeyHelper`, `awsAuthRefresh`,
`awsCredentialExport`, `gcpAuthRefresh` and `otelHeadersHelper`, its
sign-in keys `forceLoginMethod` and `forceLoginOrgUUID`,
`disableAllHooks`, and `skipDangerousModePermissionPrompt` and
`skipAutoPermissionPrompt`; and take every other key as written (34,
144, 162, 172, 175; core.12, core.30).

#### Scenario: Each reserved key

- **WHEN** the profile is asked to take an entry holding one of the reserved keys, `permissions = { defaultMode = "plan" }` among them
- **THEN** it refuses naming the key, `permissions.defaultMode` by its path, and its owner, the account, the model, the mode, the account's sign-in, flume's plugin or a person (175)

#### Scenario: Keys taken as written

- **WHEN** the profile is asked to take `statusLine`, `hooks`, `permissions = { allow = ["Bash(git log:*)"] }` and `enableAllProjectMcpServers = true`
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
