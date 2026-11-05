## MODIFIED Requirements

### Requirement: A place waiting under no hold's words is stalled

A place waiting on its caller under no hold's words for five minutes, or
for the launch's bound where it sets one, SHALL be stalled: status SHALL
say so, and the settle that reads it SHALL deliver to each recipient,
and to the host's answerer where it is none of them, once each, that it
is stalled, which move is theirs, and the words and key of the
stop-short it waits on (43, 130, 190).

#### Scenario: Stalled after five minutes

- **WHEN** a construct ends short needing an answer and no retry comes for five minutes of the fake clock under a watch, on a host that names no answerer
- **THEN** the watch's settle reads the place stalled, status says so, and each recipient gets one notice naming the retry of construct as the move, and none at later settles

#### Scenario: The stall reaches the host's answerer

- **WHEN** the same place, launched from the pane `ops`, stalls on a host whose configuration names `ops/answerer` its answerer with a run, and no run of it stands
- **THEN** the stall is delivered once to `ops` and stands pending for `ops/answerer`, whose run the settle wakes, and the run's `flume inbox` gives the stall with the stop-short's words and its key (190, S92)

#### Scenario: The answerer is the recipient

- **WHEN** a place whose one recipient is the host's answerer stalls
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
[--for <time>]`. The place SHALL not stall while it holds. A retry
naming the stage or the submission, or a drop, SHALL end it. Past its
bound the settle SHALL lapse it and the place SHALL stall at once. It
SHALL be refused on a place that waits on no move of its caller's (47,
130).

#### Scenario: The answerer holds the question

- **WHEN** a construct ended short needing an answer, and a caller holds the waiting move with "asked the user on card req_1" and no bound
- **THEN** status gives the hold with its words and since when, and the place is never stalled while it holds (S91)

#### Scenario: A hold's bound passes

- **WHEN** the hold gives `--for 1h` and the fake clock passes an hour under a watch with no retry
- **THEN** the watch's settle lapses the hold, status says it lapsed and when, and the place's stall is delivered to each recipient and to the host's answerer

#### Scenario: The move made

- **WHEN** a retry names construct while the hold stands
- **THEN** the hold ends, and status gives none

#### Scenario: Nothing waits

- **WHEN** a caller holds the waiting move of a place whose code stage works
- **THEN** the hold is refused naming the place and that it waits on no move of its caller's, and nothing changes

### Requirement: A stall recorded before reaches the answerer

A stall an earlier flume recorded, which names no answerer, SHALL be
read at the first settle after the build as the host's answerer's too,
where the place still waits on its caller under no hold's words, and
delivered once (130, 190).

#### Scenario: A stall from the earlier release

- **WHEN** a place's held state, written by an earlier flume, holds a stall delivered to nobody, and the host's configuration now names `ops/answerer`
- **THEN** the next settle delivers the stall to `ops/answerer` once, and later settles deliver it again to no one
