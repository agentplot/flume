## MODIFIED Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios S1 to S38, with the
statement's revision they were read at. A clause the statement adds or
removes SHALL change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** Part A holds 1 to 77, 107 to 114, 118 to 120, 123, 124 and 129 to 132; Part B holds 78 to 99, 101 to 106, 115 to 117, 121, 122, 125 to 128 and 133; and the list names the statement's revision, edf0de2 or later

### Requirement: A clause cited by nothing fails the suite

The coverage check SHALL fail naming every clause of Part A and Part B,
every invariant and every scenario of section 11 that no scenario of
the suite cites.

#### Scenario: A clause left out

- **WHEN** the check runs over scenarios that between them leave clause 70 out
- **THEN** it fails naming 70

#### Scenario: The suite as this change leaves it

- **WHEN** the coverage check runs over the suite as this change lands it
- **THEN** it passes: every clause of Parts A and B, I1 to I8 and S1 to S38 is cited, 128, 129, 130, 132 and S38 by this change's scenarios
