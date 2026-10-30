## MODIFIED Requirements

### Requirement: SessionStart compares the start's record before it acks

The SessionStart hook SHALL compare its program, read from its parent
process, the digest of each variable the start's record names, in its
own environment and keyed by `FLUME_SECRET`, and, where the record
names a mode and the start holds no started ack yet, its input's
`permission_mode`, `default` read as `manual`, with the start's record
before it acks; on any difference it SHALL ack nothing, record the
start's variables that differ by name and a mode that differs with the
mode asked and the mode begun, start a settle in the background and
exit 0 (35, 121, 161, 162, C.4).

#### Scenario: A variable with another value

- **WHEN** the hook runs for a start whose record names `CLAUDE_CONFIG_DIR` and its environment holds another value
- **THEN** no started ack is written, the agent's record names `CLAUDE_CONFIG_DIR` as different with no value, a settle of the place is started, and the hook exits 0 (161)

#### Scenario: Another program

- **WHEN** the hook's parent runs a program other than the one the start's record names
- **THEN** no started ack is written and the record says the program differed (161)

#### Scenario: A session flume did not start

- **WHEN** the hook runs with no `FLUME_AGENT` in its environment
- **THEN** it compares nothing, writes nothing and exits 0 (C.4)

#### Scenario: Another mode

- **WHEN** the hook runs for a start whose record names `bypassPermissions`, its input's `permission_mode` reading `default`
- **THEN** no started ack is written, the agent's record names the mode asked `bypassPermissions` and the mode begun `manual`, a settle is started, and the hook exits 0 (161, 162, S67)

#### Scenario: manual read back as default

- **WHEN** the hook runs for a start whose record names `manual`, its input's `permission_mode` reading `default`
- **THEN** the started ack is written (161, C.4)

#### Scenario: No mode in the record

- **WHEN** the hook runs for a start whose record names no mode, its input's `permission_mode` reading `acceptEdits`
- **THEN** the started ack is written, no mode compared (161, 162)

#### Scenario: No mode in the input

- **WHEN** the hook runs for a start whose record names `acceptEdits` and its input carries no `permission_mode`
- **THEN** no started ack is written, and the agent's record names the mode asked `acceptEdits` and no mode begun (161)

## ADDED Requirements

### Requirement: A later SessionStart of a started session compares no mode

A SessionStart of a start whose started ack already stands, after its
session resumed, cleared or compacted, SHALL compare its program and
variables and not its mode, and SHALL ack where they match, recording
the session's transcript, since the mode is read at the agent's start
and a started session's mode is its own (161, 162).

#### Scenario: A compaction after the mode moved

- **WHEN** a start acked started in `acceptEdits`, and the hook runs again for it with `source` `compact` and `permission_mode` `default`
- **THEN** the started ack is written again with the session's transcript, and nothing is recorded as unsettled (161, 162)

### Requirement: flume ack started takes the mode the session began in

`flume ack started` SHALL take `--mode <value>`, the mode the session
began in as the program names it, and compare it as the hook compares
its input's `permission_mode` (135, 161).

#### Scenario: An ack by hand with no mode

- **WHEN** `flume ack started` runs for a start whose record names `acceptEdits`, once with `--mode acceptEdits` and once with no `--mode`
- **THEN** the first is taken, and the second acks nothing, naming the mode asked and no mode begun (135, 161)
