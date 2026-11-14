## ADDED Requirements

### Requirement: The terminal through Rex

The Rex profile SHALL open, send to, read and close agents through Rex,
and keep no watch and settle nothing; every command, field and refusal
it reads SHALL be checked against the pinned Rex, 0.1.0-preview.14 or
later at API revision 12 (94 to 98, 119, 122, C.5; core.22).

#### Scenario: Seam scenarios against Rex

- **WHEN** the terminal seam's scenarios run against the profile, its servers under a state folder the test makes
- **THEN** each passes: no second agent under a live identity, and no close reported that did not happen (98)

#### Scenario: A quiet block with nothing delivered

- **WHEN** a stage's agent under Rex reports `done` with no deliverable and no stop-short
- **THEN** its state reads waiting, and no settle ends the stage or its agent (39, I1; core.9)

### Requirement: Every Rex call is bounded and names its server

Every Rex call SHALL be `rex --autostart=false -S unix://<socket>
<words>`, with the calling process's `REX_*` stripped, under a bound,
ten seconds where the call states none; a call past its bound SHALL
read unknown or refused in words naming the bound, and a nonzero exit
SHALL be a refusal in Rex's words (98, C.5; core.29).

#### Scenario: A Rex call that never answers

- **WHEN** an agent's server is stopped so that it answers nothing, and the agent is sent a prompt and read
- **THEN** the send is refused in words naming the bound and the state reads unknown, each within its bound, and nothing waits on Rex (98; core.29)

#### Scenario: A call from inside a Rex block

- **WHEN** the profile reads an agent on one server from a process whose `REX_SERVER`, `REX_SESSION` and `REX_BLOCK` name a block of another
- **THEN** the call reaches the agent's server alone, and nothing of the other server is read or changed

### Requirement: Rex's servers are flume's own

