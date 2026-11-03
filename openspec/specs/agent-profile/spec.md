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
login environment with its account's variables over it, and
`FLUME_AGENT`, `FLUME_SECRET`, `FLUME_PLACE` and `FLUME_STAGE`, the
identity naming the repository by an id from its common git directory;
and the prompt as its argument under the printed command (29, 32, 33,
88, 144, C.4).

#### Scenario: The command for a code stage

- **WHEN** the profile gives the command for a stage with a model and instructions under an account giving `CLAUDE_CONFIG_DIR`
- **THEN** it runs `claude` with `--plugin-dir`, `--model` and `--append-system-prompt`, the account's `CLAUDE_CONFIG_DIR`, the four `FLUME_*` words, and nothing of the caller's session

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
prompt sent with Claude Code's paste tags taken off and its whitespace
then collapsed, in the received ack and the transcript alike (38, 89;
record 0016).

#### Scenario: A prompt submitted inside paste tags

- **WHEN** a terminal pastes the prompt and Claude Code submits it inside `<pasted_content id="fad0">` and `</pasted_content id="fad0">`, newlines around them
- **THEN** the received ack carries the digest of the prompt as sent, `received` reads yes, and the stage begins

#### Scenario: Text that only looks like a paste tag

- **WHEN** a prompt holds `<pasted_content id="FAD0">`, whose id is not four lowercase hex digits
- **THEN** the text stays as it is in the comparison

### Requirement: A usage limit read from Claude Code's own words

The Claude Code profile's `spent` SHALL read yes only where the
StopFailure hook recorded `rate_limit` with words that open as the
pinned release's usage-limit messages do, with the reset its words'
` · resets <when> (<zone>)` part names, read in that zone from when
the hook ran, a time of day being the first such time no more than a
minute before the hook ran, or none; a `rate_limit` with other words SHALL read no
(146, 151, C.4).

#### Scenario: A session limit with a time of day

- **WHEN** the hook ran at 13:10 in New York and recorded `rate_limit` with "You've hit your session limit · resets 3pm (America/New_York)"
- **THEN** `spent` reads yes until 15:00 that day in New York (151, C.4)

#### Scenario: A weekly limit with a date

- **WHEN** the words are "You've hit your weekly limit · resets Nov 2, 3:30pm (UTC) · progress saved"
- **THEN** `spent` reads yes until 2 November at 15:30 UTC of the hook's year (151, C.4)

#### Scenario: A reset past midnight

- **WHEN** the hook ran at 23:40 in UTC and the words say "resets 1am (UTC)"
- **THEN** `spent` reads yes until 01:00 UTC the next day (151)

#### Scenario: No reset in the words

- **WHEN** the words are "You've hit your limit"
- **THEN** `spent` reads yes with no reset (146, 151)

#### Scenario: A model under high load

- **WHEN** the hook recorded `rate_limit` with "Opus is experiencing high load, please use /model to switch to Sonnet"
- **THEN** `spent` reads no, and the words stay the agent's last words (43, 151)

### Requirement: The credentials Claude Code keeps from its hooks

The Claude Code profile's `withheld` SHALL name the credential
variables the pinned release keeps from every process it starts, its
hooks among them: its own list of credentials, the file-descriptor ones
it spreads into that list, and the memory service's token (161, C.4).

#### Scenario: The account's token

- **WHEN** a start's account gives `CLAUDE_CONFIG_DIR` and `CLAUDE_CODE_OAUTH_TOKEN`
- **THEN** `withheld` names `CLAUDE_CODE_OAUTH_TOKEN` and not `CLAUDE_CONFIG_DIR`, the start's record names `CLAUDE_CONFIG_DIR` alone of the two, and the SessionStart hook, whose environment holds no token, acks (161)

### Requirement: Own work read before the first turn

The Claude Code profile's `own_work` SHALL read no for a session whose
transcript the release has not yet written, since it writes the
transcript at the session's first message and own work starts only in a
turn; a transcript that cannot be read SHALL read unknown (91, 93).

#### Scenario: Idle at its prompt

- **WHEN** an agent's SessionStart hook recorded its session and transcript, and the agent sits at its prompt with no turn taken and no transcript written
- **THEN** `own_work` reads no and `asking` reads no (31, 90, 91)

### Requirement: The words pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source of its usage-limit words to stand byte for byte as the profile
reads them: its list of usage-limit openings, the `You've hit your`
template, the ` · resets ` part and the zone's suffix (core.22, C.4).

#### Scenario: The pinned release read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the profile's reading rests on is found in it, and the test names any that is not (C.4)

### Requirement: The credentials pinned to the release the tree holds

A `check-tools` test SHALL require the source the profile's `withheld`
rests on to stand byte for byte in the pinned Claude Code: its lists of
credentials, the memory service's pair, the test of the environment
against the list, the names its child environment drops and the drop,
and the hook runner's environment built on that child environment
(core.22, C.4).

#### Scenario: What the pinned release keeps from its hooks

- **WHEN** the test reads the program devenv.nix pins
- **THEN** each fragment `withheld` rests on is found in it, and the test names any that is not (161, C.4)

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
