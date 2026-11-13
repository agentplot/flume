## ADDED Requirements

### Requirement: A notice's claim says whether its send is unanswered

`NoticeClaim` SHALL hold `sending`, true from the claim until the
terminal answers its send sent and written only while true, so a claim
written before, which has none, reads answered (43, 61, 75).

#### Scenario: A claim sending and a claim answered

- **WHEN** `round_trip.rs` writes an end with a notice claimed answered and one claimed sending, and reads each back
- **THEN** each reads as written, and the answered claim is written with no `sending`, as a claim written before
