## RENAMED Requirements

- FROM: `### Requirement: A role's run is a place named by the role`
- TO: `### Requirement: A role's run is a place named per address`

## MODIFIED Requirements

### Requirement: The host's configuration names each role's run

flume SHALL read the roles a host wakes from the `[roles]` table of the
host's own configuration, each address `<partition>/<role>` with the
repository, kind, priority, base (the repository's default branch where
it names none), and the optional brief, name, intent, settings of its
agents under `agents`, and `answers`, the words of what the role
answers, of its run; or `carried = true` alone, for an address whose
run is on another host. It SHALL take the run's settings as a launch's,
in their layers, and SHALL choose none of them of its own (7, 32, 70,
132, 147, C.7).

#### Scenario: A role configured

- **WHEN** the host's configuration names `madswan/planner` with a repository, the kind `planner` and priority 5
- **THEN** a wake of `madswan/planner` launches a place of kind `planner` in that repository at priority 5, cut from the repository's default branch, with no brief

#### Scenario: A base and a brief

- **WHEN** the table gives `madswan/planner` the base `next` and a brief
- **THEN** the woken place is cut from `next` and its stages that ask for the brief get it whole

#### Scenario: The run's intent and settings

- **WHEN** the table gives `madswan/planner` the intent `madswan-planner` and, under `agents`, `model = "opus"`
- **THEN** the woken place serves the intent `madswan-planner`, and its agent starts with the model `opus`, which status gives as from the launch (32, 147)

#### Scenario: An address with no kind

- **WHEN** an end is pending for `madswan/ops` and the table names nothing for it
- **THEN** nothing is launched, and status names the end pending for `madswan/ops` and that the address wakes nothing

#### Scenario: A carried address

- **WHEN** the table gives `wldn/planner` `carried = true` and an end stands pending for it
- **THEN** nothing is launched, the end stays pending, and status names the address carried

#### Scenario: A carried address with a kind

- **WHEN** the table gives `wldn/planner` both `carried = true` and a kind
- **THEN** every command of flume's says the host's configuration is unreadable, naming `wldn/planner`

### Requirement: A role's run is a place named per address

A role's run SHALL be a place of its configured kind and repository,
named by the entry's `name`, else `<partition>-<role>`, its agent's
session named by the address, so at most one run of an address is live
at once and two partitions' runs of one role are two places. A host's
configuration in which two addresses would make one place, of one
repository, kind and name, SHALL be unreadable, naming both. A run an
earlier flume named by the role alone SHALL be found by its role (3,
132, C.7).

#### Scenario: The run's session

- **WHEN** a wake launches `madswan/planner`'s run, whose entry names no name, and its first stage starts
- **THEN** the place is `planner--madswan-planner`, the fake terminal opens the agent under the session name `madswan/planner`, and status names the place's role address

#### Scenario: Two partitions' planners

- **WHEN** ends come at once for `madswan/planner` and `swancloud/planner`, whose runs the host's configuration names in one repository and the kind `crew-planner`, named `madswan` and `swancloud`
- **THEN** two places are launched, `crew-planner--madswan` and `crew-planner--swancloud`, and each run's `flume inbox` gives its own address's ends (S95)

#### Scenario: Two addresses, one place

- **WHEN** the configuration names `madswan/planner` and `swancloud/planner` in one repository and one kind, both named `planner`
- **THEN** every command of flume's says the host's configuration is unreadable, naming both addresses

#### Scenario: A run an earlier flume named

- **WHEN** a run of `madswan/planner` stands as `planner--planner`, made by an earlier flume, and an end comes for the address
- **THEN** no run is launched, and the end is delivered into that run's pane

## ADDED Requirements

### Requirement: A role's run is over at its delivery

Once a role's run's last stage delivers, the settle that reads it SHALL
declare the run done, with no caller's word, and tidy it, so a later end
pending for the address wakes another run (114, 132).

#### Scenario: A turn delivers

- **WHEN** the run of `madswan/planner` delivers its one stage's report and its agent is quiet
- **THEN** the settle declares it done and tidies the place, and status gives no run of `madswan/planner` standing (S95)

#### Scenario: The next end

- **WHEN** an end for `madswan/planner` comes after that run was tidied
- **THEN** the settle that reads it launches a new run (S95)

### Requirement: A role's run's stall reaches the answerer behind it

A role's run that waits on its caller, lost, never begun or stopped
short, SHALL be stalled as any place is, and its stall SHALL be
delivered to the answerer behind its stream, else the host's, and never
to the run's own address (130, 132, 191).

#### Scenario: A planner's run is lost

- **WHEN** the run of `madswan/planner`, in the stream `planning` whose answerer is `madswan/operator`, is reported lost and nobody retries it for five minutes
- **THEN** its stall is delivered to `madswan/operator`, whose run is woken

#### Scenario: The door's own run

- **WHEN** the run of `madswan/operator`, the answerer of its stream and of the host, is reported lost and stalls
- **THEN** its stall is delivered to no address, and status names it under the host's addresses
