## ADDED Requirements

### Requirement: The agent seam reads the windows its program reported

The agent seam SHALL give, for an agent read in the place's own folder,
the usage windows its program last reported for the account it runs
under, with when flume's own part of the program took them and the
session they came from where its record names one, or none yet, or
unknown; and SHALL say whether its profile builds that part. Both
SHALL be unknown and not offered by default, so a program whose profile
builds none is degraded there (171, 174, 211).

#### Scenario: Windows over the seam's scenarios

- **WHEN** the agent seam's scenarios run against a profile that reports windows, for an agent that has reported none, then a report taken at 14:02, then one taken at 14:05
- **THEN** `reports_windows` answers offered, and `windows` reads none yet, then the 14:02 report, then the 14:05 one, each with the agent's session, and unknown read in another place's folder (211)

#### Scenario: A profile that reports no windows

- **WHEN** the seam's scenarios run against a profile that builds no windows
- **THEN** `reports_windows` answers not offered, and `windows` reads unknown for any agent (171, 174)

## MODIFIED Requirements

### Requirement: The types of a reading and a move past a threshold

The seams SHALL pass one type each for a reading, with when flume took
it, the agent and session it came from and its windows, for an
account's document of its spent time and its reading, for a pool's
threshold, the strategy `most-left`, and a move whose `why` is
`threshold` with its window and end; each SHALL serialize to JSON and
back unchanged, a reading's times as RFC 3339's with their offset and
each window's use as a share from 0 to 100; an account's document an
earlier release wrote SHALL read as spent with no reading, and a move
recorded before SHALL read with no window and no end (147, 153, 184,
185, C.7; core.14).

#### Scenario: A pool and a reading

- **WHEN** a most-left pool with a threshold of 85, and a reading taken from an agent's session of two windows, one 23.5% used with no reset, are written and read
- **THEN** each reads back unchanged, the strategy spelled `most-left`, the reading's `taken_at` RFC 3339's and the window's `used` 23.5 (184, 185, C.7)

#### Scenario: An account's document

- **WHEN** an account's document holding a spent time and a reading is written and read, and then one an earlier release wrote, holding `until`, `words` and `recorded_at` alone
- **THEN** the first reads back unchanged, and the second reads spent until its `until` with no reading (153, C.7)

#### Scenario: A move past a threshold

- **WHEN** an intent's move with `why` `threshold`, `window` `5-hour` and `until` 17:00 is written and read, and a move written before with neither
- **THEN** the first reads back unchanged and the second with no window and no end (147, 185)

### Requirement: The store keeps the host's own documents

The store seam SHALL read and write the host's own documents, each
account's spent time and reading in one document and each intent's
affinity, by key, compare-and-set on a version as a place's held state
is, and reached by every flume process on the host (102 to 104, 153).

#### Scenario: A host document written at a moved version

- **WHEN** two writers write one key at the version both read
- **THEN** one write lands and the other is refused as moved (103, 153)

## REMOVED Requirements

### Requirement: The host gives an account's reading

**Reason**: No host writes a reading: an account's reading is the
windows its agent program reports in a session flume started, kept in
the account's own document (184, 211; record 0062).

**Migration**: The core reads the reading from the account's document
through the store seam, and the agent seam's windows give what the
program reported; an account's `usage` is refused as no word of an
account.
