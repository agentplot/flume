## MODIFIED Requirements

### Requirement: A person opens a stream's session in its view

`flume open [<session>]` SHALL show the session of that name, always a
session and never a stream: a stream's own session, the one its
`session` names, else the stream's name, or a session a stream names for
a kind or a stage; the host's default stream's own where none is named.
It SHALL be shown in its stream's `view`, else the host's, else
`terminal`, through its stream's multiplexer: for `terminal`, the
terminal's attach for a window giving the command and the stream's
terminal app showing it, under the stream's `title`, else its name, for
the stream's own session, and under the session's name for a session the
stream names for a kind or a stage; for `app`, the terminal's attach for
its app alone (107, 186 to 188, 199).

#### Scenario: A stream's window

- **WHEN** the fake host declares `work` with the title `Work Agents` and no session, and `open_session` is asked for `work`
- **THEN** the fake terminal's log names `attach work for terminal`, the fake window's log names its command under `Work Agents`, and the answer gives the stream `work`, the session `work` from `stream-name`, the view `terminal` from `neither`, `Work Agents` from `stream` and the window opened (186, S90)

#### Scenario: A stream that names its session and no title

- **WHEN** the fake host's `work` gives `session = "office"` and no title, and `open_session` is asked for `office`
- **THEN** the terminal is asked to attach `office`, and the window's title is `work` from `stream-name` (107, 143, 186)

#### Scenario: The default stream

- **WHEN** `open_session` is asked for no session on a host whose `default` names no session
- **THEN** the terminal is asked to attach its own session, the session given as none from `own`, under the title `default` (107, S90)

#### Scenario: The same session and terminal as a stage's agent

- **WHEN** the fake host's `trial` names the multiplexer `other`, a fix is launched on `trial`, and `open_session` is asked for `trial`
- **THEN** the fix's agent opened in `other`'s session `trial`, and `other` is the terminal asked to attach `trial` (107, 190, S90)

#### Scenario: A stream's own terminal app

- **WHEN** the fake host's own terminal app is `fake-window`, its `office` names the terminal app `other-window`, and `open_session` is asked for `office` and then `work`
- **THEN** `other-window` logs the show for `office` and `fake-window` the one for `work`, the answers giving each from `stream` and `host` (186, 188, S90)

#### Scenario: The design session by its name

- **WHEN** the fake host's `work` names `work-design` for research and declares the title `Work Agents`, and `open_session` is asked for `work-design`
- **THEN** the fake terminal's log names `attach work-design for terminal`, the fake window's log names its command under `work-design`, and the answer gives the stream `work`, `work-design` from `stream-kind` named for research (186, 199, S107)

#### Scenario: A stage's session by its name

- **WHEN** the fake host's `work` names `work-review` for release's review, and `open_session` is asked for `work-review`
- **THEN** the fake terminal's log names `attach work-review for terminal` under the title `work-review`, and the answer gives `work-review` from `stream-stage` named for `release.review` (186, 199)

#### Scenario: A stream's own session named for a kind

- **WHEN** the fake host's `[streams.work]` gives `session = "office"` and the title `Work Agents` and names `office` for research, and `open_session` is asked for `office`
- **THEN** the terminal is asked to attach `office`, under the title `Work Agents`, as the stream's own (186, 199)

#### Scenario: A design session in the stream's app

- **WHEN** the fake host's `work` gives `view = "app"` and names `work-design` for research, and `open_session` is asked for `work-design`
- **THEN** the fake terminal's log names `attach work-design for app`, and the fake window logs nothing (186, 187, 199)

### Requirement: Opening a view changes nothing flume holds

`open` SHALL take no lock, write no held state, settle nothing and keep
nothing of the window; a second asking SHALL open a second window, and
no launch, settle, start or wake SHALL ask the window seam for one or a
terminal's attach (120, 186, 189; core.17, core.48).

#### Scenario: Two askings

- **WHEN** `open_session` is asked for `work` twice over a host holding one live place
- **THEN** the fake window logs two shows, and the store and the place's held state are as they were before the first (186, 189, S90)

#### Scenario: A place run through

- **WHEN** a change runs through on the fakes from its launch to its landing, and its tidy
- **THEN** the fake window logs no show and the fake terminal no attach (186; core.48)

### Requirement: What refuses an open

`open` SHALL be refused, naming why and opening nothing, for a name that
is no session a stream's word gives, naming each stream's sessions; a
session two streams' own words give, naming both; a stream whose
multiplexer or terminal app names no profile; a terminal's refused
attach in its words; and a window's refused show in its words (4, 186
to 188, 190, 199).

#### Scenario: An undeclared stream

- **WHEN** `open_session` is asked for `nightly`, and the host declares no `nightly` and no stream's word gives a session of that name
- **THEN** it is refused naming `nightly` and each stream's sessions, and the fake terminal and window log nothing (4, 186)

#### Scenario: A name no stream's word gives

- **WHEN** the fake host declares `work`, naming `work-design` for research, and `open_session` is asked for `nightly`
- **THEN** it is refused naming `nightly`, the stream `work` and its sessions `work` and `work-design`, and the fake terminal and the fake window log nothing (4, 186, 199, S107)

#### Scenario: A stream's name that is not its session's

- **WHEN** the fake host's `work` gives `session = "office"`, and `open_session` is asked for `work`
- **THEN** it is refused naming `work` and the stream `work`'s session `office`, and the fake terminal and the fake window log nothing (186)

#### Scenario: A session two streams' own words give

- **WHEN** the fake host's `work` and `personal` each give `session = "shared"`, and `open_session` is asked for `shared`
- **THEN** it is refused naming `work`, `personal` and `shared`, and the fake terminal and the fake window log nothing (4, 186)

#### Scenario: A terminal that holds no session

- **WHEN** the fake terminal is scripted to refuse attach for a window with `no session here`
- **THEN** the open is refused with those words, and the fake window logs nothing (187)

#### Scenario: A terminal app no profile carries

- **WHEN** the fake host's `work` names the terminal app `kitty`, which the fake resolver does not carry
- **THEN** the open is refused naming `kitty` and the window profiles carried, the fake terminal logging no attach and no window asked (188, C.8)

#### Scenario: A window that refuses

- **WHEN** the fake window is scripted to refuse with `no display`
- **THEN** the open is refused with those words (188)
