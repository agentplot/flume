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

- **WHEN** a stage ends under a fake terminal that refuses send
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

### Requirement: A wait drives no settle while a caller keeps watch

While a caller keeps watch over the host, a wait SHALL settle nothing
and read only the host's state. While none does, it SHALL settle every
place before each reading, as every acting command does, whatever
terminal the host binds (119, 122, 208; core.13).

#### Scenario: Under a watch

- **WHEN** a wait follows a place while a caller keeps watch
- **THEN** the wait makes no call to any seam but the store, and the place moves on only by the caller's settles

#### Scenario: Attended under any terminal

- **WHEN** a wait follows a place while no caller keeps watch
- **THEN** the wait settles every place before each reading (119, 208)

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

### Requirement: A lost conversation's end says flume left an agent in its pane

The end of a conversation's stage read lost while an agent the
terminal brought back stands in its pane SHALL say that flume left that
agent in its pane, in the end held on the place and in its notice to
each recipient (43, 77).

#### Scenario: A lost talk with a bare agent

- **WHEN** after a rebuild the fake terminal brings a talk's agent back bare, with the session `ops` waiting, and the settle reads talk lost
- **THEN** the end gives `left_in_pane`, and the fake terminal records one notice to `ops` naming talk lost and that flume left the agent in its pane (43, 77, S113)

#### Scenario: A lost talk with no agent

- **WHEN** a talk's agent is killed and the settle reads talk lost
- **THEN** the end gives no `left_in_pane`, and the notice says nothing of an agent left (43, 45)

### Requirement: An end for a standing address goes into the agent found by its name

At each settle that reads an end pending for a standing address, the
terminal of the entry's stream SHALL be asked for the session's agents,
and the end sent through it as a notice to the one agent of the entry's
name while it asks no question; none, more than one, or a part not read
SHALL leave it pending; nothing of where the agent was SHALL be kept
(43, 132, 190, C.7; core.11, core.28).

#### Scenario: A standing agent waiting

- **WHEN** a fix names `madswan/planner` as its recipient, the entry gives `agent = "madswan-planner"` and `session = "madswan"`, the fake terminal holds that agent waiting in `madswan`, and the fix's code stage delivers
- **THEN** the agent receives one notice naming the place and the end, the end is recorded delivered to `madswan/planner`, and no place is made (43, 132)

#### Scenario: A standing agent asking

- **WHEN** the standing agent is asking when the end is read, and later reads waiting
- **THEN** no notice is sent while it asks, the end stays pending, and the first settle that reads it waiting delivers it once (43; core.11)

#### Scenario: No agent of the name

- **WHEN** the fake terminal holds no agent named `madswan-planner` in `madswan` when the end is read
- **THEN** the end stays pending, nothing is claimed, and the settle goes on with the place's other steps (132)

#### Scenario: Two agents of the name

- **WHEN** the fake terminal holds two agents named `madswan-planner` in `madswan`
- **THEN** neither receives a notice, and the end stays pending (132)

#### Scenario: An agent revived under its name

- **WHEN** the standing agent takes one end, is removed, and another agent named `madswan-planner` is added in `madswan` under a new session key before the next end is read
- **THEN** the next notice reaches the new agent, and the removed one receives no second notice (132)

#### Scenario: The session from the entry's stream

- **WHEN** the entry gives `agent = "w-planner"` and `stream = "w"`, the stream `w` names `session = "w-main"`, and the fake terminal holds `w-planner` in `w-main`
- **THEN** the notice reaches it, asked of `w`'s terminal in `w-main` (143, 190, C.7)

#### Scenario: The default stream's own session

- **WHEN** the entry gives only `agent = "w-planner"`, the default stream names no session, and the fake terminal holds `w-planner` in its own part
- **THEN** the notice reaches it, asked of the terminal's own part (107, 143, C.7)

#### Scenario: A stream the host does not declare

- **WHEN** the entry gives `stream = "gone"`, which the host does not declare
- **THEN** no terminal is asked, and the end stays pending (143, C.7)

#### Scenario: The inbox collects for a standing address

- **WHEN** an end is pending for `madswan/planner`, whose agent is away, and `flume inbox --recipient madswan/planner` runs
- **THEN** it collects the end as for any recipient, marked delivered to the address, and the next settle sends no notice of it (43)

#### Scenario: A standing agent under herdr through the binary

- **WHEN** under the pinned herdr and the stand-in, a fix names `w/planner` as its recipient, the host's configuration gives it `agent = "w-planner"` and `session = "w"`, and a stand-in runs in `w` under the herdr name `w-planner`
- **THEN** the fix's end reaches that pane as one notice, and no place of a planner is made (43, 132, C.5)

### Requirement: A stop-short's end says until when its agent is kept open

The end of a stage ended short needing an answer SHALL carry, where its
agent is kept open for the answer, the time it is kept open until, in
the JSON form and in the notice's words (43, 198).

#### Scenario: The notice of a question

- **WHEN** a code agent stops short needing an answer at a time t, on a terminal that hands a running agent words
- **THEN** the end delivered to each recipient gives its agent kept open until t plus its `keep_open`, and the notice's words say so (43, 198)

### Requirement: A wait after an answer waits on the next end

A wait on a stage SHALL answer only an end of its current start read
after the last answer that start took in its session, so a wait after
an answer never answers the stop-short it answered (43, 198).

#### Scenario: Waiting again after an answer

- **WHEN** a caller's wait on code delivered its stop-short, the caller answered it in its session, and the caller waits on code again
- **THEN** the wait does not answer the stop-short as a read, and answers code's next end once it is read (43)
