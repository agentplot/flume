## RENAMED Requirements

- FROM: `### Requirement: An end pending for an address with no run wakes the role`
- TO: `### Requirement: A delivery pending for an address with no run wakes the role`

## MODIFIED Requirements

### Requirement: A delivery pending for an address with no run wakes the role

A settle that finds an end or a message pending for an address, handed
to it by nothing, with no run live or waiting for room SHALL launch the
address's run, inside that settle, and a message's own act SHALL do the
same; deliveries for an address whose run is live SHALL be handed to
that run's session at its next wait, as to any recipient; a delivery a
run was handed and did not confirm SHALL wake no run again (43, 132,
214, core.13, core.28).

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

#### Scenario: A message wakes the role

- **WHEN** a caller messages `madswan/planner` while no planner run is live or waiting
- **THEN** the message's own act launches one planner run, and a settle that follows launches none (132, 214)

### Requirement: A role's run takes no notice before its prompt

While the current start of an address's run is starting, or started
with its prompt not yet received, a settle SHALL hand nothing into the
run's session, which its agent would take as its prompt, and what came
after the prompt was filled SHALL wait for the run's next wait. A
standing agent of the address SHALL be delivered to as before (37, 38,
43, 132, 214).

#### Scenario: A run not yet given its prompt

- **WHEN** a fix's end wakes `east/planner`'s run, whose agent waits in the session `east/planner` before it says it started, and the fix is settled again
- **THEN** no notice goes into the run's session, and once the agent says it started and its prompt is received the end is recorded handed to `east/planner`, carried in that prompt, and no settle sends it again (37, 38, 43)

#### Scenario: What came after the prompt

- **WHEN** an end comes for `east/planner` after its run's prompt was filled and before it was received
- **THEN** the end is handed to none until the run's agent next waits, and then goes in one prompt alone (214; record 0058)

### Requirement: A role run collects for its address

`flume inbox` run in a role's run with no recipient named SHALL collect
for the run's address, its ends and its messages, and flume SHALL decide
nothing else of what the run does (43, 69, 72, 132, 214).

#### Scenario: The run starts

- **WHEN** the planner run's agent runs `flume inbox --confirm` with no recipient
- **THEN** it collects every end and every message pending for `madswan/planner`, each handed to that address by the inbox and confirmed by it

## ADDED Requirements

### Requirement: A role's run's first prompt carries its batch

Each start of an address's run SHALL fill its prompt with the batch
pending at its address when the prompt is filled, after the kind's
template and before the way to stop short, claiming those deliveries
for the address under the start's identity and recording them handed
once its prompt is read received; a start read never begun, or ended
before its prompt was received, SHALL let its claims go and record
nothing (28, 37, 43, 214).

#### Scenario: The first prompt carries the batch

- **WHEN** a message and a fix's end are pending for `east/planner` when its run's start fills its prompt
- **THEN** the prompt holds the kind's template, then the message and the end, each with its confirm, then the way to stop short (28, 214)

#### Scenario: A start that never began

- **WHEN** the run's start carrying a batch is read never begun
- **THEN** its claims are let go, nothing is recorded handed, and the next start's prompt carries the same batch

### Requirement: A role's run takes nothing while it waits on an answer

While a role's run's stage ended short and waits on its answer, a
settle SHALL hand its address's batch to no one, and its take SHALL
answer nothing, so a delivery is never taken for the answer (42, 53,
214).

#### Scenario: A message while the run waits on its answer

- **WHEN** a role's run's code ended short needing an answer, its agent kept open, and a message comes for its address
- **THEN** nothing is typed into its session and the message stays handed to none; once a retry answers the stop-short and the agent goes quiet again, the next settle hands it the message

### Requirement: A launch from a role's run reports to its address

A launch that names no recipient, with `$FLUME_RECIPIENT` unset, run
where `$FLUME_PLACE` names a role's run, SHALL have that run's address
as its recipient, `recipients_from` `role-run`, ahead of the terminal's
session and the answerer behind its stream (7, 132, 214).

#### Scenario: A role's run launches its project

- **WHEN** the run of `east/ops` launches a fix naming no recipient, its environment naming the run's place, and the fix delivers
- **THEN** the fix's recipients are `east/ops` from `role-run`, and its end is pending at `east/ops` with the address's other deliveries (S128)

#### Scenario: Through MCP

- **WHEN** the same run's session calls flume's MCP `launch` for a chore naming no recipient
- **THEN** the chore's recipients are `east/ops` from `role-run` (S128)
