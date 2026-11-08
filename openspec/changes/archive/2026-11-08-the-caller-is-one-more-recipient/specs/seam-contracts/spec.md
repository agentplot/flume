## ADDED Requirements

### Requirement: The type of a drop's or a done's end

The held state SHALL hold an end of `of` `over` for a drop or a done,
holding the over it records, keyed `over/dropped` or `over/done`; it
SHALL serialize to JSON and back unchanged, and its key SHALL be read
back from its token alone. A place's record SHALL hold no caller, and
held state that holds one SHALL read with it ignored (43, 52, 61, 75,
114).

#### Scenario: A drop round-trips

- **WHEN** held state holding the end of a drop is written and read back
- **THEN** it reads back equal, and the drop's end keys `over/dropped`, which reads back to the same key, while `over/landed` is refused naming the key's forms (43, 61, 75)

#### Scenario: State written with a caller

- **WHEN** held state whose place holds a `caller` field is read
- **THEN** it reads with the field ignored, and is written back without it (75)

### Requirement: Whom a drop's or a done's end is pending for

A drop's or a done's end SHALL stand pending for each recipient of the
place that is no role's address and has no delivery of it recorded, no
notice SHALL carry it, and no wake SHALL be owed for it. Every other end
SHALL stand pending for its recipients as before (43, 132).

#### Scenario: Whom each end is pending for

- **WHEN** a place's recipients are `east/planner` and `boomdrive`, and it holds a code delivered, a landing and a drop, none delivered
- **THEN** code's end and the landing stand pending for `east/planner` and `boomdrive`, the drop for `boomdrive` alone, and the drop is the one end no notice carries (43)

## REMOVED Requirements

### Requirement: The types of a caller and of the end of a place's work

**Reason**: A place holds no caller (record 0060, rewritten); the drop's end's type is "The type of a drop's or a done's end".

**Migration**: None: held state that holds a caller reads with it ignored.

### Requirement: Whom the end of a place's work is pending for

**Reason**: Replaced by "Whom a drop's or a done's end is pending for": the end of a place's work is owed to its recipients alone.

**Migration**: None.
