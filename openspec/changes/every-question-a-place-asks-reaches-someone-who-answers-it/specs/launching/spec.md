## MODIFIED Requirements

### Requirement: A launch's recipients

A launch naming no recipient SHALL take `$FLUME_RECIPIENT`, or else the
session that launched it as the terminal names it, or else the answerer
the host's configuration names for the place's stream or for the host,
and SHALL record the place with its priority, its recipients and its
holds' words, which nothing after the launch changes (7, 8, 43, 70,
190).

#### Scenario: Recipients at the launch

- **WHEN** one launch names two recipients, another names none with `FLUME_RECIPIENT` set, and a third names none from a session the fake terminal names
- **THEN** status gives the first place both, the second the variable's, the third that session, and nothing after the launch changes them (7, 8)

#### Scenario: A launch from no session takes the host's answerer

- **WHEN** a launch names no recipient, `FLUME_RECIPIENT` is unset, the fake terminal names no session, and the host's configuration gives `[host] answerer = "ops/answerer"` with a run for it
- **THEN** the place's one recipient is `ops/answerer`, and status gives it as from the host's answerer (7, 190, S91)

#### Scenario: A stream's answerer comes first

- **WHEN** that launch names the stream `work`, whose configuration gives `answerer = "work/ops"` with a run for it
- **THEN** the place's one recipient is `work/ops`, from the stream (143, 190)

## ADDED Requirements

### Requirement: A launch no one would answer is refused

A launch that finds no recipient, no `$FLUME_RECIPIENT`, no session and
no answerer SHALL be refused, naming `--recipient`, `$FLUME_RECIPIENT`
and `[host] answerer`, and SHALL make nothing. So SHALL a launch whose
recipients hold a role's address that the host's configuration names no
run for and no session serves, naming the address (4, 7, 132, 190;
core.8).

#### Scenario: A launch from a shell on a host with no answerer

- **WHEN** a launch names no recipient, `FLUME_RECIPIENT` is unset, the fake terminal names no session, and the host's configuration names no answerer
- **THEN** the launch is refused naming the three ways to name one, and no branch, worktree or held state is made

#### Scenario: An address that wakes nothing

- **WHEN** a launch names `madswan/planner`, the host's `[roles]` names nothing for it, and no session serves it
- **THEN** the launch is refused naming `madswan/planner`, and nothing is made (S93)

#### Scenario: An address a session serves

- **WHEN** the same launch is made after a session ran `flume serve madswan/planner` and the fake terminal reads that session live
- **THEN** the launch is taken, its recipient `madswan/planner` (132, S93)

#### Scenario: An answerer that wakes nothing

- **WHEN** a launch would take the host's answerer `ops/answerer`, which `[roles]` names nothing for and no session serves
- **THEN** the launch is refused naming `ops/answerer` and `[host] answerer`, and nothing is made (190)
