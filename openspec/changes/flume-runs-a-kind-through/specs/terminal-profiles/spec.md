## ADDED Requirements

### Requirement: The terminal through herdr

The herdr profile SHALL open, send to, read and close agents through
herdr, and keep no watch (94 to 98, 122, C.5).

#### Scenario: Seam scenarios against herdr

- **WHEN** the terminal seam's scenarios run against the profile on a herdr server the test starts
- **THEN** each passes: no second agent under a live identity, and no close reported that did not happen (98)

#### Scenario: A quiet pane with nothing delivered

- **WHEN** a stage's agent under herdr goes idle with no deliverable and no stop-short
- **THEN** no settle ends the stage or its agent (39, I1)

### Requirement: Where herdr opens an agent

The herdr profile SHALL open an agent in a pane of the session the
launch's settings name, flume's own otherwise, one tab per place named
`<kind>--<name>`, or a split beside the pane the settings name, the
pane named by the identity, and SHALL answer the agent program's
folder-trust question with the key its command names (34, 94, 107).

#### Scenario: A conversation beside the operator's pane

- **WHEN** a stage's settings name the pane `operator`
- **THEN** the agent opens in a split beside that pane, named by its identity

### Requirement: What herdr reads of an agent

The herdr profile SHALL read herdr's working as working, idle as
waiting, done as stopped and blocked as asking, which the core SHALL
take as asking only where the agent seam's asking reads yes, and SHALL report a close only once the pane is gone
(90, 96, 97).

#### Scenario: Blocked with no mark

- **WHEN** herdr reads a pane blocked and the agent profile holds no asking mark
- **THEN** the core reads the agent not asking (90, core.2)

### Requirement: The printed command

The printed-command profile SHALL answer open with the command, its
prompt carried in it, for a person to run in the place; refuse send and
close; read the agent stopped only once the agent's own record says its
session ended; and keep no watch, so flume runs attended (38, 119,
C.5).

#### Scenario: A tell under the printed command

- **WHEN** a caller tells a place's agent under the printed command
- **THEN** the tell is refused with nothing typed (53)

#### Scenario: Seam scenarios over the printed command

- **WHEN** the terminal seam's scenarios run against the profile
- **THEN** each passes, its refusals among them
