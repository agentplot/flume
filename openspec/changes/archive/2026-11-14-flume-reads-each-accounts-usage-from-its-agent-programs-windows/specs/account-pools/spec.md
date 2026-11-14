## MODIFIED Requirements

### Requirement: A most-left pool chooses the account with the most left

Under most-left flume SHALL choose the usable account with the most
left, the least that any window of its fresh reading leaves, then each
usable account with no fresh reading, equals in either by fewest of
flume's agents running and then in the pool's order (145, 184).

#### Scenario: Four places of four intents

- **WHEN** a most-left pool holds work-1, reading 40% and 80% used two minutes ago, work-2, reading 60% and 30%, and work-3, with no reading, no session of flume's having reported for it, and four places of four intents start in turn: the second while work-2's reading is unchanged, the third once it is twenty minutes old, the fourth once work-1 is spent
- **THEN** the first and second start on work-2, the third on work-1, and the fourth on work-3, which runs fewer of flume's agents than work-2 (S86)

#### Scenario: No reading anywhere

- **WHEN** a most-left pool's two accounts give no reading, and three stages of three intents start
- **THEN** they alternate first, second, first, as least-busy's do (145, S54)

#### Scenario: The affinity first

- **WHEN** an intent's affinity for `agent` is work-1, usable, and work-2's reading leaves more
- **THEN** the intent's next start on `agent` stays on work-1, from the affinity (147)

## ADDED Requirements

### Requirement: An account's reading is what its program reported

An account's reading SHALL be the windows its agent program last
reported for it in a session flume started, kept in the account's own
document and asked of no program, maker or host's file. It SHALL be
fresh for fifteen minutes from when flume took it, a window past its
reset reading as nothing used; an account with none SHALL be refused
nothing, status saying why (146, 153, 171, 184, 211; core.32).

#### Scenario: A fresh reading

- **WHEN** work-1's reading, taken two minutes ago, gives its `five_hour` window 40% used and its `seven_day` window 80%
- **THEN** status gives both windows with their resets and the reading fresh, and work-1 has 20 left (184)

#### Scenario: A stale reading

- **WHEN** work-1's reading was taken twenty minutes ago and no session has reported for it since
- **THEN** status gives it stale, and a most-left pool ranks work-1 with the accounts that have no reading (184)

#### Scenario: A window past its reset

- **WHEN** work-1's reading gives its `five_hour` window 95% used, resetting an hour ago
- **THEN** the window reads as nothing used, and work-1 is not past its threshold (184, 185)

#### Scenario: An account no session reported for

- **WHEN** work-3's document holds no reading, and its program's profile reports windows
- **THEN** work-3 has no reading, status gives `no session flume started has reported its windows`, and a start on its pool is not refused (184)

#### Scenario: A program whose profile gives no windows

- **WHEN** a pool's other-1 runs a program whose profile reports no windows
- **THEN** other-1 has no reading, status gives `its program's profile gives no windows`, and a start of that program on the pool is taken (171, 174, 184)

#### Scenario: An account flume runs no agent on

- **WHEN** work-2's reading was taken three hours ago, no agent of flume's has run on it since, and its `seven_day` window is 95% used, resetting Monday at 08:00
- **THEN** status gives the reading stale, and work-2 takes no start until Monday at 08:00 (184, 185)

#### Scenario: Nothing asked of the program

- **WHEN** flume chooses an account of a most-left pool whose accounts hold readings, and status is asked
- **THEN** each reading is as the account's document holds it, and the fake agent's log shows no `windows` read by the choice or by status (184; core.32)

### Requirement: A live start's report is taken as its account's reading

The core SHALL take the windows a live current start's program
reported, asked with that start's secret, as the reading of the account
the start runs under, with when they were taken, the start's identity
and its session, by compare-and-set on the account's document. A report
taken no later than the reading kept SHALL change nothing, and a start
under no account or a secret not the start's SHALL take nothing (153,
184, 211; core.17).

#### Scenario: A report taken

- **WHEN** a fix's code agent runs under work-1, its program reports `five_hour` 42% used resetting at 17:00 and `seven_day` 18% used, taken at 14:02, and its reading is taken with its start's secret
- **THEN** work-1's document holds that reading, taken at 14:02 from the code agent's identity and its session, and status gives it fresh (184, 211)

#### Scenario: An earlier report

- **WHEN** work-1's document holds a reading taken at 14:05, and a report taken at 14:02 is then taken
- **THEN** the document holds the 14:05 reading, at the version it had (184; core.17)

#### Scenario: No account or another secret

- **WHEN** a start under no account reports windows and its reading is taken, and the reading of a start under work-1 is asked with a secret not its own
- **THEN** no account's document is written, and the second ask is refused naming the start (153, 184)

### Requirement: An account's spent time and its reading each keep the other

Taking an account's reading SHALL keep its spent time, and recording it
spent SHALL keep its reading; a reading SHALL never set, shorten or
lengthen a spent time (146, 153, 185).

#### Scenario: A spent time and a reading

- **WHEN** work-1 is recorded spent until 15:00, then a reading of 10% used is taken for it, then a later usage limit names 16:00
- **THEN** work-1's document holds the reading and a spent time of 16:00, and work-1 takes no start before 16:00 (146, 185)

## REMOVED Requirements

### Requirement: An account's reading

**Reason**: No host writes a reading and an account names none: its
reading is the windows its agent program reports in a session flume
started (184, 211; record 0062). Replaced by "An account's reading is
what its program reported".

**Migration**: Drop `usage` from each account of the host's
configuration; flume keeps each account's reading in its own document.
