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

`flume open [<stream>]` SHALL open the stream's window, `default`'s
where it names none, binding the window `[host] window` names for this
command alone, needing no repository, and printing the printed window's
line on a line of its own (61, 105, 186, C.8).

#### Scenario: open under the printed command

- **WHEN** `flume open work --json` runs on a host declaring `work` whose terminal is the printed command
- **THEN** the JSON is a refusal saying the printed command holds no session, and the exit is non-zero (187)

#### Scenario: open of an undeclared stream

- **WHEN** `flume open nightly` runs outside any repository on a host that declares no `nightly`
- **THEN** it is refused naming `nightly`, and the exit is non-zero (4, 186)

#### Scenario: A stream's window through the binary

- **WHEN** `flume open w` runs on a host whose terminal is herdr under a scratch session home, its window `ghostty` over a stand-in, `[streams.w]` titled `Work Herdr`, and then a fix is launched on `w` and `flume open w` runs again
- **THEN** the stand-in records two windows titled `Work Herdr`, each running the pinned herdr's `session attach w`, and the fix's agent opens in the session `w` (186, S90)

#### Scenario: The printed window through the binary

- **WHEN** `flume open w` runs on the same host naming no window
- **THEN** it prints the line `<herdr> session attach w` preceded by `XDG_CONFIG_HOME=<set>`, the exit is zero, and nothing is run (188, S90)
