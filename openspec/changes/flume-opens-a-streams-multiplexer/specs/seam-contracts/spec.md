## MODIFIED Requirements

### Requirement: One interface per seam

There SHALL be exactly one interface for each of the eight seams of
Part B: the place (78 to 83), the kinds (84, 85), the change (86, 87,
127), the agent (88 to 93, 115, 116, 121, 134, 135), the terminal (94
to 98, 122, 136 to 139, 187), the merge queue (99, 101, 117, 125, 126,
133), the store (102 to 104, 128) and the window (188, 189). Each SHALL
offer the operations its section names and no operation of the core's,
and no interface SHALL name a tool (core.1, core.2).

#### Scenario: Each seam's operations

- **WHEN** each seam's scenarios run against its fake
- **THEN** each operation its section of Part B names is exercised, and each scenario cites the clause of its operation

#### Scenario: No tool named outside a profile

- **WHEN** the seams', core's, fakes', suite's and commands' sources are searched for `wt`, `worktrunk`, `openspec`, `claude`, `herdr`, `sluice`, `ghostty` and `osascript` as words
- **THEN** none is found outside a `profiles` directory

## ADDED Requirements

### Requirement: The terminal seam's attach by view

The terminal seam SHALL offer attach: given a session by its name or
the terminal's own, and a view, for a window the command that shows it,
its program by its full path, its arguments and the variables it needs
with their values, the session started first where it does not run; for
the terminal's own app, the app told to show it; or a refusal where the
terminal holds no such part or offers no such view (187).

#### Scenario: Attach over the fake

- **WHEN** the fake terminal is asked to attach `work` for a window, then its own session for a window, then `work` for its app, then is scripted to refuse its app
- **THEN** it answers a command for `work`, one for its own session, shown, then the refusal, and logs each asking with its view (187)

### Requirement: The window seam's show

The window seam SHALL offer show: given a command and a title, a window
running the command under that title, opened; the line a person runs,
printed, where no window program is named; or a refusal in the window
program's words; within a bound the profile states, opening at most one
window at each asking (188, 189).

#### Scenario: Show over the fake

- **WHEN** the fake window is asked to show a command under `Work Agents` twice, then is scripted to answer printed and then refused
- **THEN** it answers opened twice, logging two shows with the command and the title, then the printed line, then the refusal (188, 189)

### Requirement: The resolver gives a terminal and a window by name

The resolver SHALL give a terminal profile and a window profile by
name, none for a name it carries none for, and list the names it
carries, the one the host's multiplexer names being the terminal the
core is built with (105, 190).

#### Scenario: Named terminals and windows

- **WHEN** the fake resolver is given a terminal `other` and a window `other-window`, and asked for `fake`, `other`, `rex`, `fake-window` and `kitty`
- **THEN** it gives the core's own terminal, `other`, none, the fake window and none, and lists `fake` and `other`, and `fake-window` and `other-window` (105, 190)

### Requirement: The types of a view, a window and a stream's terminal

The seams SHALL pass one type each for a view's command, attach's
answer, show's answer, a view, where a stream's multiplexer, view and
terminal app came from, and a start's terminal; each SHALL serialize to
JSON and back unchanged, a stream written before reading with no
multiplexer, view, terminal app or title, and a start written before
with no terminal (143, 187, 188, 190).

#### Scenario: A view, a window and a stream round-trip

- **WHEN** a command needing `XDG_CONFIG_HOME`, each of attach's and show's answers, each view, a stream's display from each of `stream`, `host` and `neither`, a stream with all four keys, and a start with its terminal are written to JSON and read back, and a stream and a start written with none of those keys are read
- **THEN** each equals the value written, the stream written before reads with none of its four keys, and the start with no terminal (143, 187, 188, 190)
