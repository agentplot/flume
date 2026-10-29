# caller-waits

## Purpose

What a place waits on its caller for, recorded in flume and never in a
caller's memory: a hold put after the launch with its words, the move a
waiting place waits on and since when, and a stalled place said once to
each recipient (8, 47, 56, 130; record 0008; core.24).

## ADDED Requirements

### Requirement: A hold after the launch carries its caller's words

A caller SHALL be able to put a place's next stage, or its submission,
on hold after the launch, with words saying what for; flume SHALL record
the hold with its words and since when, and a retry naming that stage
or the submission SHALL end it. A hold on a stage already started, or
on a place whose work is over, SHALL be refused naming why (8, 47, 49,
130).

#### Scenario: A hold for a card's answer

- **WHEN** a caller holds a change's code stage with the words "waiting on the user's card about the schema" while construct works
- **THEN** construct's delivery starts no code stage, status gives code on hold with those words and since when, and the caller's retry of code starts it

#### Scenario: A hold on the submission

- **WHEN** a caller holds a fix's submission after the launch and its code stage delivers
- **THEN** the place is not submitted until a retry names the submission

#### Scenario: A hold on a running stage

- **WHEN** a caller holds the stage whose agent is working
- **THEN** the hold is refused naming the stage, and nothing changes

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
say so, and the settle that reads it SHALL deliver to each recipient
once that it is stalled and which move is theirs (43, 130).

#### Scenario: Stalled after five minutes

- **WHEN** a construct ends short needing an answer and no retry comes for five minutes of the fake clock under a watch
- **THEN** the watch's settle reads the place stalled, status says so, and each recipient gets one notice naming the retry of construct as the move, and none at later settles

#### Scenario: The launch's bound

- **WHEN** a launch sets a bound of thirty minutes and its place waits on its caller under no hold's words
- **THEN** it is not stalled at five minutes, and is at thirty

#### Scenario: A held place is not stalled

- **WHEN** a place waits for a day on a hold with its caller's words
- **THEN** status gives the hold and since when, never says it stalled, and no recipient is delivered a stall

### Requirement: A place waiting for room is never stalled

A stage waiting for room SHALL wait on the host and not on its caller:
status SHALL give it with its priority and its place in the queue, and
such a place SHALL never be stalled (70, 130).

#### Scenario: A long queue

- **WHEN** a place's next stage waits for room for an hour
- **THEN** status gives it waiting for room, never waiting on its caller, and no stall is delivered
