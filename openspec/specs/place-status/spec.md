# place-status Specification

## Purpose
Status of one place or every place on the host, each fact read where it
lives when asked, and every answer of flume's in one JSON form a program
reads, from one core, that only ever gains fields (43, 56 to 62, 69).

## Requirements

### Requirement: Status gives every fact of a place

Status SHALL give, for one place or every place, what clause 56 lists:
the place's repository, kind at its version with its declaration, name,
branch, base and mark; each stage's state, end and start count; the
current agent with its identity, settings, state and whether it works
past its limit; whether the deliverable exists; the submission and how
often flume submitted; the change's tasks; refused acts; a refused
stage's part; and a stop-short's or failed submission's words.

#### Scenario: A stage's end with what it delivered

- **WHEN** status is asked of a place whose code stage delivered
- **THEN** the stage's end carries the head it delivered at, and a stage that ended short carries the agent's words (43, 56)

### Requirement: Status reads where each fact lives and writes nothing

Status SHALL read every fact of a deliverable, a change or a submission
from its seam when asked, at one head per place, SHALL keep none of
them between asks, and SHALL start no agent, send nothing and write
nothing (57, 58).

#### Scenario: A change ticked between two asks

- **WHEN** status is asked, the fake change then ticks a task at a new head, and status is asked again
- **THEN** the second answer gives the new count, and neither ask wrote to the fake store or called the fake terminal's open or send

### Requirement: One answer from one core

Every act's answer SHALL come from the one core, the same for the same
question about the same places whoever asks, and no surface SHALL
compute a stage's end, a status, a submission or a tidy of its own (60,
69).

#### Scenario: Asked three times

- **WHEN** status of one place is asked three times over the same fakes
- **THEN** the three answers are equal and none starts or tells an agent (S14)

### Requirement: One JSON form that only gains fields

Every answer SHALL serialize to one JSON form, the same through every
surface, and the suite SHALL hold each answer's field list so that an
answer losing a field fails the suite (61).

#### Scenario: A field removed

- **WHEN** an answer's form drops a field the suite's list holds
- **THEN** the suite fails naming the answer and the field

#### Scenario: A refusal is an answer

- **WHEN** a launch is refused
- **THEN** its answer is the same JSON form with the refusal and its reason, not an error of another shape (2, 61)
