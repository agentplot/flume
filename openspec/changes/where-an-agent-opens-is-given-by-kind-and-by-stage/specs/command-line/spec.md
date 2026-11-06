## MODIFIED Requirements

### Requirement: open names a stream or none

`flume open [<name>]` SHALL show the view of the stream of that name, or
of the session of that name a stream names for a kind, `default`'s
where it names none, needing no repository, its multiplexer and terminal
app bound through the resolver for that stream alone, and printing the
printed window's line on a line of its own; its help SHALL say the name
is a stream's or a session's a stream names for a kind (61, 105, 186,
190, 199, C.8).

#### Scenario: open under the printed command

- **WHEN** `flume open work --json` runs on a host declaring `work` whose multiplexer is the printed command
- **THEN** the JSON is a refusal saying the printed command holds no session, and the exit is non-zero (187)

#### Scenario: open of an undeclared stream

- **WHEN** `flume open nightly` runs outside any repository on a host that declares no `nightly`
- **THEN** it is refused naming `nightly`, and the exit is non-zero (4, 186)

#### Scenario: A stream's view through the binary

- **WHEN** `flume open w` runs on a host whose multiplexer is herdr under a scratch session home, its terminal app `ghostty` over a stand-in, `[streams.w]` titled `Work Herdr`; then a fix is launched on `w` and `flume open w` runs again; then `flume open o` runs for `[streams.o]` naming the terminal app `terminal` and the title `Office Herdr`
- **THEN** the stand-in records two windows titled `Work Herdr`, each running the pinned herdr's `session attach w`, the fix's agent opens in the session `w`, and on macOS the stand-in `osascript` records `exec <herdr> session attach o` under `Office Herdr`, and elsewhere `o`'s open is refused saying Terminal.app runs only on macOS (186 to 189, S90)

#### Scenario: The printed window through the binary

- **WHEN** `flume open w` runs on the same host naming no terminal app
- **THEN** it prints the line `<herdr> session attach w` preceded by `XDG_CONFIG_HOME=<set>`, the exit is zero, and nothing is run (188, S90)

#### Scenario: Two streams under two multiplexers through the binary

- **WHEN** the host's multiplexer is herdr, `[streams.t]` names the printed command, and a fix is launched on `w` and one on `t`, and then `flume status --json` runs
- **THEN** the `w` fix's agent opens in herdr's session `w`, the `t` fix's command is printed, and status gives each agent's terminal and each stream's multiplexer with where it came from (190, S91)

#### Scenario: A design session through the binary

- **WHEN** the same host's `[streams.w.kinds.research]` gives `session = "w-design"`, a research place and a fix are launched on `w`, `flume open w-design --json` runs, each place's agent is read by `flume status --json` and the places are tidied
- **THEN** the pinned herdr lists the sessions `w-design` and `w` running, the research agent's pane on `w-design`'s server in its place's tab and the fix's on `w`'s; the open's JSON gives the stream `w`, the session `w-design` from `stream-kind` named for research and the title `w-design`, and the stand-in records a window titled `w-design` running `session attach w-design`; status gives each agent's session with its origin; and after the tidies neither server holds a tab or a workspace of flume's (107, 137, 138, 186, 190, 199, S106, S107)

#### Scenario: A name that is no stream and no session

- **WHEN** `flume open nightly` runs over a scratch host whose `work` names `work-design` for research
- **THEN** it exits refused naming `nightly`, `work` and `work-design`, and starts no session (186, 199)
