## ADDED Requirements

### Requirement: A launch may name its kind's version

A launch's kind SHALL be a name, or a name and a version written
`<name>@<version>`; the name SHALL resolve as before, the launch's
declaration over the repository's over flume's own, and the version
SHALL be the one named, else, among flume's own, the newest flume
holds; a launch naming a version its resolved declaration does not hold
SHALL be refused naming the version and the versions held, and leave
nothing made (4, 19, 22, 154).

#### Scenario: No version named

- **WHEN** a repository that declares no `change` launches `change`
- **THEN** the place runs change@2 and status names change@2 (22, 154)

#### Scenario: change@1 named

- **WHEN** the same repository launches `change@1`
- **THEN** the place runs change@1, its code stage one agent for the whole change, and status names change@1 (22, 154)

#### Scenario: A version flume does not hold

- **WHEN** a launch names `change@3`
- **THEN** it is refused naming change@3 and the versions held, 1 and 2, and nothing is made (4)

#### Scenario: A version the repository's declaration does not hold

- **WHEN** a repository declares `change` at version 1 and a launch names `change@2`
- **THEN** the launch is refused naming the version, since the repository's declaration resolves the name, and nothing is made (4, 19, 154)
