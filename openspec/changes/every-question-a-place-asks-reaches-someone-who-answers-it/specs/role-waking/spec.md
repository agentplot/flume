## RENAMED Requirements

- FROM: `### Requirement: The host's configuration names each role's run`
- TO: `### Requirement: A source declares each role's run`
- FROM: `### Requirement: A role's run is a place named by the role`
- TO: `### Requirement: A role's run is a place named per address`
- FROM: `### Requirement: A role's run names its stream`
- TO: `### Requirement: A role's run runs in its address's stream`

## MODIFIED Requirements

### Requirement: A source declares each role's run

flume SHALL read the roles a host wakes from its sources alone, each
from the file a source's declaration names as `roles`, as it stands at
each launch, wake, start, stall and status, and SHALL write nothing in
it and choose nothing of a run of its own. A host's configuration that
gives a `[roles]` table SHALL be unreadable, naming it and that a
source declares a role (132, 191, 193, C.7).

#### Scenario: A role configured

- **WHEN** the scratch host declares the stream `east`, the scratch source `acme`'s roles file declares `east/planner` with a repository, the kind `turn` and priority 5, and `acme/turn@1` is locked
- **THEN** a wake of `east/planner` launches a place of `acme/turn@1` in that repository at priority 5, cut from the repository's default branch, with no brief

#### Scenario: A base and a brief

- **WHEN** the entry gives `east/planner` the base `next` and a brief
- **THEN** the woken place is cut from `next` and its stages that ask for the brief get it whole

#### Scenario: Read as it stands

- **WHEN** `acme`'s program rewrites its roles file, renamed over the first, to declare `east/ops` beside `east/planner`
- **THEN** the next wake of `east/ops` launches its run, with no command of flume's run between, and flume writes nothing in the file

#### Scenario: An address with no kind

- **WHEN** an end is pending for `east/ops` and no source declares it
- **THEN** nothing is launched, and status names the end pending for `east/ops` and that the address wakes nothing

#### Scenario: The roles in the host's configuration

- **WHEN** the scratch host's `config.toml` gives `[roles."east/planner"]`
- **THEN** every command of flume's says the host's configuration is unreadable, naming `[roles]` and that a source declares a role

### Requirement: A role's run is a place named per address

A role's run SHALL be a place of its declared kind and repository,
named by the entry's `name`, else `<stream>-<role>`, its agent's
session named by the address, so at most one run of an address is live
at once and two streams' runs of one role are two places. A run an
earlier flume named by the role alone SHALL be found by its role (3,
132, 193).

#### Scenario: The run's session

- **WHEN** a wake launches `east/planner`'s run, whose entry names no name, and its first stage starts
- **THEN** the place is `turn--east-planner`, the fake terminal opens the agent under the session name `east/planner`, and status names the place's role address

#### Scenario: Two streams' planners from one declaration

- **WHEN** ends come at once for `east/planner` and `west/planner`, which `acme`'s one roles file declares in one repository and the kind `turn`, named `east` and `west`
- **THEN** two places are launched, `turn--east` and `turn--west`, and each run's `flume inbox` gives its own address's ends (S98)

#### Scenario: A run an earlier flume named

- **WHEN** a run of `east/planner` stands as `turn--planner`, made by an earlier flume, and an end comes for the address
- **THEN** no run is launched, and the end is delivered into that run's pane

### Requirement: A role's run runs in its address's stream

A role's run SHALL be launched in the stream its address names, so the
run is placed and refused as any launch naming that stream is, and its
label SHALL be its kind's as 180 gives it. An entry SHALL name no
stream of its own (132, 143, 180, 193).

#### Scenario: A role in its stream

- **WHEN** `acme`'s file declares `east/planner`, whose kind declares the label `planning`, and an end pending for its address wakes it
- **THEN** the woken place runs in `east`, from its address, with the label `planning` from its kind (143, 180, S98)

#### Scenario: An entry that names a stream

- **WHEN** `acme`'s file gives `east/planner` `stream = "west"`
- **THEN** `east/planner` wakes nothing, and status names the entry and that its address names its stream

## ADDED Requirements

### Requirement: An address is a role of a stream

A role's address SHALL be `<stream>/<role>`, each part a word of
letters, digits, `.`, `_` and `-`: the role a source declares for a
stream the host declares. An entry whose address names a stream the
host does not declare SHALL declare nothing for it, unless it is
carried, and status SHALL name the address and the stream. A
recipient's name of any other form SHALL be a session's (4, 7, 132,
143, 193).

#### Scenario: A stream the host does not declare

