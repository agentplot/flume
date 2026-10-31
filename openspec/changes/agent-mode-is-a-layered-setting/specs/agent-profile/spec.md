## ADDED Requirements

### Requirement: The settings' mode passed by --permission-mode

The agent profile SHALL start `claude` with `--permission-mode` and the
value its map gives the settings' mode where they give one, and with no
`--permission-mode` where they give none (162, 169, C.4).

#### Scenario: The command for a stage with no mode

- **WHEN** the profile gives the command for a stage whose settings give no mode
- **THEN** it runs `claude` with no `--permission-mode` (162)

#### Scenario: The command for a stage in edits

- **WHEN** the profile gives the command for a stage whose settings give `edits`
- **THEN** it runs `claude` with `--permission-mode acceptEdits` (S66, 169)

### Requirement: Claude Code's map of flume's modes and its own values

The Claude Code profile SHALL map `plan` to `plan`, `ask` to `manual`,
`edits` to `acceptEdits` and `full` to `bypassPermissions`, and SHALL
take its own values under `claude` in a table of programs, the pinned
2.1.296's `--permission-mode` choices `acceptEdits`, `auto`,
`bypassPermissions`, `manual`, `dontAsk` and `plan`, each passed as
written; any other own value SHALL be refused naming it, `claude` and
the values it names; and it SHALL read the mode back (161, 169, C.4).

#### Scenario: Each of the four

- **WHEN** the profile maps `plan`, `ask`, `edits` and `full`
- **THEN** it gives `plan`, `manual`, `acceptEdits` and `bypassPermissions` (169, C.4)

#### Scenario: Its own value under its name

- **WHEN** the profile maps `{ claude = "auto" }`'s entry, and then `{ claude = "yolo" }`'s
- **THEN** the first gives `auto`, and the second is refused naming `yolo`, `claude` and its six values (169, C.4)

#### Scenario: The mode read back

- **WHEN** the profile reads back `default`, `acceptEdits` and `plan`
- **THEN** it reads `manual`, `acceptEdits` and `plan` (161, C.4)

### Requirement: The modes pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source of its modes to stand byte for byte as the profile's map rests
on it: the release's own modes, its alias reading `manual` as
`default`, the `--permission-mode` option, the hook input's builder
naming the mode, UserPromptSubmit's input built with the session's
mode, and a hook's block holding back the prompt (core.22, C.4).

#### Scenario: The pinned release's modes read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the map rests on is found in it, and the test names any that is not (169, C.4)
