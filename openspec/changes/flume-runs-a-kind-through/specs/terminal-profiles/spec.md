## ADDED Requirements

### Requirement: The terminal through herdr

The herdr profile SHALL open, send to, read and close agents through
herdr, and keep no watch (94 to 98, 122, C.5). Every herdr command,
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

### Requirement: Where herdr opens an agent

The herdr profile SHALL open an agent in a pane of the session the
launch's settings name, flume's own otherwise, one tab per place named
`<kind>--<name>`, or a split beside the pane the settings name, the
pane held as the next requirement says, and SHALL answer the agent program's
folder-trust question with the key its command names (34, 94, 107).

#### Scenario: A conversation beside the operator's pane

- **WHEN** a stage's settings name the pane `operator`
- **THEN** the agent opens in a split beside that pane, held by its pane and socket and given a name made from its identity

### Requirement: herdr's panes by id and socket, and a launcher typed

The herdr profile SHALL hold each pane by its id and its server's
socket, give its agent a herdr name of at most 32 of herdr's legal
characters made from the identity for display only, never addressing it
by that name, and type into the pane's shell one line running a launcher
file in the place's state folder, never the agent's command and
arguments (94; record 0016).

#### Scenario: Instructions longer than the terminal's line

- **WHEN** a stage's instructions run to 40,000 characters
- **THEN** open types one short line running the launcher file, and the agent starts with the instructions whole

### Requirement: What herdr reads of an agent

The herdr profile SHALL read herdr's working as working, idle and done
as waiting, blocked as asking, which the core SHALL take as asking only
where the agent seam's asking reads yes and as unknown otherwise, and
herdr's unknown as unknown; it SHALL read an agent stopped only when its
pane is gone, herdr lists no agent in it with the shell in its
foreground, or herdr lists it with a `restore_error`, and report a
close only once the pane is gone (90, 96, 97; record 0016).

#### Scenario: Blocked with no mark

- **WHEN** herdr reads a pane blocked and the agent profile holds no asking mark
- **THEN** the core reads the agent's state unknown, neither asking nor waiting, and types nothing into the pane (90, core.2)

#### Scenario: A finished turn nobody looked at

- **WHEN** a code stage's agent commits, ends its turn, and herdr reads it done
- **THEN** the profile reads it waiting, and the stage delivers and the next starts (S42, 96)

#### Scenario: A pane herdr could not bring back

- **WHEN** after herdr's server restarts it lists a place's pane with a `restore_error`
- **THEN** the profile reads the agent stopped, and the core reads the stage lost (45, 77)

### Requirement: herdr's send leaves nothing typed, and its screen

The herdr profile SHALL clear the pane's held input before each send and
after a send the core reads not received, so no text is left typed, and
SHALL give `screen` from the pane's visible lines (38, 95, 136; record
0016).

#### Scenario: A prompt left typed

- **WHEN** herdr's prompt leaves the text typed and unsubmitted, and no received word comes within 60 seconds
- **THEN** the input is cleared before the second sending, and nothing is left typed whatever the second comes to (S43, 38, 95)

#### Scenario: The screen of a start held at a dialog

- **WHEN** a start's agent shows the warning about bypassing permissions
- **THEN** `screen` gives the warning's visible lines (136, S40)

### Requirement: The caller's session as the terminal names it

Each terminal profile SHALL name the session a command of flume's runs
in, for a launch's default recipient: under herdr the pane it runs in,
under the printed command none (7).

#### Scenario: A launch from a herdr pane

- **WHEN** a launch names no recipient and `FLUME_RECIPIENT` is unset, run from a herdr pane
- **THEN** the place's recipient is `herdr:<pane>@<socket>`, that pane on its server

### Requirement: The printed command

The printed-command profile SHALL answer open with the command, its
prompt carried in it, for a person to run in the place; refuse send and
close; read the agent stopped only once the agent's own record says its
session ended (134); give no screen (136); and keep no watch, so flume runs attended (38,
119, C.5).

#### Scenario: A tell under the printed command

- **WHEN** a caller tells a place's agent under the printed command
- **THEN** the tell is refused with nothing typed (53)

#### Scenario: Seam scenarios over the printed command

- **WHEN** the terminal seam's scenarios run against the profile
- **THEN** each passes, its refusals among them
