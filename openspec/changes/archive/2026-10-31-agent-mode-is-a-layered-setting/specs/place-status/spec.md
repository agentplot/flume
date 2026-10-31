## ADDED Requirements

### Requirement: Status gives an agent's mode

Status SHALL give the current agent's mode as its layer wrote it, with
the layer it came from, the program whose entry of a table of programs
the agent took, the value its profile passed, and whether its profile
read the mode back; an agent whose settings give no mode SHALL be given
none. A start that began in another mode, and a never-begun end of
one, SHALL name the mode asked and the mode it began in, in the JSON
form and the person's (56, 161, 162, 169).

#### Scenario: A mode from the host's table

- **WHEN** status is asked of a fix whose agent took `edits` from the host's `[kinds.fix.agents]`
- **THEN** the agent's settings give the mode `edits` from `host-kind`, the value its profile passed and that it was read back, in the JSON and the person's form (S66, 56)

#### Scenario: A table's entry

- **WHEN** status is asked of a place whose agent took the `agent` entry of `{ agent = "careful", other = "edits" }` from `launch`
- **THEN** the agent's mode reads the table as written from `launch`, the entry `agent` and the value `careful` passed (56, 169)

#### Scenario: A mode not read back

- **WHEN** status is asked of a place whose agent's profile reads no mode back
- **THEN** the agent's mode says it was not read back (169)

#### Scenario: A start in another mode

- **WHEN** status is asked of a place whose code start began in `ask-each` where its record named `all`
- **THEN** it names the start, the mode asked `all` and the mode begun `ask-each`, and no variable (56, 161)
