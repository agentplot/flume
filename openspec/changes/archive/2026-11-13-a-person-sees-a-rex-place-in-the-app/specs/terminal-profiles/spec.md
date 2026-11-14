## REMOVED Requirements

### Requirement: Rex's attach and show

**Reason**: The Rex profile tells the Rex app to show a session and
labels a place's window and its base's Rex session, as C.5's attach and
show rows say (139, 187).

**Migration**: "Rex's attach for a window is refused", "Rex's attach
tells the app to show the newest Rex session flume made", "Rex's attach
for the app refuses naming why", "Rex labels a place's window and its
base's Rex session", "What Rex's show answers", "Where Rex shows a
place's clues" and "Rex carries no plugin" below.

## ADDED Requirements

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
