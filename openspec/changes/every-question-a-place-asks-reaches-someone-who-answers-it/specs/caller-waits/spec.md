## MODIFIED Requirements

### Requirement: A place waiting under no hold's words is stalled

A place waiting on its caller under no hold's words for five minutes, or
for the launch's bound where it sets one, SHALL be stalled: status SHALL
say so, and the settle that reads it SHALL deliver to each recipient, to
the role the stop-short it waits on named, and to the answerer behind
its stream where it is none of them, once each, that it is stalled,
which move is theirs, and the words, key and addressee of the
stop-short it waits on (43, 130, 193).

#### Scenario: Stalled after five minutes

- **WHEN** a construct ends short needing an answer and no retry comes for five minutes of the fake clock under a watch, in a stream no role stands behind
- **THEN** the watch's settle reads the place stalled, status says so, and each recipient gets one notice naming the retry of construct as the move, and none at later settles

#### Scenario: The stall reaches the answerer

- **WHEN** the same place, launched from the pane `ops`, stalls in the stream `work`, behind which the scratch source `acme` declares `acme/operator` with a run, and no run of it stands
- **THEN** the stall is delivered once to `ops` and stands pending for `acme/operator`, whose run the settle wakes, and the run's `flume inbox` gives the stall with the stop-short's words and its key (193, S93)

#### Scenario: The role a question named

- **WHEN** a construct of a place whose recipient is `acme/planner` stopped short naming `acme/ops`, and no one holds or answers it for five minutes
- **THEN** the stall is delivered to `acme/planner`, to `acme/ops` and to `acme/operator`, the answerer, once each

#### Scenario: The answerer is the recipient

- **WHEN** a place whose one recipient is its stream's answerer stalls
- **THEN** the stall is delivered to the answerer once

#### Scenario: The launch's bound

- **WHEN** a launch sets a bound of thirty minutes and its place waits on its caller under no hold's words
- **THEN** it is not stalled at five minutes, and is at thirty

#### Scenario: A held place is not stalled

- **WHEN** a place waits for a day on a hold with its caller's words
- **THEN** status gives the hold and since when, never says it stalled, and no recipient and no answerer is delivered a stall

## ADDED Requirements

### Requirement: A hold on the waiting move

A caller SHALL be able to hold the move a place waits on it for, with
words and an optional bound, by `flume hold <place> <words> --waiting
[--for <time>]`. A later hold on the move SHALL replace its words and
bound. The place SHALL not stall while it holds. A retry naming the
stage or the submission, or a drop, SHALL end it. It SHALL be refused
on a place that waits on no move of its caller's (47, 130).

#### Scenario: A role takes the question

- **WHEN** a construct ended short needing an answer, and a role's run holds the waiting move with "acme-planner has it" for an hour
- **THEN** status gives the hold with its words, since when and its bound, and the place is not stalled while it holds

#### Scenario: Passed on

- **WHEN** the same run then holds the move with "passed to acme/design" for an hour
- **THEN** status gives the new words and bound in place of the first

#### Scenario: The move made

- **WHEN** a retry names construct while the hold stands
- **THEN** the hold ends, and status gives none

#### Scenario: Nothing waits

- **WHEN** a caller holds the waiting move of a place whose code stage works
- **THEN** the hold is refused naming the place and that it waits on no move of its caller's, and nothing changes

### Requirement: A hold on the waiting move lapses at its bound

Once a hold on the waiting move has stood past its bound, the settle
that reads it SHALL lapse it, and the place SHALL stall again at once,
its stall a new end with a key of its own, naming the lapsed hold's
words (130, 193).

#### Scenario: A hold's bound passes

- **WHEN** the hold's hour passes on the fake clock under a watch with no retry
- **THEN** the watch's settle lapses the hold, status says it lapsed and when, and the place's stall, naming the lapsed hold's words, is delivered to each recipient, to the role the stop-short named and to the answerer (S94)

### Requirement: A stall recorded before reaches the answerer

A stall an earlier flume recorded, which names no answerer, SHALL be
read at the first settle after the build as the answerer's too, where
the place still waits on its caller under no hold's words, and
delivered once (130, 193).

#### Scenario: A stall from the earlier release

- **WHEN** a place's held state, written by an earlier flume, holds a stall delivered to nobody, and a source now declares `acme/operator` standing behind the place's stream
- **THEN** the next settle delivers the stall to `acme/operator` once, and later settles deliver it again to no one
