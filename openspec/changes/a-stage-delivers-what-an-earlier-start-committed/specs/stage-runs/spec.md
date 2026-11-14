## MODIFIED Requirements

### Requirement: A stage ends at its deliverable, read at one head

A stage that is not a conversation SHALL end only when its deliverable
exists in its form, made since the stage started, every fact of it read
at one head of the place's branch, or when its agent stops short (24,
25, 39). A delivered stage SHALL end once its agent is quiet (40).

A stage SHALL have started, for this, at the first of its starts since
it last delivered: a retry, the start after a spent account and any
other start of a stage that has not delivered since that first start
SHALL count what an earlier start of the stage made, so an agent that
finds its stage's work already committed ends it delivered with no
commit of its own (25, 49, 146). A fix round, by failed submission or
by finding, and a retry of a stage that has delivered, SHALL start the
stage afresh, counting only what is made after (25, 112, 131, S52). A
fix round or a retry of a stage before it SHALL leave a stage's count
as it stands. A stage whose held state, written before, holds no count
SHALL count from its next start (25).

#### Scenario: Waiting on own work

- **WHEN** a code stage's agent ticks the last task, then waits twelve fake minutes on own work it started
- **THEN** the stage is neither ended nor sent back nor stuck until own work ends and the agent is quiet (S3)

#### Scenario: Tasks ticked over four commits

- **WHEN** a code stage's agent ticks two of five tasks, then all five, over four commits
- **THEN** status shows two of five, then five of five, and the stage delivers only at the head that ticks the fifth (S2, 24)

#### Scenario: Facts never paired across heads

- **WHEN** the head moves between the change's read and the newest commit's read in one settle
- **THEN** the settle reads every fact again at the new head and never pairs the two (25)

#### Scenario: An answered stop-short delivers what its start committed

- **WHEN** a fix place's code agent commits its fix, then stops short needing an answer, and the caller retries code with the answer
- **THEN** the fresh agent, making no commit, goes quiet and code delivers at the head the first start's commit reached, and the place is submitted (25, 49, S131)

#### Scenario: A fix round counts only what is made after it

- **WHEN** that place's submission then fails, and the fix round's code agent makes no commit and goes quiet
- **THEN** code is not delivered and the stop check names the commit it lacks, and once the agent stops short and the caller retries the submission, the same head is submitted again with no agent started (25, 49, 112, S52, S131)

#### Scenario: A retry of a stage that has delivered starts afresh

- **WHEN** a construct stage has delivered, and the caller retries construct and its fresh agent makes no commit and goes quiet
- **THEN** construct is not delivered, and the stop check names the change it lacks (25, 49)

#### Scenario: The start after a spent account counts the spent agent's commit

- **WHEN** a fix place's code agent commits, then stops at its account's usage limit, and the stage's next start, on another account, makes no commit and goes quiet
- **THEN** code delivers at the head the spent agent's commit reached (25, 146)

#### Scenario: A report written by an earlier start

- **WHEN** a stage that delivers a report has its agent write the report and then stop short needing an answer, and the retry's agent leaves the report as it stands and goes quiet
- **THEN** the stage delivers with that report (24, 25, 49)

#### Scenario: A going-back past a stage leaves its count

- **WHEN** a change place's code agent ticks every task and commits, then stops short needing an answer, and the caller retries construct, whose agent commits a change to the design, every task left ticked, and delivers
- **THEN** code's next agent, making no commit, goes quiet and code delivers, since code has not delivered since its first start (25, 49)

#### Scenario: A stage written before counts from its next start

- **WHEN** a place's held state, written before a stage kept its count, holds a code start that committed and stopped short, and the caller retries code, whose agent makes no commit and goes quiet
- **THEN** code is not delivered, counting from the retry's start as flume did (25)
