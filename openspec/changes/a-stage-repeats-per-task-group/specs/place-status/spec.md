## ADDED Requirements

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
