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

The herdr profile SHALL open an agent in herdr's session the core
names, its server started headless where none runs, or in flume's own,
`flume`, where the core names the terminal's own: in that session's
workspace of the place's repository and base, labelled `<repository>
<base>`, made when first needed, one tab per place named
`<kind>--<name>`, each recorded with its server; or in a split beside
the pane the settings name (94, 107, C.5; records 0018, 0031).

#### Scenario: A conversation beside the operator's pane

- **WHEN** a stage's settings name the pane `operator`
- **THEN** the agent opens in a split beside that pane, held by its pane and socket, under a herdr name made from its identity

#### Scenario: Places on two bases

- **WHEN** places on `main` and on `bolt/the-surfaces` of one repository open under herdr in flume's own session
- **THEN** each base's places are tabs of a workspace of its own, labelled `<repository> main` and `<repository> bolt/the-surfaces`

#### Scenario: Two streams' sessions under herdr

- **WHEN** under the pinned herdr, with herdr's sessions under a scratch home, a fix's agent opens in the session `work` and another's in `personal`
- **THEN** herdr lists `work` and `personal` running, `work`'s server holds the workspace `<repository> main` with the first fix's tab `fix--<name>` and its agent's pane, and `personal`'s the other's (107, S82)

#### Scenario: A session name herdr refuses

- **WHEN** the core names a session the pinned herdr refuses to start
- **THEN** the open is refused in herdr's words, and nothing of flume's is left open (98; core.29)

### Requirement: herdr starts the agent by agent start

The herdr profile SHALL make the pane with `--cwd <place>` and an
`--env` for each variable of the agent's environment and flume's own,
and start the agent by `herdr agent start <name> --kind <kind> --pane
<id> --timeout <ms> -- <arguments>`, the timeout what is left of the
start's 120 seconds, bounded ten seconds past it, typing nothing into
the pane's shell; herdr's ready SHALL never count as the start (35, 94,
C.5; record 0018; core.29).

#### Scenario: The environment from the pane

- **WHEN** the host's configuration gives the agent a variable
- **THEN** the started program sees it, set on the pane by `--env`, and nothing of it is typed

### Requirement: herdr starts the agent once the pane's shell is at its prompt

Before agent start, the herdr profile SHALL wait within the start's 120
seconds for the pane's shell's first output, `pane wait-output <id>
--source visible --regex '\S'`; SHALL read `agent_start_input_failed`
as the shell not yet taking the line and ask agent start again within
those seconds; and SHALL read a shell that shows nothing within them,
or a start still so answered at their end, as not come up with the
screen and herdr's words, never refused (35, C.5; record 0018).

#### Scenario: Instructions of 20,000 characters

- **WHEN** a stage's instructions run to 20,000 characters with quotes, `$`, backticks and non-ASCII, and the pane's shell is slow to come to its prompt, its terminal taking no line past its canonical limit
- **THEN** the profile waits for the prompt, `agent start` hands them over whole, and nothing is typed into the shell

#### Scenario: A shell that speaks before it takes the line

- **WHEN** the pane's shell says something as it starts, and herdr answers `agent_start_input_failed` until its line editor takes the terminal
- **THEN** the profile asks agent start again, and the agent starts once the shell takes the line

#### Scenario: A shell never at its prompt

- **WHEN** the pane's shell does not take the line within the start's bound
- **THEN** the core reads the start never begun with the pane's visible lines and herdr's words, and the open is not refused

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
under Rex the block it runs in, `rex:<block>@<socket>` from the
command's `REX_BLOCK` and `REX_SERVER`, both set, with no call made, and
under the printed command none (7).

#### Scenario: A launch from a herdr pane

- **WHEN** a launch names no recipient and `FLUME_RECIPIENT` is unset, run from a herdr pane
- **THEN** the place's recipient is `herdr:<pane>@<socket>`, that pane on its server

#### Scenario: A launch from a Rex block

- **WHEN** a launch names no recipient and `FLUME_RECIPIENT` is unset, run with `REX_BLOCK` and `REX_SERVER` set on a host that declares a stream under Rex, no Rex server running
- **THEN** the place's recipient is `rex:<block>@<socket>`, and no Rex server is started

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

### Requirement: herdr tidies a place in every session

The herdr profile SHALL close a place's tab in every session it made
one in when the place goes, and each of its base's workspaces once it
holds no tab of a live place and no pane flume did not open; and SHALL
read and tidy where they stand the workspaces and tabs it recorded
before it kept them by server (137, 138, C.5; record 0031).

#### Scenario: A place whose agents opened in two sessions

- **WHEN** a place's first stage opened in flume's own session, its stream then names `work`, its next stage opens in `work`, and the place lands and is tidied
- **THEN** its tab in each session is closed, and each base's workspace it was the last place of (137, 138)

#### Scenario: Records written before

- **WHEN** a place whose workspace and tab were recorded before they were kept by server is tidied
- **THEN** its tab and its base's workspace in flume's own session are closed as before (138)

### Requirement: The base's count on each of its workspaces

Status's `bases` SHALL give a repository's base once for each workspace
the terminal holds for it, each with the base's count and where that
workspace is, so the herdr plugin refreshes the count on the workspace
on its own server (139, C.5).

#### Scenario: One base in two sessions

- **WHEN** a repository's base has places in `work` and in `personal`, each session holding a workspace for it
- **THEN** `bases` gives the base twice with one count, each with its own workspace's `herdr:<id>@<socket>` (139)

### Requirement: herdr's attach

The herdr profile's attach for a window SHALL answer `<herdr> session
attach <name>`, `<herdr>` the full path of the herdr its calls run,
`flume` for its own session; SHALL first find or start the session's
server as open does; SHALL give `XDG_CONFIG_HOME` where its calls run
under one and no `HERDR_*`; and SHALL refuse a session herdr refuses in
herdr's words. Its attach for its app SHALL refuse, herdr 0.9.5 having
no app of its own (107, 187, C.5; core.29).

#### Scenario: A stream's session not yet running

- **WHEN** the profile, under a scratch session home, is asked to attach the session `w` for a window, which `herdr session list --json` lists not running
- **THEN** herdr lists `w` running, and the command is the pinned herdr's full path with `session attach w` and `XDG_CONFIG_HOME` the scratch home (187, S90)

#### Scenario: A client the command runs

- **WHEN** the command attach gave for `w` is run under a pseudo-terminal for two seconds
- **THEN** it is still running at their end, and `herdr session list --json` lists `w` running at the socket attach started, no other server of `w` started (187)

#### Scenario: The own session

- **WHEN** a profile whose own server is the session `flume` is asked to attach its own for a window, and a profile whose own server is held by its socket alone is asked the same
- **THEN** the first answers `session attach flume`, and the second refuses, saying herdr attaches by a session's name (187)

#### Scenario: A name herdr refuses

- **WHEN** the profile is asked to attach a session whose name the pinned herdr refuses
- **THEN** attach is refused with herdr's words, as open is for that name (187; core.29)

#### Scenario: herdr's app

- **WHEN** the profile is asked to attach `w` for its app
- **THEN** it refuses, saying herdr 0.9.5 has no app of its own and the Herdr app runs on herdr-core, and starts no server (187, C.5, S90)

### Requirement: The printed command refuses attach

The printed command's attach SHALL refuse for either view, saying it
holds no session a person can be shown, since its agents run in the
commands it printed (187).

#### Scenario: Attach under the printed command

- **WHEN** the printed command is asked to attach `work` for a window and then for its app
- **THEN** it refuses each with those words, and runs nothing (187, S91)

### Requirement: herdr's runs reads one session listing

The herdr profile's runs SHALL read one `herdr session list --json` for
every part asked, with the caller's `HERDR_*` stripped, and find each
session by its `name`, its own being `flume`: listed running, it runs;
listed not running or not listed, it does not. It SHALL start no server
(201, C.5; core.13).

#### Scenario: A session not listed stays so

- **WHEN** the profile, under a scratch session home, is asked runs of `w`, which `herdr session list --json` does not list
- **THEN** it answers `w` not running, and `herdr session list --json` still lists no `w` (201)

#### Scenario: A running session and the own session

- **WHEN** the profile has started its own session `flume` and the session `w` by opening an agent in each, and is asked runs of `w` and of its own
- **THEN** it answers `w` running and its own running as `flume` (107, 201)

#### Scenario: Several sessions in one listing

- **WHEN** the profile, its herdr a stand-in of the test's own that logs each call and runs the pinned herdr, has started its own session `flume` and the session `w`, and is asked runs of `w`, of `x`, which no listing names, and of its own in one call
- **THEN** it answers `w` running, `x` not running and its own running as `flume`, in that order, the stand-in logging the one `session list --json`, and `x` is still not listed (200, 201)

### Requirement: herdr's runs is bounded at one second

The herdr profile's runs SHALL bound its listing at one second, a
listing not read within it, refused or not parsed being unread in
herdr's words; and a profile whose own server is held by its socket
alone SHALL find its own part by `socket_path`, unread naming the
socket where none lists it (201, C.5; core.29).

#### Scenario: A herdr that does not answer

- **WHEN** the profile's herdr is a stand-in of the test's own that sleeps ten seconds before it answers, and the profile is asked runs of `w`
- **THEN** it answers unread within two seconds, saying herdr did not answer within its bound (201; core.29)

#### Scenario: An own server held by its socket

- **WHEN** a profile whose own server is held by the socket of the session `w` it started is asked runs of its own part, and then one held by a socket no session lists
- **THEN** the first answers running as `w`, and the second unread naming the socket (201)

### Requirement: The printed command's runs

The printed command's runs SHALL answer, for every part asked, that it
holds no part, saying it holds no session a person can be shown, since
its agents run in the commands it printed, and run nothing (187, 201).

#### Scenario: Runs under the printed command

- **WHEN** the printed command is asked runs of `work` and of its own part
- **THEN** it answers each that it holds no part, with those words, and runs nothing (201)

#### Scenario: Several parts under the printed command

- **WHEN** the printed command is asked runs of `work`, `work-design` and its own part in one call
- **THEN** it answers each of the three that it holds no part, with those words, in that order (201)

### Requirement: herdr gives a session's agents by name

The herdr profile SHALL find the session's socket by one bounded
`session list --json`, starting none, a session not running holding no
agent; then read `agent list` there, each agent with a string `name` by
its `pane_id` and `agent_status`, the status read as state reads it, at
`herdr:<pane_id>@<socket>`; and answer a call refused or unanswered as
not read, in herdr's words (132, C.5; core.29).

#### Scenario: A standing agent found and reached

- **WHEN** under the pinned herdr on a scratch server, a stand-in is started in the session `w` by `herdr agent start w-planner`, and the profile is asked for `w`'s agents
- **THEN** it lists `w-planner` waiting at `herdr:<pane>@<socket>`, and `send_to` that name delivers a notice the stand-in receives (95, 132, C.5)

#### Scenario: A session that does not run

- **WHEN** the profile is asked for the agents of `x`, which `herdr session list --json` does not list running
- **THEN** it lists none, and `x` is still not listed running (132; core.13)

#### Scenario: An agent with no name

- **WHEN** a session holds an agent herdr lists with no name beside `w-planner`
- **THEN** only `w-planner` is listed (C.5)

#### Scenario: The agent gone

- **WHEN** the stand-in under `w-planner` is closed and the profile is asked again
- **THEN** it lists no `w-planner` (132)

### Requirement: The printed command finds no named agent

The printed command SHALL answer a part's agents as not read, saying it
holds no agent a name finds, and run nothing (132, C.5).

#### Scenario: Agents under the printed command

- **WHEN** the printed command is asked for the agents of `madswan` and of its own part
- **THEN** it answers each not read, with those words, and runs nothing (132)

### Requirement: herdr's setup read for each running server

The herdr profile SHALL give as the terminal's setup the words in the
plugin's `setup.json` of each herdr server whose socket the file names
is still present, in order and once each, and nothing for a server
gone, calling no herdr command and reading no lease (122, C.5;
core.29).

#### Scenario: A running server and a gone one

- **WHEN** one server's state directory holds a `setup.json` naming its present socket and the popup's missing key, and another's names a socket now gone and the missing `$flume` rows
- **THEN** the profile's setup is the first server's words alone, and no herdr command was run

### Requirement: herdr settles nothing

The herdr profile SHALL start no settle, keep no watch over the host
and say nothing of whether flume runs attended, and nothing its plugin
runs SHALL either: flume reads attended from the host's watch alone,
running no herdr command for it (119, 122, 208, C.5).

#### Scenario: Unattended by the host's watch alone

- **WHEN** the herdr profile is bound against the pinned herdr's server, whatever the plugin's state there, and a caller settles every place with a bound of a minute, then three minutes pass with no such settle
- **THEN** flume runs unattended after the first, and attended after the three minutes, and no herdr command was run to decide either (119, 122, 208)

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

The Rex profile SHALL start the block, with no shell and not as a
login, so nothing runs between Rex and it, by a script of the start's
own, mode 0700, that removes itself and execs `/usr/bin/env -i` with
Rex's terminal variables, the login environment, the agent's
environment and flume's own, `PATH` led by a folder of the start's own
linking the program's kind to the program, then that link and the
arguments, so no variable stands in any process's arguments (29, 32,
94, C.5; core.30).

#### Scenario: No variable in any process's arguments

- **WHEN** an agent is given the variable `A_SECRET` and opens under Rex
- **THEN** the program sees `A_SECRET` and Rex's `REX_BLOCK`, no process under the block or of the server holds its value among its arguments, and the start's script is gone

#### Scenario: Nothing between Rex and the script

- **WHEN** the profile makes the block for an agent's start
- **THEN** its options give `"shell": "none"` and `"login": false`, so no `/usr/bin/login`, which a sandbox refuses to start, runs between Rex and the start's script (94, C.5)

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
process table for the block's program, the block's child, which SHALL
be the program the settings name by its path or, for a program a
script starts, the script's path among its arguments; anything else
SHALL close the block and refuse the open naming what ran (32, C.5).

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

### Requirement: Rex's attach for a window is refused

The Rex profile's attach for a window SHALL refuse before any call,
naming that the pinned Rex has no terminal client and only its app shows
a session (187, C.5).

#### Scenario: A window of Rex

- **WHEN** attach is asked of a session under Rex for a window
- **THEN** it is refused naming that Rex has no terminal client and only its app shows a session, and no Rex call is made

### Requirement: Rex's attach tells the app to show the newest Rex session flume made

The Rex profile's attach for the app SHALL find or start the part's
server as its server row does, take as the app the last client
`client.list` lists there whose `info.kind` is `app` and whose
`connection_state` is not `lost`, and tell it, by `-C <client_id> do
session.select session_id=<id>`, to show the newest Rex session the
profile's records hold for that server that `session.list` still lists,
answering shown once that call answers and recording nothing (107, 187,
C.5; core.29, core.48).

#### Scenario: The app shows the newest Rex session flume made

- **WHEN** the profile's records hold two Rex sessions on the server of `w`, the newer one made last, `session.list` lists both, and `client.list` lists a command line's client, an app whose connection is `lost` and an app connected after it
- **THEN** attach of `w` for the app answers shown, having run `-C <the connected app's client_id> do session.select session_id=<the newer session>` at `w`'s socket, and records nothing (187)

