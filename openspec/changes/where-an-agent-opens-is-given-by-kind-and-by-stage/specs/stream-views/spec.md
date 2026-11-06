## MODIFIED Requirements

### Requirement: What refuses an open

`open` SHALL be refused, naming why and opening nothing, for a name
that is no stream the host declares and no session a stream names for a
kind, naming the streams and the sessions each names for its kinds; a
session two streams' words give, naming both; a stream whose
multiplexer or terminal app names no profile; a terminal's refused
attach in its words; and a window's refused show in its words (4, 186
to 188, 190, 199).

#### Scenario: An undeclared stream

- **WHEN** `open_stream` is asked for `nightly` and the host declares no `nightly`
- **THEN** it is refused naming `nightly` and the streams the host declares, and the fake terminal and window log nothing (4, 186)

#### Scenario: A name that is no stream and no session

- **WHEN** the fake host's `work` names `work-design` for research, and `open_stream` is asked for `review`
- **THEN** it is refused naming `review`, the stream `work` and its session `work-design`, and the fake terminal and the fake window log nothing (186, 199, S107)

#### Scenario: A session two streams' words give

- **WHEN** the fake host's `work` names `personal` for research, `personal` naming no session, and `open_stream` is asked for `personal`
- **THEN** it is refused naming `work`, `personal` and the session `personal`, and the fake terminal and the fake window log nothing (4, 199)

#### Scenario: A terminal that holds no session

- **WHEN** the fake terminal is scripted to refuse attach for a window with `no session here`
- **THEN** the open is refused with those words, and the fake window logs nothing (187)

#### Scenario: A terminal app no profile carries

- **WHEN** the fake host's `work` names the terminal app `kitty`, which the fake resolver does not carry
- **THEN** the open is refused naming `kitty` and the window profiles carried, the fake terminal logging no attach and no window asked (188, C.8)

#### Scenario: A window that refuses

- **WHEN** the fake window is scripted to refuse with `no display`
- **THEN** the open is refused with those words (188)

## ADDED Requirements

### Requirement: A person opens a session a stream names for a kind by its name

`open` SHALL read the name it is given as a stream's first, and
otherwise as a session one stream names for a kind, and SHALL show that
session as it shows the stream's own, in that stream's view, through its
multiplexer and terminal app (186, 187); a window's title SHALL be the
session's name, but where the session is the stream's own, the
stream's title, and the answer SHALL give the stream, the session from
`stream-kind` and the kinds it is named for (186, 199).

#### Scenario: The design session by its name

- **WHEN** the fake host's `work` names `work-design` for research and declares the title `Work Agents`, and `open_stream` is asked for `work-design`
- **THEN** the fake terminal's log names `attach work-design for terminal`, the fake window's log names its command under `work-design`, and the answer gives the stream `work`, `work-design` from `stream-kind` named for research (186, 199, S107)

#### Scenario: The stream's name still opens its own

- **WHEN** `open_stream` is asked for `work` on the same host
- **THEN** the terminal is asked to attach `work`, under the title `Work Agents`, from `stream-name` (186)

#### Scenario: A stream's own session named for a kind

- **WHEN** the fake host's `[streams.work]` gives `session = "office"` and the title `Work Agents` and names `office` for research, and `open_stream` is asked for `office`
- **THEN** the terminal is asked to attach `office`, under the title `Work Agents`, as for `work` (186, 199)

#### Scenario: A design session in the stream's app

- **WHEN** the fake host's `work` gives `view = "app"` and names `work-design` for research, and `open_stream` is asked for `work-design`
- **THEN** the fake terminal's log names `attach work-design for app`, and the fake window logs nothing (186, 187, 199)
