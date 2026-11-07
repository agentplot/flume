## ADDED Requirements

### Requirement: Status gives a base place's fixes

Status SHALL give a base place whose kind names a fix its `fix_kind`,
with where it came from; its `fix_limit`; its `forecast`, the newest a
settle held, with when, its head, the base's revision, and clear, in
conflict with its files and entries ahead, or unread with why; and its
`fixes`, each with its number, place, cause, launch time and end: live,
landed with its commit, dropped, or refused with its words. Status SHALL
form no forecast of its own (56, 58, 203; core.15).

#### Scenario: A base place whose first fix is live

- **WHEN** `train--spring`'s kind names `mend` with a bound of 2, a forecast conflict on `README.md` at `c0ffee` launched `mend--spring-1`, which works, and status is asked
- **THEN** `train--spring` gives `fix_kind` `repo/mend@1` from the repository, `fix_limit` 2, its forecast in conflict on `README.md` at `c0ffee` with when it was read, and `fixes` holding fix 1, `mend--spring-1`, its cause the forecast, live; it waits on its places with `on_branch` naming `mend--spring-1`; and the fake merge queue logged no forecast for the status (56, 203)

#### Scenario: A fix landed, in the person's form

- **WHEN** `mend--spring-1` has landed with commit `abc123` and the next forecast was clear, and the person's form of status is asked of `train--spring`
- **THEN** it gives the kind of its fixes with where it came from and `1 of 2 fixes`, `forecast clear` with the base's revision, and fix 1 landed as `abc123`; the JSON gives the same fix with `ended` landed and `commit` `abc123` (56, 61)

### Requirement: Status gives a fix's base place

Status SHALL give a fix flume launched its `fix_of`, naming the base
place, the fix's number and its cause, and its recipients' source as
`base-place`. Every field of a base place's fixes and `fix_of` SHALL be
left out of every place it does not apply to, the JSON form only gaining
fields (56, 61, 203; core.14).

#### Scenario: The fix names its base place

- **WHEN** a forecast conflict launched `mend--spring-1` for `train--spring`, and status is asked of it
- **THEN** it gives `fix_of` naming `train--spring`, fix 1 and the forecast, and its recipients `ops` from `base-place` (43, 56, 203)

#### Scenario: Fields left out of other places

- **WHEN** status is asked of a change no base place launched, and of a base place whose kind names no fix
- **THEN** the change gives no `fix_of`, `fix_kind`, `forecast` or `fixes`, and the base place gives none of them either (61)
