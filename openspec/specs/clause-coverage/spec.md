# clause-coverage Specification

## Purpose
Keeps the conformance suite honest about the statement: every scenario
cites the clauses it exercises, and a check in the gate fails when a
clause, an invariant or a scenario of the statement is cited by nothing
(106, section 12).

## Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or removes SHALL
change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names the revision of boomdrivedev/blueprints main that holds this change's amendment, 210 and S120 among it, or the later one of main the build read; Parts A and B hold every clause that revision states, 210 in Part B and no clause it took out, 209 among them; the scenarios run as that revision holds them, S112 as amended, S119 and S120 among them; and it names as unbuilt what it named before this change but 209, which that revision no longer holds, and each clause and scenario of that revision another change has not yet built, by that change, and nothing of 210 and S120, 193, S93, S94, S98 and S99, 202 to 206 and 208, S109 to S112 and S118, 198, S104 and S105, 197 and S103, 207 and S113 to S117, or S119, whose changes have landed

### Requirement: Every scenario cites its clauses

Every scenario of the suite, a seam's or the engine's, SHALL cite the
clauses it exercises, and the check SHALL fail naming a scenario that
cites nothing or cites a number that is no clause, invariant or
scenario of the statement.

#### Scenario: A citation of no clause

- **WHEN** a scenario cites 100, which the statement does not hold
- **THEN** the check fails naming the scenario and 100

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
- **THEN** it passes: every clause of Parts A and B, I1 to I8 and every scenario the list holds but those named as unbuilt is cited, 210 and S120 by this change's scenarios and S119 by record 0054's change's scenario

### Requirement: Seam scenarios run against any profile

Each seam's scenarios SHALL be generic over that seam's interface, so a
profile unit runs the same scenarios against its real tool as the fake
passes, and SHALL sit below one marker per seam where a profile unit
adds scenarios of its own (106).

#### Scenario: The place scenarios over another implementation

- **WHEN** the place scenarios are run against a second implementation of the place seam that admits a make over an existing branch
- **THEN** the scenario citing 83 fails
