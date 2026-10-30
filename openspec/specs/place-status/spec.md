# place-status Specification

## Purpose
Status of one place or every place on the host, each fact read where it
lives when asked, and every answer of flume's in one JSON form a program
reads, from one core, that only ever gains fields (43, 56 to 62, 69).

## Requirements

### Requirement: Status gives every fact of a place

Status SHALL give, for one place or every place, what clause 56 lists of
it: the place's repository, kind at its version with its declaration,
name, branch, base, mark, priority and recipients; each stage's state,
end and start count; whether the deliverable exists; the submission, how
often flume submitted and how often the place went back; the change's
tasks; refused acts; and a refused stage's part, the host's part being
the host queue's (56, 70).

#### Scenario: A stage's end with what it delivered

- **WHEN** status is asked of a place whose code stage delivered
- **THEN** the stage's end carries the head it delivered at, and a stage that ended short carries the agent's words and its kind (43, 56)

### Requirement: Status gives a place's current agent

Status SHALL give a place's current agent with its identity, its
settings each with the layer it came from, each value as that layer
wrote it but an environment variable's, which status names with the
value `<set>` and never gives, its state and whether it works past its
limit, and the command to run under a terminal that prints it, which
names the file holding its environment and holds no value of it (32,
56; record 0017).

#### Scenario: Each setting with its layer

- **WHEN** status is asked of a change whose code agent took its model from the repository and `CLAUDE_CONFIG_DIR = "~/.claude-work"` from the host's configuration for the kind
- **THEN** the agent's settings give the model from `repository` and the variable `CLAUDE_CONFIG_DIR` from `host-kind` as `<set>` (32, 56, S44)

#### Scenario: A token in no answer and no record

- **WHEN** the host's configuration gives change's agents `CLAUDE_CODE_OAUTH_TOKEN`, a launch gives them another token, and the launch, status as text and as JSON, and a settle are asked
- **THEN** no answer holds either token's value; the host's is in no file of flume's state but the printed command's environment file, the launch's in none but that and the place's held state, which keeps the launch's layer for later starts, and each of those the host's user alone may read (32, 56)

### Requirement: Status says what a waiting place waits on

Status SHALL give a stop-short's words and whether it needs an answer or
found work, or a failed submission's words, and, for a place waiting on
its caller, the move it waits on and since when, a hold's words among it
(56, 130).

#### Scenario: A place waiting on an answer

- **WHEN** status is asked of a place whose construct ended short needing an answer
- **THEN** it gives the move waited on as a retry of construct, since when, and the question whole (56, 130)

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

### Requirement: Status gives what waits, what was made and what is pending

Status SHALL give, beside clause 56's other facts, each place's
recipients; each end with whom it was delivered to and when, and for
whom it stands pending; for a place waiting on its caller, the move,
since when, a hold's words and whether it is stalled; a role run's
address; each stage's made lines as written; and the ends pending for
an address the host names no role for (43, 56, 129, 130, 132).

#### Scenario: A stalled place with a pending end

- **WHEN** status is asked of a place stalled on a retry of construct whose end is delivered to `ops` and pending for `madswan/planner`
- **THEN** the answer gives the move, since when, stalled, the end delivered to `ops` with when, and pending for `madswan/planner`

#### Scenario: The fields kept

- **WHEN** the suite's field list is read after this change
- **THEN** it holds the recipients, the ends with their deliveries, the waiting move, the hold's words, stalled, the role address and the made lines, and an answer lacking one fails the suite (61)

### Requirement: Status gives a repeating stage's group

Status SHALL give a repeating stage with its current group's place among
the change's groups at the head and how many there are, with the group's
number and title, read from the change when asked and never from the
notes; the person's form SHALL print it as `code 3/7` (56, 58, 142).

#### Scenario: Status mid-stage

- **WHEN** status is asked while a repeating code stage's repetition works on the second of three groups
- **THEN** the JSON form gives the group's place 2 of 3 with its number and title, and the person's form prints `code 2/3`

#### Scenario: A group added mid-stage

- **WHEN** a repetition adds a fourth group to a change of three and status is asked
- **THEN** status gives the count as 4, read at the head
