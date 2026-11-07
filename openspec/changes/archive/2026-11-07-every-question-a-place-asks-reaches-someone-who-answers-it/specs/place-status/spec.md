## MODIFIED Requirements

### Requirement: Status says where each standing address lives

The host's status SHALL give every address a source gives a standing
agent, pending or not, with its agent, stream, session and where that
came from, the stream's terminal, and what the terminal found at the
ask: one agent with its state, none, more than one, or why it could not
read; each terminal and session read once per status, and nothing kept
(56, 132, 190, 193; core.14, core.15).

#### Scenario: An agent found

- **WHEN** the host's status is asked while `madswan/planner`'s standing agent waits in `madswan` and two ends are pending for it
- **THEN** the address gives `madswan-planner`, the stream `madswan` from its address, the session `madswan` from `role`, the agent found waiting, and the two pending ends (56, 132)

#### Scenario: None of the name

- **WHEN** the fake terminal holds no `madswan-planner` in `madswan`
- **THEN** the address's standing says none found, and its pending ends stand beneath (132)

#### Scenario: More than one

- **WHEN** the fake terminal holds two `madswan-planner` in `madswan`
- **THEN** the address's standing says two found (132)

#### Scenario: A part not read

- **WHEN** the fake terminal refuses to read `madswan`'s agents with its words
- **THEN** the address's standing says it could not read, with those words (132)

#### Scenario: Nothing pending

- **WHEN** a source gives `madswan/ops` a standing agent and no end is pending for it
- **THEN** the host's status still gives `madswan/ops` with its standing (56)

#### Scenario: Two addresses in one session

- **WHEN** `madswan/planner` and `madswan/ops` each name an agent in `madswan`, and the host's status is asked
- **THEN** the fake terminal is asked for `madswan`'s agents once (core.15)

#### Scenario: The fields kept

- **WHEN** the suite's field list is read after this change
- **THEN** it holds each address's standing with its agent, stream, session and its `from`, terminal and what was found, and the wake's held answer, and an answer lacking one fails the suite (61; core.14)

## ADDED Requirements

### Requirement: Status gives who answers a place

Status SHALL give each place's recipients, each with where it came
from; the role a stop-short named; each end's key, and a stop-short's
answer with who gave it and when; and a hold on the waiting move with
its words, since when, its bound or that it lapsed (56, 130, 193).

#### Scenario: Recipients and their sources

- **WHEN** one place names `ops` at its launch and another took its stream's answerer `east/operator`
- **THEN** status gives the first `ops` from the launch and the second `east/operator` from the stream's answerer, declared by the source `acme`, in the JSON form and the person's

#### Scenario: An answered question

- **WHEN** a construct's stop-short in `east`, naming `ops`, was answered by a retry naming its key from the run of `east/ops`
- **THEN** status gives the end's key, `east/ops` as its addressee, and the answer's words with `east/ops` and when

### Requirement: Status gives the roles a host wakes

Status SHALL give, for each stream, its answerer with the source that
declares it; each source's roles, or why they cannot be read; and for
each address an end stands pending for, its run, or that it is carried
or wakes nothing and why (56, 132, 191, 193).

#### Scenario: An address that wakes nothing

- **WHEN** an end stands pending for `east/planner`, which no source declares, on a place whose stream's answerer is `east/operator`
- **THEN** status gives the address with no run and that no source declares it, and the stream's answerer `east/operator`
