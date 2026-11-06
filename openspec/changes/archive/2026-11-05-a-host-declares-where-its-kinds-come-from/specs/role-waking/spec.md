## ADDED Requirements

### Requirement: A role's kind may be an identity

A role's `kind` in the host's configuration SHALL be read as a launch's
`--kind` is, a name or an identity, each with or without a version, and
resolved and locked as a launch's kind is, a role's run carrying no
declaration of its own (132, 154, 155, C.7).

#### Scenario: A role woken to a kind of a host's source

- **WHEN** the host's configuration names `madswan/planner` with `kind = "acme/turn"`, acme holds `turn@1` and `turn@2`, the host's lock records both, and an end pending for the address wakes it
- **THEN** the woken place runs acme/turn@2, and status names `acme/turn@2` with the source's file (132, 154)

#### Scenario: A role's kind unlocked

- **WHEN** a role names `kind = "acme/turn@1"`, no lock records it, and `flume wake` names its address
- **THEN** the wake is refused naming acme/turn@1 as recorded in no lock, and no place is made (4, 132, 155)
