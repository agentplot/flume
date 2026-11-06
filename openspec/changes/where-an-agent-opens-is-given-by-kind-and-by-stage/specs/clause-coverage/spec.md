## MODIFIED Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or removes SHALL
change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names agentplot/blueprints main's f676e8f, which states 32, 107, 143, 186, 190, 200 and C.7 as record 0048 amends them and holds 199, S106 and S107, or a later revision of main the build read; Parts A and B hold every clause that revision states, 199 in Part A; the scenarios run as that revision holds them, S106 and S107 among them; and it names as unbuilt what it named before this change, each where its change has not landed, and each clause of that revision another change has not yet built, by that change, and nothing of 199, S106 or S107

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
- **THEN** it passes: every clause of Parts A and B, I1 to I8 and every scenario the list holds but those named as unbuilt is cited, 199, S106 and S107 by this change's scenarios
