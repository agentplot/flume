# role-waking Specification

## Purpose
A role with no session standing still hears its ends: a recipient named
by a role's address, a run of the role launched by the settle that finds
an end pending for it or by a carrier's wake, and never two runs of one
address at once (3, 43, 132, C.7, S38; record 0013; core.28).

## Requirements

### Requirement: The host's configuration names each role's run

flume SHALL read the roles a host wakes from the `[roles]` table of the
host's own configuration, each address `<partition>/<role>` with the
repository, kind, priority, base (the repository's default branch where
it names none) and optional brief of its run, and SHALL choose none of
them of its own (70, 132, C.7).

#### Scenario: A role configured

- **WHEN** the host's configuration names `madswan/planner` with a repository, the kind `planner` and priority 5
- **THEN** a wake of `madswan/planner` launches a place of kind `planner` in that repository at priority 5, cut from the repository's default branch, with no brief

#### Scenario: A base and a brief

- **WHEN** the table gives `madswan/planner` the base `next` and a brief
- **THEN** the woken place is cut from `next` and its stages that ask for the brief get it whole

#### Scenario: An address with no kind

- **WHEN** an end is pending for `madswan/ops` and the table names nothing for it
- **THEN** nothing is launched, and status names the end pending for `madswan/ops` with no role configured

### Requirement: A role's run is a place named by the role

A role's run SHALL be a place of its configured kind and repository,
named by the role, its agent's session named by the address, so at most
one run of an address is live at once (3, 132).

#### Scenario: The run's session

- **WHEN** a wake launches `madswan/planner`'s run and its first stage starts
- **THEN** the fake terminal opens the agent under the session name `madswan/planner`, and status names the place's role address

### Requirement: An end pending for an address with no run wakes the role

A settle that finds an end pending for an address with no run live or
waiting for room SHALL launch the address's run, inside that settle; an
end for an address whose run is live SHALL be delivered into the pane of
that run's agent as to any recipient (43, 132, core.13).

#### Scenario: Three ends, one run

- **WHEN** a unit's place names `madswan/planner` as its recipient, its verify stops short needing an answer while no planner run is live, and two more ends for the address come before the run's agent starts
- **THEN** exactly one planner place is launched, and the run's `flume inbox` gives all three ends (S38)

#### Scenario: Two settles at once

- **WHEN** two settles each find an end pending for `madswan/planner` with no run and launch at once
- **THEN** one run is made, the other launch is refused naming it, and neither settle fails (3, 75)

### Requirement: A carrier may wake an address

`flume wake <address>` SHALL launch the address's run where none is live
or waiting for room, and SHALL do nothing, answering so, where one is
(132).

#### Scenario: A wake while the run waits for room

- **WHEN** crew's card carrier wakes `madswan/planner` while its run waits for room
- **THEN** the wake answers that the run waits, and no second place is made (S38)

### Requirement: A role run collects for its address

`flume inbox` run in a role's run with no recipient named SHALL collect
for the run's address, and flume SHALL decide nothing else of what the
run does (43, 69, 72, 132).

#### Scenario: The run starts

- **WHEN** the planner run's agent runs `flume inbox` with no recipient
- **THEN** it collects every end pending for `madswan/planner`, each then delivered to that address

### Requirement: A role's run names its stream

A role's `[roles]` table SHALL take an optional `stream`, given to its
woken run's launch as the launch's stream, so the run is placed and
refused as any launch naming that stream is (132, 143, C.7).

#### Scenario: A role in its stream

- **WHEN** the host's configuration names a role's run with `stream = "work"` and an end pending for its address wakes it
- **THEN** the woken place runs in `work`, from `launch` (132, 143)
