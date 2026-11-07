## ADDED Requirements

### Requirement: The fake host scripts standing agents

The fake host SHALL give each address a test gives a standing agent, as
the test sets it at the time of each call, and none for an address a
test takes it from (132).

#### Scenario: An entry set and taken away

- **WHEN** a test gives `madswan/planner` a standing agent, reads the host's standing entries, takes it away and reads them again
- **THEN** the first read holds it and the second does not

### Requirement: The fake terminal scripts named agents in a part

The fake terminal SHALL hold agents a test adds by name in a part, each
under a session key of its own that its sends and state reach, two of
one name among them, give them to the seam's operation, record what is
sent to each, and answer a part a test refuses as not read (132).

#### Scenario: An agent added, removed and refused

- **WHEN** a test adds `w-planner` waiting in `w`, sends to its key, removes it, then refuses `w` with words
- **THEN** the first read lists it waiting, its key holds the notice, the second read lists none, and the third answers not read with those words

#### Scenario: Two of one name

- **WHEN** a test adds `w-planner` twice in `w`
- **THEN** the part's agents list two named `w-planner`, each under its own key
