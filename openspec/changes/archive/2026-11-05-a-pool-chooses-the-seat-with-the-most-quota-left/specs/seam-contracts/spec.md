## ADDED Requirements

### Requirement: The host gives an account's reading

The host's facts SHALL give the reading in the file at a path, or why
there is none, and the core SHALL ask it with the account's `usage`
expanded against the login environment, a `$VAR` that environment lacks
being no reading whose words name the variable (29, 184).

#### Scenario: The fake host's reading

- **WHEN** a test sets the fake host's reading for `work-1` and a refusal for `work-2`
- **THEN** the core reads the reading for `work-1`, the refusal's words for `work-2`, and that the file is missing for `work-3` (184)

### Requirement: The types of a reading and a move past a threshold

The seams SHALL pass one type each for a reading and its windows, a
pool's threshold, an account's `usage`, the strategy `most-left`, and a
move whose `why` is `threshold` with its window and end; each SHALL
serialize to JSON and back unchanged, and a move recorded before SHALL
read with no window and no end (147, 184, 185; core.14).

#### Scenario: A pool and a reading

- **WHEN** a most-left pool with a threshold of 85 and an account naming `usage`, and a reading of two windows, one with no reset, are written and read
- **THEN** each reads back unchanged, the strategy spelled `most-left` (184, 185)

#### Scenario: A move past a threshold

- **WHEN** an intent's move with `why` `threshold`, `window` `5-hour` and `until` 17:00 is written and read, and a move written before with neither
- **THEN** the first reads back unchanged and the second with no window and no end (147, 185)
