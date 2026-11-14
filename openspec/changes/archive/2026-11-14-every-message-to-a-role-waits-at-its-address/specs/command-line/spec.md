## RENAMED Requirements

- FROM: `### Requirement: needs --to, hold --waiting, retry --answers and tell --answers`
- TO: `### Requirement: needs --to, hold --waiting, retry --answers and message --answers`

## MODIFIED Requirements

### Requirement: Every command answers from the one core

The binary SHALL offer `launch`, `status`, `list`, `prompt`, `message`,
`stop`, `retry`, `finish`, `drop`, `done`, `tidy`, `settle`, `needs`,
`priority` and `open`, each building the core over the host's profiles
and answering the core's answer, printed for a person or as its JSON
with `--json`, exiting non-zero on a refusal; no command SHALL compute a
stage's end, a status, a submission or a tidy of its own (56 to 61,
186, 200, section 9, core.14).

#### Scenario: The same answer through the command line and the core

- **WHEN** `flume status <place> --json` runs over a scratch repository and the core is asked the same in a test over the same state
- **THEN** the two answers are equal (S14, 60)

#### Scenario: A refusal

- **WHEN** `flume done` names a fix place
- **THEN** the JSON is a refusal naming the place and the exit is non-zero (S26, 61)

#### Scenario: The list through the command line and the core

- **WHEN** `flume list --json` runs over a scratch host and the core is asked the list in a test over the same state
- **THEN** the two answers are equal (60, 200)

### Requirement: needs --to, hold --waiting, retry --answers and message --answers

The command line SHALL take `flume needs [--found] --to <role or
address> "<words>"`, `flume hold <place> <words> --waiting [--for
<time>]` with the time in the forms `--stall-after` takes, `flume retry
<place> --answers <key>` and `flume message <place> --answers <key>
"<words>"`, each answering from the one core in the JSON form and the
person's, and each key as the inbox and status give it (41, 42, 49, 53,
61, 130).

#### Scenario: The forms

- **WHEN** `flume hold change--x "on the user's card" --waiting --for 1h --json` and `flume retry change--x --stage construct --answers stage/construct/1/ended-short --words "yes" --json` run in turn against the fakes
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

#### Scenario: A tell's key

- **WHEN** `flume tell change--x --answers stage/construct/1/ended-short "yes" --json` runs against the fakes, the place's construct agent kept open for that stop-short
- **THEN** it answers in the JSON form as `flume message` does, the words delivered as the answer to that stop-short

#### Scenario: A message's key

- **WHEN** `flume message change--x --answers stage/construct/1/ended-short "yes" --json` runs against the fakes, the place's construct agent kept open for that stop-short
- **THEN** it answers in the JSON form, its `act` `message`, the message delivered as the answer to that stop-short

#### Scenario: A key in a wrong form

- **WHEN** `flume retry change--x --answers construct` runs
- **THEN** it is refused naming the forms a key takes, and no agent is ended

## ADDED Requirements

### Requirement: tell is message's alias

`flume tell <place> <words>`, with any flag `flume message` takes, SHALL
run as `flume message` and answer as it does; `flume --help` SHALL list
`message` and not `tell` (53; record 0069).

#### Scenario: A caller not yet moved

- **WHEN** `flume tell change--x "go on" --json` runs against the fakes, the place's agent waiting
- **THEN** the agent receives "go on", the answer is the one `flume message change--x "go on" --json` gives, its `act` `message`, and `flume --help` names `message` alone

### Requirement: message to an address answers with its key

`flume message <stream>/<role> "<words>"` SHALL answer, in the JSON form
and the person's, the delivery's key, the address and what the act
found of the address: its run woken, live or waiting for room, its
standing agent, carried, or its batch handed (53, 61, 214).

#### Scenario: Through the binary

- **WHEN** `flume message east/planner "look at the nightly job" --json` runs against the fakes, the address's run live and working
- **THEN** the JSON gives the key `message/east/planner/<token>`, the address `east/planner` and its run live, and the exit is 0