#### Scenario: A Rex session gone with its server

- **WHEN** the newest Rex session the records hold for `w`'s server is one `session.list` no longer lists, and an older one it lists
- **THEN** attach selects the older one it still lists

### Requirement: Rex's attach for the app refuses naming why

The Rex profile's attach for the app SHALL refuse, naming the part and
its socket: where no app is a client of the server, saying to add the
socket under the Rex app's Remote Hosts; where every app listed is
`lost`, saying its connection is lost; where no Rex session the profile
made stands in the server; and where the app refuses, in Rex's words
(187, C.5).

#### Scenario: A server no app is a client of

- **WHEN** attach of `w` for the app is asked against the pinned Rex while no server runs at `w`'s socket and no app is a client of it
- **THEN** it refuses, naming `<state>/terminal/rex/w.sock` and the Rex app's Remote Hosts, `w`'s server then runs, and no select is made (187, C.5)

#### Scenario: No Rex session of flume's in the server

- **WHEN** an app is a client of `w`'s server and `session.list` lists none of the Rex sessions the records hold for it
- **THEN** attach refuses saying the server holds no Rex session of flume's yet, naming its socket, and no select is made

#### Scenario: The app refuses

- **WHEN** the app's `do session.select` exits nonzero with `Error: client is not accepting actions`
- **THEN** attach refuses with `client is not accepting actions`, naming the Rex session it would have shown (187, C.5)

