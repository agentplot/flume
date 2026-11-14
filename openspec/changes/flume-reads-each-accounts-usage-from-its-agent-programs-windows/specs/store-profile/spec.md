## ADDED Requirements

### Requirement: A pool's strategy and threshold in the configuration

The host's configuration SHALL read a pool's `strategy`, `fill-first`,
`least-busy` or `most-left`, and its optional `threshold`, a whole
number from 1 to 100, each in the managed layer alone, and an account's
`name`, `program`, `env` and `agents` alone; it SHALL refuse an
account's `usage` as every word an account does not hold, naming the
account and the key, with no alias kept, and any other value of the
pool's keys, naming the key (144, 184, 185, C.7; record 0062).

#### Scenario: A most-left pool

- **WHEN** the configuration gives `[pools.work] strategy = "most-left"`, `threshold = 85`, and an account `{ name = "work-1", program = "claude" }`
- **THEN** the pool reads most-left with a threshold of 85, and work-1 names no reading (184, C.7)

#### Scenario: C.7's example

- **WHEN** the configuration is C.7's example, its accounts of `work` naming no reading
- **THEN** each of work-1 and work-2 reads, and `work` reads fill-first with a threshold of 90 (185, C.7)

#### Scenario: An account naming usage

- **WHEN** the configuration gives an account `{ name = "work-1", program = "claude", usage = "~/.local/state/usage/work-1.json" }`
- **THEN** every launch and start is refused naming `[pools.work.accounts[0]]` and `usage` as no word of an account (184, C.7; record 0062)

#### Scenario: Values that refuse

- **WHEN** a pool gives `threshold = 0`, then `101`, then `"90%"`, or `strategy = "round-robin"`
- **THEN** every launch and start is refused naming the key, and the strategy's words name fill-first, least-busy and most-left (4, C.7)

## MODIFIED Requirements

### Requirement: The host's own documents

The store profile SHALL keep each account's spent time and reading in
one document at `accounts/<pool>/<account>.json` and each intent's
affinity at `intents/<digest>.json` under `$XDG_STATE_HOME/flume/`,
each with its version, written whole by compare-and-set under the
store's lock and readable by the host's user alone, and reached by
every flume process on the host; the reading SHALL be in C.7's form,
and an account's document or an intent's document an earlier release
wrote SHALL read as written (146, 147, 153, 184, C.7).

#### Scenario: Two processes record one account spent

- **WHEN** two engines over one state directory each write an account's spent time at the version both read
- **THEN** one write lands and the other reads the version moved and writes again on what it reads (153)

#### Scenario: A spent time seen by another process

- **WHEN** one engine records work-1 spent until 15:00
- **THEN** a second engine built over the same state directory reads work-1 spent until 15:00, and the file's mode is 0600 (146, 153)

#### Scenario: An intent's document an earlier release wrote

- **WHEN** `intents/<digest>.json` holds an affinity with one `pool` and `account`, as an earlier release wrote it, and an affinity of two programs is then written over it
- **THEN** the first reads with that pool and account, and the file then holds an account for each program and no `pool` or `account` key (147, 153)

#### Scenario: A reading beside a spent time

- **WHEN** a document of work-1 holding a spent time until 15:00 and a reading is written over the state directory
- **THEN** `accounts/work/work-1.json` holds `until` and `reading`, the reading's `taken_at` and each reset RFC 3339's with their offset and each window's `used` a share from 0 to 100, its mode 0600, and a second store over the same state directory reads both (153, 184, C.7)

#### Scenario: An account's document an earlier release wrote

- **WHEN** `accounts/work/work-1.json` holds `until`, `words` and `recorded_at` alone, as an earlier release wrote it, and a document holding those and a reading is then written over it at its version
- **THEN** the first reads spent with no reading, and the file then holds the spent keys as they stood and the reading (153)

## REMOVED Requirements

### Requirement: A pool's strategy and threshold and an account's reading in the configuration

**Reason**: An account names no reading (184; record 0062): the
requirement is replaced by "A pool's strategy and threshold in the
configuration", which refuses `usage`.

**Migration**: Drop `usage` from each account of the host's
configuration; flume takes the reading from the agent program.

### Requirement: An account's reading is read from its file

**Reason**: No host writes a reading, and flume reads no file a host
writes for it (184; core.32; record 0062).

**Migration**: The reading is kept in the account's own document,
`accounts/<pool>/<account>.json`, by flume alone.
