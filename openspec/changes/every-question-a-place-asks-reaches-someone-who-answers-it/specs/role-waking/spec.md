## MODIFIED Requirements

### Requirement: A role's run is a place named by the role

A role's run SHALL be a place of its configured kind and repository,
named by its whole address, `<partition>-<role>`, its agent's session
named by the address, so at most one run of an address is live at once
and two partitions' runs of one role are two places; a run standing
under its role alone SHALL be found by its role (3, 132).

#### Scenario: The run's session

- **WHEN** a wake launches `madswan/planner`'s run and its first stage starts
- **THEN** the place is `planner--madswan-planner`, the fake terminal opens the agent under the session name `madswan/planner`, and status names the place's role address

#### Scenario: Two partitions' planners

- **WHEN** ends come at once for `madswan/planner` and `swancloud/planner`, whose runs the host's configuration names in one repository and the kind `planner`
- **THEN** two places are launched, `planner--madswan-planner` and `planner--swancloud-planner`, and each run's `flume inbox` gives its own address's ends (S94)

#### Scenario: A run an earlier flume named

- **WHEN** a run of `madswan/planner` stands as `planner--planner`, made by an earlier flume, and an end comes for the address
- **THEN** no run is launched, and the end is delivered into that run's pane

### Requirement: A carrier may wake an address

`flume wake <address>` SHALL launch the address's run where none is
live or waiting for room and no live session serves it, and SHALL do
nothing, answering so and naming the run or the session, where one is
(132).

#### Scenario: A wake while the run waits for room

- **WHEN** crew's card carrier wakes `madswan/planner` while its run waits for room
- **THEN** the wake answers that the run waits, and no second place is made (S38)

#### Scenario: A wake of a served address

- **WHEN** a carrier wakes `madswan/planner` while a live session serves it
- **THEN** the wake answers naming that session, and no place is made

## ADDED Requirements

### Requirement: A session serves an address

`flume serve <address>`, run in a session the terminal names, SHALL
record in the host's state that the session serves the address, with
since when, and SHALL answer what `flume inbox --recipient <address>`
answers. While the terminal reads that session live and no run of the
address is live, the address's ends SHALL be delivered into it, and no
run SHALL be woken for one it can be delivered (43, 132).

#### Scenario: A standing planner serves its address

- **WHEN** a session runs `flume serve madswan/planner`, and a place naming the address delivers construct
- **THEN** the notice is delivered into that session, and no run is launched (S93)

#### Scenario: The serving session ends

- **WHEN** the fake terminal reads that session stopped, and the place's next end is read on a host whose `[roles]` names nothing for the address
- **THEN** the end stands pending, nothing is woken, and status gives the address with no run and no session serving it (S93)

#### Scenario: A later session serves

- **WHEN** a second session runs `flume serve madswan/planner` while the first is live
- **THEN** the second serves from then, and status gives it and since when

#### Scenario: Serving outside a session

- **WHEN** `flume serve madswan/planner` runs where the terminal names no session
- **THEN** it is refused naming that no session is named, and records nothing
