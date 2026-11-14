## MODIFIED Requirements

### Requirement: settings names the managed file's tables and keys

`services.flume.settings` SHALL be free-form TOML whose keys are the
managed file's, as 143 and C.7 name them. It SHALL declare `host`,
`streams`, `pools` with their `accounts`, `kinds`, `repos` and `agents`,
SHALL type each key C.7 gives one type that flume reads at the revision
the module is built from, each `agents` a whole number from 1, a
label's `owed` and `reserved` from 0 and a pool's `threshold` from 1 to
100, and SHALL leave an agents table free-form whole. Each declared
number's option, and the agents table's for its `keep_open`, SHALL say
in its description what the number counts, its range and what holds
where it is unset (record 0055; record 0063; principles 13).

#### Scenario: A mistyped declared key

- **WHEN** `nix:test` evaluates the options with `settings.streams.work.agents = "five"`
- **THEN** evaluation fails naming `services.flume.settings.streams.work.agents`

#### Scenario: A bound of zero

- **WHEN** `nix:test` evaluates the options with `settings.host.agents = 0`, and again with `settings.streams.work.agents = 0`
- **THEN** each evaluation fails, since flume reads each bound from 1 (70, C.7)

#### Scenario: Each number says its unit, range and default

- **WHEN** `nix:test` reads the descriptions of `host.agents`, `host.ends_expire`, a stream's `agents` and `priority`, a label's `owed` and `reserved`, a pool's `threshold`, an account's `agents` and the agents table
- **THEN** each says what its number counts, its range, `from 1`, `from 0`, `from 1 to 100`, a whole number for `priority` with which way it orders, or seconds, minutes and hours for a time, and what holds where it is unset (70, 143, 181, 185, 198, C.7)

### Requirement: Only the keys flume reads are declared

The module SHALL declare no key flume does not read at the revision it
is built from, so a stream's `points` and `pace` pass undeclared until
flume reads them (record 0025). It SHALL declare no word written before
the current one, so `host.terminal`, which flume reads as `multiplexer`
and refuses beside it, and a label's `reserve` and `keep_free`, which
flume refuses, pass undeclared (record 0055; record 0063).

#### Scenario: The declared keys

- **WHEN** `nix:test` reads the options `services.flume.settings` declares
- **THEN** `host` declares `multiplexer` and `ends_expire` and no `terminal`, a label declares `owed` and `reserved` and neither `reserve` nor `keep_free`, and a stream declares neither `points` nor `pace`
