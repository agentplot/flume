## ADDED Requirements

### Requirement: Claude Code's own settings passed by --settings

Where the agent's resolved keys for `claude` are not empty, the agent
profile SHALL write them as one JSON object to `own-settings.json` in
the start's folder and start `claude` with `--settings` naming that
file; where they are empty it SHALL pass no `--settings` and write no
file. It SHALL never pass a file a layer named. The release lays them
over the user's, the project's and the local settings (88, 195, 196,
C.4).

#### Scenario: The command for a stage with own settings

- **WHEN** the profile gives the command for a stage whose resolved keys for `claude` give `tui = "fullscreen"` from a named file and `statusLine` from a table's `settings`
- **THEN** it runs `claude` with `--settings <folder>/claude/<identity>/own-settings.json` after `--plugin-dir`, its files hold that path with both keys as JSON, and its line names no path of the named file (S100, 196)

#### Scenario: The command for a stage with none

- **WHEN** the profile gives the command for a stage whose settings give `claude` no key
- **THEN** it runs `claude` with no `--settings`, and its files are empty (195)

### Requirement: Claude Code's reserved keys named

The agent profile SHALL name as reserved, each with what owns it,
Claude Code's `env`, `model` and `permissions.defaultMode`, its
credential helpers `apiKeyHelper`, `awsAuthRefresh`,
`awsCredentialExport`, `gcpAuthRefresh` and `otelHeadersHelper`, its
sign-in keys `forceLoginMethod` and `forceLoginOrgUUID`,
`disableAllHooks`, and `skipDangerousModePermissionPrompt` and
`skipAutoPermissionPrompt`; and take every other key as written (34,
144, 162, 172, 195; core.12, core.30).

#### Scenario: Each reserved key

- **WHEN** the profile's reserved keys are read
- **THEN** they hold each of those keys, `permissions.defaultMode` by its path, each with its owner, the account, the model, the mode, the account's sign-in, flume's plugin or a person (195)

#### Scenario: A people's settings file

- **WHEN** a host's named file gives `tui = "fullscreen"`, `statusLine`, `hooks`, `env`, `permissions = { allow = ["Bash(git log:*)"], defaultMode = "bypassPermissions" }` and `skipDangerousModePermissionPrompt = true`
- **THEN** the command's file holds `tui`, `statusLine`, `hooks` and `permissions = { allow = ["Bash(git log:*)"] }`, and no `env`, `defaultMode` or `skipDangerousModePermissionPrompt` (S100, 195)

#### Scenario: Keys taken as written

- **WHEN** the profile is asked to take `statusLine`, `tui`, `hooks`, `permissions = { allow = ["Bash(git log:*)"] }` and `enableAllProjectMcpServers = true`
- **THEN** it takes them, and its command's file holds each as written (195)

### Requirement: A repository's tagline reaches Claude Code's settings file

Claude Code's `attribution`, its commit and pull request text and its
session link, SHALL be an own setting like any other: given by a
repository's layers for every kind, taken whole, and written into the
`--settings` file of each agent of that repository as the layer wrote
it, with nothing of flume's added to it or read in it; a repository no
layer gives it for SHALL get no `attribution` from flume (195, C.4).

#### Scenario: A fork that carries no tagline

- **WHEN** the host's `[repos."<fork>".agents.programs.claude.settings]` gives `attribution = { commit = "", pr = "", sessionUrl = false }`, and a fix and a chore run in the fork
- **THEN** the `--settings` file of every stage of each holds `attribution` as `{"commit":"","pr":"","sessionUrl":false}`, the stand-in read it, and status names `programs.claude.settings.attribution` from `host-repository` (S102, 195)

#### Scenario: A repository's own words from its file

- **WHEN** the repository's `.config/flume.toml` gives `[agents.programs.claude.settings] attribution = { commit = "Assisted-by: Claude Code", pr = "" }` and no host table gives `attribution`
- **THEN** a fix's `--settings` file holds that `attribution` as written, and status names it from `repository` (S102, 195, C.2)

#### Scenario: A repository nothing names

- **WHEN** no layer gives `attribution` and no other own setting for a fix's repository
- **THEN** the fix's agent starts with no `--settings`, and the stand-in read no `attribution` but its account's (S102, 195)

### Requirement: Own settings pinned to the release the tree holds

A check-tools test SHALL find in the pinned Claude Code, byte for byte,
the `--settings <file-or-json>` option and its words, its sources'
order with `flagSettings` after the user's, the project's and the local
settings, the merge that joins a source's lists with those beneath it,
each reserved key as its settings schema names it, and the
`attribution` schema's commit and pull request texts, each saying an
empty string hides attribution, naming any it does not find (195, C.4;
core.22).

#### Scenario: The pinned release read

- **WHEN** the test reads the program the `claude` wrapper from devenv.nix execs
- **THEN** every fragment of the profile's settings stands in it (C.4)
