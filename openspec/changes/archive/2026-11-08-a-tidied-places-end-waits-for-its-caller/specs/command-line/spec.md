## ADDED Requirements

### Requirement: launch takes a caller and inbox a place

`flume launch` SHALL take `--caller <name>`, and the launch's JSON form
the field `caller`, passed to the core as the launch's. `flume inbox`
SHALL take a place, named as `flume status` names one, beside
`--recipient`. Each command's help SHALL say what it does: that the end
of the place's work, its landing, drop or done, waits for the caller
until it collects it by `flume inbox` or `flume wait`, and that an inbox
naming a place collects that place's ends alone (7, 43, 61; core.35).

#### Scenario: A caller named and its place's end collected

- **WHEN** `flume launch --caller boomdrive --recipient ops` runs over a scratch repository, the place is dropped, and `flume inbox <the place> --recipient boomdrive --json` runs
- **THEN** status gave the caller `boomdrive` before the drop, the inbox gives the end `over/dropped` delivered to `boomdrive` by the inbox, and a second like inbox gives nothing (7, 43, 52)

#### Scenario: The helps

- **WHEN** `flume launch --help` and `flume inbox --help` run
- **THEN** the first names `--caller` and that the end of the place's work waits for it, and the second names the place it may be given
