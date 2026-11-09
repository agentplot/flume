## MODIFIED Requirements

### Requirement: An end pending for an address with no run wakes the role

A settle that finds an end pending for an address, handed to it by
nothing, with no run live or waiting for room SHALL launch the address's
run, inside that settle; an end for an address whose run is live SHALL
be sent into the pane of that run's agent as a notice, as to any
recipient; an end a run was handed and did not confirm SHALL wake no run
again (43, 132, core.13, core.28).

#### Scenario: Three ends, one run

- **WHEN** a unit's place names `madswan/planner` as its recipient, its verify stops short needing an answer while no planner run is live, and two more ends for the address come before the run's agent starts
- **THEN** exactly one planner place is launched, and the run's `flume inbox` gives all three ends (S38)

#### Scenario: Two settles at once

- **WHEN** two settles each find an end pending for `madswan/planner` with no run and launch at once
- **THEN** one run is made, the other launch is refused naming it, and neither settle fails (3, 75)

#### Scenario: A run that did not confirm

- **WHEN** the planner run's `flume inbox` is handed a stop-short, the run is dropped before it confirms it, and the place is settled again
- **THEN** no planner run is launched for it, and status gives the stop-short handed to `madswan/planner` by the inbox and pending for it (S122)

#### Scenario: The next run is handed it

- **WHEN** the unit's next end comes for `madswan/planner` with no run live, and the run it wakes runs `flume inbox`
- **THEN** one run is launched, its inbox gives both ends, and once it runs `flume confirm` with both delivery keys both are delivered to the address (S122)

### Requirement: A role run collects for its address

`flume inbox` run in a role's run with no recipient named SHALL collect
for the run's address, and flume SHALL decide nothing else of what the
run does (43, 69, 72, 132).

#### Scenario: The run starts

- **WHEN** the planner run's agent runs `flume inbox --confirm` with no recipient
- **THEN** it collects every end pending for `madswan/planner`, each handed to that address by the inbox and confirmed by it
