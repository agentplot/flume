## ADDED Requirements

### Requirement: A wrapped Claude Code starts as the program it wraps

The Claude Code agent profile SHALL name as the program the one a nix
wrapper the settings name execs, with the variables the wrapper sets,
so that herdr detects the agent and flume's foreground check takes the
program that runs (32; record 0018; C.5).

#### Scenario: A wrapper exec'ing its program

- **WHEN** the settings name a wrapper `bin/claude` that execs `bin/.claude` with variables of its own
- **THEN** the start runs `bin/.claude` under the name `claude` with those variables, herdr reads it as Claude Code, and the start is taken

### Requirement: A prompt submitted inside paste tags is received as sent

The Claude Code agent profile SHALL compare a submitted prompt with the
prompt sent with the tags Claude Code wraps pasted text in taken off,
`<pasted_content id="....">` and `</pasted_content id="....">` with a
four-hex-digit id, and its whitespace then collapsed, in the received
ack and in the transcript's user turn and queued record alike, since a
terminal sends a prompt as a paste and the pinned Claude Code wraps a
paste in these tags where the account's features turn them on (38, 89;
record 0016).

#### Scenario: A prompt submitted inside paste tags

- **WHEN** a terminal pastes the prompt and Claude Code submits it inside `<pasted_content id="fad0">` and `</pasted_content id="fad0">`, newlines around them
- **THEN** the received ack carries the digest of the prompt as sent, `received` reads yes, and the stage begins
