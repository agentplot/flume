## MODIFIED Requirements

### Requirement: A repetition delivers its group and the stage delivers every task

A repetition with a group SHALL have delivered once the change at the
branch's head holds that group, by number and title, with no task
unticked, and the newest commit touching the change was made since it
started, read at one head. flume SHALL then end its agent once quiet
and, while a task stays unticked, start the next repetition as a next
stage starts; the stage SHALL deliver once every task is ticked (24, 25,
40, 140).

A repetition with a group SHALL have started, for this, at the first
start given its group since its stage last delivered, so a start given
the group its stage's last start was given counts what that start made;
a start with no group SHALL deliver as its stage does, counting from
the first of the stage's starts since it last delivered (25, 140).

#### Scenario: Three groups, three agents

- **WHEN** a repeating code stage's change holds three groups and each repetition ticks its group, commits and goes quiet
- **THEN** three agents are opened in turn, never two live at once, and only after the third does the stage deliver and verify start (30, 40, S50)

#### Scenario: More than its group ticked

- **WHEN** a repetition given group 1 ticks groups 1 and 2 and commits
- **THEN** the next repetition is given group 3

#### Scenario: The last group ticked before a stop-short

- **WHEN** the repetition on group 3 of three ticks its group, commits, then stops short needing an answer, and the caller retries the stage with the answer
- **THEN** one repetition starts with no group, and the stage delivers once its agent, making no commit, is quiet (25, 49, 140)

#### Scenario: A repetition given its group again

- **WHEN** a repetition on group 2 of a stage that declares `preview` writes the line for `preview`, then stops short needing an answer, and the retry's agent, given group 2, ticks group 2 and commits without writing a line
- **THEN** the repetition delivers, the line the first start given group 2 wrote counting for it (25, 129, 140)

### Requirement: A retry starts the unticked group again

A retry naming a repeating stage SHALL start one fresh repetition given
the first group with a task unticked, with the retry's words, and never
the stage's first group (49, 142). A retry given the group the stage's
last start was given SHALL count from the first start given that group
since the stage last delivered (25, 140).

#### Scenario: A retry after a stop-short

- **WHEN** a repetition on group 2 stops short needing an answer and the caller retries the stage with the answer
- **THEN** a fresh agent starts on group 2 with the question and the answer in its prompt, never on group 1 (42, 49, S51)
