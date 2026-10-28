## ADDED Requirements

### Requirement: An agent started as Claude Code with flume's plugin

The agent profile SHALL start an agent as `claude` in the place, with
the plugin of the flume that started it loaded by `--plugin-dir`, the
settings' model and instructions, an environment of the host user's
login environment with the settings' over it, and `FLUME_AGENT`,
`FLUME_SECRET`, `FLUME_PLACE` and `FLUME_STAGE`, the identity naming the
repository by an id from its common git directory; and the prompt as its
argument under the printed command (29, 32, 33, 88, C.4).

#### Scenario: The command for a code stage

- **WHEN** the profile gives the command for a stage with a model and instructions
- **THEN** it runs `claude` with `--plugin-dir`, `--model` and `--append-system-prompt`, the four `FLUME_*` words, and nothing of the caller's session

### Requirement: Readings from the agent's own record

The profile SHALL read received, own-work and asking from the
transcript of the session the plugin's SessionStart hook recorded for
that start in the place's own folder, and whether the session ended from
the SessionEnd hook's mark (134); an answer the record does not give
SHALL be unknown (77, 89 to 91, 93).

#### Scenario: Seam scenarios over fixture transcripts

- **WHEN** the agent seam's scenarios run against the profile over fixture transcripts of the pinned Claude Code
- **THEN** each passes: a prompt received as a user turn, a background shell started and not finished as own work, an open question as asking, and a missing transcript as unknown

### Requirement: The stop check, the guard and on-quiet through hooks

The profile SHALL offer send-back, the guard and on-quiet: the Stop hook
blocks a stop with the core's stop-check words and, when it lets the
agent stop, starts `flume settle <place> --on-quiet` detached; the
PreToolUse hook refuses a shell command that runs `sluice submit`, `wt
submit`, `wt merge` or `git push`, saying flume submits the place, and
records the refusal status reads, and never refuses `git merge` into the
place's branch (108, 110, 112, 115, 116, 121, C.4).

#### Scenario: An agent that would stop with nothing made

- **WHEN** the Stop hook runs for a code stage's agent with no commit since its start
- **THEN** it blocks the stop with words naming commits and `flume needs` (S24)

#### Scenario: An agent tries to push

- **WHEN** the PreToolUse hook is handed `git add -A && git push origin HEAD`
- **THEN** it refuses the command, and status lists the refused act (S25)

#### Scenario: An agent merges the base for a conflict

- **WHEN** the PreToolUse hook is handed `git fetch origin && git merge 4f2a9c1` in the place
- **THEN** it allows the command and records no refusal (112)

#### Scenario: A conversation's agent stops

- **WHEN** the Stop hook runs for a conversation's agent
- **THEN** it lets it stop and blocks nothing (109)
