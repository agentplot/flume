## ADDED Requirements

### Requirement: What flume sets for an agent's telemetry is in no record

`OTEL_RESOURCE_ATTRIBUTES` and `TRACEPARENT` as flume sets them for an
agent SHALL be in no start's record and SHALL never be compared, so
whether flume sends changes no started ack; a variable of either name
that the start's account gives SHALL be recorded as any account's
variable is (161, 216, 219; core.10).

#### Scenario: The ack taken whatever telemetry gave

- **WHEN** a start runs on a host that sends, and its agent's word that it started reports `OTEL_RESOURCE_ATTRIBUTES` and `TRACEPARENT` as flume set them, and another start's word reports neither
- **THEN** both starts' records name neither variable, and each word is taken (161, 216)
