## ADDED Requirements

### Requirement: The fake telemetry keeps every signal

There SHALL be a fake of the telemetry the core hands its signals to: a
test SHALL set which signals it sends, none by default, and SHALL read
back every signal the core handed it, in order; and the fake host SHALL
script the managed layer's `[telemetry]` (216, 217).

#### Scenario: A fake that sends nothing

- **WHEN** a place runs to its landing over fakes whose telemetry sends nothing
- **THEN** the fake keeps the signals the core handed it, and the fake terminal's opens carry no `OTEL_RESOURCE_ATTRIBUTES` or `TRACEPARENT` of flume's (216, 219)

#### Scenario: A fake that sends traces

- **WHEN** a stage starts over fakes whose telemetry sends traces
- **THEN** the fake terminal's open carries a `TRACEPARENT` naming the span the fake telemetry was later handed for that start (217, 219)
