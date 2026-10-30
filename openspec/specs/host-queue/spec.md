# host-queue Specification

## Purpose
The host's bound on agents running: every start of a stage takes the
host's room or waits for it, what waits starts by the priority its
caller gave, and status shows the agents running, the bound, the room
and the queue.

## Requirements

### Requirement: A stage starts only under the host's bound

A stage that may start SHALL start only while the agents running on
the host, every caller's counted together, are under the host's bound,
its stream's are under the stream's, and an account of its pool is
usable; otherwise it SHALL wait for room. The count and the start SHALL
be taken under one host-wide lock. A host that states no bound of its
own and no stream's bound SHALL have no queue (70, 75, 143, 145).

#### Scenario: A third launch on a host of two

- **WHEN** the host states two agents and three fixes are launched over the fakes
- **THEN** two code stages start, the third launch is taken and its stage waits for room, and no launch is refused

#### Scenario: Two settles at once

- **WHEN** one agent ends and two settles run at once with two stages waiting for room on a host of two
- **THEN** exactly one stage starts

#### Scenario: Two streams under the host's bound

- **WHEN** the host states six agents, `work` four at priority 10 and `personal` three, and five work and three personal places are launched
- **THEN** four work agents and two personal agents start, one work place waits for its stream's room and one personal place for the host's, and when a work agent ends the waiting work place starts first (S53)

#### Scenario: A stream's bound on a host with no bound

- **WHEN** the host states no bound of its own, `work` states one agent, and two work places and a place in `default` are launched
- **THEN** one work agent and the default place's agent start, the second work place waits for its stream's room, and when the work agent ends it starts (70, 143)

### Requirement: What waits starts by priority

Every settle SHALL start the stages waiting for room, by their stream's
priority, then their place's, then the order they came to wait, as many
as there is room for, passing over one whose stream is at its bound or
whose pool has no usable account without holding the host's room for
it, and SHALL end no agent for one (31, 46, 70, 143).

#### Scenario: Nine units on a host of four

- **WHEN** nine changes are launched on a host of four, one waiting place's priority is raised, and an agent ends
- **THEN** four constructs start, five wait in order, and the raised place's construct starts at the next room (S37)

#### Scenario: A full stream holds no other's room

- **WHEN** `work` at priority 10 is at its bound with a place waiting, `personal` at priority zero has a place waiting, and the host has room
- **THEN** the personal place starts, and the work place keeps its turn for its stream's next room (70, 143)

#### Scenario: The stream's priority before the place's

- **WHEN** a personal place at priority 50 and a work place at priority zero wait, `work` ranking above `personal`, and one agent's room opens
- **THEN** the work place starts (70, 143)

#### Scenario: A pool with nothing usable

- **WHEN** a stage's pool has every account spent and a stage of another pool waits behind it
- **THEN** the stage behind starts, and the first keeps its turn until an account of its pool is usable (70, 145)

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
with its stage, stream, priority and since when, and for each stream
the same; a place waiting for room SHALL read as waiting on the host,
naming its stream's or pool's want of room, and never on its caller (56,
70, 130, 143).

#### Scenario: Status of a full host

- **WHEN** status is asked on a host of two with two agents running and one place waiting
- **THEN** it gives two running, a bound of two, no room, and the waiting place with its stream, its priority and since when

#### Scenario: A host with no bound

- **WHEN** the host's configuration states no bound and five places are launched
- **THEN** every first stage starts, and status gives five running and judges no room

#### Scenario: Waiting on a stream's room

- **WHEN** a place waits because its stream is at its bound while the host has room
- **THEN** status reads it waiting on the host, naming its stream's bound, never on its caller and never stalled (70, 130)
