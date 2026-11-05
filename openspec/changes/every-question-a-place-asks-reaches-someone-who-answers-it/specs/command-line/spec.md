## ADDED Requirements

### Requirement: needs --to, hold --waiting and retry --answers

The command line SHALL take `flume needs [--found] --to <role or
address> "<words>"`, `flume hold <place> <words> --waiting [--for
<time>]` with the time in the forms `--stall-after` takes, and `flume
retry <place> --answers <key>`, each answering from the one core in the
JSON form and the person's, and each key as the inbox and status give
it (41, 42, 49, 61, 130).

#### Scenario: The forms

- **WHEN** `flume hold change--x "asked the user, msg_1" --waiting --for 1h --json` and `flume retry change--x --stage construct --answers stage/construct/1/ended-short --words "yes" --json` run in turn against the fakes
- **THEN** each answers in the JSON form, the hold with its words and bound, and the retry with the place's status

#### Scenario: A stop-short addressed by its role

- **WHEN** a stage's agent of a place in `east` runs `flume needs --to ops "which queue's settings are live?"` against the fakes, with `ops` in its list
- **THEN** the stop-short is taken naming `east/ops`, and its answer says so

#### Scenario: A stop-short addressed to another stream

- **WHEN** the same agent runs `flume needs --to west/ops "which queue's settings are live?"`, with `west/ops` in its list
- **THEN** the stop-short is taken naming `west/ops`, and its answer says so

#### Scenario: A hold's bound in a wrong form

- **WHEN** `flume hold change--x "words" --waiting --for soon` runs
- **THEN** it is refused naming the forms a time takes, and nothing is held

#### Scenario: A key in a wrong form

- **WHEN** `flume retry change--x --answers construct` runs
- **THEN** it is refused naming the forms a key takes, and no agent is ended

### Requirement: wake names an address of a stream

`flume wake <address>`'s help and its refusal of a name of another form
SHALL give the address as `<stream>/<role>` (61, 132; core.35).

#### Scenario: The help

- **WHEN** `flume wake --help` runs
- **THEN** it names the address `<stream>/<role>`, and the word partition nowhere
