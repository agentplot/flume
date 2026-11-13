## ADDED Requirements

### Requirement: The fake store moves a document while it reads every place, and refuses one place's write

The fake store SHALL run an act once inside its next read of every
place, after it lists the documents standing and before it reads each,
so a document the act writes is not listed and one it removes reads as
none; and SHALL refuse the next write of a place it is told, once, as
moved, writing nothing (75, 102, 103).

#### Scenario: A document moved inside a read of every place

- **WHEN** the act armed for the next read of every place writes a place's kept document and removes the place's own
- **THEN** that read holds neither, and the next read holds the kept one

#### Scenario: A place's write refused

- **WHEN** the next write of a place's kept document is refused and a tidy writes it
- **THEN** the write is refused as moved, nothing is written, and the next write of it lands
