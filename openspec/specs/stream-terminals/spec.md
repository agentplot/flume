# stream-terminals Specification

## Purpose
Each stream's agents run in a terminal of the stream's own: the
multiplexer the host names for the stream, else the host's, read at
each start and recorded with it. An agent stays in the terminal it
opened in, a place is tidied in every terminal its agents opened in, a
recipient is found in the terminal that names it, and flume runs
attended while any terminal the host binds keeps no watch (7, 43, 105,
119, 137, 143, 190).

## Requirements

### Requirement: A stage's agent opens in its stream's multiplexer

A stage's agent SHALL open in the terminal its place's stream's
`multiplexer` names, else the host's, else the printed command, read at
each start from the host's configuration as it stands then, and the
start SHALL record it with where it came from, `stream`, `host` or
`neither`. No launch's setting, kind, repository or label SHALL name
one (105, 143, 190).

#### Scenario: Two streams under two terminals

- **WHEN** the fake host's own multiplexer is `fake`, it declares `trial` with the multiplexer `other`, and a fix is launched on `work` and one on `trial`
- **THEN** the work fix's agent opens in `fake` from `host`, the trial fix's in `other` from `stream`, and each terminal's log names only its own (190, S91)

#### Scenario: A stream naming none on a host naming none

- **WHEN** neither the fake host nor its stream `work` names a multiplexer, and a fix is launched on `work`
- **THEN** its agent opens in the terminal the host's reader names for neither, from `neither` (105, 190)

### Requirement: An agent stays in the terminal it opened in

Every call on an agent after its open, a send, a state, a screen, an
answer and a close, SHALL go to the terminal its start recorded, a start
recording none going to the host's; a stream's multiplexer changed
between starts SHALL leave a live agent where it opened and open the
next start in the new one (32, 107, 190).

#### Scenario: A multiplexer changed while a change runs

- **WHEN** a change on `trial` runs its construct stage in `other`, and the fake host's `trial` is then set to the multiplexer `fake`
- **THEN** the construct agent's state, its tell and its end are read and sent through `other`, and the code stage opens in `fake` (190, S91)

#### Scenario: A start written before

- **WHEN** a place's held state written before holds a start with no terminal, its agent live
- **THEN** its agent is read and closed through the host's terminal, and status gives its terminal as the host's (190)

### Requirement: A place is tidied in each terminal its agents opened in

A place's tidy SHALL ask tidy of each distinct terminal its starts
recorded, the host's for a start recording none, and its stream's as
the configuration stands where it has no start; its clues SHALL go to
the terminal of its live start, else its newest, else its stream's
(137, 139, 190).

#### Scenario: A place in two terminals

- **WHEN** a change ran its construct stage in `other` and its code stage in `fake`, and it lands
- **THEN** its tidy asks tidy of `other` and of `fake`, each once, and the tidy's answer names both (137, 190)

### Requirement: A recipient is found in the terminal that names it

The calling command's session SHALL be the one the first terminal the
host binds that names one gives, the host's own first, then each
stream's by the stream's name; a notice and a recipient's state SHALL
go to the first terminal in that order that holds a session by its
name, and none holding it SHALL be not found (7, 43, 95, 190).

#### Scenario: A caller under a stream's terminal

- **WHEN** the host's own fake terminal names no calling session, `trial`'s names `caller-1`, and a fix is launched naming no recipient
- **THEN** the fix's recipient is `caller-1`, and its end is sent to `caller-1` through `trial`'s terminal (7, 43, 190)

### Requirement: Status gives each stream's terminal and each agent's

Status SHALL give each stream's multiplexer with where it came from,
each start's terminal beside its session, and each terminal the host
binds with what it keeps and its watch, in the JSON form and the
person's (56, 190; core.14).

#### Scenario: The terminals in status

- **WHEN** status is asked of the host of two streams under two terminals, each running a fix
- **THEN** `work` gives `fake` from `host` and `trial` `other` from `stream`, each agent gives its terminal, and the host's terminals list `fake` and `other`, each with its watch (56, 190, S91)

### Requirement: No terminal decides whether flume runs attended

The terminals the host binds, its own and each declared stream's that a
profile carries, SHALL settle nothing and decide nothing of whether
flume runs attended: while no caller keeps watch, every command that
acts on places SHALL settle every place first, and a wait SHALL drive
its settles, under every terminal alike (119, 122, 190, 208).

#### Scenario: One stream under another terminal

- **WHEN** a caller keeps watch, the host's terminal is `fake`, `trial`'s is `other`, and a caller drops a place on `work`
- **THEN** the drop settles nothing first, and `trial`'s places move at the caller's settles (190, 208, S91)

#### Scenario: Two terminals and no watch

- **WHEN** no caller keeps watch, the host's terminal is `fake` and `trial`'s is `other`
- **THEN** a caller's drop settles every place first, under either terminal (119, 208)

### Requirement: A stream naming Rex runs its places in Rex

A stream whose `multiplexer` is `rex` SHALL have its agents opened, sent
prompts, read and closed in the Rex server of its session, a server of
flume's own, its places' clues shown on their windows and their bases'
Rex sessions, its places tidied there, and its launches and starts taken
as any stream's are (94 to 98, 137, 139, 190, C.5).

#### Scenario: A fix on a stream under Rex

- **WHEN** the host's `[streams.trial]` gives `multiplexer = "rex"`, and a fix is launched on `trial` through the binary with the stand-in for Claude Code
- **THEN** its agent opens in a block of the Rex server at `<state>/terminal/rex/trial.sock`, in its base's Rex session and the window made for `fix--<name>`; while it works, the window is labelled `fix--<name> · ` and the place's short line, and the Rex session `<repository> main · 1 running · 0 asking · 0 waiting on caller · 0 stalled`; it takes its prompt, delivers, and the fix lands; status gives the agent's terminal `rex` from `stream`, and once the place is tidied its window and its Rex session are gone and the server still runs (139, 190, S91)

### Requirement: A stream whose multiplexer flume carries no profile for is refused alone

A stream whose `multiplexer` names a terminal profile the resolver does
not carry SHALL refuse a launch on it whole and each start of a place
on it, naming the stream, the name and the profiles carried, and
nothing else on the host; status SHALL give its refusal with the stream
(4, 190; core.8).

#### Scenario: A stream naming cmux

- **WHEN** the fake host's `trial` names the multiplexer `cmux`, and a fix is launched on `trial` and one on `work`
- **THEN** the trial fix is refused naming `trial`, `cmux` and the profiles carried, nothing made, the work fix opens its agent, and status gives `trial`'s refusal (4, 190, S91)

#### Scenario: A stream changed to cmux while it waits

- **WHEN** a place on `trial` waits for room, and `trial` is then set to the multiplexer `cmux`
- **THEN** its start is refused naming `cmux` as a setting is, and nothing opens (4, 190)

### Requirement: A Rex stream's session is shown in the Rex app

`flume open` of a session of a stream whose `multiplexer` is `rex` and
whose view is `app` SHALL show it through the Rex profile's attach for
the app, and SHALL be refused in the profile's words where it refuses,
naming the socket a person adds under the Rex app's Remote Hosts where
the app is no client of the session's server (186, 187, C.5).

#### Scenario: A Rex stream's session with no app added

- **WHEN** the host's `[streams.trial]` gives `multiplexer = "rex"` and `view = "app"`, and `flume open trial` is run through the binary with no app a client of trial's server
- **THEN** it is refused naming `<state>/terminal/rex/trial.sock` and the Rex app's Remote Hosts, trial's server runs, and no place's held state is written (186, 187, C.5)
