## ADDED Requirements

### Requirement: settings declares the telemetry table

`services.flume.settings` SHALL declare `telemetry`: `send`, `traces`,
`metrics` and `logs`, each a boolean; `collector`, a string of
`http://`, a host of the loopback, a port and at most a path;
`service`, a string; and `attributes`, a table of strings; each null
unless set and left out of the file (216, C.7; record 0055).

#### Scenario: A collector off the loopback

- **WHEN** `nix:test` evaluates the options with `settings.telemetry.collector = "http://otel.example:4318"`
- **THEN** evaluation fails naming `services.flume.settings.telemetry.collector`

#### Scenario: The table rendered and read whole

- **WHEN** `nix:test` renders `settings` with `telemetry` sending to `http://localhost:4318` with an attribute
- **THEN** the file holds `[telemetry]` and `[telemetry.attributes]` as set, and flume's reader of the host's configuration reads it with no fault
