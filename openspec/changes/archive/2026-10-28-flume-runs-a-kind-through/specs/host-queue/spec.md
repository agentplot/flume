## ADDED Requirements

### Requirement: A stage starts only under the host's bound

A stage that may start, a launch's first, a delivery's next, a fix
round's or a retry's, SHALL start only while the agents running on the
host, every caller's counted together, are under the bound the host's
configuration states; otherwise it SHALL wait for room. The count and
the start SHALL be taken under one host-wide lock, so two settles at
once never start past the bound. A host that states no bound SHALL
have no queue (70, 75).

#### Scenario: A third launch on a host of two

- **WHEN** the host states two agents and three fixes are launched over the fakes
- **THEN** two code stages start, the third launch is taken and its stage waits for room, and no launch is refused

#### Scenario: Two settles at once

- **WHEN** one agent ends and two settles run at once with two stages waiting for room on a host of two
- **THEN** exactly one stage starts

### Requirement: What waits starts by priority

Every settle SHALL start the stages waiting for room on the host, the
highest priority first and equals in the order they came to wait, as
many as there is room for, and SHALL end no agent for one (31, 46, 70).

#### Scenario: Nine units on a host of four

- **WHEN** nine changes are launched on a host of four, one waiting place's priority is raised, and an agent ends
- **THEN** four constructs start, five wait in order, and the raised place's construct starts at the next room (S37)

### Requirement: Priority is the caller's

A launch SHALL carry the place's priority, zero where it gives none, and
a caller SHALL change it after, which reorders only what has not
started; a change of a place whose work is over SHALL be refused naming
it. flume SHALL assign or change no priority itself (7, 70).

#### Scenario: Priority raised while running

- **WHEN** a caller raises the priority of a place whose stage runs
- **THEN** the running agent is unchanged, the place's next start orders by the new priority, and status gives it

### Requirement: Status shows the host's queue

Status SHALL give, for the host, the agents running, the bound, the room
and the places waiting for room in the order they would start, each
with its stage, priority and since when; a place waiting for room SHALL
read as waiting on the host and never on its caller (56, 70, 130).

#### Scenario: Status of a full host

- **WHEN** status is asked on a host of two with two agents running and one place waiting
- **THEN** it gives two running, a bound of two, no room, and the waiting place with its priority and since when

#### Scenario: A host with no bound

- **WHEN** the host's configuration states no bound and five places are launched
- **THEN** every first stage starts, and status gives five running and judges no room
