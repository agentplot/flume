## MODIFIED Requirements

### Requirement: A usage limit read from Claude Code's own words

The Claude Code profile's `spent` SHALL read yes only where the
StopFailure hook recorded `rate_limit` with words that open as the
pinned release's usage-limit messages do, with the reset its words'
` · resets <when> (<zone>)` part names, or, where the limit hit is a
budget, their ` · your <limit> resets <when> (<zone>)` part, read in
that zone from when the hook ran. `<when>` SHALL be read as the pinned
release's reset formatter writes it: a time of day being the first such
time no more than a minute before the hook ran, and a date and a time
joined by ` at ` being that date, in the year the hook ran in the zone
where the date names none. Words that name no reset, or one in another
form, SHALL read yes with none; a `rate_limit` with other words SHALL
read no (146, 151, C.4).

#### Scenario: A session limit with a time of day

- **WHEN** the hook ran at 13:10 in New York and recorded `rate_limit` with "You've hit your session limit · resets 3pm (America/New_York)"
- **THEN** `spent` reads yes until 15:00 that day in New York (151, C.4)

#### Scenario: A weekly limit with a date

- **WHEN** the hook ran on 2026-11-05 at 20:40 UTC and recorded `rate_limit` with "You've hit your weekly limit · resets Nov 8 at 12am (America/New_York)"
- **THEN** `spent` reads yes until 2026-11-08 at 05:00 UTC (151, C.4)

#### Scenario: A date in another year

- **WHEN** the hook ran on 2026-12-30 in London and the words say "resets Jan 3, 2027 at 9am (Europe/London)"
- **THEN** `spent` reads yes until 3 January 2027 at 09:00 in London (151, C.4)

#### Scenario: A budget with its plan limit's reset

- **WHEN** the words are "You've hit your team's shared budget · raise it at https://claude.ai/admin-settings/usage · your weekly limit resets Nov 8 at 12am (America/New_York)"
- **THEN** `spent` reads yes until 2026-11-08 at 05:00 UTC (151, C.4)

#### Scenario: A reset past midnight

- **WHEN** the hook ran at 23:40 in UTC and the words say "resets 1am (UTC)"
- **THEN** `spent` reads yes until 01:00 UTC the next day (151)

#### Scenario: No reset in the words

- **WHEN** the words are "You've hit your limit"
- **THEN** `spent` reads yes with no reset (146, 151)

#### Scenario: A reset in a form the release does not write

- **WHEN** the words are "You've hit your weekly limit · resets Nov 2, 3:30pm (UTC) · progress saved"
- **THEN** `spent` reads yes with no reset (146, 151, C.4)

#### Scenario: A model under high load

- **WHEN** the hook recorded `rate_limit` with "Opus is experiencing high load, please use /model to switch to Sonnet"
- **THEN** `spent` reads no, and the words stay the agent's last words (43, 151)

### Requirement: The words pinned to the release the tree holds

A `check-tools` test SHALL read the pinned Claude Code and require the
source of its usage-limit words to stand byte for byte as the profile
reads them: its list of usage-limit openings, the `You've hit your`
template, the ` · resets ` part, the budget's ` · your <limit> resets `
part, the zone's suffix, and its reset formatter's choice of a date past
a day off and its options for a date and for a time of day (core.22,
C.4).

#### Scenario: The pinned release read

- **WHEN** the test reads the program devenv.nix pins, the file its `claude` wrapper runs
- **THEN** each fragment the profile's reading rests on is found in it, and the test names any that is not (C.4)
