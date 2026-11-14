## MODIFIED Requirements

### Requirement: Only the keys flume reads are declared

The module SHALL declare no key flume does not read at the revision it
is built from, so a stream's `points` and `pace` pass undeclared until
flume reads them (record 0025), and an account's `usage`, which flume
refuses, passes undeclared to be refused (record 0062). It SHALL declare
no word written before the current one, so `host.terminal`, which flume
reads as `multiplexer` and refuses beside it, passes undeclared (record
0055).

#### Scenario: The declared keys

- **WHEN** `nix:test` reads the options `services.flume.settings` declares
- **THEN** `host` declares `multiplexer` and no `terminal`, a stream declares neither `points` nor `pace`, and an account declares `name`, `program`, `env` and `agents` and no `usage`
