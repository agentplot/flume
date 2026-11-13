## MODIFIED Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or removes SHALL
change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names the revision of boomdrivedev/blueprints main that holds this change's amendment, 210 and S120 among it, or the later one of main the build read; Parts A and B hold every clause that revision states, 210 in Part B and no clause it took out, 209 among them; the scenarios run as that revision holds them, S112 as amended, S119 and S120 among them; and it names as unbuilt what it named before this change but 209, which that revision no longer holds, and each clause and scenario of that revision another change has not yet built, by that change, and nothing of 210 and S120, 193, S93, S94, S98 and S99, 202 to 206 and 208, S109 to S112 and S118, 198, S104 and S105, 197 and S103, 207 and S113 to S117, or S119, whose changes have landed

