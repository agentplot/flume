# start-integrity Specification

## Purpose
TBD - created by archiving change work-runs-in-streams-on-pools-of-accounts. Update Purpose after archive.

## Requirements

### Requirement: A start records what its agent runs under

At each start flume SHALL record, as the start's `start_record`, the
agent's program, its account and,
for each variable of its resolved environment and `FLUME_AGENT`,
`FLUME_PLACE` and `FLUME_STAGE`, a digest keyed by the start's secret
over the variable's name and value, and never a value; the variables a
terminal sets for itself and the login environment beneath the settings
SHALL not be in it (161; core.10, core.30).

#### Scenario: The record holds no value

- **WHEN** a start's resolved environment gives `CLAUDE_CONFIG_DIR` and a token
- **THEN** its held record names both with digests, the token's value is in no held state, answer or log, and a digest computed with another secret differs (161)

#### Scenario: What the terminal sets is left out

- **WHEN** the herdr profile sets `PATH` and `SHELL` on the pane for the start
- **THEN** neither is in the start's record (161)

### Requirement: What the agent program keeps from its hooks is in no record

A start's record SHALL leave out each variable the agent program keeps
from every process it starts, as its profile's `withheld` names them,
since the program's word that it started cannot show them (161).

#### Scenario: A token the program keeps to itself

- **WHEN** a start's account gives `AGENT_HOME` and `AGENT_TOKEN`, and the agent program keeps `AGENT_TOKEN` from every process it starts
- **THEN** the start's record names `AGENT_HOME` and not `AGENT_TOKEN`, the agent is started with both, and its word that it started is taken (161)

### Requirement: The started ack is taken only where the record matches

The agent's word that it started SHALL be taken only where its program
and every variable the record names match the start's record; an agent
whose word shows otherwise SHALL not be the stage's, and its start SHALL
be read as started without its settings, naming each variable that
differs by its name and whether the program did (35, 135, 161).

#### Scenario: A variable missing

- **WHEN** a start's agent says it started with every `FLUME_*` of its start and no `CLAUDE_CONFIG_DIR`, which its record names
- **THEN** no started ack is recorded, `started` reads no, and the start reads started without its settings, naming `CLAUDE_CONFIG_DIR` as missing (135, 161)

#### Scenario: Everything matches

- **WHEN** a start's agent says it started with its program and every variable as its record names them
- **THEN** the started ack is taken and its prompt is sent (35, 37)

### Requirement: A stage whose agent began without its settings starts afresh

flume SHALL end an agent that began without its settings once quiet and
start its stage afresh under the settings resolved then, with no
caller's word and counting toward no bound of the kind's; a fresh start
that begins without its settings again, or a start so begun that is
not quiet by the stage's limit, SHALL be never begun, naming the same;
status SHALL name each variable by its name alone (31, 35, 112, 161).

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
- **THEN** no prompt is sent to its agent before flume ends it (37, 161)
