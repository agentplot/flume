## Purpose

How a person is shown a stream's agents: `flume open [<stream>]` shows
the stream's own session in its view, the stream's else the host's, a
window its terminal app opens under the stream's title, or its
multiplexer's own app; flume keeps nothing of the window, and the
answer names no variable's value (107, 120, 186 to 189).

## ADDED Requirements

### Requirement: A person opens a stream's session in its view

`flume open [<stream>]` SHALL show the stream's own session, `default`'s
where none is named, resolved as a start resolves its stream's, in the
stream's `view`, else the host's, else `terminal`, through the stream's
multiplexer: for `terminal`, the terminal's attach for a window giving
the command and the stream's terminal app showing it under the stream's
`title`, else its name; for `app`, the terminal's attach for its app
alone (107, 186 to 188).

#### Scenario: A stream's window

- **WHEN** the fake host declares `work` with the title `Work Agents` and no session, and `open_stream` is asked for `work`
- **THEN** the fake terminal's log names `attach work for terminal`, the fake window's log names its command under `Work Agents`, and the answer gives `work` from `stream-name`, the view `terminal` from `neither`, `Work Agents` from `stream` and the window opened (186, S90)

#### Scenario: A stream that names its session and no title

- **WHEN** the fake host's `work` gives `session = "office"` and no title
- **THEN** the terminal is asked to attach `office`, and the window's title is `work` from `stream-name` (107, 143)

#### Scenario: The default stream

- **WHEN** `open_stream` is asked for no stream on a host whose `default` names no session
- **THEN** the terminal is asked to attach its own session, the session given as none from `own`, under the title `default` (107, S90)

#### Scenario: The same session and terminal as a stage's agent

- **WHEN** the fake host's `trial` names the multiplexer `other`, a fix is launched on `trial`, and `open_stream` is asked for `trial`
- **THEN** the fix's agent opened in `other`'s session `trial`, and `other` is the terminal asked to attach `trial` (107, 190, S90)

#### Scenario: A stream's own terminal app

- **WHEN** the fake host's own terminal app is `fake-window`, its `office` names the terminal app `other-window`, and `open_stream` is asked for `office` and then `work`
- **THEN** `other-window` logs the show for `office` and `fake-window` the one for `work`, the answers giving each from `stream` and `host` (186, 188, S90)

### Requirement: A stream whose view is the app

Where a stream's view is `app`, `open` SHALL ask the terminal's attach
for its app alone, give the window no title, read no terminal app and
ask nothing of the window seam; a terminal that offers no app SHALL
refuse in its own words (186, 187).

#### Scenario: The app shows the session

- **WHEN** the fake host's `work` gives `view = "app"` and the fake terminal answers attach for its app `Shown`
- **THEN** the fake terminal's log names `attach work for app`, the answer gives no title, no terminal app, no command and the window as the app's, and the fake window logs nothing (186, 187)

#### Scenario: A terminal with no app

- **WHEN** the fake terminal is scripted to refuse attach for its app with `no app of its own`
- **THEN** the open is refused with those words, and the fake window logs nothing (187, S90)

### Requirement: Opening a view changes nothing flume holds

`open` SHALL take no lock, write no held state, settle nothing and keep
nothing of the window; a second asking SHALL open a second window, and
no launch, settle, start or wake SHALL ask the window seam for one or a
terminal's attach (120, 186, 189; core.17, core.48).

#### Scenario: Two askings

- **WHEN** `open_stream` is asked for `work` twice over a host holding one live place
- **THEN** the fake window logs two shows, and the store and the place's held state are as they were before the first (186, 189, S90)

#### Scenario: A place run through

- **WHEN** a change runs through on the fakes from its launch to its landing, and its tidy
- **THEN** the fake window logs no show and the fake terminal no attach (186; core.48)

### Requirement: What refuses an open

`open` SHALL be refused, naming why and opening nothing, for a stream
the host does not declare, a stream whose multiplexer or terminal app
names no profile, a terminal's refused attach in its words, and a
window's refused show in its words (4, 186 to 188, 190).

#### Scenario: An undeclared stream

- **WHEN** `open_stream` is asked for `nightly` and the host declares no `nightly`
- **THEN** it is refused naming `nightly` and the streams the host declares, and the fake terminal and window log nothing (4, 186)

#### Scenario: A terminal that holds no session

- **WHEN** the fake terminal is scripted to refuse attach for a window with `no session here`
- **THEN** the open is refused with those words, and the fake window logs nothing (187)

#### Scenario: A terminal app no profile carries

- **WHEN** the fake host's `work` names the terminal app `kitty`, which the fake resolver does not carry
- **THEN** the open is refused naming `kitty` and the window profiles carried, the fake terminal logging no attach and no window asked (188, C.8)

#### Scenario: A window that refuses

- **WHEN** the fake window is scripted to refuse with `no display`
- **THEN** the open is refused with those words (188)

### Requirement: The open's answer names no variable's value

The open's answer SHALL give the stream, its multiplexer, session, view,
title and terminal app each with where it came from, the command's
program, its arguments and the names of the variables it needs, never
their values, and what the window did: opened, the line printed, or the
app's own (56, 61, 186; core.14, core.30).

#### Scenario: A command that needs a variable

- **WHEN** the fake terminal answers attach with a command needing `XDG_CONFIG_HOME` set to `/scratch/home`, and the fake window answers printed
- **THEN** the answer's command names `XDG_CONFIG_HOME` and holds no `/scratch/home`, in its JSON form and the person's, and gives the printed line (186; core.30)
