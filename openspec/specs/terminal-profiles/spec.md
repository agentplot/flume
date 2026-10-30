# terminal-profiles Specification

## Purpose
The terminal seam, Part B.5, through herdr and the printed command: an
agent opened in a pane of a workspace per repository and base and a tab
per place, or printed for its caller to run; its state, its screen and
its question read; what it made tidied; and a place's clues shown.

## Requirements

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
launch's settings name, flume's own otherwise: in the workspace of the
place's repository and base, labelled `<repository> <base>` and made
when the first place on that base opens, one tab per place named
`<kind>--<name>` whose panes are the place's agents, each recorded by
id and socket in the host's state; or a split beside the pane the settings
name, left where its caller put it (94, 107, C.5; record 0018).

#### Scenario: A conversation beside the operator's pane

- **WHEN** a stage's settings name the pane `operator`
- **THEN** the agent opens in a split beside that pane, held by its pane and socket, under a herdr name made from its identity

#### Scenario: Places on two bases

- **WHEN** places on `main` and on `bolt/the-surfaces` of one repository open under herdr in flume's own session
- **THEN** each base's places are tabs of a workspace of its own, labelled `<repository> main` and `<repository> bolt/the-surfaces`

### Requirement: herdr starts the agent by agent start

The herdr profile SHALL make the pane with `--cwd <place>` and an
`--env` for each variable of the agent's environment and flume's own,
and start the agent by `herdr agent start <name> --kind <kind> --pane
<id> --timeout 120000 -- <arguments>`, bounded ten seconds past its
timeout, typing nothing into the pane's shell; herdr's ready SHALL
never count as the start (35, 94, C.5; record 0018; core.29).

#### Scenario: The environment from the pane

- **WHEN** the host's configuration gives the agent a variable
- **THEN** the started program sees it, set on the pane by `--env`, and nothing of it is typed

#### Scenario: Instructions of 20,000 characters

- **WHEN** a stage's instructions run to 20,000 characters with quotes, `$`, backticks and non-ASCII
- **THEN** `agent start` hands them over whole, and nothing is typed into the shell

### Requirement: herdr reads a start held or never come up

The herdr profile SHALL read the folder-trust question off the pane's
screen whatever `agent start` answered, and SHALL read
`agent_name_not_found` from it as `timeout`. After `agent_not_ready` it
SHALL wait within its bound until herdr reads the agent idle, a screen
with no folder-trust question by ten seconds held with the screen; a
`timeout` with no folder-trust question SHALL be not come up with the
screen (34, 35, 94, C.5; record 0018).

#### Scenario: The folder-trust question at the start

- **WHEN** `agent start` answers `agent_not_ready` and the pane shows the folder-trust question
- **THEN** the profile moves the focus to the yes choice, confirms it, waits within its bound until herdr reads the agent idle, and the prompt is sent only after the started ack (S39)

#### Scenario: The folder-trust question drawn late

- **WHEN** `agent start` answers `agent_not_ready` while the pane shows only the dialog's first line, and the choices are drawn a second later
- **THEN** the profile waits for the choices, answers yes from the screen, and the agent comes up (S39)

#### Scenario: herdr reads the dialog ready

- **WHEN** `agent start` answers `agent_started` while the pane still shows the shell's line, and the folder-trust question is drawn after it
- **THEN** the profile waits for the program's screen, answers yes from it, and the start is opened, its started ack deciding it (S39)

#### Scenario: The dialog drawn after the open returned

- **WHEN** the folder-trust question is drawn only after open has returned, and a settle reads the start, whose agent has not said it started
- **THEN** the settle answers it yes from the screen through the terminal's `answer_trust`, as open would have (S39)

#### Scenario: A folder-trust dialog herdr does not see

- **WHEN** the program shows the folder-trust question and herdr reads it neither ready nor held, so `agent start` answers `timeout`, or `agent_name_not_found` where its server dropped the pending name first
- **THEN** the profile answers yes from the screen and the start is opened, its started ack deciding it (S39)

#### Scenario: The bypass warning at the start

- **WHEN** `agent start` answers `agent_not_ready` and the pane shows the warning about bypassing permissions
- **THEN** nothing is answered, and the core reads the start never begun with the warning's lines (S40)

#### Scenario: A program that never comes up

- **WHEN** the program the stage names is not on the pane's `PATH` and `agent start` answers `timeout`
- **THEN** the core reads the start never begun with the pane's visible lines, and sends no prompt (S45)

### Requirement: herdr answers the folder-trust question from the screen

Where the screen shows the folder-trust question, the herdr profile
SHALL send the next key by `pane send-keys` while another choice holds
the focus and the confirming key only while the yes choice holds it,
each once the screen shows the last, until the question is gone, within
30 seconds, and the same at a settle of a start not yet said started. After `agent_started` it SHALL read the screen until
it shows the question or holds still a second, up to ten seconds (34,
35, C.5; record 0018).

#### Scenario: The question answered key by key

