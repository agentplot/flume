## MODIFIED Requirements

### Requirement: inbox takes a place

`flume inbox` SHALL take a place, named as `flume status` names one,
beside `--recipient` and `--confirm`, and its help SHALL say that an
inbox naming a place collects that place's ends alone, and that an end
it gives stays pending until its recipient confirms it. `flume launch`
SHALL take no `--caller` (7, 43, 61; core.35).

#### Scenario: A reader's place collected

- **WHEN** `flume launch --recipient boomdrive --recipient ops` runs over a scratch repository, the place is dropped, `flume inbox <the place> --recipient boomdrive --json` runs, then `flume inbox <the place> --recipient boomdrive --confirm --json`
- **THEN** status gave the recipients `boomdrive` and `ops` before the drop; the first inbox gives the end `over/dropped` among the place's ends, each handed to `boomdrive` by the inbox and pending for it; the second gives them again, each confirmed; and a third gives nothing (7, 43, 52)

#### Scenario: The helps

- **WHEN** `flume launch --help` and `flume inbox --help` run, and `flume launch` runs with `--caller boomdrive`
- **THEN** the first names no `--caller`, the second names the place it may be given and `--confirm`, and the launch is refused naming the argument, nothing made

## ADDED Requirements

### Requirement: confirm, and wait --confirm

`flume confirm <KEY>...` SHALL confirm each delivery key as the
recipient it names by `--recipient`, else as `flume inbox`'s default,
answering in the JSON form each key with when it was confirmed and
whether it was confirmed before, and exiting non-zero on a refusal;
`flume wait` SHALL take `--confirm`. Each help SHALL say a hand-over
delivers nothing until its recipient confirms it (43, 61; core.35).

#### Scenario: A key confirmed through the binary

- **WHEN** a fix launched with `--recipient boomdrive` over a scratch repository is dropped, `flume inbox <the place> --recipient boomdrive --json` gives its end's delivery key, and `flume confirm <the key> --recipient boomdrive --json` runs twice
- **THEN** the first answer gives the key confirmed with when and exits 0, the second gives it confirmed before with the first time, and the next `flume inbox` gives nothing

#### Scenario: A key refused through the binary

- **WHEN** `flume confirm no/such/key --recipient boomdrive --json` runs
- **THEN** it is refused naming the key, in the JSON form, and exits non-zero

#### Scenario: The helps of confirm and wait

- **WHEN** `flume confirm --help` and `flume wait --help` run
- **THEN** the first names the keys and `--recipient`, the second names `--confirm`, and each says a hand-over delivers nothing until its recipient confirms it
