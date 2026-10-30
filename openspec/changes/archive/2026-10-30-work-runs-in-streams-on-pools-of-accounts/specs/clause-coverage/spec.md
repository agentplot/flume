## MODIFIED Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or removes SHALL
change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names revision 8a11fbb or the later one the build read; Part A holds 1 to 77, 107 to 114, 118 to 120, 123, 124, 129 to 132, 137, 140 to 149, 154 to 159 and 161; Part B holds 78 to 99, 101 to 106, 115 to 117, 121, 122, 125 to 128, 133 to 136, 138, 139, 151 to 153 and 160; the scenarios run S1 to S65 as that revision holds them; and it names as unbuilt 148, 149, 152, S57 and S58 by record 0021's change, and 155, 156, 158 to 160 and S59 to S63 by record 0022's, and nothing else
