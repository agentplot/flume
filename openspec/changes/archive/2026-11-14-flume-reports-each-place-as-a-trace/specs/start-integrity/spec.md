## ADDED Requirements

### Requirement: What flume sets for an agent's telemetry is in no record

`OTEL_RESOURCE_ATTRIBUTES` and `TRACEPARENT` as flume sets them for an
agent SHALL be in no start's record and SHALL never be compared, so
whether flume sends changes no started ack; a variable of either name
that the start's account gives SHALL be in no start's record either,
since flume rewrites the first while it sends, and SHALL be named with
its layer among the start's settings as any account's variable is
(161, 216, 219; core.10, core.30; record 0067).

#### Scenario: The ack taken whatever telemetry gave

- **WHEN** a start runs on a host that sends, and its agent's word that it started reports `OTEL_RESOURCE_ATTRIBUTES` and `TRACEPARENT` as flume set them, and another start's word reports neither
- **THEN** both starts' records name neither variable, and each word is taken (161, 216)

#### Scenario: An account's variable of either name is in no record

- **WHEN** a start's account gives `OTEL_RESOURCE_ATTRIBUTES` as `team=ops` and a `TRACEPARENT`, beside a variable of its own
- **THEN** the start's record names the account and holds its own variable's digest and neither of the two (161, 219)
