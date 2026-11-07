## MODIFIED Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or removes SHALL
change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names the revision of agentplot/blueprints main that holds 198, S104, S105 and 207 among it, or the later one of main the build read; Parts A and B hold every clause that revision states, 198 in Part A; the scenarios run as that revision holds them, S104 and S105 among them; and it names as unbuilt what it named before this change but 198, S104 and S105, and each clause and scenario of that revision another change has not yet built, by that change, and nothing of 198, S104 or S105

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
- **THEN** it passes: every clause of Parts A and B, I1 to I8 and every scenario the list holds but those named as unbuilt is cited, 198, S104 and S105 by this change's scenarios
