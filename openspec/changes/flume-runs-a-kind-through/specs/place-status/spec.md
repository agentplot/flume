## MODIFIED Requirements

### Requirement: Status gives every fact of a place

Status SHALL give, for one place or every place, what clause 56 lists:
the place's repository, kind at its version with its declaration, name,
branch, base, mark and priority; each stage's state, end and start
count; the current agent with its identity, settings, state and whether
it works past its limit, and the command to run under a terminal that
prints it; whether the deliverable exists; the submission, how often
flume submitted and how often the place went back; the change's tasks;
refused acts; a refused stage's part; a stop-short's words and whether
it needs an answer or found work, or a failed submission's words; for a
place waiting on its caller, the move it waits on and since when; and,
for the host, the agents running, the bound, the room and the queue
(56, 70, 130).

#### Scenario: A stage's end with what it delivered

- **WHEN** status is asked of a place whose code stage delivered
- **THEN** the stage's end carries the head it delivered at, and a stage that ended short carries the agent's words and its kind (43, 56)

#### Scenario: A place waiting on an answer

- **WHEN** status is asked of a place whose construct ended short needing an answer
- **THEN** it gives the move waited on as a retry of construct, since when, and the question whole (56, 130)
