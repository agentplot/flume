## MODIFIED Requirements

### Requirement: Status gives every fact of a place

Status SHALL give, for one place or every place, what clause 56 lists of
it: the place's repository, kind at its version with its declaration,
name, branch, base with where its revision was read (`merge-queue`,
`origin`, `no-origin` or `origin-lacks`, none for a place launched
before), mark, priority and recipients; each stage's state, end and
start count; whether the deliverable exists; the submission, how often
flume submitted and how often the place went back; the change's tasks;
refused acts; and a refused stage's part, the host's part being the
host queue's (56, 70, C.1).

#### Scenario: A stage's end with what it delivered

- **WHEN** status is asked of a place whose code stage delivered
- **THEN** the stage's end carries the head it delivered at, and a stage that ended short carries the agent's words and its kind (43, 56)

#### Scenario: Where the base was read

- **WHEN** status is asked of a place cut at origin's base, and of one cut at the host's own where origin holds none
- **THEN** the JSON form gives `base_from` `origin` and `origin-lacks`, and the person's form `from main (at origin)` and `from trial (the host's own, origin holding none)` (56, C.1)
