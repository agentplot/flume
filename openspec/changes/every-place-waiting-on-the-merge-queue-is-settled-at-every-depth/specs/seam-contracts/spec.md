## ADDED Requirements

### Requirement: The merge queue's open and close

The merge-queue seam SHALL give open, a branch made at its base's
current revision as one the merge queue lands on, answering the
revision, or refused with its words; and close, a branch it lands on
removed into the branch its work landed in, or kept with its words, or
gone (205, 206; core.2).

#### Scenario: Open, then close

- **WHEN** the seam's scenarios ask a profile to open `train/a` from `main`, then to close `train/a` into `main` with nothing landed on it
- **THEN** open answers the revision `main` stood at, and close answers the branch removed; a second close answers it gone (205, 206)

#### Scenario: Open refused

- **WHEN** a profile is asked to open a branch already present
- **THEN** it answers refused with the merge queue's words, and the branch is as it was (205)

### Requirement: A submit by name

Submit SHALL take what it sends as the place's worktree head, from the
place, or a branch the merge queue lands on named with its revision,
from the repository with no worktree as its submitter, and answer
either as it answers a submission today; read-submission SHALL give a
submission by name's outcome with its words (99, 117, 203; core.2).

#### Scenario: A submit by name

- **WHEN** a base place submits the branch `train/a`, named, at its revision, to `main`
- **THEN** the seam answers the submission it made with `train/a` as its branch, held with no wait, and read-submission of `train/a` gives it (99, 117, 203)

### Requirement: The terminal's nudge

The terminal seam SHALL answer whether the terminal settles when an
agent's state changes, or nudges nothing, in place of a watch with a
bound; no operation of the seam SHALL answer a bound on which the
terminal settles (122, 208).

#### Scenario: A nudge or none

- **WHEN** the seam's scenarios ask a profile its nudge
- **THEN** it answers that it nudges at each change or that it nudges nothing, and gives no bound (122)

### Requirement: The store holds the host's watch

The store seam's host documents SHALL hold the host's watch, when the
newest settle a caller asked as its watch began and the bound it
stated, written by compare-and-set on its version as every host
document is, a host state written before reading as holding none (73,
153, 208).

#### Scenario: Two watches at once

- **WHEN** two settles with a bound write the host's watch from the same version
- **THEN** one write takes and the other is refused, and the one that read again writes over it (73, 208)

### Requirement: The types of a base place and the watch

`flume-seams` SHALL hold `SubmitHead`, `Opened`, `Closed`,
`BranchRemoved::Refused`, `Nudge`, `HostKey::Watch` and `Watched`, each
written and read back by its round trip, a document written before
reading as it did (61, 202 to 206 and 208).

#### Scenario: Round trips

- **WHEN** `round_trip.rs` writes each new type and each new variant and reads it back
- **THEN** each reads as written, and a held state and a host's state written before this change read unchanged (61)
