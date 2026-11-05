## ADDED Requirements

### Requirement: The host's answerer

The host's facts SHALL give, as its configuration stands when asked, the
answerer `[host] answerer` names and each stream's `answerer`, each with
the key it came from, and none where the configuration names none; a
value that is no role's address SHALL refuse the launches that would
take it, naming the key; none SHALL be kept between asks (143, 191,
C.7).

#### Scenario: A stream's answerer and the host's

- **WHEN** the configuration gives `[host] answerer = "acme/operator"` and `[streams.work] answerer = "work/operator"`
- **THEN** the host names `work/operator` for `work`, from the stream, and `acme/operator` for `default`, from the host

#### Scenario: No answerer

- **WHEN** the configuration names no answerer
- **THEN** the host names none, for every stream

#### Scenario: An answerer that is no address

- **WHEN** the configuration gives `[host] answerer = "ops"`
- **THEN** a launch that would take it is refused naming `[host] answerer` and that `ops` is no `<partition>/<role>`

### Requirement: The roles a partition answers with

The host's facts SHALL give, for a partition, every address of it that
`[roles]` names a run for or carries, each with its `answers` words, as
the configuration stands when asked (28, 41, 132, C.7).

#### Scenario: A partition's roles

- **WHEN** `[roles]` names runs for `acme/planner` and `acme/ops`, each with its `answers`, carries `acme/design`, and names `other/ops`
- **THEN** the host gives `acme`'s roles as `acme/design`, `acme/ops` and `acme/planner`, each with its words, and not `other/ops`