- **WHEN** the pane shows the folder-trust question with "No, exit" holding the focus
- **THEN** the profile sends the next key, reads the screen until the yes choice holds the focus, and only then sends the confirming key (S39)

### Requirement: herdr runs the program the settings name

The herdr profile SHALL lead the pane's `PATH` with a folder of the
start's own linking the kind's name to the program the settings name,
SHALL give the pane the host user's login environment under the agent's
and a shell of the start's own that reads no startup file, and SHALL
close and refuse, naming what ran, a start whose pane runs any other
program (29, 32, C.4, C.5; record 0018).

#### Scenario: Startup files that put another program first

- **WHEN** the host user's login shell is zsh, bash or sh, and its every startup file puts another `claude` ahead of the start's own folder on `PATH`
- **THEN** the pane's shell reads none of them, and the program the settings name starts

#### Scenario: A program that runs another in its place

- **WHEN** the program the settings name execs another `claude`
- **THEN** the start is closed and refused, naming the program that ran

### Requirement: herdr's panes by id and socket, found again by name

The herdr profile SHALL hold each pane by its id and its server's
socket, give its agent a herdr name of at most 32 of herdr's legal
characters made from the identity and ending in its digest, and, where
the pane's id no longer answers, find the pane again by `agent get
<name>`, taking it only once herdr lists it under that name and
recording its new id before anything is sent to it (30, 95, 96, 98;
record 0018).

#### Scenario: A pane moved to another workspace

- **WHEN** the operator moves a working agent's pane into another workspace, and its id no longer answers
- **THEN** the profile finds it by the agent's name, records its new id, and reads and sends there (S46)

#### Scenario: A name already taken

- **WHEN** `agent start` answers `agent_name_taken`
- **THEN** open is refused, an agent being live under the identity (98)

### Requirement: herdr tidies what it made

The herdr profile SHALL close a place's recorded tab when the place
goes, and its base's recorded workspace once it holds no tab of a live
place and no pane flume did not open, keeping and naming either
where a pane flume did not open stands, and never closing what flume did
not make (137, 138, C.5; record 0018).

#### Scenario: The last place on a base

- **WHEN** the last place on a repository's base lands and is tidied
- **THEN** its tab and the base's workspace are closed (S47)

#### Scenario: A person's pane in flume's workspace

- **WHEN** a person has opened a pane in a base's workspace and the last place on that base is dropped
- **THEN** the place's tab is closed, the workspace left open, and status names it with the person's pane (S48)

### Requirement: herdr shows clues

The herdr profile SHALL show, by `pane report-metadata` and `workspace
report-metadata` with `--source flume`, the pane's stage and state as
its title and the count of the places on its base as the workspace's
`flume` token, a failure logged and failing nothing (139, C.5; record
0018).

#### Scenario: Titles follow a stage

- **WHEN** a stage's agent goes from starting to waiting to working to asking
- **THEN** its pane's title follows each settle that changes it, and a report that fails is logged while the settle goes on (S49)

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
close; tidy nothing and show nothing; read
the agent stopped only once the agent's own record says its session
ended (134); give no screen (136); and keep no watch, so flume runs
attended (32, 38, 119, C.5).

#### Scenario: A tell under the printed command

- **WHEN** a caller tells a place's agent under the printed command
- **THEN** the tell is refused with nothing typed (53)

#### Scenario: Seam scenarios over the printed command

- **WHEN** the terminal seam's scenarios run against the profile
- **THEN** each passes, its refusals among them

### Requirement: The printed command's environment

The printed-command profile SHALL keep the command's environment in a
file beside its mark in the place's folder that the host's user alone
may read and that the command names, so the command holds no value of
it (32, C.5; record 0017).

#### Scenario: The environment beside the command

- **WHEN** the host's configuration gives the agent a variable and the command is printed
- **THEN** the command names its environment file and not the variable, and the file, readable by the host's user alone, holds the variable read

### Requirement: herdr reaches a recipient by pane and socket

The herdr terminal profile SHALL name a session `herdr:<pane>@<socket>`
and send to, and read the state of, a recipient by that pane on that
server; a role run's session SHALL be the pane its live start holds,
found again by that start's herdr name where the id no longer answers
and taken only once herdr lists it under that name; and each call SHALL
run under the profile's bound with the caller's `HERDR_*` stripped and
the socket passed (43, 95, 96, 98, 132, C.5; records 0016, 0018;
core.29).

#### Scenario: A recipient whose agent herdr renamed

- **WHEN** a recipient's session `/clear`s, so herdr drops its agent's name, and an end is delivered to it
- **THEN** the notice reaches the pane by its id on its server

#### Scenario: A recipient on another server

- **WHEN** a settle runs from a pane of one herdr server and the recipient's pane is on another
- **THEN** the notice is sent through the recipient's own socket, none of the settle's `HERDR_*` reaching herdr

#### Scenario: A role run's pane moved to another workspace

- **WHEN** a role run's pane is moved into another workspace, so its id no longer answers, and an end is delivered to the role
- **THEN** the profile finds the pane by the run's herdr name, records its new id, and the notice reaches it there
