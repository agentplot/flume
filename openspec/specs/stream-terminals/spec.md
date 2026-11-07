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

### Requirement: flume runs attended while any terminal it binds keeps no watch

The terminals the host binds SHALL be its own and each declared
stream's that a profile carries, each once; where any keeps no watch,
every command that acts on places SHALL settle every place first, and a
wait SHALL drive its settles (119, 122, 190).

#### Scenario: One stream under a terminal with no watch

- **WHEN** the host's own fake terminal keeps a watch, `trial`'s keeps none, and a caller drops a place on `work`
- **THEN** the drop settles every place first (119, S91)

### Requirement: A stream whose multiplexer names no profile is refused alone

A stream whose `multiplexer` names a terminal profile the resolver does
not carry SHALL refuse a launch on it whole and each start of a place
on it, naming the stream, the name and the profiles carried, and
nothing else on the host; status SHALL give its refusal with the stream
(4, 190; core.8).

#### Scenario: A stream naming rex

- **WHEN** the fake host's `trial` names the multiplexer `rex`, and a fix is launched on `trial` and one on `work`
- **THEN** the trial fix is refused naming `trial`, `rex` and the profiles carried, nothing made, the work fix opens its agent, and status gives `trial`'s refusal (4, 190, S91)

#### Scenario: A stream changed to rex while it waits

- **WHEN** a place on `trial` waits for room, and `trial` is then set to the multiplexer `rex`
- **THEN** its start is refused naming `rex` as a setting is, and nothing opens (4, 190)

### Requirement: Status gives each stream's terminal and each agent's

Status SHALL give each stream's multiplexer with where it came from,
each start's terminal beside its session, and each terminal the host
binds with what it keeps and its watch, in the JSON form and the
person's (56, 190; core.14).

#### Scenario: The terminals in status

- **WHEN** status is asked of the host of two streams under two terminals, each running a fix
- **THEN** `work` gives `fake` from `host` and `trial` `other` from `stream`, each agent gives its terminal, and the host's terminals list `fake` and `other`, each with its watch (56, 190, S91)
