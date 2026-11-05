## MODIFIED Requirements

### Requirement: A launch's recipients

A launch naming no recipient SHALL take `$FLUME_RECIPIENT`, or else the
session that launched it as the terminal names it, or else the answerer
behind the place's stream, the role a source of the host declares to
stand behind it, and SHALL record the place with its priority, its
recipients, where each came from, and its holds' words, which nothing
after the launch changes (7, 8, 43, 70, 193).

#### Scenario: Recipients at the launch

- **WHEN** one launch names two recipients, another names none with `FLUME_RECIPIENT` set, and a third names none from a session the fake terminal names
- **THEN** status gives the first place both, the second the variable's, the third that session, and nothing after the launch changes them (7, 8)

#### Scenario: A launch from no session takes its stream's answerer

- **WHEN** a launch in the stream `work` names no recipient, `FLUME_RECIPIENT` is unset, the fake terminal names no session, and the scratch source `acme` declares `acme/operator` with a run, standing behind `work`
- **THEN** the place's one recipient is `acme/operator`, and status gives it as from the stream's answerer, declared by `acme` (7, 193, S92)

#### Scenario: Each stream its own answerer

- **WHEN** `acme` also declares `beta/operator` standing behind the stream `beta`, and a like launch names `beta`
- **THEN** the place's one recipient is `beta/operator` (143, 193)

## ADDED Requirements

### Requirement: A launch no one would answer is refused

A launch that finds no recipient, no `$FLUME_RECIPIENT`, no session and
no answerer behind its stream SHALL be refused, naming `--recipient`,
`$FLUME_RECIPIENT`, a session and a stream a role stands behind, and
SHALL make nothing. So SHALL such a launch into a stream two roles stand
behind, naming both (4, 7, 193; core.8).

#### Scenario: A launch from a shell into a stream no role stands behind

- **WHEN** a launch in the stream `default` names no recipient, `FLUME_RECIPIENT` is unset, the fake terminal names no session, and no source declares a role standing behind `default`
- **THEN** the launch is refused naming the four ways to give a recipient, and no branch, worktree or held state is made

#### Scenario: Two roles behind one stream

- **WHEN** the sources `acme` and `beta` each declare a role standing behind `work`, and a launch from no session names `work` and no recipient
- **THEN** the launch is refused naming both roles and their sources, and nothing is made (193)

### Requirement: A launch naming an address that wakes nothing

A launch whose recipients hold a role's address that no source of the
host declares, or that two declare, SHALL be refused, naming the
address and making nothing, where no answerer stands behind its stream,
and SHALL be taken where one does. An address a source declares
carried SHALL be taken (4, 132, 193; core.8).

#### Scenario: An address that wakes nothing, with no answerer

- **WHEN** a launch names `madswan/planner`, no source declares it, and no role stands behind the launch's stream
- **THEN** the launch is refused naming `madswan/planner`, and nothing is made

#### Scenario: An address that wakes nothing, behind an answerer

- **WHEN** the same launch is made in a stream behind which a source declares `madswan/operator`, with a run
- **THEN** the launch is taken, its recipient `madswan/planner`, and status says the address wakes nothing

#### Scenario: A carried address

- **WHEN** a launch names `wldn/planner`, which a source declares carried, in a stream no role stands behind
- **THEN** the launch is taken, its recipient `wldn/planner`