### Requirement: Rex labels a place's window and its base's Rex session

The Rex profile's show SHALL label each window the records hold for the
place, but one a tidy kept for a block flume did not open,
`<kind>--<name> · <line>` by `session.set_window_label`, `<kind>--<name>`
alone where the clues hold no line, and each Rex session the records
hold for its repository and base `<repository> <base> · <count>` by
`session.set_label`, each on its recorded id and each label cut to at
most 128 characters between characters; it SHALL relabel no block (139,
C.5; core.18).

#### Scenario: The labels follow the clues

- **WHEN** an agent of the place `fix--one` on `main` opens under Rex, and show is given the line `code · run 1 · working` and a count of 1 running
- **THEN** `session.view` gives its window the label `fix--one · code · run 1 · working`, `session.list` gives its base's Rex session `<repository> main · 1 running · 0 asking · 0 waiting on caller · 0 stalled`, and its block keeps its Rex name (139)

#### Scenario: A long label

- **WHEN** show is given a line that makes the window's label longer than 128 characters, non-ASCII characters among them
- **THEN** the label set is the first 128 characters of it, cut between characters

### Requirement: What Rex's show answers

The Rex profile's show SHALL answer shown where it set a label, shows
none where the records hold no window of the place and no Rex session of
its base, and failed, naming each not labelled with Rex's words or the
bound, where any was not; a window or Rex session Rex answers not found,
or whose server does not run, SHALL be passed over (139; core.29).

