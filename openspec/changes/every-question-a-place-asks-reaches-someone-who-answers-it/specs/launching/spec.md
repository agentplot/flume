## MODIFIED Requirements

### Requirement: A launch's recipients

A launch naming no recipient SHALL take `$FLUME_RECIPIENT`, or else the
session that launched it as the terminal names it, or else the answerer
the host's configuration names for the place's stream or for the host,
and SHALL record the place with its priority, its recipients, where
each came from, and its holds' words, which nothing after the launch
changes (7, 8, 43, 70, 191).

#### Scenario: Recipients at the launch

- **WHEN** one launch names two recipients, another names none with `FLUME_RECIPIENT` set, and a third names none from a session the fake terminal names
- **THEN** status gives the first place both, the second the variable's, the third that session, and nothing after the launch changes them (7, 8)

#### Scenario: A launch from no session takes the answerer

- **WHEN** a launch names no recipient, `FLUME_RECIPIENT` is unset, the fake terminal names no session, and the host's configuration gives `[host] answerer = "acme/operator"` with a run for it
- **THEN** the place's one recipient is `acme/operator`, and status gives it as from the host's answerer (7, 191, S92)

#### Scenario: A stream's answerer comes first

- **WHEN** that launch names the stream `work`, whose configuration gives `answerer = "work/operator"` with a run for it
- **THEN** the place's one recipient is `work/operator`, from the stream (143, 191)

## ADDED Requirements

### Requirement: A launch no one would answer is refused

A launch that finds no recipient, no `$FLUME_RECIPIENT`, no session and
no answerer SHALL be refused, naming `--recipient`, `$FLUME_RECIPIENT`
and `[host] answerer`, and SHALL make nothing. So SHALL a launch that
would take an answerer the host's configuration names no run for and
does not carry, naming the answerer and its key (4, 7, 191; core.8).

#### Scenario: A launch from a shell on a host with no answerer

- **WHEN** a launch names no recipient, `FLUME_RECIPIENT` is unset, the fake terminal names no session, and the host's configuration names no answerer
- **THEN** the launch is refused naming the three ways to name one, and no branch, worktree or held state is made

#### Scenario: An answerer that wakes nothing

- **WHEN** a launch would take the host's answerer `acme/operator`, which `[roles]` names nothing for and does not carry
- **THEN** the launch is refused naming `acme/operator` and `[host] answerer`, and nothing is made (191)

### Requirement: A launch naming an address that wakes nothing

A launch whose recipients hold a role's address the host's
configuration names no run for and does not carry SHALL be refused,
naming the address and making nothing, where no answerer stands behind
its stream, and SHALL be taken where one does (4, 132, 191; core.8).

#### Scenario: An address that wakes nothing, with no answerer

- **WHEN** a launch names `madswan/planner`, the host's `[roles]` names nothing for it, and no answerer stands behind the launch's stream
- **THEN** the launch is refused naming `madswan/planner`, and nothing is made

#### Scenario: An address that wakes nothing, behind an answerer

- **WHEN** the same launch is made on a host whose configuration names `madswan/operator`, with a run, the answerer of the launch's stream
- **THEN** the launch is taken, its recipient `madswan/planner`, and status says the address wakes nothing

#### Scenario: A carried address

- **WHEN** a launch names `wldn/planner`, which the host's `[roles]` carries, on a host that names no answerer
- **THEN** the launch is taken, its recipient `wldn/planner`
