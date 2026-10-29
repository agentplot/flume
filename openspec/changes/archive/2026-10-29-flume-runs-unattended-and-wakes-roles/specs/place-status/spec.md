## ADDED Requirements

### Requirement: Status gives what waits, what was made and what is pending

Status SHALL give, beside clause 56's other facts, each place's
recipients; each end with whom it was delivered to and when, and for
whom it stands pending; for a place waiting on its caller, the move,
since when, a hold's words and whether it is stalled; a role run's
address; each stage's made lines as written; and the ends pending for
an address the host names no role for (43, 56, 129, 130, 132).

#### Scenario: A stalled place with a pending end

- **WHEN** status is asked of a place stalled on a retry of construct whose end is delivered to `ops` and pending for `madswan/planner`
- **THEN** the answer gives the move, since when, stalled, the end delivered to `ops` with when, and pending for `madswan/planner`

#### Scenario: The fields kept

- **WHEN** the suite's field list is read after this change
- **THEN** it holds the recipients, the ends with their deliveries, the waiting move, the hold's words, stalled, the role address and the made lines, and an answer lacking one fails the suite (61)
