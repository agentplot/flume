## ADDED Requirements

### Requirement: The agent seam reads a spent account and a start without its settings

The agent seam SHALL say whether the agent's own record shows its
account reached a usage limit, with the reset it names, or unknown
(151), and whether its agent began without the settings its start
recorded, naming each variable that differs and whether its program
did (135, 161).

#### Scenario: Spent and without its settings over the fake

- **WHEN** the fake agent scripts a usage limit with a reset for one start and a missing variable for another
- **THEN** `spent` reads yes with that reset for the first and `started_without` names the variable for the second (135, 151, 161)

### Requirement: The store keeps the host's own documents

The store seam SHALL read and write the host's own documents, each
account's spent time and each intent's affinity, by key, compare-and-set
on a version as a place's held state is, and reached by every flume
process on the host (102 to 104, 153).

#### Scenario: A host document written at a moved version

- **WHEN** two writers write one key at the version both read
- **THEN** one write lands and the other is refused as moved (103, 153)

### Requirement: The types of streams, pools and a start's record

The seams SHALL pass one type each for a stream, a pool, its strategy
and its accounts, an account chosen and where from, a move between
accounts, and a start's record of what its agent runs under, holding
digests and no value; each SHALL serialize to JSON and back unchanged,
and a held state written before them SHALL read as before (143 to 147,
161).

#### Scenario: A held state from before streams

- **WHEN** a held state written before this change is read
- **THEN** its place reads the stream `default` from `default`, its repository's and base's intent, and starts with no account and no record (143, 147)
