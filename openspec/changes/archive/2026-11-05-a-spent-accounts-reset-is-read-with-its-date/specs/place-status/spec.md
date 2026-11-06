## MODIFIED Requirements

### Requirement: Status gives the host's pools

Status SHALL give, for the host, each pool with its strategy and its
threshold, and each account with its agents running, its bound, the
time it stops being spent, the words in which no reset was read while
it is spent for the hour 146 gives, its reading, and, while it is past
its threshold, the window that holds it and until when; and never an
account's environment's value or a reading's path (56, 144 to 146,
185).

#### Scenario: A pool with a spent account

- **WHEN** status is asked while work-1 runs two agents under a bound of four and work-2 is spent until 15:00
- **THEN** the pool `work` gives fill-first, work-1 with two running and a bound of four, and work-2 spent until 15:00 (56, 146)

#### Scenario: An account spent for an hour

- **WHEN** status is asked while work-3 is spent for an hour because no reset was read in "You've hit your weekly limit · resets Nov 8, 12am (UTC)"
- **THEN** work-3 gives its spent time and `spent_unread` with those words, and the person's form gives them under its line (56, 146)

#### Scenario: A pool with readings

- **WHEN** status is asked of a most-left pool stating no threshold, whose work-1 reads `5-hour` 42% used resetting at 17:00, read two minutes ago, whose work-2's reading is twenty minutes old, and whose work-3's file is missing
- **THEN** the pool gives most-left and a threshold of 90, work-1 its window with 42, 17:00 and fresh, work-2 its reading stale, and work-3 no reading with why, in the JSON form and the person's, and neither gives a path (56, 184, 185)

#### Scenario: An account past its threshold

- **WHEN** status is asked while work-1's reading gives `7-day` 95% used, resetting Monday at 08:00
- **THEN** work-1 gives `past` with `7-day` and Monday at 08:00, and its window `past` (56, 185)
