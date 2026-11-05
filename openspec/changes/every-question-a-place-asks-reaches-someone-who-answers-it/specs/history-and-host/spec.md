## ADDED Requirements

### Requirement: The host's answerer

The host's facts SHALL give, as its configuration stands when asked, the
answerer `[host] answerer` names and each stream's `answerer`, each with
the key it came from, and none where the configuration names none; a
value that is no role's address SHALL refuse the launches that would
take it, naming the key; none SHALL be kept between asks (143, 190,
C.7).

#### Scenario: A stream's answerer and the host's

- **WHEN** the configuration gives `[host] answerer = "ops/answerer"` and `[streams.work] answerer = "work/ops"`
- **THEN** the host names `work/ops` for `work`, from the stream, and `ops/answerer` for `default`, from the host

#### Scenario: No answerer

- **WHEN** the configuration names no answerer
- **THEN** the host names none, for every stream

#### Scenario: An answerer that is no address

- **WHEN** the configuration gives `[host] answerer = "ops"`
- **THEN** a launch that would take it is refused naming `[host] answerer` and that `ops` is no `<partition>/<role>`
