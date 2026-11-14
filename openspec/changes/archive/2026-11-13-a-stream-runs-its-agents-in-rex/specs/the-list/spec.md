## MODIFIED Requirements

### Requirement: The list gives a line for each stream the host declares

The core SHALL give a line for each stream the host declares, `default`
among them, in status's order: `<stream> · session <name> · <runs>`,
the session resolved as an open resolves it, the terminal's own named
as its runs names it, and the runs part `runs`, `not running`, `not
read`, or `no profile for <multiplexer>` where the stream's multiplexer
names no profile flume carries, which asks no terminal; and
`<stream> · no session` where the terminal holds no part (107, 143,
186, 190, 200, 201).

#### Scenario: Three streams over the fake terminal

- **WHEN** the fake host declares `work`, naming no session, and `office`, naming the session `desk`, and the fake terminal is scripted to read `work` running and `desk` not, and its own unread
- **THEN** the list gives `work · session work · runs`, `office · session desk · not running` and `default · its terminal's own session · not read`, and the fake terminal logs one runs for each (107, 186, 201)

#### Scenario: A multiplexer flume carries no profile for

- **WHEN** the fake host's `trial` names the multiplexer `cmux`, which the resolver carries no terminal for
- **THEN** `trial`'s line is `trial · session trial · no profile for cmux`, the other streams' lines stand, the list is not refused, and no terminal is asked of `trial` (190, 200)
