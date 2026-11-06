## ADDED Requirements

### Requirement: A person opens a session a stream names for a kind or a stage

`flume open [<stream>] --session <name>` SHALL show the session the
stream names for a kind or a stage, or its own session where the name
is its own, as it shows the stream's own (186, 187); a window's title
SHALL be the session's name for a session named for a kind or a stage,
and the answer SHALL give the session with where it came from and the
kinds and stages it is named for (186, 199).

#### Scenario: The design session

- **WHEN** the fake host's `work` names `work-design` for research and declares the title `Work Agents`, and `open_stream` is asked for `work` with the session `work-design`
- **THEN** the fake terminal's log names `attach work-design for terminal`, the fake window's log names its command under `work-design`, and the answer gives `work-design` from `stream-kind`, named for research (186, 199, S107)

#### Scenario: The stream's own session by its name

- **WHEN** `open_stream` is asked for `work` with the session `work`, `work` naming no session of its own
- **THEN** the terminal is asked to attach `work`, under the title `Work Agents`, as for `work` with no session named (186)

### Requirement: A session the stream names for nothing is refused

A session the stream names for no kind or stage, and that is not its
own, SHALL be refused naming the stream and its sessions, before the
terminal or the window is asked anything, so a mistyped name starts no
session (4, 186, 199).

#### Scenario: A session the stream names for nothing

- **WHEN** `open_stream` is asked for `work` with the session `review`, which `work` names for no kind or stage
- **THEN** it is refused naming `work`, `work` and `work-design`, and the fake terminal and the fake window log nothing (186, 199, S107)
