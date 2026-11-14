## ADDED Requirements

### Requirement: Claude Code's deliver part at the pinned release

Claude Code's profile SHALL answer deliver not offered while flume pins
a release no check of the tree has read running a hooks module's
`$.prompt.submit` in an interactive session, 2.1.296 among them, the
release the row was read on, 2.1.322, named with its reason; its
command SHALL set `FLUME_DELIVER=1` in the agent's environment only
where it answers offered, and never otherwise (54, 171, 215, C.4;
core.39; record 0069).

#### Scenario: The pinned release

- **WHEN** `deliver` is asked of Claude Code's profile, and a command is built for a start
- **THEN** it answers not offered, and the command's environment names no `FLUME_DELIVER`

#### Scenario: A profile that offers it

- **WHEN** the profile is built answering deliver offered, as the chore that moves the pin will build it, and a command is built for a start
- **THEN** the command's environment gives `FLUME_DELIVER=1` beside `FLUME_AGENT`, `FLUME_SECRET`, `FLUME_PLACE` and `FLUME_STAGE`

### Requirement: flume hook deliver prints what the take answers

`flume hook deliver` SHALL ask the core's take for the agent
`FLUME_AGENT` names with the secret `FLUME_SECRET` gives, in the place
`FLUME_PLACE` names, print the words it answers, nothing where it
answers none or refuses, and exit 0 whatever it answers (54, 215,
C.4).

#### Scenario: A message waiting for the take

- **WHEN** a start's agent, whose profile offers deliver, waits with a message waiting for it, and `flume hook deliver` runs with its variables
- **THEN** it prints the message's words alone and exits 0, and the message is marked taken

### Requirement: The take answers its own agent alone

The take SHALL be refused unless its agent is the live agent of its
place's current stage, its secret that start's and its profile offers
deliver, and SHALL answer nothing while the agent asks, its prompt is
not received or its stage waits on an answer (41, 42, 214, 215).

#### Scenario: Another start's secret

- **WHEN** `flume hook deliver` runs with a secret that is not the start's
- **THEN** it prints nothing, exits 0, and nothing is taken or recorded handed

#### Scenario: An agent asking

- **WHEN** the agent's asking mark stands while a message waits for it, and `flume hook deliver` runs with its variables
- **THEN** it prints nothing, and the message still waits untaken (core.11)
