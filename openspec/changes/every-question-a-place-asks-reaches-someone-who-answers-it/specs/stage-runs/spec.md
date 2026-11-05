## ADDED Requirements

### Requirement: The way to stop short lists the roles an agent may address

The way to stop short in a stage's prompt SHALL list, with its
source's `answers` words, every address the host's sources declare with
`answers`, but a role's run's own: the place's own stream's first, by
the role alone, then every other stream's, by the whole address. It
SHALL be read at each start, and placed after every part the stage's
repetitions share (28, 41, 132, 193; core.31).

#### Scenario: A place of the stream's work

- **WHEN** a place in the stream `east` names `east/planner`, a source declares runs for `east/planner`, `east/design`, `east/ops` and `west/ops`, each with its `answers`, and a stage's prompt is rendered
- **THEN** the way to stop short names `design`, `ops` and `planner` first, each with its source's words, then `west/ops` with its words, after the rest of the prompt

#### Scenario: A role's own run

- **WHEN** a stage of the run of `east/ops` is rendered under the same sources
- **THEN** the way to stop short names `design` and `planner`, then `west/ops`, and not `ops`

#### Scenario: No roles

- **WHEN** a place's recipient is a pane and no source declares a role with `answers`
- **THEN** the way to stop short names no role

### Requirement: A stop-short may name the role it is for

A stop-short SHALL take an optional role or address, `flume needs
[--found] --to <role or address> "<words>"`, among those the stage's
prompt listed: a role alone names that role of the place's own stream,
and an address names a role of any stream. Its end SHALL be delivered
to that address alone and not to the place's recipients, and status
SHALL give the whole address it named. A name outside the list SHALL be
refused, naming the list, and the stage SHALL go on (41, 43, 56, 132).

#### Scenario: A question for its own stream's ops

- **WHEN** a construct of a place in `east`, whose recipient is `east/operator`, stops short naming `ops`, and no run of `east/ops` stands
- **THEN** the stage ends short, the end stands pending for `east/ops` alone, the settle wakes `east/ops`, and `east/operator` is woken for none of it (S92)

#### Scenario: A question for another stream's role

- **WHEN** a construct of a place in `east` stops short naming `west/ops`, which its list gave
- **THEN** the end stands pending for `west/ops` alone, and the settle wakes `west/ops`, whose run runs in `west`

#### Scenario: A name outside the list

- **WHEN** an agent stops short naming `south/ops` or `reviewer`, neither in its list
- **THEN** the stop-short is refused naming the roles and addresses it may name, and the stage goes on

#### Scenario: No address named

- **WHEN** an agent stops short naming none
- **THEN** the end goes to the place's recipients, as before
