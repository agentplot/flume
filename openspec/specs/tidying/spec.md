# tidying Specification

## Purpose
Removing a place whose work is over through its repository's teardown,
and keeping a place or a branch that holds what must not be lost (I4;
16, 62 to 68).

## Requirements

### Requirement: Only what is over, with no agent live

A tidy SHALL remove only places flume made, whose work is over and whose
agent has stopped, through the place seam's remove, which runs the
repository's teardown; and SHALL end every agent flume started in a
place it removes (16, 63, 64, 66).

#### Scenario: A place whose work is not over

- **WHEN** a tidy is asked of a place whose stages still run
- **THEN** the fake place records no remove and the answer names the place as not over

### Requirement: A place that holds work is kept

A tidy SHALL keep a place that the place seam reads as holding a change
not committed, a file neither committed nor ignored, or a lock, naming
what it holds; and SHALL keep a place whose teardown fails, naming the
failed step (65, 66).

#### Scenario: An untracked file and a failed teardown

- **WHEN** one place's read names a file neither committed nor ignored, and another's remove fails at a teardown step
- **THEN** both are kept, status names the file for the first and the step for the second (S10)

### Requirement: A branch goes only when its base holds its work

When a place goes, its branch SHALL go through the place seam's
remove-branch only when it holds no commit whose change its base lacks;
a branch it keeps SHALL be named once, in the answer to the act that
removed its place (67).

#### Scenario: A done round that keeps its branch

- **WHEN** a kind that does not land delivers a report under the place's own folder, is retried with words and delivers a second report by its write time, then is declared done
- **THEN** the place is removed with its folder, no submission is ever made, and the branch holding commits its base lacks is kept and named (S18, 123)

### Requirement: Nothing removed while over cannot be read

A tidy SHALL remove nothing while the merge queue seam cannot be read
for a place whose kind lands (68).

#### Scenario: The merge queue unreadable

- **WHEN** the fake merge queue answers that it could not be read
- **THEN** no place is removed and status names what could not be read
