## MODIFIED Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or removes SHALL
change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names the revision of agentplot/blueprints main that holds record 0039's amendment and states 193, S93, S94, S98 and S99, or the later one the build read; Part A holds what main's list holds when this change builds, and 193; Part B holds what main's list holds then; the scenarios run S1 to S103 as that revision holds them, or to the last a later one holds; and it names as unbuilt what main's list names then, each with its change, and nothing of this change's

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
- **THEN** it passes: every clause of Parts A and B, I1 to I8 and every scenario of section 11 but those named as unbuilt is cited, 193, S93, S94, S98 and S99 by this change's engine scenarios
