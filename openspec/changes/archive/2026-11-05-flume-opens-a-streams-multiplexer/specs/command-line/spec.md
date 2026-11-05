## MODIFIED Requirements

### Requirement: Every command answers from the one core

The binary SHALL offer `launch`, `status`, `prompt`, `tell`, `stop`,
`retry`, `finish`, `drop`, `done`, `tidy`, `settle`, `needs`,
`priority` and `open`, each building the core over the host's profiles
and answering the core's answer, printed for a person or as its JSON
with `--json`, exiting non-zero on a refusal; no command SHALL compute a
stage's end, a status, a submission or a tidy of its own (56 to 61,
186, section 9, core.14).

#### Scenario: The same answer through the command line and the core

- **WHEN** `flume status <place> --json` runs over a scratch repository and the core is asked the same in a test over the same state
- **THEN** the two answers are equal (S14, 60)

#### Scenario: A refusal

- **WHEN** `flume done` names a fix place
- **THEN** the JSON is a refusal naming the place and the exit is non-zero (S26, 61)

## ADDED Requirements

### Requirement: open names a stream or none

`flume open [<stream>]` SHALL show the stream's view, `default`'s where
it names none, needing no repository, its multiplexer and terminal app
bound through the resolver for that stream alone, and printing the
printed window's line on a line of its own (61, 105, 186, 190, C.8).

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
