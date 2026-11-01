## MODIFIED Requirements

### Requirement: The suite knows the statement's clauses

The suite SHALL hold the numbers of every clause of Part A and Part B,
the invariants I1 to I8 and the scenarios of section 11, with the
statement's revision they were read at, and SHALL name each clause no
change has built yet, each named as the change that builds it. A clause the statement adds or removes SHALL
change this list in the commit that adopts it.

#### Scenario: The clauses listed

- **WHEN** the suite's list is read
- **THEN** it names the statement's revision the build read, which states the host's sources as the user ruled on this change's design; Part A and Part B hold every clause that revision holds, the host's sources' among them; the scenarios run from S1 with no gap to that revision's last; and it names as unbuilt no clause or scenario of record 0022's, 154 to 160 and S59 to S63 among them, nor any the revision adds for the host's sources, each named otherwise as the revision's list holds it

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
- **THEN** it passes: every clause of Parts A and B, I1 to I8 and every scenario but those named as unbuilt is cited, 155, 156, 158 to 160, S59 to S63 and the clauses and scenarios stating the host's sources by this change's scenarios
