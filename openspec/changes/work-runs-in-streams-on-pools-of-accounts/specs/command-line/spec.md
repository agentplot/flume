## ADDED Requirements

### Requirement: launch takes a stream and an intent

`flume launch` SHALL take `--stream <stream>` and `--intent <name>`, and
the launch's JSON form and the MCP launch tool the same two fields,
each passed to the core as the launch's; the binary SHALL choose
neither itself (7, 143, 147).

#### Scenario: A stream and an intent named

- **WHEN** `flume launch` runs with `--stream personal --intent the-surfaces` over a scratch repository on a host declaring `personal`
- **THEN** the place's status names `personal` from `launch` and the intent `the-surfaces` from `launch` (143, 147)
