## ADDED Requirements

### Requirement: The merge queue's forecast

The merge-queue seam SHALL give forecast: given a repository, a branch
it lands on and its base, the conflicts the branch's head would meet
were it submitted now behind every entry already submitted: clear, with
the head and the base's revision; in conflict, with those, the files,
each entry ahead it meets, none where it meets the base itself, and the
merge queue's words; or could not read, with why. A forecast SHALL
create no entry and move no branch (203, 210; core.2).

#### Scenario: Clear, then in conflict

- **WHEN** the seam's scenarios ask a profile to forecast `train/a` onto `main` while the two merge, then move `main` with a change to the file `train/a` also changed, and ask again
- **THEN** the first answers clear with `train/a`'s head and `main`'s revision; the second answers in conflict on that file with `main`'s new revision and no entry ahead; and read-submission of `train/a` gives no submission after either (210)

### Requirement: The types of a forecast and a base place's fixes

`flume-seams` SHALL hold `Forecast`, the kind's fix kind and its bound
on `Kind`, `FixKind` and `FixOf` on `PlaceRecord`, `ForecastHeld` and
`BaseFix` with `FixCause` and `FixEnded` on `HeldState`, and
`RecipientsFrom::BasePlace`, each written and read back by its round
trip, a kind, a place's record and a held state written before reading
as naming no fix, no forecast and no fixes (21, 61, 203, 210).

#### Scenario: Round trips

- **WHEN** `round_trip.rs` writes each new type and each new variant and reads it back, and reads a held state written before this change
- **THEN** each reads as written, and the held state written before reads unchanged, naming no fix kind, no forecast, no fixes and no base place (61)
