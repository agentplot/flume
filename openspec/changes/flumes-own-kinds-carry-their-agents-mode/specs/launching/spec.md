## MODIFIED Requirements

### Requirement: A launch may name its kind's version

A launch's kind SHALL be a name, a name and a version, or an identity,
`<source>/<name>` with or without `@<version>`, resolved as the
kind-sources capability says: a name over the launch's declaration, the
repository's and flume's own, and a version the one named, else the
newest the locks record, else the newest its source holds; a version its
source does not hold SHALL refuse the launch naming it and the versions
held, and leave nothing made (4, 19, 22, 154).

#### Scenario: No version named

- **WHEN** a repository that declares no `change` and locks nothing launches `change`
- **THEN** the place runs change@4, its code stage one agent for the whole change and each stage's agent in `full` from `kind`, and status names `flume/change@4` (22, 154, 162, S119; record 0064)

#### Scenario: change@2 named

- **WHEN** the same repository launches `change@2`
- **THEN** the place runs change@2, its code stage repeating per task group, and status names `flume/change@2` (22, 154)

#### Scenario: change@1 named

- **WHEN** the same repository launches `change@1`
- **THEN** the place runs change@1, its code stage one agent for the whole change, and status names `flume/change@1` (22, 154)

#### Scenario: A version flume does not hold

- **WHEN** a launch names `change@5`
- **THEN** it is refused naming change@5 and the versions held, 1, 2, 3 and 4, and nothing is made (4)

#### Scenario: A version the repository's declaration does not hold

- **WHEN** a repository declares `change` at version 1 and a launch names `change@2`
- **THEN** the launch is refused naming the version, since the repository's declaration resolves the name, and nothing is made (4, 19, 154)

#### Scenario: A kind of a host's source named whole

- **WHEN** the host's source acme holds `review@2`, locked by the host, and a launch names `acme/review@2`
- **THEN** the place runs acme's review@2, and status names `acme/review@2` (154, 155)
