## MODIFIED Requirements

### Requirement: The host's own documents

The store profile SHALL keep each account's spent time at
`accounts/<pool>/<account>.json` and each intent's affinity at
`intents/<digest>.json` under `$XDG_STATE_HOME/flume/`, each with its
version, written whole by compare-and-set under the store's lock and
readable by the host's user alone, and reached by every flume process
on the host; an intent's document an earlier release wrote SHALL read
as written (146, 147, 153, C.7).

#### Scenario: Two processes record one account spent

- **WHEN** two engines over one state directory each write an account's spent time at the version both read
- **THEN** one write lands and the other reads the version moved and writes again on what it reads (153)

#### Scenario: A spent time seen by another process

- **WHEN** one engine records work-1 spent until 15:00
- **THEN** a second engine built over the same state directory reads work-1 spent until 15:00, and the file's mode is 0600 (146, 153)

#### Scenario: An intent's document an earlier release wrote

- **WHEN** `intents/<digest>.json` holds an affinity with one `pool` and `account`, as an earlier release wrote it, and an affinity of two programs is then written over it
- **THEN** the first reads with that pool and account, and the file then holds an account for each program and no `pool` or `account` key (147, 153)
