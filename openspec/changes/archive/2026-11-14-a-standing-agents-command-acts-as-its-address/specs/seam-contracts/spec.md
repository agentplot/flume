## ADDED Requirements

### Requirement: The type of a recipient a standing agent's launch gave

`flume-seams` SHALL hold `RecipientsFrom::Standing`, with the standing
agent's name, `agent`, and the source that declares it, `source`,
serialized with `from` `standing`, written and read back by its round
trip beside the variants before it, and a held state written before
reading with the recipients' source it held (7, 61, 132; core.14).

#### Scenario: Round trips

- **WHEN** `round_trip.rs` writes a held state whose recipients came from the standing agent `east-planner`, declared by `acme`, and reads it back, beside each variant before it
- **THEN** each reads as written, the standing one's JSON giving `from` `standing`, `agent` `east-planner` and `source` `acme`, and a held state written before reads with the source it held (61)
