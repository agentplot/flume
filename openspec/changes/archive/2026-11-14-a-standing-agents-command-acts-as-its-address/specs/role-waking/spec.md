## ADDED Requirements

### Requirement: A standing agent's command acts as the address it holds

A command that names no recipient, with `$FLUME_RECIPIENT` unset and
run in no role's run, SHALL act as the address a source gives the
standing agent it runs in: the one agent of the entry's name that the
address's delivery reaches, at the pane its stream's terminal names as
the command's session. `flume confirm`, `flume inbox` and `flume wait`
SHALL act as that address, and a retry's or a tell's answer SHALL be
recorded as given by it (7, 43, 132, 193; core.28).

#### Scenario: A confirm from the standing agent's pane

- **WHEN** `acme` gives `east/planner` the standing agent `east-planner` in the session `east`, the fake terminal holds it waiting there, a fix for `east/planner` delivers and its end is handed to that agent by notice, and a confirm of the end's delivery key naming no recipient runs in that agent's pane with `$FLUME_RECIPIENT` unset
- **THEN** it confirms as `east/planner`, and the end is delivered and pending for no one (43, 132; weir 102)

#### Scenario: An inbox and a wait from the standing agent's pane

- **WHEN** an end stands pending for `east/planner`, and an inbox and then a wait on the fix's code stage, each naming no recipient, run in `east-planner`'s pane
- **THEN** the inbox gives the end, handed to `east/planner` by the inbox, and the wait answers it as `east/planner`'s (43, 132)

#### Scenario: An answer from the standing agent's pane

- **WHEN** a construct of a place whose recipient is `east/planner` ends short needing an answer, and a retry naming its key and words runs in `east-planner`'s pane
- **THEN** status gives the end answered by `east/planner`, with when (42, 49, 132)

#### Scenario: A standing agent under herdr confirms through the binary

- **WHEN** under the pinned herdr and the stand-in, a scratch source's roles file gives `w/planner` the agent `w-planner` in the session `w`, a stand-in runs in `w` under that herdr name, a fix for `w/planner` delivers, and the stand-in runs `flume confirm <key>` in its pane with the delivery key its notice names and no recipient
- **THEN** the confirm acts as `w/planner`, and status gives the fix's code end delivered to `w/planner`, pending for no one (43, 132, 193, C.5)

### Requirement: Elsewhere a command acts as before

A command that names its recipient, or runs with `$FLUME_RECIPIENT` set
or in a role's run, SHALL act as that, and one run in a pane no single
standing agent of a declared address is found at SHALL act as the
terminal's session, as before; the terminal SHALL be asked for agents
only where a source declares a standing agent (7, 132; core.15,
core.28).

#### Scenario: What the command names comes first

- **WHEN** in `east-planner`'s pane a confirm runs with `$FLUME_RECIPIENT` set to `ops`, and an inbox runs with `$FLUME_PLACE` naming the place of the run of `east/ops`
- **THEN** the confirm acts as `ops` and the inbox as `east/ops` (7, 132; core.28)

#### Scenario: A pane no standing agent holds

- **WHEN** an inbox naming no recipient runs in a pane of `east` that the fake terminal lists under the name `east-helper`, which no source gives an address
- **THEN** it collects as that pane's session, and nothing pending for `east/planner` is handed over (7)

#### Scenario: Two agents of the standing agent's name

- **WHEN** the fake terminal holds two agents named `east-planner` in `east`, and an inbox naming no recipient runs in the pane of one
- **THEN** it collects as that pane's session, as no delivery reaches either agent, and nothing pending for `east/planner` is handed over (132)

#### Scenario: Nothing asked where no standing agent is declared

- **WHEN** no source gives any address a standing agent, and an inbox naming no recipient runs in a pane the fake terminal names
- **THEN** it collects as that pane's session, and the fake terminal is asked for no part's agents (core.15)

#### Scenario: A part that cannot be read

- **WHEN** the fake terminal refuses `east`'s agents, and an inbox naming no recipient runs in `east-planner`'s pane
- **THEN** it collects as that pane's session, and status names `east/planner` with why its part was not read (132)

### Requirement: An agent standing for more than one address names which

A command that names no recipient, with `$FLUME_RECIPIENT` unset and
run in no role's run, in the pane of an agent its sources give more than
one address SHALL act as none of them: `flume confirm`, `flume inbox`
and `flume wait` SHALL be refused, naming the agent and each address and
asking for `--recipient`, confirming, handing over and waiting on
nothing. Named by `--recipient`, each SHALL act as the address named
(132; core.28).

#### Scenario: One agent for two addresses

- **WHEN** `acme` gives `east/planner` and `east/ops` each the standing agent `east-planner` in `east`, an end stands pending for each, and an inbox naming no recipient runs in that agent's pane
- **THEN** it is refused naming `east-planner`, `east/ops`, `east/planner` and `--recipient`, and both ends stay handed to none (132)

#### Scenario: The address named

- **WHEN** the same pane runs `flume inbox --recipient east/ops`
- **THEN** it collects `east/ops`'s end alone, handed to `east/ops` by the inbox (43, 132)

#### Scenario: A retry from the pane of two addresses

- **WHEN** a construct ended short needing an answer, its place's recipient `east/planner`, and a retry naming its key and words runs in the pane of `east-planner`, which stands for `east/planner` and `east/ops`
- **THEN** the retry is taken, and status gives the end answered with no one named as its giver (42, 49)
