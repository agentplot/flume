## REMOVED Requirements

### Requirement: herdr's setup read beside a fresh lease

**Reason**: flume's herdr plugin keeps no watch and no lease; its
startup writes `setup.json` with its server's socket (122, C.5).

**Migration**: "herdr's setup read for each running server" below.

## ADDED Requirements

### Requirement: herdr's setup read for each running server

The herdr profile SHALL give as the terminal's setup the words in the
plugin's `setup.json` of each herdr server whose socket the file names
is still present, in order and once each, and nothing for a server
gone, calling no herdr command and reading no lease (122, C.5;
core.29).

#### Scenario: A running server and a gone one

- **WHEN** one server's state directory holds a `setup.json` naming its present socket and the popup's missing key, and another's names a socket now gone and the missing `$flume` rows
- **THEN** the profile's setup is the first server's words alone, and no herdr command was run

## MODIFIED Requirements

### Requirement: The terminal through herdr

The herdr profile SHALL open, send to, read and close agents through
herdr, and settle nothing, keeping no watch and nudging nothing (94 to
98, 122, C.5). Every herdr command, state name, error code and field
it reads SHALL be checked against the pinned herdr, and every herdr
call SHALL run under a bound, ten seconds where the call states none,
with the caller's `HERDR_*` stripped and the recorded server's socket
passed; a call past its bound SHALL read unknown or refused in herdr's
words (98, core.29; record 0016).

#### Scenario: Seam scenarios against herdr

- **WHEN** the terminal seam's scenarios run against the profile on a herdr server the test starts
- **THEN** each passes: no second agent under a live identity, and no close reported that did not happen (98)

#### Scenario: A herdr call that never answers

- **WHEN** herdr does not answer a send within its bound
- **THEN** the send is refused in words naming the bound, and nothing waits on herdr (98, core.29)

#### Scenario: A quiet pane with nothing delivered

- **WHEN** a stage's agent under herdr goes idle with no deliverable and no stop-short
- **THEN** no settle ends the stage or its agent (39, I1)

#### Scenario: Unattended by the host's watch alone

- **WHEN** the herdr profile is bound against the pinned herdr's server, whatever the plugin's state there, and a caller settles every place with a bound of a minute, then three minutes pass with no such settle
- **THEN** flume runs unattended after the first, and attended after the three minutes, and no herdr command was run to decide either (119, 122, 208)

### Requirement: The printed command

The printed-command profile SHALL answer open with the command, its
prompt carried in it, for a person to run in the place; refuse send and
close; tidy nothing and show nothing; read
the agent stopped only once the agent's own record says its session
ended (134); give no screen (136); and settle nothing (32, 38, 122,
C.5).

#### Scenario: A tell under the printed command

- **WHEN** a caller tells a place's agent under the printed command
- **THEN** the tell is refused with nothing typed (53)

#### Scenario: Seam scenarios over the printed command

- **WHEN** the terminal seam's scenarios run against the profile
- **THEN** each passes, its refusals among them

### Requirement: herdr shows clues

The herdr profile SHALL show, by `pane report-metadata` and `workspace
report-metadata` with `--source flume`, the pane's stage and state as
its title and the short form of its place's line as its `flume` token,
the row herdr's sidebar shows, and the count of the places on its base
as the workspace's `flume` token, a failure logged and failing nothing
(139, C.5; record 0018).

#### Scenario: Titles follow a stage

- **WHEN** a stage's agent goes from starting to waiting to working to asking
- **THEN** its pane's title follows each settle that changes it, and a report that fails is logged while the settle goes on (S49)

#### Scenario: The row follows the line

- **WHEN** a change place's code stage keeps working while the change at its head goes from 2 tasks ticked of 5 to 3, and a settle runs against the pinned herdr's server
- **THEN** that settle reports the pane's title and its token `flume=code 1/2 · run 1 · tasks 3/5 · working`, and the next settle, with nothing changed, reports nothing (139)