#### Scenario: A window Rex closed

- **WHEN** the place's agent is closed, so Rex closes the window with its last block, and show runs
- **THEN** the window is passed over with nothing failed, the base's Rex session is labelled, and show answers shown

#### Scenario: A server that does not answer

- **WHEN** the place's server is stopped so that it answers nothing, and show runs
- **THEN** show answers failed within its bound, naming the window and the Rex session not labelled, and nothing waits on Rex (139; core.29)

#### Scenario: Nothing recorded

- **WHEN** show is asked of a place for which the records hold no window and its base no Rex session
- **THEN** it answers shows none, and no Rex call is made

### Requirement: Where Rex shows a place's clues

The Rex profile's shown_at SHALL give, from its records alone and with
no Rex call, the place's window on its live agent's server, else its
newest, as `rex:<window_id>@<socket>`; the live agent's block as
`rex:<block>@<socket>`; the base's Rex session on that window's server,
else its newest, as `rex:<session_id>@<socket>`; and each of the base's
Rex sessions on every server; each none where the records hold none
(139).

#### Scenario: A place on two servers

- **WHEN** a place's first agent opened in flume's own server, its live agent in `w`'s, and its base has a Rex session in each
- **THEN** shown_at gives its window and its base's Rex session in `w`'s server, its live agent's block, and both of its base's Rex sessions, each by its id and socket (139)

