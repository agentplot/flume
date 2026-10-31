## MODIFIED Requirements

### Requirement: The started ack is taken only where the record matches

The agent's word that it started SHALL be taken only where its program,
every variable the record names and, where the record names a mode its
profile reads back, its mode match the start's record; an agent whose
word shows otherwise SHALL not be the stage's, and its start SHALL be
read as started without its settings, naming each variable that differs
by its name, whether the program did, and a mode that did by the mode
asked and the mode begun (35, 135, 161, 162).

#### Scenario: A variable missing

- **WHEN** a start's agent says it started with every `FLUME_*` of its start and no `CLAUDE_CONFIG_DIR`, which its record names
- **THEN** no started ack is recorded, `started` reads no, and the start reads started without its settings, naming `CLAUDE_CONFIG_DIR` as missing (135, 161)

#### Scenario: Everything matches

- **WHEN** a start's agent says it started with its program, its mode and every variable as its record names them
- **THEN** the started ack is taken and its prompt is sent (35, 37)

#### Scenario: Another mode

- **WHEN** a start's record names the mode `all`, and its agent says it started with every variable as recorded but in the mode `ask-each`
- **THEN** no started ack is recorded, and the start reads started without its settings, naming the mode asked `all` and the mode begun `ask-each` (135, 161, 162)

### Requirement: A stage whose agent began without its settings starts afresh

flume SHALL end an agent that began without its settings once quiet and
start its stage afresh under the settings resolved then, with no
caller's word and counting toward no bound of the kind's; a fresh start
that begins without its settings again, or a start so begun that is
not quiet by the stage's limit, SHALL be never begun, naming the same;
status SHALL name each variable by its name alone, and a mode that
differed by the mode asked and the mode begun (31, 35, 112, 161, 162).

#### Scenario: A pane brought back without its environment

- **WHEN** a code stage starts under work-1 and its program comes up in a pane that lacks work-1's `CLAUDE_CONFIG_DIR`
- **THEN** the hook acks nothing, flume ends that agent once quiet and starts code afresh under work-1 with its environment, status naming `CLAUDE_CONFIG_DIR` as missing with no value, and the kind's bound is not counted (S65)

#### Scenario: Twice in a row

- **WHEN** the fresh start's agent also comes up without `CLAUDE_CONFIG_DIR`
- **THEN** the stage ends never begun, its end naming `CLAUDE_CONFIG_DIR`, delivered to the place's recipients once (35, 43, 161)

#### Scenario: Never quiet by the stage's limit

- **WHEN** an agent begun without `CLAUDE_CONFIG_DIR` still runs own work at the stage's limit
- **THEN** the stage ends never begun, its end naming `CLAUDE_CONFIG_DIR`, and no prompt was sent to it (35, 43, 161)

#### Scenario: No prompt to the agent ended

- **WHEN** a start reads started without its settings
- **THEN** no prompt is sent to its agent from then until flume ends it, and one its program held back at its first prompt is not sent again (37, 161)

#### Scenario: A mode the account's configuration disables

- **WHEN** a code stage's settings give `full`, and its agent begins in another mode, as its account's configuration disabling the mode leaves it, at its start and at the fresh start after it
- **THEN** flume ends the first agent once quiet and starts code afresh, counting no bound, and the fresh start is never begun, its end and status naming the mode asked and the mode it began in (S67, 35, 161, 162)

## ADDED Requirements

### Requirement: A start records the mode its profile passes

At each start flume SHALL record in the start's `start_record` the mode
its agent's profile passes, and none where its settings give none (161,
162; core.10).

#### Scenario: The mode as passed

- **WHEN** a start's settings give `edits`, which its profile passes as `edit-files`, and another start's settings give no mode
- **THEN** the first start's record names the mode `edit-files` and the second's names none (161, 162)

### Requirement: A mode its program names only at the first prompt

Where the agent's program names its mode only at its start's first
prompt, flume SHALL compare the mode in the agent's word at that
prompt; where it differs, that word SHALL not be taken as received, and
the start SHALL be read as started without its settings although its
started ack was taken (37, 161, 162).

#### Scenario: Another mode at the first prompt

- **WHEN** a start's record names the mode `all`, its agent's word that it started is taken, and its word at its start's first prompt names `ask-each`
- **THEN** the prompt is sent once and not received, the start reads started without its settings, naming the mode asked `all` and the mode begun `ask-each`, and no prompt is sent to it again before flume ends it (37, 161, 162)

### Requirement: A profile that reads no mode back compares none

Where the agent's profile does not read back the mode its program began
in, a start SHALL record the mode its profile passes and compare none,
and status SHALL say the mode was not read back (161, 169).

#### Scenario: The fake reading no mode back

- **WHEN** the fake agent's profile reads no mode back, and a start's settings give `edits`
- **THEN** the start's record names the value passed, its word that it started is taken with no mode in it, and status gives the agent's mode as not read back (161, 169)
