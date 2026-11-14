## ADDED Requirements

### Requirement: The managed layer's telemetry table

The managed layer MAY give `[telemetry]`: `send`, a boolean, `false`
where unset; `collector`, the address flume sends to; `service`, a
non-empty string, `flume` where unset; `traces`, `metrics` and `logs`,
each a boolean, `true` where unset; and `[telemetry.attributes]`, each
a string. Status SHALL give each value with its layer (216, C.7).

#### Scenario: A table that sends

- **WHEN** the managed layer gives `[telemetry]` with `send = true`, `collector = "http://localhost:4318/"` and `traces = false`
- **THEN** the host reads telemetry sent to `http://localhost:4318` as the service `flume`, with metrics and logs and no traces, and status gives each value from `managed` (216, C.7)

### Requirement: A telemetry table that does not read is refused

A `[telemetry]` SHALL be refused naming the key, as any key of the
managed layer that does not read is, where it gives a key it does not
read, a value its key does not take, a `send` of `true` with no
collector, an attribute shaped as a credential, or a collector that is
not `http://`, a host of the loopback (`localhost`, an address of
`127.0.0.0/8` or `[::1]`), a port and at most a path (C.7; core.50,
core.56).

#### Scenario: A table that sends names its collector

- **WHEN** the managed layer gives `[telemetry] send = true` with no collector
- **THEN** the configuration is refused naming `[telemetry] collector` and `send = true` (C.7)

#### Scenario: A collector off the loopback

- **WHEN** the managed layer gives `[telemetry]` a collector of `https://localhost:4318`, `http://otel.example:4318`, `http://localhost` or `localhost:4318`
- **THEN** each is refused naming `[telemetry] collector` and what it lacks: plain HTTP, the host's loopback, or a port (C.7; core.56)

#### Scenario: A key telemetry does not read

- **WHEN** the managed layer gives `[telemetry] headers = "x"`, or an attribute `token` whose value is shaped as a credential
- **THEN** the configuration is refused naming `telemetry.headers`, and in the second `telemetry.attributes.token` and never its value (C.7; core.50)

### Requirement: The local layer sets no telemetry

A local layer that sets any key of `[telemetry]` SHALL not be taken,
as for any key outside C.7's list (C.7, 216; core.45).

#### Scenario: The local layer turns telemetry on

- **WHEN** the local layer gives `[telemetry] send = true`
- **THEN** no key of it is taken, status names `telemetry.send`, and nothing is sent (C.7, 216; core.45)
