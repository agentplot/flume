## ADDED Requirements

### Requirement: The merge queue's open and close, and a submit by name

The merge-queue seam SHALL give open, a branch made at its base's
current revision as one the merge queue lands on, answering the
revision, or refused with its words; and close, a branch it lands on
removed into the branch its work landed in, or kept with its words, or
gone. Submit SHALL take what it sends as the place's worktree head or
a branch the merge queue lands on named with its revision, and answer
either as it answers a submission today (117, 203, 205, 206; core.2).

#### Scenario: Open, then close

- **WHEN** the seam's scenarios ask a profile to open `train/a` from `main`, then to close `train/a` into `main` with nothing landed on it
- **THEN** open answers the revision `main` stood at, and close answers the branch removed; a second close answers it gone (205, 206)

#### Scenario: Open refused

- **WHEN** a profile is asked to open a branch already present
- **THEN** it answers refused with the merge queue's words, and the branch is as it was (205)

#### Scenario: A submit by name

- **WHEN** a place submits the branch `train/a`, named, at its revision, to `main`
- **THEN** the seam answers the submission it made with `train/a` as its branch, held with no wait (117, 203)

### Requirement: The place seam makes a detached worktree

The place seam's make SHALL take the branch as optional, and given
none SHALL make the worktree detached at the revision, touching no
branch; every other answer of make stays as it is (78, 83, 202).

#### Scenario: A detached make

- **WHEN** the seam's scenarios make a place at a fresh location with no branch at a revision
- **THEN** the place's worktree stands at that revision on no branch, and no branch of the repository moved or was made (78, 83)

### Requirement: The types of a base place

`flume-seams` SHALL hold `SubmitHead`, `Opened`, `Closed` and
`BranchRemoved::Refused`, each written and read back by its round
trip, a document written before reading as it did (61, 202 to 206).

#### Scenario: Round trips

- **WHEN** `round_trip.rs` writes each new type and each new variant and reads it back
- **THEN** each reads as written, and a held state written before this change reads unchanged (61)
