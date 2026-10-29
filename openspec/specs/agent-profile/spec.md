# agent-profile Specification

## Purpose
The agent seam, Part B.4, through Claude Code: the command that starts
the pinned program with flume's plugin and the start's variables, its
folder-trust question and answer, and the readings of a start (started,
received, asking, ended, last words) from the plugin's acknowledgements
and the program's transcript.

## Requirements

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

The profile SHALL read own-work and asking from the transcript of the
session the plugin's SessionStart hook recorded for that start in the
place's own folder and the plugin's marks; last words from the
transcript's last message or the reason the StopFailure hook recorded;
and whether the session ended from the SessionEnd hook's mark (134); an
answer the record does not give SHALL be unknown (43, 77, 90, 91, 93).

#### Scenario: Seam scenarios over fixture transcripts

- **WHEN** the agent seam's scenarios run against the profile over fixture transcripts of the pinned Claude Code
- **THEN** each passes: a prompt received by its ack, a background shell started and not finished as own work, an open question as asking, and a missing transcript as unknown

### Requirement: Started and received from the plugin's acks

The profile SHALL read started from the start's started ack, carrying
its identity, secret, the plugin's version and the session, and
received from the start's received ack, carrying the submitted prompt's
digest; only where no ack can come SHALL it read received from the
transcript's user turn or queued `enqueue` record (38, 77, 89, 135;
record 0016).

#### Scenario: A bare resume

- **WHEN** herdr resumes a start's session with a bare `claude --resume`, no `FLUME_*` in its environment and no `--plugin-dir`
- **THEN** no started ack comes for the start, and `started` reads no (77, S41)

#### Scenario: A prompt queued while the agent worked

- **WHEN** a prompt is typed while the agent works and Claude Code submits it after the turn
- **THEN** the received ack comes at the submission, and `received` reads yes then and not before (38, 89)

#### Scenario: A plugin of another version

- **WHEN** the started ack names a plugin version other than the binary's
- **THEN** status reports the difference with the agent

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
