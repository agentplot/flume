## ADDED Requirements

### Requirement: inbox takes a place

`flume inbox` SHALL take a place, named as `flume status` names one,
beside `--recipient`, and its help SHALL say that an inbox naming a
place collects that place's ends alone. `flume launch` SHALL take no
`--caller` (7, 43, 61; core.35).

#### Scenario: A reader's place collected

- **WHEN** `flume launch --recipient boomdrive --recipient ops` runs over a scratch repository, the place is dropped, and `flume inbox <the place> --recipient boomdrive --json` runs
- **THEN** status gave the recipients `boomdrive` and `ops` before the drop, the inbox gives the end `over/dropped` among the place's ends, each delivered to `boomdrive` by the inbox, and a second like inbox gives nothing (7, 43, 52)

#### Scenario: The helps

- **WHEN** `flume launch --help` and `flume inbox --help` run, and `flume launch` runs with `--caller boomdrive`
- **THEN** the first names no `--caller`, the second names the place it may be given, and the launch is refused naming the argument, nothing made

## REMOVED Requirements

### Requirement: launch takes a caller and inbox a place

**Reason**: A launch names no caller (record 0060, rewritten); "inbox takes a place" keeps the inbox's place.

**Migration**: Name the reader a recipient, `--recipient <name>`.
