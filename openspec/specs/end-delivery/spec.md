# end-delivery Specification

## Purpose
How whoever a place's work is for learns how it ended with no polling and
nothing lost to a restart: each end delivered once into its recipient's
pane and recorded, pending until then, waited on by place and collected
by `flume inbox` (43, 46, 119, 120, 122; record 0009; core.25).

## Requirements

### Requirement: The settle delivers each end once to each recipient

The settle that reads a stage's end, a stage lost, never begun or
stuck, a submission's outcome or a stall SHALL record it on the place
as an end, with what flume read, and SHALL deliver each end once to
each recipient the launch named: a notice through the terminal into
the recipient's own pane, recorded with to whom and when, and pending
until so recorded (7, 43, 46, 95).

#### Scenario: A delivery and its record

- **WHEN** a fix's code stage delivers at a head and the settle reads it, with the session `ops` waiting in the fake terminal
- **THEN** the fake terminal records one notice to `ops` naming the place and the end, and status gives the end delivered to `ops` with when

#### Scenario: Settled twice

- **WHEN** the same place is settled again after the end was delivered
- **THEN** the fake terminal records no second notice to `ops` (74)

#### Scenario: An agent killed

- **WHEN** a code stage's agent is killed with nothing delivered and the watch's settle reads the stage lost
- **THEN** each recipient gets one notice that the stage is lost, and the end carries what flume read of it (43, 45, S28)

#### Scenario: A landing and a failure

- **WHEN** the merge queue's inbox gives the place's submission failed, and later landed
- **THEN** each outcome is an end delivered once to each recipient, the failure with the merge queue's words and the landing with the commit it landed as (43, 133)

### Requirement: A notice is never sent into a question or left typed

A notice SHALL name the place, the end, and the command that reads it.
It SHALL be sent only while the terminal reads the recipient asking no
question, neither asking nor unknown, and SHALL count delivered on the
terminal's answer sent, with no reading back; a send refused, or a
recipient not found, SHALL leave the end pending (43, 95, core.11).

#### Scenario: A recipient asking

- **WHEN** an end is read while the recipient's session is asking a person a question
- **THEN** no notice is sent, the end stays pending, and the first settle that reads the session not asking delivers it

#### Scenario: A recipient gone

- **WHEN** the fake terminal finds no session by the recipient's name
- **THEN** the end stays pending and the settle goes on with the place's other steps

### Requirement: The printed command delivers nothing

Where the terminal has no pane to deliver to, no notice SHALL be sent,
every end SHALL stay pending until a wait answers it or the inbox
collects it, and status SHALL give it (43, C.5).

#### Scenario: Under the printed command

- **WHEN** a stage ends under a fake terminal that keeps no watch and refuses send
- **THEN** status gives the end pending for each recipient and the fake terminal records no send

### Requirement: A wait is by place

`flume wait` SHALL name a place and its stage or its submission, and the
recipient it waits as. It SHALL answer at once with an end of that stage
or submission pending for the recipient, recording it delivered; answer
one already delivered as a read; and otherwise follow the host's state
until such an end, or exit still waiting when its timeout passes (43).

#### Scenario: A restarted recipient waits again

- **WHEN** a recipient's session restarts while waiting on a place's submission, the submission lands, and the recipient runs the same wait
- **THEN** the wait answers at once with the landing and its commit, and the end reads delivered to that recipient

#### Scenario: The timeout

- **WHEN** a wait with a timeout of one minute finds nothing pending and the place's stage still runs after a minute
- **THEN** the wait exits still waiting, with its own exit, and records nothing delivered

### Requirement: A wait drives no settle where the terminal keeps watch

Under a terminal that keeps watch, a wait SHALL settle nothing and read
only the host's state. Under one that keeps none, it SHALL settle every
place before each reading, as every acting command does (119, 122,
core.13).

#### Scenario: Under a watch

- **WHEN** a wait follows a place under a fake terminal that keeps watch
- **THEN** the wait makes no call to any seam but the store, and the place moves on only by the watch's settles

### Requirement: The inbox collects what is pending

`flume inbox` SHALL give every end pending for a recipient, of every
place on the host, with what 43 names, and SHALL record each delivered
to that recipient. Status, a wait's read and an inbox for another
recipient SHALL mark nothing delivered (43, core.25).

#### Scenario: Collected after a restart

- **WHEN** two ends of two places are pending for `ops` while its session is gone, and its restarted session runs `flume inbox` as `ops`
- **THEN** the answer holds both ends whole, both read delivered to `ops`, and a second `flume inbox` gives nothing

### Requirement: A tidied place's document stays while an end is pending

A place whose work is over SHALL be tidied as always, its worktree and
branch removed (66, 67), and while an end of it stands pending its
document alone SHALL stay, naming the recipients it waits on, until
every end is delivered or its caller drops it. A launch of the same
name SHALL not be refused for it (3, 43, 62).

#### Scenario: A landing with its recipient away

- **WHEN** a place lands while its one recipient's session is gone
- **THEN** its worktree and branch are removed, status gives the tidied place pending for that recipient, and once the recipient collects by `flume inbox` the next settle removes the document

#### Scenario: Dropped while pending

- **WHEN** the caller drops a tidied place whose end stands pending
- **THEN** the document and its pending ends are gone, and the inbox gives nothing of it

#### Scenario: Launched again

- **WHEN** a place of the same repository, kind and name is launched while the tidied document stays
- **THEN** the launch makes its place, and the tidied document's ends stay pending as they were
