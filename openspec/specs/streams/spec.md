# streams Specification

## Purpose
The host's named streams of work: each with its own bound, priority and
pool, a place's stream resolved once at its launch from the launch, its
intent or the host's configuration, and the queue taking stages by the
stream's priority before the place's.

## Requirements

### Requirement: A host declares its streams

The host's configuration SHALL declare streams by name, each with its
bound on agents running, the host's bound alone holding it where it
gives none, its priority, a whole number, a stream of higher priority
starting first, zero where it gives none, and its pool. The stream
`default` SHALL always stand, held by the host's bound alone unless a
`[streams.default]` table states its own. The host's bound SHALL cap
every stream's agents together (70, 143, C.7).

#### Scenario: Two streams declared

- **WHEN** the host's configuration gives `[host] agents = 8`, `[streams.work]` with `agents = 5`, `priority = 10` and `pool = "work"`, and `[streams.personal]` with `agents = 3`
- **THEN** the host reads three streams, `work`, `personal` and `default`, `personal` at priority zero and `default` bounded by the host's eight alone (143, C.7)

#### Scenario: No streams

- **WHEN** the host's configuration declares no stream
- **THEN** every place runs in `default`, bounded by `[host] agents` alone (70)

### Requirement: A place's stream is chosen once, at its launch

A place's stream SHALL be the launch's, else its intent's affinity's,
else the one the host names for its repository and kind, for its
repository, or for its kind, in that order, else `default`; it SHALL
be fixed at the launch and kept with where it came from. A launch
naming a stream the host does not declare SHALL be refused naming it,
leaving nothing made (4, 8, 143, 147).

#### Scenario: Each source in its order

- **WHEN** the host names `stream = "work"` for `[repos."/r"]` and `stream = "personal"` for `[kinds.chore]`, and chore@1 is launched in `/r` naming no stream and no intent with an affinity
- **THEN** the place runs in `work`, from `host-repository`, since the repository's word comes before the kind's (143)

#### Scenario: The launch's stream

- **WHEN** the same launch names `--stream personal`
- **THEN** the place runs in `personal`, from `launch` (143)

#### Scenario: An undeclared stream

- **WHEN** a launch names `--stream nightly` and the host declares no `nightly`
- **THEN** the launch is refused naming `nightly`, and nothing is made (4)

#### Scenario: The stream fixed at the launch

- **WHEN** the host's configuration moves `/r` to `personal` while a place of `/r` runs in `work`
- **THEN** the place's later stages still run in `work`, and a place launched after runs in `personal` (8, 143)

#### Scenario: A later place of an intent

- **WHEN** an intent's affinity stands in `personal` and a launch of the intent in `/r`, whose host's word is `work`, names no stream
- **THEN** the place runs in `personal`, from `intent` (143, 147)

### Requirement: Status gives each place's stream and each stream's queue

Status SHALL give each place's stream and where it came from, and for
each stream of the host its agents running, its bound, its room and the
stages waiting in it in the order they would start (56, 143).

#### Scenario: A stream's part of status

- **WHEN** status is asked of a host running three agents in `work`, bounded at four, with one place waiting in it
- **THEN** the host's `streams` give `work` with three running, a bound of four, a room of one, and the waiting place, and the place's own status names `work` and where it came from (56)

### Requirement: A stream names its multiplexer, its view, its terminal app and its title

`[host]` SHALL take `multiplexer`, `view` and `terminal_app`, and
`[streams.<stream>]` the same three and `title`, a stream's over the
host's. Neither naming a multiplexer or a terminal app SHALL be the
printed command, neither naming a view `terminal`, and a stream naming
no title its name (105, 143, 186, 190, C.7).

#### Scenario: The keys and none

- **WHEN** the host's configuration gives `[host] multiplexer = "herdr"` and `terminal_app = "ghostty"`, `[streams.office]` with `terminal_app = "terminal"` and `title = "Office Herdr"`, and `[streams.personal]` with none
- **THEN** `office` reads `herdr` from `host`, the view `terminal` from `neither`, `terminal` from `stream` and `Office Herdr` from `stream`, and `personal` reads `herdr` and `ghostty` from `host`, the view `terminal` from `neither` and the title `personal` from `stream-name` (143, 186, 190)

### Requirement: The three settings' values

`multiplexer` and `terminal_app` SHALL be names and `title` a text, and
`view` SHALL be `terminal` or `app`, any other value refusing the
configuration; `[host] terminal` SHALL be read as `[host] multiplexer`,
a `[host]` giving both refused naming both (186, 190, C.7).

#### Scenario: The word written before

- **WHEN** the host's configuration gives `[host] terminal = "herdr"`, and then `[host] terminal = "herdr"` and `multiplexer = "printed"` together
- **THEN** the first reads the host's multiplexer `herdr`, and the second is refused naming `terminal` and `multiplexer` (C.7)

#### Scenario: A view no word names

- **WHEN** `[streams.work]` gives `view = "tab"`
- **THEN** the configuration is refused naming `streams.work`'s `view` and its two values (C.7)

#### Scenario: A title that is no text

- **WHEN** `[streams.work]` gives `title = 3`
- **THEN** the configuration is refused naming `streams.work`'s `title` (C.7)

### Requirement: Status gives each stream's display

Status SHALL give each stream's multiplexer, view, terminal app and
title, each with where it came from, and a stream's refusal where its
multiplexer or terminal app names no profile, in the JSON form and the
person's (56, 143, 186, 190; core.14).

#### Scenario: The display in status

- **WHEN** status is asked of a host whose `office` names its terminal app and title and whose `personal` names none
- **THEN** `office` gives its terminal app and title from `stream`, and `personal` the host's multiplexer and terminal app from `host`, the view from `neither` and its name as its title from `stream-name` (56, 143)

### Requirement: A stream's agents take the host's settings for the stream

A place's agents SHALL take the settings `[agents.streams.<stream>]`
gives for its stream, its own keys and the file it names, read as it
stands at each start, after the host's for the repository and kind and
for the kind and before the host's for every agent;
`[streams.<stream>]` SHALL keep its bound, priority and pool, and give
no setting (32, 143).

#### Scenario: Two streams' status lines

- **WHEN** the host's `[agents.programs.agent]` names a file giving `line = "host"` and `tui = "fullscreen"`, its `[agents.streams.work.programs.agent.settings]` gives `line = "work"`, and two fixes are launched, one in `work` and one in `default`
- **THEN** the fix in `work` is given `line = "work"` from `host-stream` and the file's `tui` from `host`, and the one in `default` the file's `line = "host"` and `tui`, each from `host` (S100, 143)

### Requirement: A bound on agents is a whole number from 1

The host's bound and each stream's SHALL each be a whole number of
agents from 1, the host's none where it gives none, and a bound of 0,
the host's or a stream's, SHALL refuse the configuration naming its
key (70, 143, C.7).

#### Scenario: A host's bound of zero

- **WHEN** the managed layer gives `[host] agents = 0`
- **THEN** reading the configuration is refused naming `[host] agents` and that it is a whole number from 1 (70, C.7)
