## ADDED Requirements

### Requirement: A stage counts from the first of its starts since it last delivered

A stage SHALL have started, for its deliverable, at the first of its
starts since it last delivered: a retry, the start after a spent account
and any other start of a stage that has not delivered since that first
start SHALL count what an earlier start of the stage made, so an agent
that finds its stage's work already committed ends it delivered with no
commit of its own (25, 49, 146). A fix round or a retry that goes back
to a stage before it SHALL leave its count as it stands (25, 49).

#### Scenario: An answered stop-short delivers what its start committed

- **WHEN** a fix place's code agent commits its fix, then stops short needing an answer, and the caller retries code with the answer
- **THEN** the fresh agent, making no commit, goes quiet and code delivers at the head the first start's commit reached, and the place is submitted (25, 49, S131)

#### Scenario: The start after a spent account counts the spent agent's commit

- **WHEN** a fix place's code agent commits, then stops at its account's usage limit, and the stage's next start, on another account, makes no commit and goes quiet
- **THEN** code delivers at the head the spent agent's commit reached (25, 146)

#### Scenario: A report written by an earlier start

- **WHEN** a stage that delivers a report has its agent write the report and then stop short needing an answer, and the retry's agent leaves the report as it stands and goes quiet
- **THEN** the stage delivers with that report (24, 25, 49)

#### Scenario: A going-back past a stage leaves its count

- **WHEN** a change place's code agent ticks every task and commits, then stops short needing an answer, and the caller retries construct, whose agent commits a change to the design, every task left ticked, and delivers
- **THEN** code's next agent, making no commit, goes quiet and code delivers, since code has not delivered since its first start (25, 49)

### Requirement: A fix round and a delivered stage's retry start the stage afresh

A fix round, by failed submission or by finding, and a retry of a stage
that has delivered, SHALL start the stage afresh, counting only what is
made after it (25, 112, 131, S52). A stage whose held state, written
before stages kept their count, holds none SHALL count from its next
start (25).

#### Scenario: A fix round counts only what is made after it

- **WHEN** a fix place whose code delivered on an earlier start's commit is submitted, the submission fails, and the fix round's code agent makes no commit and goes quiet
- **THEN** code is not delivered and the stop check names the commit it lacks, and once the agent stops short and the caller retries the submission, the same head is submitted again with no agent started (25, 49, 112, S52, S131)

#### Scenario: A retry of a stage that has delivered starts afresh

- **WHEN** a construct stage has delivered, and the caller retries construct and its fresh agent makes no commit and goes quiet
- **THEN** construct is not delivered, and the stop check names the change it lacks (25, 49)

#### Scenario: A stage written before counts from its next start

- **WHEN** a place's held state, written before a stage kept its count, holds a code start that committed and stopped short, and the caller retries code, whose agent makes no commit and goes quiet
- **THEN** code is not delivered, counting from the retry's start as flume did (25)
