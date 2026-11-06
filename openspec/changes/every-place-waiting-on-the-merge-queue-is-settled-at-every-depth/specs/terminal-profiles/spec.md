## MODIFIED Requirements

### Requirement: The terminal through herdr

The herdr profile SHALL open, send to, read and close agents through
herdr, and answer its nudge from the plugin's watch, keeping no watch
of its own (94 to 98, 122, C.5). Every herdr command,
state name, error code and field it reads SHALL be checked against the
pinned herdr, and every herdr call SHALL run under a bound, ten seconds
where the call states none, with the caller's `HERDR_*` stripped and
the recorded server's socket passed; a call past its bound SHALL read
unknown or refused in herdr's words (98, core.29; record 0016).

#### Scenario: Seam scenarios against herdr

- **WHEN** the terminal seam's scenarios run against the profile on a herdr server the test starts
- **THEN** each passes: no second agent under a live identity, and no close reported that did not happen (98)

#### Scenario: A herdr call that never answers

- **WHEN** herdr does not answer a send within its bound
- **THEN** the send is refused in words naming the bound, and nothing waits on herdr (98, core.29)

#### Scenario: A quiet pane with nothing delivered

- **WHEN** a stage's agent under herdr goes idle with no deliverable and no stop-short
- **THEN** no settle ends the stage or its agent (39, I1)

### Requirement: The printed command

The printed-command profile SHALL answer open with the command, its
prompt carried in it, for a person to run in the place; refuse send and
close; tidy nothing and show nothing; read
the agent stopped only once the agent's own record says its session
ended (134); give no screen (136); and nudge nothing (32, 38, 122,
C.5).

#### Scenario: A tell under the printed command

- **WHEN** a caller tells a place's agent under the printed command
- **THEN** the tell is refused with nothing typed (53)

#### Scenario: Seam scenarios over the printed command

- **WHEN** the terminal seam's scenarios run against the profile
- **THEN** each passes, its refusals among them
