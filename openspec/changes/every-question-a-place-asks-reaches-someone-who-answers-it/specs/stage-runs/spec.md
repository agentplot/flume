## ADDED Requirements

### Requirement: The way to stop short lists the roles an agent may address

The way to stop short in a stage's prompt SHALL list each role's
address the agent may name, with the `answers` words its source gives
for it: every address a source of the host declares with `answers`, a
run's or a carried one's, in the partition of the place's recipients'
addresses, else of the answerer behind the place's stream; read at each
start, and placed after every part the stage's repetitions share. Where
there is none, it SHALL list none (28, 41, 132, 193; core.31).

#### Scenario: A place of crew's work

- **WHEN** a place names `acme/planner`, a source declares runs for `acme/planner`, `acme/design` and `acme/ops`, each with its `answers`, and a stage's prompt is rendered
- **THEN** the way to stop short names the three addresses, each with its source's words, after the rest of the prompt

#### Scenario: A person's pane

- **WHEN** a place's one recipient is a pane, its stream's answerer is `acme/operator`, and a stage's prompt is rendered
- **THEN** the way to stop short names `acme`'s roles

#### Scenario: No roles

- **WHEN** a place's recipient is a pane and no role stands behind its stream
- **THEN** the way to stop short names no role

### Requirement: A stop-short may name the role it is for

A stop-short SHALL take an optional address, `flume needs [--found]
--to <address> "<words>"`, among those the stage's prompt listed. Its
end SHALL be delivered to that address alone and not to the place's
recipients, and status SHALL give whom it named. A name outside the
list SHALL be refused, naming the list, and the stage SHALL go on (41,
43, 56, 132).

#### Scenario: A question for ops

- **WHEN** a construct of a place whose recipient is `acme/operator` stops short naming `acme/ops`, and no run of `acme/ops` stands
- **THEN** the stage ends short, the end stands pending for `acme/ops` alone, the settle wakes `acme/ops`, and `acme/operator` is woken for none of it (S92)

#### Scenario: An address outside the list

- **WHEN** an agent stops short naming `other/ops`, which is not in its list
- **THEN** the stop-short is refused naming the addresses it may name, and the stage goes on

#### Scenario: No address named

- **WHEN** an agent stops short naming none
- **THEN** the end goes to the place's recipients, as before
