## ADDED Requirements

### Requirement: The settings' effort passed by --effort

The agent profile SHALL start `claude` with `--effort` and the
settings' effort where they give one, and with no `--effort` where they
give none (197, C.4).

#### Scenario: The command for a stage with no effort

- **WHEN** the profile gives the command for a stage whose settings give no effort
- **THEN** it runs `claude` with no `--effort` (197)

#### Scenario: The command for a stage at xhigh

- **WHEN** the profile gives the command for a stage whose settings give the model `opus` and the effort `xhigh`
- **THEN** it runs `claude` with `--model opus --effort xhigh` (S103, C.4)

### Requirement: Claude Code takes the pinned release's five efforts

The Claude Code profile SHALL take `low`, `medium`, `high`, `xhigh` and
`max`, the pinned 2.1.296's `--effort` words, and SHALL refuse any
other, naming it, `claude` and the five; and SHALL name
`CLAUDE_CODE_EFFORT_LEVEL` as the variable its program reads over
`--effort` (197, C.4).

#### Scenario: Each of the five, and two the release reads otherwise

- **WHEN** the profile is asked of `low`, `medium`, `high`, `xhigh`, `max`, `med` and `ultracode`
- **THEN** it takes the first five, and refuses `med` and `ultracode`, each naming it, `claude` and the five (197, C.4)

### Requirement: Claude Code's effortLevel is flume's effort's

The Claude Code profile SHALL reserve its settings' `effortLevel` with
the owner flume's `effort`, beside `model`: a layer's `settings` that
writes it SHALL be refused naming the key and its owner, and a named
file's SHALL be left out and named in status; `modelSettings` and
`maxEffortLevel` SHALL NOT be reserved (195, 197, C.4).

#### Scenario: effortLevel written in a table

- **WHEN** the host's `[agents.programs.claude.settings]` gives `effortLevel = "xhigh"`
- **THEN** every launch and start of an agent on Claude Code is refused naming `effortLevel`, `host` and flume's `effort` (195, 197)

#### Scenario: effortLevel in a named file

- **WHEN** the host's `[agents.programs.claude]` names a file giving `effortLevel`, `maxEffortLevel` and `tui`
- **THEN** the agent's `--settings` file holds `maxEffortLevel` and `tui`, and status names `effortLevel` left out with the file, `host` and flume's `effort` (195)

### Requirement: The efforts pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source the profile's efforts rest on to stand byte for byte: the
`--effort` option with its words, the release's list of the five, the
variable read over it, and `effortLevel`'s declaration in the settings
schema (core.22, C.4).

#### Scenario: The pinned release's efforts read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the efforts rest on is found in it, and the test names any that is not (197, C.4)

## MODIFIED Requirements

### Requirement: Claude Code's reserved keys named

The agent profile SHALL name as reserved, each with what owns it,
Claude Code's `env`, `model`, `effortLevel` and
`permissions.defaultMode`, its credential helpers `apiKeyHelper`, `awsAuthRefresh`,
`awsCredentialExport`, `gcpAuthRefresh` and `otelHeadersHelper`, its
sign-in keys `forceLoginMethod` and `forceLoginOrgUUID`,
`disableAllHooks`, and `skipDangerousModePermissionPrompt` and
`skipAutoPermissionPrompt`; and take every other key as written (34,
144, 162, 172, 195, 197; core.12, core.30).

#### Scenario: Each reserved key

- **WHEN** the profile's reserved keys are read
- **THEN** they hold each of those keys, `permissions.defaultMode` by its path, each with its owner, the account, the model, the effort, the mode, the account's sign-in, flume's plugin or a person (195, 197)

#### Scenario: A people's settings file

- **WHEN** a host's named file gives `tui = "fullscreen"`, `statusLine`, `hooks`, `env`, `permissions = { allow = ["Bash(git log:*)"], defaultMode = "bypassPermissions" }` and `skipDangerousModePermissionPrompt = true`
- **THEN** the command's file holds `tui`, `statusLine`, `hooks` and `permissions = { allow = ["Bash(git log:*)"] }`, and no `env`, `defaultMode` or `skipDangerousModePermissionPrompt` (S100, 195)

#### Scenario: Keys taken as written

- **WHEN** the profile is asked to take `statusLine`, `tui`, `hooks`, `permissions = { allow = ["Bash(git log:*)"] }` and `enableAllProjectMcpServers = true`
- **THEN** it takes them, and its command's file holds each as written (195)
