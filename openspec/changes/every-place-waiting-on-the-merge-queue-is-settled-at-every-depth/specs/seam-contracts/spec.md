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

### Requirement: The terminal seam starts no settle

The terminal seam SHALL offer no operation that settles, keeps watch
over the host, nudges a settle at an agent's change, or answers a
bound on which the terminal settles; whether flume runs attended SHALL
be read from the host's watch alone, whatever terminal is bound (119,
122, 208; core.2, core.13).

#### Scenario: The same reading under every terminal

- **WHEN** the engine is asked whether it runs attended under the fake terminal, then under each terminal profile the tree carries, with the same host's watch held
- **THEN** it answers the same under each, and no terminal operation was called to answer it (122, 208)

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
`BranchRemoved::Refused`, `HostKey::Watch`, `Watched` and the line
`Clues` carries, each written and read back by its round trip, a
document written before reading as it did (61, 139, 202 to 206 and
208).

#### Scenario: Round trips

- **WHEN** `round_trip.rs` writes each new type and each new variant and reads it back
- **THEN** each reads as written, and a held state and a host's state written before this change read unchanged (61)

## MODIFIED Requirements

### Requirement: The terminal seam's open, tidy and show

The terminal seam's open SHALL take where the agent runs, beside a
pane, in a session by its name or in the terminal's own, and SHALL
answer, beside opened and refused, a start held at a question
its program asks before it starts and a program that did not come up
within its bound, each with the screen (94, 107); it SHALL tidy what it
made for a place gone (138) and show a place's clues, its current
stage and that stage's state, the short form of its line and the count
of the places of its repository on its base (139); and every terminal
operation SHALL answer within a bound the profile states (98).

#### Scenario: Held, tidied and shown

- **WHEN** the fake terminal holds a start at a question, makes a place's tab in its base's workspace, and has a person's pane in that workspace
- **THEN** open answers held with the screen, tidy closes the place's tab and keeps the workspace naming the person's pane, and show takes the place's clues with its line's short form (94, 138, 139)

#### Scenario: Where an agent opens

- **WHEN** the fake terminal is asked to open one agent beside the pane `operator`, one in the session `work` and one in its own
- **THEN** its log names each where it was asked, and the open's answer is the same for each (94, 107)
