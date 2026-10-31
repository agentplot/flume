## MODIFIED Requirements

### Requirement: The agent seam reads a spent account and a start without its settings

The agent seam SHALL say whether the agent's own record shows its
account reached a usage limit, with the reset it names, or unknown
(151), whether its agent began without the settings its start
recorded, naming each variable that differs, whether its program did,
and, where its profile reads the mode back, the mode asked and the mode
begun (135, 161, 162), and which variables its program keeps from every
process it starts, none by default (161).

#### Scenario: Spent and without its settings over the fake

- **WHEN** the fake agent scripts a usage limit with a reset for one start and a missing variable for another
- **THEN** `spent` reads yes with that reset for the first and `started_without` names the variable for the second (135, 151, 161)

#### Scenario: Begun without its settings and no turn taken

- **WHEN** an agent's word at its start shows a variable missing and it has taken no turn
- **THEN** `started_without` names the variable, and `own_work` and `asking` read no (31, 91, 161)

#### Scenario: Begun in another mode over the fake

- **WHEN** the fake agent scripts a start that began in `ask-each` where its record named `all`
- **THEN** `started_without` names the mode asked `all` and the mode begun `ask-each`, and no variable (135, 161, 162)

## ADDED Requirements

### Requirement: The agent seam maps a mode and says whether it reads one back

The agent seam SHALL map an agent's entry of a mode, one of flume's
four or a value of its program's own, to the value of its program's
that it passes, or refuse it with words naming the mode and the
program, refusing every entry by default; SHALL say whether its
program's word at its start, its word that it started or at its start's
first prompt, reads back the mode it began in, not by default; and its
command SHALL pass the value mapped, and none where
the settings give no mode (88, 161, 169).

#### Scenario: A profile that maps no mode

- **WHEN** an agent profile that keeps the seam's defaults is asked to map `edits`
- **THEN** it refuses with words naming `edits`, and says it reads no mode back (169)

#### Scenario: The fake's map over the seam's scenarios

- **WHEN** the agent seam's scenarios run against the fake agent scripted with a map of the four and the own value `careful`
- **THEN** each of the four and `careful` maps as scripted, any other own value is refused naming it, and the fake's command passes the value mapped (88, 169)

### Requirement: The types of a mode

The seams SHALL pass one type each for flume's four modes, a mode as a
layer gives it, one of the four or a table of programs, an agent's
entry of it, a start's record of its mode, and a mode a start began in
beside the mode asked; each SHALL serialize to JSON and back unchanged,
a held state written before the mode SHALL read with none, and a bare
word that is not one of the four SHALL be refused naming it (161, 162,
169).

#### Scenario: A held state with a mode

- **WHEN** a held state whose launch gives a table of programs, whose kind gives `edits`, and whose start records a mode and began in another is written and read
- **THEN** it reads back unchanged, and the same state written without any mode reads with none (161, 162)

#### Scenario: A bare word of a program's own

- **WHEN** a mode is read from the JSON word `"auto"`
- **THEN** it is refused naming `auto` and saying a program's own value goes under its program's name (169)
