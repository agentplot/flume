## ADDED Requirements

### Requirement: The types of a caller and of the end of a place's work

The held state SHALL hold a place's `caller`, the name its launch gave,
and an end of `of` `over` for a drop or a done, holding the over it
records, keyed `over/dropped` or `over/done`. Each SHALL serialize to
JSON and back unchanged, a key SHALL be read back from its token alone,
and held state written before SHALL read with no caller and each of its
ends keyed as it was (7, 43, 52, 61, 75, 114).

#### Scenario: A caller and a drop round-trip

- **WHEN** held state whose place names the caller `boomdrive` and holds the end of a drop is written and read back
- **THEN** it reads back equal, and the drop's end keys `over/dropped`, which reads back to the same key, while `over/landed` is refused naming the key's forms (43, 61, 75)

#### Scenario: State written before

- **WHEN** held state written before this change is read
- **THEN** its place has no caller, and each of its ends keys and stands pending as it did (75)

### Requirement: Whom the end of a place's work is pending for

The end of a place's work, its submission's landing or its over end,
SHALL stand pending for the place's caller until a delivery to the
caller's name by the inbox or a wait, a notice's delivery to that name
counting for none. An over end SHALL have no recipient but the caller,
and every other end SHALL stand pending for its recipients alone, as
before (7, 43, 62).

#### Scenario: The end of the work owed its caller

- **WHEN** a place's recipients are `ops`, its caller is `boomdrive`, and it holds a construct delivered, a failed submission, a landing and a drop, none delivered
- **THEN** the construct and the failure stand pending for `ops` alone, the landing for `ops` and `boomdrive`, and the drop for `boomdrive` alone (43)

#### Scenario: A caller that is also a recipient

- **WHEN** a place's recipient and caller are both `ops`, and its landing is delivered to `ops` by a notice
- **THEN** the landing still stands pending for `ops`, and once delivered to `ops` by the inbox it stands pending for no one (43, 62)
