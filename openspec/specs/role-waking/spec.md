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

### Requirement: A role's kind may be an identity

A role's `kind` in the host's configuration SHALL be read as a launch's
`--kind` is, a name or an identity, each with or without a version, and
resolved and locked as a launch's kind is, a role's run carrying no
declaration of its own (132, 154, 155, C.7).

#### Scenario: A role woken to a kind of a host's source

- **WHEN** the host's configuration names `madswan/planner` with `kind = "acme/turn"`, acme holds `turn@1` and `turn@2`, the host's lock records both, and an end pending for the address wakes it
- **THEN** the woken place runs acme/turn@2, and status names `acme/turn@2` with the source's file (132, 154)

#### Scenario: A role's kind unlocked

- **WHEN** a role names `kind = "acme/turn@1"`, no lock records it, and `flume wake` names its address
- **THEN** the wake is refused naming acme/turn@1 as recorded in no lock, and no place is made (4, 132, 155)

### Requirement: The host's configuration may name a standing agent for an address

A `[roles]` entry SHALL be a standing agent's where it gives `agent`, a
string naming the agent as the terminal knows it, with an optional
string `session` and `stream`; the configuration SHALL be unreadable,
naming the address, where an entry gives `agent` beside `kind`, or
beside `repository`, `priority`, `base` or `brief`, naming the key, or
`session` with no `agent` (3, 132, C.7).

#### Scenario: A standing agent's entry read

- **WHEN** the host's configuration gives `[roles."madswan/planner"]` `agent = "madswan-planner"` and `session = "madswan"`, and `[roles."madswan/ops"]` a repository and a kind
- **THEN** the host gives `madswan/planner` a standing agent `madswan-planner` in `madswan` and no run, and `madswan/ops` its run and no standing agent (132, C.7)

#### Scenario: A standing agent and a kind

- **WHEN** an entry gives `agent = "madswan-planner"` and `kind = "planner"`
- **THEN** the configuration is unreadable, naming `madswan/planner` as given both a standing agent and a kind (3, 132)

#### Scenario: A run's key beside a standing agent

- **WHEN** an entry gives `agent = "madswan-planner"` and `priority = 5`
- **THEN** the configuration is unreadable, naming `madswan/planner` and `priority` (C.7)

#### Scenario: A session with no agent

- **WHEN** an entry gives a repository, a kind and `session = "madswan"`
- **THEN** the configuration is unreadable, naming `session` as no word of a role's run (C.7)

### Requirement: A standing address wakes no run

A settle that finds an end pending for an address the configuration
gives a standing agent SHALL launch nothing for it, record nothing of
it as unwoken, and open, start and watch nothing for it, whatever the
terminal finds (132; core.13, core.28).

#### Scenario: Ends pending while the agent is away

- **WHEN** three ends are pending for `madswan/planner`, whose standing agent the fake terminal holds nowhere, and the place is settled twice
- **THEN** no place is made, the fake terminal records no open, and the place's unwoken names nothing for the address (132, S38)

### Requirement: A wake of a standing address launches nothing

`flume wake <address>` of an address the configuration gives a standing
agent SHALL launch nothing and SHALL answer that a standing agent holds
it, with what status says of the address: its agent, its session, what
the terminal found and its pending ends (132).

#### Scenario: A carrier wakes a standing address

- **WHEN** a carrier runs `flume wake madswan/planner`, whose standing agent the fake terminal holds waiting
- **THEN** the answer is held, naming `madswan-planner`, its session and the agent found waiting, and no place is made (132)

### Requirement: The configuration decides who holds an address

Where the configuration gives an address a standing agent, an end for
it SHALL go to that agent and not to a live run of an earlier entry;
where it gives a kind, an end SHALL wake or reach the run and not a
standing agent; each read as the configuration stands at the delivery
(32, 132).

#### Scenario: A standing agent over a run still live

- **WHEN** `madswan/planner`'s run, woken under an entry with a kind, is live, the entry then gives `agent = "madswan-planner"`, and an end for the address is read
- **THEN** the notice goes to the standing agent, and the run's session receives none (132)

#### Scenario: A standing agent's entry becomes a run's

- **WHEN** `madswan/planner`'s entry gives `agent`, its agent takes one end, the entry then gives a repository and a kind, and another end for the address is read
- **THEN** a run of the kind is woken for the second end, and the standing agent receives no second notice (132)
