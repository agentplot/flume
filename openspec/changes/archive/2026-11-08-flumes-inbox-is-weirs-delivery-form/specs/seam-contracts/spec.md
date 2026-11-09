## MODIFIED Requirements

### Requirement: Whom a drop's or a done's end is pending for

A drop's or a done's end SHALL have as its recipients each recipient of
the place that is no role's address, no notice SHALL carry it, and no
wake SHALL be owed for it. Every other end SHALL have its recipients as
before. An end SHALL stand pending for each of its recipients that has
not confirmed its delivery (43, 132; weir 102).

#### Scenario: Whom each end is pending for

- **WHEN** a place's recipients are `east/planner` and `boomdrive`, and it holds a code delivered, a landing and a drop, none confirmed
- **THEN** code's end and the landing stand pending for `east/planner` and `boomdrive`, the drop for `boomdrive` alone, and the drop is the one end no notice carries (43)

## ADDED Requirements

### Requirement: The types of the host's deliveries and a place's token

`flume-seams` SHALL hold `HostKey::Deliveries` and
`HostDocument::Deliveries`, its records each a `DeliveryRecord` written
and read through `weir-delivery`'s JSON; `PlaceRecord::token`;
`EndHeld::noticing`, each notice claimed with its recipient and when;
and `DeliveredBy::SessionStart`; each written and read back by its round
trip, and a held state written before reading with no token, no claim
and its old `deliveries` as written (43, 61, C.7; weir C.8; core.4).

#### Scenario: Round trips

- **WHEN** `round_trip.rs` writes a deliveries document holding one record of each of weir's four formats, a place's record with a token, an end with a notice claimed, and a delivery by `session_start`, and reads each back
- **THEN** each reads as written, the document's records spelled as weir's `json::write` spells them

#### Scenario: A record weir refuses

- **WHEN** a deliveries document holding a `weir-delivery/1` record with no reader is read
- **THEN** it is refused naming the field `readers`, in weir's words (weir 100)

#### Scenario: State written before

- **WHEN** held state written before this change, an end holding deliveries by notice and by the inbox, is read and written back
- **THEN** it reads with no token and no claim, and its deliveries read and write back as they were (61, 75)