Each of the host's terminal sessions under Rex SHALL be a Rex server at
`<state>/terminal/rex/<session>.sock`, its state folder beside it,
flume's own session named `flume`, started by `server start --state-dir
<folder> --restore=false --json` where `server.status` finds none at its
socket; a server whose `api_revision` is below 12 SHALL be refused in
Rex's words, and the profile SHALL never stop a server or name the one
the Rex app runs for its user (107, 143, C.5; core.13).

#### Scenario: A session's server started where none answers

- **WHEN** an agent opens in the session `work`, no server running at its socket
- **THEN** a server runs at `<state>/terminal/rex/work.sock`, and the agent's block is in it

#### Scenario: A server stopped and started again

- **WHEN** a session's server holding an agent is stopped, and another agent then opens in that session
- **THEN** the server is started again with none of the sessions it saved, the first agent reads stopped, and the second opens in a new Rex session of its base (77)

### Requirement: A session's name Rex cannot hold is refused

The Rex profile SHALL refuse a session's name, naming it, where it makes
the socket's path longer than 104 bytes, or where it is empty, `.` or
`..`, or holds `/` or a NUL, and SHALL start no server for it (107,
C.5).

#### Scenario: A session name the socket cannot hold

- **WHEN** the core names a session whose socket's path would pass 104 bytes, or the session `../x`
- **THEN** the open is refused naming the session, and no server is started

### Requirement: Where Rex opens an agent

The Rex profile SHALL open an agent in the server of the session the
core names, or flume's own where it names the terminal's own: in that
server's Rex session of the place's repository and base, labelled
`<repository> <base>`, in the place's window labelled `<kind>--<name>`,
a further agent of the place splitting that window, each made when
first needed and recorded by id and server (94, 107, 199, C.5).

#### Scenario: Places on two bases

- **WHEN** places on `main` and on `bolt/the-surfaces` of one repository open under Rex in flume's own session
- **THEN** flume's own server holds the Rex sessions `<repository> main` and `<repository> bolt/the-surfaces`, each holding its place's window and its agent's block

#### Scenario: A stage of two agents

- **WHEN** a second agent of a place opens while its first runs
- **THEN** its block is a split of the place's window, and no second window is made

### Requirement: A block is labelled its agent's Rex name

The Rex profile SHALL label an agent's block its Rex name,
`f-<place>-<stage>-<start>` lowercased with each character outside
`[a-z0-9_-]` made `-`, cut to 119 characters, then `-` and the first
eight hex digits of the identity's SHA-256 (98, C.5).

#### Scenario: A long place's Rex name

- **WHEN** an agent of a place whose name runs past 119 characters, upper case and `/` among them, opens under Rex
- **THEN** its block's label is 128 characters of `[a-z0-9_-]`, ending in `-` and eight hex digits of its identity's digest

### Requirement: Rex opens beside a named block

Where the settings name a block `rex:<block>@<socket>`, the Rex profile
SHALL open the agent in a split beside it, recording no window, and
SHALL refuse a block named in another terminal's form, naming it (107,
C.5).

#### Scenario: Beside a named block

- **WHEN** a stage's settings name the block `rex:<block>@<socket>`
- **THEN** the agent opens in a split beside that block, and the profile records no window for it; a block named `herdr:w1-p2@/tmp/s` refuses the open naming it

### Requirement: Rex starts the agent by a script of the start's own

The Rex profile SHALL start the block, with no shell, by a script of
the start's own, mode 0700, that removes itself and execs `/usr/bin/env
-i` with Rex's terminal variables, the login environment, the agent's
environment and flume's own, `PATH` led by a folder of the start's own
linking the program's kind to the program, then that link and the
arguments, so no variable stands in any process's arguments (29, 32,
94, C.5; core.30).

#### Scenario: No variable in any process's arguments

- **WHEN** an agent is given the variable `A_SECRET` and opens under Rex
- **THEN** the program sees `A_SECRET` and Rex's `REX_BLOCK`, no process under the block or of the server holds its value among its arguments, and the start's script is gone

### Requirement: Rex's ready is the program's first report

Until 120 seconds have passed, every 250 milliseconds, the Rex profile
SHALL read the block's process and its Program Status: a record whose
owner is the block's child is Rex's ready, never the start; the program
exited, or the bound passed with no record, SHALL be the start never
begun with the screen (35, C.5).

#### Scenario: A program that never reports

- **WHEN** the program the stage names writes no Program Status report within the start's bound
- **THEN** the core reads the start never begun with the block's screen, and no prompt is sent (35, S45)

#### Scenario: A program that exits as it starts

- **WHEN** the program the stage names exits before it reports
- **THEN** the start is never begun with the screen, at once rather than at the bound (35)

### Requirement: Rex claims the identity before it makes a block

The Rex profile SHALL write the agent's record in the host's state only
where none stands before it makes the block, and SHALL refuse an open
whose identity's record names an agent that does not read stopped, or
whose session name a live agent carries; a failed open SHALL close the
block it made and remove the record and the start's folder (98, 132,
C.5).

#### Scenario: Two opens of one identity

- **WHEN** an identity's agent is live under Rex and another open is asked under that identity
- **THEN** the second is refused naming the identity, and the server holds one block of that Rex name (98)

### Requirement: Rex runs only a program that reports its state to it

The Rex profile SHALL refuse an open whose command names a kind whose
program the pinned Rex was not read to take Program Status reports
from, naming the program and the kinds it runs, `claude` among them
(32, C.5).

#### Scenario: A program that reports nothing to Rex

- **WHEN** a stage's command names the kind `codex`
- **THEN** the open is refused naming `codex` and `claude`, and nothing is made in Rex

### Requirement: Rex runs the program the settings name

Once Rex reads the program ready, the Rex profile SHALL read the host's
process table for the block's program, the block's child or, where that
is `/usr/bin/login`, its child, which SHALL be the program the settings
name by its path or, for a program a script starts, the script's path
among its arguments; anything else SHALL close the block and refuse the
open naming what ran (32, C.5).

#### Scenario: A program that runs another in its place

- **WHEN** the program the settings name execs another program that reports Program Status
- **THEN** the block is closed and the open refused naming what ran

### Requirement: Rex answers the folder-trust question from the screen

While a start waits for Rex's ready, and at a settle of a start whose
agent has not said it started, the Rex profile SHALL read the block's
screen for the command's folder-trust question and answer it yes by
`send-key`, the focus moved to the yes choice with the command's next
key and confirmed only there, read again until it is gone within 30
seconds; it SHALL answer nothing else on a screen (34, C.5; core.12).

#### Scenario: The folder-trust question at the start

- **WHEN** the program shows the folder-trust question before it reports
- **THEN** the profile moves the focus to the yes choice, confirms it, the program reports, and the agent opens (S39)

#### Scenario: The question drawn after the open returned

- **WHEN** the folder-trust question is drawn only after open has returned, and a settle reads the start, whose agent has not said it started
- **THEN** `answer_trust` answers it yes from the screen as open would have (S39)

### Requirement: Rex's send leaves nothing typed

The Rex profile's send SHALL clear held input with one `send-key
ctrl+c` only where the agent reads waiting, then type the text by `send`
in pieces of at most 60,000 bytes cut between characters, then press
Enter by `send-key enter`; it SHALL never interrupt a working agent, and
received SHALL be read through the agent (89, 95, C.5; core.11).

#### Scenario: A prompt left typed

- **WHEN** text is left typed in a waiting agent's block and a prompt is then sent
- **THEN** the agent takes the prompt alone, the text left typed cleared first (S43)

#### Scenario: A working agent sent a tell

- **WHEN** a tell is sent to an agent that reads working
- **THEN** no ctrl+c is pressed, and the agent takes the tell as its next input

#### Scenario: A text past sixty thousand bytes

- **WHEN** a prompt of 150,000 bytes, non-ASCII among them, is sent
- **THEN** the agent takes it whole and in order as one prompt

### Requirement: What Rex reads of an agent

The Rex profile SHALL read an agent stopped where `process` answers its
block not found, no server runs at its socket, or `last_exit` is set;
and otherwise from `program_status`'s newest record by `updated_at`
whose owner is the block's child: `working` as working, `idle` and
`done` as waiting, `blocked` as asking, `error` as waiting, none as
unknown. Its screen SHALL be the block's last lines, as many as it has
rows (96, 136, C.5).

#### Scenario: Rex's readings as the profile reads them

- **WHEN** an agent reports `idle`, then `working`, `blocked`, `done` and `error`, then exits
- **THEN** the profile reads waiting, working, asking, waiting, waiting, then stopped

#### Scenario: The screen

- **WHEN** an agent's block has printed more lines than it has rows
- **THEN** the screen is its last lines, as many as its rows

### Requirement: Rex's close reads that the block is gone

The Rex profile's close SHALL `block close` the agent's block, a block
not found read as already closed, and report it closed only once
`process` answers it not found or no server runs (97, 98, C.5).

#### Scenario: A close that does not happen

- **WHEN** an agent's server is stopped so that its block cannot be closed, and the agent is closed
- **THEN** the close is refused, and is not reported (98)

### Requirement: Rex tidies what it made

A place's tidy SHALL close the place's recorded window in each server
the Rex profile made one in, and the base's recorded Rex session once it
holds no window but those of live places the profile made and no block
it did not open; a block it did not open SHALL keep its window or Rex
session, named by `kept`, and nothing it did not make SHALL be closed
(137, 138, C.5; core.18).

#### Scenario: The last place on a base

- **WHEN** the last place on a base under Rex is tidied
- **THEN** its window and its base's Rex session are closed, and the server still runs (S47)

#### Scenario: A person's block keeps the Rex session

- **WHEN** a person opens a block of their own in a base's Rex session, and the base's last place is tidied
- **THEN** the place's window is closed, the Rex session is kept, and `kept` names it with the person's block (S48)

### Requirement: Rex reaches a session by its name

The Rex profile SHALL name a recipient's session `rex:<block>@<socket>`
and reach that block on that server: a block that waits has its held
input cleared first, one working is typed into without a ctrl+c, one
asking or unread is refused with nothing typed, and one gone is not
found. A role run's address given to open SHALL reach the block its live
start holds. A name in any other form SHALL be no session of Rex's,
`send_to` answering not found and `session_state` none (7, 43, 132,
190, C.5).

#### Scenario: A recipient's block

- **WHEN** a block the test opened, running no agent of flume's and waiting, is sent a notice by `rex:<block>@<socket>`
- **THEN** it takes the notice, and `session_state` reads it waiting

#### Scenario: A role run's address

- **WHEN** a role run opens under the address `east/ops`, and a notice is sent to `east/ops`
- **THEN** the notice reaches the run's block, and no second open carries `east/ops` while it lives (132)

#### Scenario: A herdr session's name

- **WHEN** `send_to` and `session_state` are asked of `herdr:w1-p2@/tmp/s`
- **THEN** the first answers not found and the second none, and no Rex call is made

### Requirement: Rex gives a session's agents by name

The Rex profile's agents in a part SHALL read `server.status` at the
part's socket, a server not running holding no agent and nothing
started, then `session.list` and `session.view` of each of its Rex
sessions, giving each block by its `label`, at `rex:<block>@<socket>`,
with its state as state reads it, two of one label both given; a
listing refused or not answered within its bound SHALL be not read, in
Rex's words (132, 193, C.5).

#### Scenario: A standing agent by its label

- **WHEN** a block labelled `ops` runs a program that reports `idle` in the server of the session `east`, and the agents of `east` are asked
- **THEN** `ops` is given waiting at `rex:<block>@<socket>`, which `send_to` reaches, and a part whose server does not run gives none and starts nothing

### Requirement: Rex's runs asks each server

The Rex profile's runs SHALL call `server.status` at each part's socket,
every part at once and each call bounded at one second: a server that
answers runs, under the part's name, `flume` for the terminal's own; one
with no server at its socket does not; anything else, or a name the
profile refuses, is not read, in its words; and nothing SHALL be started
(200, 201, C.5).

#### Scenario: Three parts in one reading

- **WHEN** runs is asked of `work`, whose server runs, `desk`, whose server does not, and the terminal's own, whose server is stopped so it answers nothing
- **THEN** it gives `work` running, `desk` not running and `flume` not read within about a second, and no server is started at `desk`'s socket

### Requirement: Rex's attach and show

The Rex profile's attach SHALL refuse a window, naming that the pinned
Rex has no terminal client and only its app shows a session, and SHALL
refuse the app's view in its words; its show SHALL show none and its
`shown_at` give nothing; its setup SHALL give nothing, flume carrying no
plugin for Rex (122, 139, 187, C.5).

#### Scenario: A Rex stream's view

- **WHEN** attach is asked of a session under Rex for a window and for the app
- **THEN** each is refused in the profile's words, and no server is started for either

## MODIFIED Requirements

### Requirement: The caller's session as the terminal names it

Each terminal profile SHALL name the session a command of flume's runs
in, for a launch's default recipient: under herdr the pane it runs in,
under Rex the block it runs in, `rex:<block>@<socket>` from the
command's `REX_BLOCK` and `REX_SERVER`, both set, with no call made, and
under the printed command none (7).

#### Scenario: A launch from a herdr pane

- **WHEN** a launch names no recipient and `FLUME_RECIPIENT` is unset, run from a herdr pane
- **THEN** the place's recipient is `herdr:<pane>@<socket>`, that pane on its server

#### Scenario: A launch from a Rex block

- **WHEN** a launch names no recipient and `FLUME_RECIPIENT` is unset, run with `REX_BLOCK` and `REX_SERVER` set on a host that declares a stream under Rex, no Rex server running
- **THEN** the place's recipient is `rex:<block>@<socket>`, and no Rex server is started
