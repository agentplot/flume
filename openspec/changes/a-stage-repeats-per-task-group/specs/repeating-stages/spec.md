# repeating-stages

## Purpose

A stage worked one task group of its change at a time, a fresh agent
for each, with flume settling between them: a repetition's group and
delivery, the stage's delivery, the group and notes parts, and what is
counted, named and retried per repetition (30, 140 to 142; record 0019;
core.10, core.31).

## ADDED Requirements

### Requirement: A repetition is a fresh start given the first unticked group

Each start of a repeating stage SHALL be one fresh agent in the place,
given the first task group of the change holding a task unticked at the
head it starts at, recorded on the start; a start finding every task
ticked SHALL be one repetition with no group. Each repetition SHALL be a
start of the stage, with its own identity, secret and start count (30,
33, 48, 140).

#### Scenario: The first repetition

- **WHEN** a repeating code stage starts on a change of three groups, none ticked
- **THEN** one agent is opened, its start records group 1, and its identity names start 1

#### Scenario: A send-back with every task ticked

- **WHEN** a failed submission sends a place back to its repeating code stage with every task ticked
- **THEN** one repetition starts with no group and the merge queue's words, and delivers once a commit since its start touching the change leaves every task ticked (112, 140, S52)

### Requirement: A repetition delivers its group and the stage delivers every task

A repetition with a group SHALL have delivered once the change at the
branch's head holds that group, by number and title, with no task
unticked, and the newest commit touching the change was made since it
started, read at one head. flume SHALL then end its agent once quiet
and, while a task stays unticked, start the next repetition as a next
stage starts; the stage SHALL deliver once every task is ticked (24, 25,
40, 140).

#### Scenario: Three groups, three agents

- **WHEN** a repeating code stage's change holds three groups and each repetition ticks its group, commits and goes quiet
- **THEN** three agents are opened in turn, never two live at once, and only after the third does the stage deliver and verify start (30, 40, S50)

#### Scenario: More than its group ticked

- **WHEN** a repetition given group 1 ticks groups 1 and 2 and commits
- **THEN** the next repetition is given group 3

### Requirement: A repetition's delivery is no end of its stage

flume SHALL record no end and deliver nothing to a recipient for a
repetition that delivered while a task stays unticked; the stage's end
SHALL be recorded and delivered once, when the stage delivers (43, 140).

#### Scenario: One notice for the stage

- **WHEN** a repeating stage of three groups delivers with a recipient waiting on it
- **THEN** the recipient is delivered one end, the stage's, carrying the head its last repetition delivered at

### Requirement: The next repetition waits for room like any start

The next repetition SHALL be admitted under the host's bound and queue
as any start is, and flume SHALL keep no room for a place between its
repetitions (70, 140; core.26).

#### Scenario: A full host between repetitions

- **WHEN** a repetition ends on a host at its bound with a stage of higher priority waiting
- **THEN** the waiting stage starts first, and the next repetition waits for room with its place in the queue

### Requirement: The group and notes parts

`{{group}}` SHALL fill, for a repeating stage, with the repetition's
group, its number, title and tasks as the change gives them, or words
saying it has none; `{{notes}}` with the path `notes/<stage>.md` in the
place's own folder. Neither SHALL fill for a stage that does not repeat.
flume SHALL create, read and remove nothing at the notes' path (27, 62,
141; core.31).

#### Scenario: The parts rendered

- **WHEN** `flume prompt` renders a repeating code stage on group 2
- **THEN** the text holds `Group 2.`, its title and its tasks, and the notes' path under the place's folder

#### Scenario: Notes never read

- **WHEN** a repetition writes the notes and the next repetition starts
- **THEN** status, the end and the next prompt hold nothing of what the notes say, the file is as the agent left it, and it is gone after the tidy

#### Scenario: A part asked for by a stage that does not repeat

- **WHEN** a stage that does not repeat asks for `{{group}}`
- **THEN** the stage is refused naming the part, and no agent starts (27)

### Requirement: Limits run per repetition

The quiet limit, working past it and a stall SHALL run from each
repetition's own start or end, never from the stage's first start (44,
118, 130, 142).

#### Scenario: A long stage with short repetitions

- **WHEN** a repeating stage's four repetitions each work twenty minutes under a thirty-minute limit
- **THEN** none is reported working past its limit, though the stage has run eighty minutes

### Requirement: A repetition ends short as a stage does, naming its group

A repetition SHALL be sent back by the stop check, and end short, stuck,
lost or never begun, as a stage does; the stop check's words, the end,
its notice and a stop SHALL name its group (38, 41 to 45, 51, 110, 142).

#### Scenario: Quiet with its group unticked

- **WHEN** a repetition on group 2 goes quiet with one of its tasks unticked
- **THEN** the stop check sends it back naming group 2 and the task left (110, S51)

#### Scenario: A stop names the group

- **WHEN** a caller stops a repeating stage's agent on group 4
- **THEN** the stage waits for a retry, and status names group 4

### Requirement: A retry starts the unticked group again

A retry naming a repeating stage SHALL start one fresh repetition given
the first group with a task unticked, with the retry's words, and never
the stage's first group (49, 142).

#### Scenario: A retry after a stop-short

- **WHEN** a repetition on group 2 stops short needing an answer and the caller retries the stage with the answer
- **THEN** a fresh agent starts on group 2 with the question and the answer in its prompt, never on group 1 (42, 49, S51)
