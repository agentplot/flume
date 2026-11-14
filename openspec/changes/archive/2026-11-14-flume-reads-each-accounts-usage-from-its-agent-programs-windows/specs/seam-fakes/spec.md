## ADDED Requirements

### Requirement: The fake agent scripts the windows its program reports

The fake agent SHALL keep, for a start a test names, the windows its
program reports with when they were taken, read by `windows` in the
folder the start recorded with the session the test gave the start, the
later of two reports standing, log each `windows` read, and report no
windows where the test says its profile builds none; it SHALL pass the
agent seam's windows scenarios both ways (106, 211).

#### Scenario: A scripted report

- **WHEN** a test scripts a start's program to report `five_hour` 42% used taken at 14:02, then 50% used taken at 14:05
- **THEN** `windows` reads the 14:05 report with the start's session, and the fake's log shows each `windows` read (211)

#### Scenario: A profile scripted to report none

- **WHEN** a test scripts the fake agent's profile to build no windows
- **THEN** `reports_windows` answers not offered, and `windows` reads unknown for every start (171, 174)

## REMOVED Requirements

### Requirement: The fake host scripts each account's reading

**Reason**: The host gives no reading: an account's reading is the
windows its agent program reports, kept in the account's own document
(184, 211; record 0062).

**Migration**: A test scripts the fake agent's reported windows and
takes the reading, or writes the reading into the account's document
through the fake store.