- **WHEN** `acme`'s file declares a run for `south/planner`, and the host declares no stream `south`
- **THEN** a wake of `south/planner` is refused naming `south` as not declared, no place is made, and status names the address and the stream (4, 143)

#### Scenario: A session's name

- **WHEN** a launch names the recipient `planner`
- **THEN** the place's recipient is the session `planner`, and nothing is woken for it

#### Scenario: The wake's words

- **WHEN** `flume wake planner` runs
- **THEN** it is refused naming the form `<stream>/<role>`, and nothing is made

### Requirement: A role's entry gives its run

An entry SHALL give its address's run: a kind of its own source's, its
version the locks'; its repository by its location on the host; its
priority; its base, else the repository's default branch; and an
optional brief, points, name, intent, `agents`, `answers` and
`answerer`. The run's settings SHALL be taken as a launch's, in their
layers (7, 32, 70, 147, 155, 193, C.7).

#### Scenario: The run's intent and settings

- **WHEN** the entry gives `east/planner` the intent `east-planner` and, under `agents`, `model = "opus"`
- **THEN** the woken place serves the intent `east-planner`, and its agent starts with the model `opus`, which status gives as from the launch (32, 147)

### Requirement: An address carried to another host

An entry MAY give `carried = true`, with an optional `answers` and
nothing else, for an address whose run is on another host, whose stream
the host need not declare. flume SHALL launch nothing for it and SHALL
keep its ends pending (132, 193).

#### Scenario: A carried address

- **WHEN** `acme`'s file gives `north/planner` `carried = true`, the host declares no stream `north`, and an end stands pending for it
- **THEN** nothing is launched, the end stays pending, and status names the address carried

### Requirement: A fault in a source's roles stays its own

A roles file that cannot be read SHALL declare no role. An entry that
names a kind not its source's, an `env`, a pool, an account or a
`stream`, whose address names a stream the host does not declare and
that is not carried, or that gives `carried` beside a run's keys or
`answerer`, SHALL declare nothing for its address. Status SHALL name
each, and every other role and launch SHALL go on (4, 56, 191, 193;
core.30).

#### Scenario: A roles file that is not TOML

- **WHEN** `acme`'s roles file holds a line that is no TOML, and the source `beta` declares `west/ops`
- **THEN** no address of `acme`'s wakes, status names `acme`'s file and why, and a wake of `west/ops` launches its run

#### Scenario: Another source's kind

- **WHEN** `acme`'s file gives `east/ops` `kind = "beta/turn"`
- **THEN** `east/ops` wakes nothing, and status names the entry and that its kind is not `acme`'s

### Requirement: What two entries claim is neither's

An address two entries declare, of one source or two, and two addresses
whose runs would be one place, of one repository, kind and name, SHALL
be declared by neither: a wake of either SHALL be refused naming both,
and status SHALL name both (3, 4, 132, 193).

#### Scenario: One address, two sources

- **WHEN** `acme` and `beta` each declare `east/planner`
- **THEN** a wake of `east/planner` is refused naming both sources, and no place is made

#### Scenario: Two addresses, one place

- **WHEN** `acme`'s file declares `east/planner` and `west/planner` in one repository and one kind, both named `planner`
- **THEN** a wake of either is refused naming both addresses, and no place is made

### Requirement: A role's run is over at its delivery

Once a role's run's last stage delivers, the settle that reads it SHALL
declare the run done, with no caller's word, and tidy it, so a later end
pending for the address wakes another run (114, 132).

#### Scenario: A turn delivers

- **WHEN** the run of `east/planner` delivers its one stage's report and its agent is quiet
- **THEN** the settle declares it done and tidies the place, and status gives no run of `east/planner` standing (S98)

#### Scenario: The next end

- **WHEN** an end for `east/planner` comes after that run was tidied
- **THEN** the settle that reads it launches a new run (S98)

### Requirement: A role's run's stall reaches the answerer behind it

A role's run that waits on its caller, lost, never begun or stopped
short, SHALL be stalled as any place is, and its stall SHALL be
delivered to the answerer behind its stream, and never to the run's own
address (130, 132, 193).

#### Scenario: A planner's run is lost

- **WHEN** the run of `east/planner`, in the stream `east` behind which `east/operator` stands, is reported lost and nobody retries it for five minutes
- **THEN** its stall is delivered to `east/operator`, whose run is woken

#### Scenario: The door's own run

- **WHEN** the run of `east/operator`, the answerer of its own stream, is reported lost and stalls
- **THEN** its stall is delivered to no address, and status names it under the host's addresses
