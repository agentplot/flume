## MODIFIED Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or removes SHALL
change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names revision 5caff76 or the later one the build read; Part A holds 1 to 77, 107 to 114, 118 to 120, 123, 124, 129 to 132, 137, 140 to 149, 154 to 159, 161 to 163 and 165 to 173; Part B holds 78 to 99, 101 to 106, 115 to 117, 121, 122, 125 to 128, 133 to 136, 138, 139, 151 to 153, 160, 164 and 174; the scenarios run S1 to S75 as that revision holds them; and it names as unbuilt 148, 149, 152, S57 and S58 by record 0021's change, 155, 156, 158 to 160 and S59 to S63 by record 0022's, 163, 164 and S68 by record 0024's plan stage's, 165 to 168, S69 and S70 by record 0025's, and 170 to 174 and S71 to S75 by record 0026's, and nothing else

### Requirement: A clause cited by nothing fails the suite

The coverage check SHALL fail naming every clause of Part A and Part B,
every invariant and every scenario of section 11 that no scenario of
the suite cites, but for those the list names as built by no change yet,
and SHALL fail naming any of those a scenario cites.

#### Scenario: A clause left out

- **WHEN** the check runs over scenarios that between them leave clause 70 out
- **THEN** it fails naming 70

#### Scenario: The suite as this change leaves it

- **WHEN** the coverage check runs over the suite as this change lands it
- **THEN** it passes: every clause of Parts A and B, I1 to I8 and S1 to S75 but those named as unbuilt is cited, 162, 169, S66 and S67 by this change's scenarios and S64 by a scenario of a fix whose submission is held for its caller's word