#### Scenario: Nothing recorded for the place

- **WHEN** shown_at is asked of a place for which the records hold nothing
- **THEN** it gives none of each, and no Rex call is made

### Requirement: Rex carries no plugin

The Rex profile's setup SHALL give nothing, flume carrying no plugin for
Rex (122).

#### Scenario: Rex's setup

- **WHEN** status asks the Rex profile what its terminal's configuration lacks
- **THEN** it gives nothing, and no Rex call is made

### Requirement: A command's pane is named as its agents are given

A terminal profile that gives a part's agents by name SHALL name the
session a command of flume's runs in in the form its `agents_in` gives
an agent's session name, so a command in an agent's pane names that
agent's: `herdr:<pane>@<socket>` under herdr and `rex:<block>@<socket>`
under Rex, each with the socket its `agents_in` gives the part's agents
at, checked against the pinned release (7, 132, C.5; core.29).

#### Scenario: A command in a herdr session's pane

- **WHEN** under the pinned herdr, the session `w` runs the agent `w-planner`, and a command of the test's own runs in another pane of `w`'s server
- **THEN** the session the profile names for that command is `herdr:<that pane>@<socket>`, the socket the profile's `agents_in` of `w` gives `w-planner` at (132, C.5)

#### Scenario: A command in a labelled Rex block

- **WHEN** under the pinned Rex, a block labelled `ops` in the session `east` runs the stand-in, and the profile is given the `REX_BLOCK` and `REX_SERVER` that block's program ran under
- **THEN** the session the profile names for a command run there is the session name its `agents_in` of `east` gives `ops` at (132, C.5)
