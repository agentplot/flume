## MODIFIED Requirements

### Requirement: An agent started as Claude Code with flume's plugin

The agent profile SHALL start an agent as `claude` in the place, with
the plugin of the flume that started it loaded by `--plugin-dir`, the
settings' model and instructions, an environment of the host user's
login environment with its account's variables over it, and
`FLUME_AGENT`, `FLUME_SECRET`, `FLUME_PLACE` and `FLUME_STAGE`, the
identity naming the repository by an id from its common git directory;
and the prompt as its argument under the printed command (29, 32, 33,
88, 144, C.4).

#### Scenario: The command for a code stage

- **WHEN** the profile gives the command for a stage with a model and instructions under an account giving `CLAUDE_CONFIG_DIR`
- **THEN** it runs `claude` with `--plugin-dir`, `--model` and `--append-system-prompt-file`, naming a file of the start's own beside its record that holds the instructions as given, newlines and all, so no argument holds a newline, the account's `CLAUDE_CONFIG_DIR`, the four `FLUME_*` words, and nothing of the caller's session
