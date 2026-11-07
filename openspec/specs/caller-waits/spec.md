# caller-waits Specification

## Purpose
What a place waits on its caller for, recorded in flume and never in a
caller's memory: a hold put after the launch with its words, the move a
waiting place waits on and since when, and a stalled place said once to
each recipient (8, 47, 56, 130; record 0008; core.24).

## Requirements

### Requirement: A hold after the launch carries its caller's words

A caller SHALL be able to put a place's next stage, the next repetition
of a repeating stage that runs, or its submission, on hold with words
saying what for; flume SHALL record the hold with its words and since
when, and a retry naming that stage or the submission SHALL end it. A
hold on another stage already started, or on a place whose work is over,
SHALL be refused naming why (8, 47, 49, 130, 142).

#### Scenario: A hold for a card's answer

- **WHEN** a caller holds a change's code stage with the words "waiting on the user's card about the schema" while construct works
- **THEN** construct's delivery starts no code stage, status gives code on hold with those words and since when, and the caller's retry of code starts it

#### Scenario: A hold on the submission

- **WHEN** a caller holds a fix's submission after the launch and its code stage delivers
- **THEN** the place is not submitted until a retry names the submission

#### Scenario: A hold on a running stage

- **WHEN** a caller holds a stage that does not repeat while its agent is working
- **THEN** the hold is refused naming the stage, and nothing changes

#### Scenario: A hold on a running repeating stage

- **WHEN** a caller holds a repeating code stage with words while its repetition on group 2 works
- **THEN** that repetition goes on, and once it delivers no next repetition starts; status gives code on hold with the words, naming group 3, and a retry of code starts group 3 (142)

### Requirement: A launch's hold carries its words

A launch SHALL be able to give each stage it holds words saying what
for, spelled `--hold <stage>=<words>`, kept and given by status as a
later hold's are (7, 130).

#### Scenario: Held at the launch with words

- **WHEN** a caller launches a change with `--hold code=waiting on the schema card`
- **THEN** construct's delivery leaves code on hold with those words, and the place is never stalled

### Requirement: flume watches no event for a hold

flume SHALL end a hold only on its caller's retry, and SHALL read no
card, tell, landing or other place for the event a hold's words name
(70, 72, 130).

#### Scenario: Another place lands

- **WHEN** a place's code is held with words naming another place, and that place lands
- **THEN** the held stage stays on hold until a retry names it

### Requirement: A waiting place says which move and since when

For a place whose next move is its caller's, a stage or a submission on
hold, a stage ended short needing an answer, a stage lost, a prompt
never begun, a readiness that failed, a non-landing kind's last stage
delivered, or a place past its kind's
bound of goings-back, flume SHALL record the move and when it first read
the place so, and status SHALL give both (38, 45, 56, 112, 130, 131).

#### Scenario: A lost stage waits on its caller

- **WHEN** a code stage is read lost
- **THEN** status gives the place waiting on a retry of code since that reading, and five minutes on it is stalled

#### Scenario: Ended short needing an answer

- **WHEN** a construct stops short needing an answer at a fake clock's 10:00
- **THEN** status gives the place waiting on a retry of construct since 10:00, with the agent's words

### Requirement: A place waiting under no hold's words is stalled

A place waiting on its caller under no hold's words for five minutes, or
for the launch's bound where it sets one, SHALL be stalled: status SHALL
say so, and the settle that reads it SHALL deliver to each recipient, to
the role the stop-short it waits on named, of whichever stream, and to
the answerer behind the place's own stream where it is none of them,
once each, that it is stalled,
which move is theirs, and the words, key and addressee of the
stop-short it waits on (43, 130, 193).

#### Scenario: Stalled after five minutes

- **WHEN** a construct ends short needing an answer and no retry comes for five minutes of the fake clock under a watch, in a stream no role stands behind
- **THEN** the watch's settle reads the place stalled, status says so, and each recipient gets one notice naming the retry of construct as the move, and none at later settles

#### Scenario: The stall reaches the answerer

- **WHEN** the same place, launched from the pane `ops`, stalls in the stream `east`, behind which the scratch source `acme` declares `east/operator` with a run, and no run of it stands
- **THEN** the stall is delivered once to `ops` and stands pending for `east/operator`, whose run the settle wakes, and the run's `flume inbox` gives the stall with the stop-short's words and its key (193, S94)

#### Scenario: The role a question named

- **WHEN** a construct of a place in `east` whose recipient is `east/planner` stopped short naming `ops`, and no one holds or answers it for five minutes
- **THEN** the stall is delivered to `east/planner`, to `east/ops` and to `east/operator`, the answerer, once each

#### Scenario: Another stream's role named

- **WHEN** the same construct stopped short naming `west/design` instead, and `west/operator` stands behind `west`
- **THEN** the stall is delivered to `east/planner`, to `west/design` and to `east/operator`, the answerer behind the place's own stream, once each, and not to `west/operator` (S98)

#### Scenario: The answerer is the recipient

- **WHEN** a place whose one recipient is its stream's answerer stalls
- **THEN** the stall is delivered to the answerer once

#### Scenario: The launch's bound

- **WHEN** a launch sets a bound of thirty minutes and its place waits on its caller under no hold's words
- **THEN** it is not stalled at five minutes, and is at thirty

#### Scenario: A held place is not stalled

- **WHEN** a place waits for a day on a hold with its caller's words
- **THEN** status gives the hold and since when, never says it stalled, and no recipient and no answerer is delivered a stall

### Requirement: A place waiting for room is never stalled

A stage waiting for room SHALL wait on the host and not on its caller:
status SHALL give it with its priority and its place in the queue, and
such a place SHALL never be stalled (70, 130).

#### Scenario: A long queue

- **WHEN** a place's next stage waits for room for an hour
- **THEN** status gives it waiting for room, never waiting on its caller, and no stall is delivered

### Requirement: A hold on the waiting move

A caller SHALL be able to hold the move a place waits on it for, with
words and an optional bound, by `flume hold <place> <words> --waiting
[--for <time>]`. A later hold on the move SHALL replace its words and
bound. The place SHALL not stall while it holds. A retry naming the
stage or the submission, or a drop, SHALL end it. It SHALL be refused
on a place that waits on no move of its caller's (47, 130).

#### Scenario: A role takes the question

- **WHEN** a construct ended short needing an answer, and a role's run holds the waiting move with "east-planner has it" for an hour
- **THEN** status gives the hold with its words, since when and its bound, and the place is not stalled while it holds

#### Scenario: Passed on

- **WHEN** the same run then holds the move with "passed to east/design" for an hour
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
- **THEN** the watch's settle lapses the hold, status says it lapsed and when, and the place's stall, naming the lapsed hold's words, is delivered to each recipient, to the role the stop-short named and to the answerer (S98)

### Requirement: A stall recorded before reaches the answerer

A stall an earlier flume recorded, which names no answerer, SHALL be
read at the first settle after the build as the answerer's too, where
the place still waits on its caller under no hold's words, and
delivered once (130, 193).

#### Scenario: A stall from the earlier release

- **WHEN** a place's held state, written by an earlier flume, holds a stall delivered to nobody, and a source now declares `east/operator` standing behind the place's stream
- **THEN** the next settle delivers the stall to `east/operator` once, and later settles deliver it again to no one
