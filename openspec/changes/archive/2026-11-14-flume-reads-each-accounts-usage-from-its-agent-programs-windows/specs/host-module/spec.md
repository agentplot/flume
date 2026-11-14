## MODIFIED Requirements

### Requirement: Only the keys flume reads are declared

The module SHALL declare no key flume does not read at the revision it
is built from, so a stream's `points` and `pace` pass undeclared until
flume reads them (record 0025), and an account's `usage`, which flume
refuses, passes undeclared (record 0062). It SHALL declare no word
written before the current one, so `host.terminal` and a label's
`reserve` and `keep_free`, which flume refuses, pass undeclared (record
0055; record 0063).

#### Scenario: The declared keys

- **WHEN** `nix:test` reads the options `services.flume.settings` declares
- **THEN** `host` declares `multiplexer` and `ends_expire` and no `terminal`, a label declares `owed` and `reserved` and neither `reserve` nor `keep_free`, a stream declares neither `points` nor `pace`, and an account declares `name`, `program`, `env` and `agents` and no `usage`
- **AND** `host.terminal`, which flume reads as `multiplexer` and refuses beside it, passes undeclared to be refused
