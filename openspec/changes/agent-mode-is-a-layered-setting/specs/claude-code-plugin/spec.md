## ADDED Requirements

### Requirement: UserPromptSubmit compares the mode at the start's first prompt

At the start's first prompt, the first with no received ack where no
word that the agent began otherwise stands, the UserPromptSubmit hook
SHALL compare its input's `permission_mode`, `default` read as
`manual`, with the start's record before it acks received, where the
record names a mode, since 2.1.296 gives a SessionStart input none
(161, 162, C.4).

#### Scenario: SessionStart acks on the program and the variables

- **WHEN** the SessionStart hook runs for a start whose record names `bypassPermissions`, its input naming no mode, as 2.1.296's never does, and its program and variables as the record names them
- **THEN** the started ack is written (35, 161)

#### Scenario: manual read back as default

- **WHEN** the hook runs at the first prompt of a start whose record names `manual`, its input's `permission_mode` reading `default`
- **THEN** the received ack is written and nothing is blocked (161, C.4)

#### Scenario: No mode in the record

- **WHEN** the hook runs at the first prompt of a start whose record names no mode, its input's `permission_mode` reading `acceptEdits`
- **THEN** the received ack is written, no mode compared (161, 162)

### Requirement: A first prompt in another mode is blocked

Where the mode a start's first prompt names is not its record's, or
the input names none, the UserPromptSubmit hook SHALL answer a block
with words naming both modes, so the prompt is held back and the agent
takes no turn, ack nothing, record the mode asked and the mode begun
with the started ack's session, start a settle in the background and
exit 0 (35, 121, 161, 162, S67).

#### Scenario: Another mode

- **WHEN** the hook runs at the first prompt of a start whose record names `bypassPermissions`, its input's `permission_mode` reading `default`
- **THEN** it answers a block naming `manual` begun where `bypassPermissions` was asked, no received ack is written, the agent's record names the mode asked `bypassPermissions` and the mode begun `manual`, a settle is started, and the hook exits 0 (161, 162, S67)

#### Scenario: No mode in the input

- **WHEN** the hook runs at the first prompt of a start whose record names `acceptEdits` and its input carries no `permission_mode`
- **THEN** the prompt is blocked, no received ack is written, and the agent's record names the mode asked `acceptEdits` and no mode begun (161)

### Requirement: A later prompt or SessionStart of a started session compares no mode

A prompt after the start's first, a tell among them, and a SessionStart
after its session resumed, cleared or compacted, SHALL compare no mode,
and SHALL ack where everything else matches, a SessionStart recording
the session's transcript, since the mode is read at the agent's start
and a started session's mode is its own (161, 162).

#### Scenario: A later prompt and a compaction after the mode moved

- **WHEN** a start's first prompt was taken in `acceptEdits`, a later prompt's input reads `default`, and the session compacts
- **THEN** the later prompt is acked received, the started ack is written again with the session's transcript, and nothing is recorded as unsettled (161, 162)

### Requirement: flume ack received takes the mode the session runs in

`flume ack received` SHALL take `--mode <value>`, the mode the session
runs in as the program names it, and compare it at the start's first
prompt as the hook compares its input's `permission_mode`; `flume ack
started` SHALL compare no mode (135, 161).

#### Scenario: An ack by hand with no mode

- **WHEN** `flume ack received` runs at the first prompt of a start whose record names `acceptEdits`, once with `--mode acceptEdits` and once with no `--mode`
- **THEN** the first is taken, and the second acks nothing, naming the mode asked and no mode begun (135, 161)
