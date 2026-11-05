## ADDED Requirements

### Requirement: A person opens a stream's session in a window

`flume open [<stream>]` SHALL open a window on the stream's own session,
`default` where none is named, resolved as a start resolves its stream's,
under the stream's `title`, else its name: the terminal's attach giving
the command, then the window seam's show running it under that title
(107, 186 to 188).

#### Scenario: A stream's window

- **WHEN** the fake host declares `work` with the title `Work Herdr` and no session, and `open_stream` is asked for `work`
- **THEN** the fake terminal's log names `attach work titled Work Herdr`, the fake window's log names its command under `Work Herdr`, and the answer gives `work` from `stream-name`, `Work Herdr` from `stream` and the window opened (186, S90)

#### Scenario: A stream that names its session and no title

- **WHEN** the fake host's `work` gives `session = "office"` and no title
- **THEN** the terminal is asked to attach `office`, and the window's title is `work` from `stream-name` (107, 143)

#### Scenario: The default stream

- **WHEN** `open_stream` is asked for no stream on a host whose `default` names no session
- **THEN** the terminal is asked to attach its own session, the session given as none from `own`, under the title `default` (107, S90)

#### Scenario: The same session as a stage's agent

- **WHEN** the fake host declares `work` with no session, a fix is launched on `work`, and `open_stream` is asked for `work`
- **THEN** the fix's agent opened in the session the window was asked to attach, `work` (107, S90)

### Requirement: Opening a window changes nothing flume holds

`open` SHALL take no lock, write no held state, settle nothing and keep
nothing of the window; a second asking SHALL open a second window, and
no launch, settle, start or wake SHALL ask the window seam for one (120,
186, 189; core.17, core.48).

#### Scenario: Two askings

- **WHEN** `open_stream` is asked for `work` twice over a host holding one live place
- **THEN** the fake window logs two shows, and the store and the place's held state are as they were before the first (186, 189, S90)

#### Scenario: A place run through

- **WHEN** a change runs through on the fakes from its launch to its landing, and its tidy
- **THEN** the fake window logs no show (186; core.48)

### Requirement: What refuses an open

`open` SHALL be refused, naming why and opening nothing, for a stream
the host does not declare, a terminal's refused attach in its words, and
a window's refused show in its words; a terminal that shows the part in
its own window SHALL ask nothing of the window seam (4, 186 to 188).

#### Scenario: An undeclared stream

- **WHEN** `open_stream` is asked for `nightly` and the host declares no `nightly`
- **THEN** it is refused naming `nightly` and the streams the host declares, and the fake terminal and window log nothing (4, 186)

#### Scenario: A terminal that holds no session

- **WHEN** the fake terminal is scripted to refuse attach with `no session here`
- **THEN** the open is refused with those words, and the fake window logs nothing (187)

#### Scenario: A terminal that draws its own window

- **WHEN** the fake terminal is scripted to answer attach `Shown`
- **THEN** the answer gives no command and the window as the terminal's, and the fake window logs nothing (187)

#### Scenario: A window that refuses

- **WHEN** the fake window is scripted to refuse with `no display`
- **THEN** the open is refused with those words (188)

### Requirement: The open's answer names no variable's value

The open's answer SHALL give the stream, its session and title each with
where it came from, the command's program, its arguments and the names of
the variables it needs, never their values, and what the window did:
opened, the line printed, or the terminal's own window (56, 61, 186;
core.14, core.30).

#### Scenario: A command that needs a variable

- **WHEN** the fake terminal answers attach with a command needing `XDG_CONFIG_HOME` set to `/scratch/home`, and the fake window answers printed
- **THEN** the answer's command names `XDG_CONFIG_HOME` and holds no `/scratch/home`, in its JSON form and the person's, and gives the printed line (186; core.30)
