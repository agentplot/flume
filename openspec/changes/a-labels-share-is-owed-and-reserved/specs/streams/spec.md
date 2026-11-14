## MODIFIED Requirements

### Requirement: A host declares its streams

The host's configuration SHALL declare streams by name, each with its
bound on agents running, the host's bound alone holding it where it
gives none, its priority, a whole number, a stream of higher priority
starting first, zero where it gives none, and its pool. The stream
`default` SHALL always stand, held by the host's bound alone unless a
`[streams.default]` table states its own. The host's bound SHALL cap
every stream's agents together (70, 143, C.7).

#### Scenario: Two streams declared

- **WHEN** the host's configuration gives `[host] agents = 8`, `[streams.work]` with `agents = 5`, `priority = 10` and `pool = "work"`, and `[streams.personal]` with `agents = 3`
- **THEN** the host reads three streams, `work`, `personal` and `default`, `personal` at priority zero and `default` bounded by the host's eight alone (143, C.7)

#### Scenario: No streams

- **WHEN** the host's configuration declares no stream
- **THEN** every place runs in `default`, bounded by `[host] agents` alone (70)

## ADDED Requirements

### Requirement: A bound on agents is a whole number from 1

The host's bound and each stream's SHALL each be a whole number of
agents from 1, the host's none where it gives none, and a bound of 0,
the host's or a stream's, SHALL refuse the configuration naming its
key (70, 143, C.7).

#### Scenario: A host's bound of zero

- **WHEN** the managed layer gives `[host] agents = 0`
- **THEN** reading the configuration is refused naming `[host] agents` and that it is a whole number from 1 (70, C.7)
