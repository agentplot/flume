## MODIFIED Requirements

### Requirement: The settle delivers each end once to each recipient

The settle that reads a stage's end, a stage lost, never begun or
stuck, a submission's outcome or a stall SHALL record it on the place
as an end, with what flume read, and record its delivery to each of its
recipients, as "An end's recipients" gives them; and SHALL hand it to
each recipient once, as a notice into the recipient's own session at
its next wait, recorded as a hand-over with to whom and when, the end
pending until the recipient confirms it (7, 43, 46, 95, 214, 215).

#### Scenario: A delivery and its record

- **WHEN** a fix's code stage delivers at a head and the settle reads it, with the session `ops` waiting in the fake terminal
- **THEN** the fake terminal records one notice to `ops` naming the place and the end, and status gives the end handed to `ops` by notice with when, still pending for `ops`

#### Scenario: Settled twice

- **WHEN** the same place is settled again after the end was handed over
- **THEN** the fake terminal records no second notice to `ops` (74)

#### Scenario: An agent killed

- **WHEN** a code stage's agent is killed with nothing delivered and the watch's settle reads the stage lost
- **THEN** each recipient gets one notice that the stage is lost, and the end carries what flume read of it (43, 45, S28)

#### Scenario: A landing and a failure

- **WHEN** the merge queue's inbox gives the place's submission failed, and later landed
- **THEN** each outcome is an end handed once to each recipient, the failure with the merge queue's words and the landing with the commit it landed as (43, 133)

### Requirement: A notice is never sent into a question or left typed

A notice SHALL name the place, the end, the command that reads it and
the delivery's key. It SHALL be sent only while the terminal reads the
recipient waiting, never working, asking or unknown, and SHALL count
handed over on the terminal's answer sent, with no reading back,
delivering nothing; a send refused, or a recipient not found, SHALL
record no hand-over (43, 54, 95, 214; core.11).

#### Scenario: A recipient asking

- **WHEN** an end is read while the recipient's session is asking a person a question
- **THEN** no notice is sent, no hand-over is recorded, and the first settle that reads the session waiting sends it

#### Scenario: A recipient at work

- **WHEN** an end is read while the recipient's session reads working, and a later settle reads it waiting
- **THEN** nothing is typed into it while it works, no hand-over is recorded, and the later settle sends the notice once (54, 214; S126)

#### Scenario: A recipient gone

- **WHEN** the fake terminal finds no session by the recipient's name
- **THEN** no hand-over is recorded, the end stays pending, and the settle goes on with the place's other steps

### Requirement: The inbox collects what is pending

`flume inbox` SHALL give every end pending for a recipient, of every
place on the host, with what 43 names, its key and its delivery's key,
whether expired, and every message pending at it, with its key, who
sent it, when and its words, and SHALL record a hand-over of each to
that recipient, confirming each where `--confirm` is given. An inbox
naming a place SHALL give that place's ends alone. Status, a wait's
read and an inbox for another recipient SHALL confirm nothing (43, 214,
core.25).

#### Scenario: Collected after a restart

- **WHEN** two ends of two places are pending for `ops` while its session is gone, and its restarted session runs `flume inbox` as `ops`
- **THEN** the answer holds both ends whole, each with its key and its delivery's key, each handed to `ops` by the inbox, and a second `flume inbox` gives both again

#### Scenario: Collected and confirmed

- **WHEN** `ops` runs `flume inbox --confirm`
- **THEN** the answer holds both ends, each confirmed by this call, and a third `flume inbox` gives nothing

#### Scenario: A message collected

- **WHEN** a message is pending at `east/planner` with an end of a fix, and `flume inbox --recipient east/planner` runs
- **THEN** the answer gives the end under `ends` and the message under `messages`, with its key, its sender and its words, each handed to `east/planner` by the inbox and pending for it

### Requirement: An end for a standing address goes into the agent found by its name

At each settle that reads an end pending for a standing address and
handed to it by nothing, the terminal of its address's stream SHALL be
asked for the session's agents, and the end sent through it as a notice,
with every other delivery pending for the address in one prompt, to the
one agent of the entry's name while it waits; none, more than one, or a
part not read SHALL leave it handed to none; nothing of where the agent
was SHALL be kept (43, 132, 190, 193, 214, C.7; core.11, core.28).

#### Scenario: A standing agent waiting

- **WHEN** a fix names `madswan/planner` as its recipient, the entry gives `agent = "madswan-planner"` and `session = "madswan"`, the fake terminal holds that agent waiting in `madswan`, and the fix's code stage delivers
- **THEN** the agent receives one notice naming the place and the end, the end is recorded handed to `madswan/planner` by notice and pending for it, and no place is made (43, 132)

#### Scenario: A standing agent asking

- **WHEN** the standing agent is asking when the end is read, and later reads waiting
- **THEN** no notice is sent while it asks, the end is handed to none, and the first settle that reads it waiting sends it once (43; core.11)

#### Scenario: No agent of the name

- **WHEN** the fake terminal holds no agent named `madswan-planner` in `madswan` when the end is read
- **THEN** the end stays handed to none, nothing is claimed, and the settle goes on with the place's other steps (132)

#### Scenario: Two agents of the name

- **WHEN** the fake terminal holds two agents named `madswan-planner` in `madswan`
- **THEN** neither receives a notice, and the end stays handed to none (132)

#### Scenario: An agent revived under its name

- **WHEN** the standing agent takes one end, is removed, and another agent named `madswan-planner` is added in `madswan` under a new session key before the next end is read
- **THEN** the next notice reaches the new agent, and the removed one receives no second notice (132)

#### Scenario: The session from the entry's stream

- **WHEN** the entry gives `w/planner` `agent = "w-planner"` and no session, the stream `w` names `session = "w-main"`, and the fake terminal holds `w-planner` in `w-main`
- **THEN** the notice reaches it, asked of `w`'s terminal in `w-main` (143, 190, 193)

#### Scenario: The default stream's own session

- **WHEN** the entry gives `default/planner` only `agent = "w-planner"`, the default stream names no session, and the fake terminal holds `w-planner` in its own part
- **THEN** the notice reaches it, asked of the terminal's own part (107, 143, 193)

#### Scenario: A stream the host does not declare

- **WHEN** the entry gives `gone/planner` an agent, and the host declares no stream `gone`
- **THEN** no terminal is asked, and the end stays handed to none (143, 193)

#### Scenario: The inbox collects for a standing address

- **WHEN** an end is pending for `madswan/planner`, whose agent is away, and `flume inbox --recipient madswan/planner` runs
- **THEN** it collects the end as for any recipient, handed to the address by the inbox and pending for it, and the next settle sends no notice of it (43)

#### Scenario: A message to a standing agent at the watch's settle

- **WHEN** a caller messages `madswan/planner`, whose standing agent works, and the host's watch settles once it reads waiting
- **THEN** nothing is typed while it works, and the watch's settle hands it the message once, recorded handed by `message` (214)

#### Scenario: A standing agent under herdr through the binary

- **WHEN** under the pinned herdr and the stand-in, a fix names `w/planner` as its recipient, a scratch source's roles file gives it `agent = "w-planner"` and `session = "w"`, the host declares the stream `w`, and a stand-in runs in `w` under the herdr name `w-planner`
- **THEN** the fix's end reaches that pane as one notice, and no place of a planner is made (43, 132, 193, C.5)

## ADDED Requirements

### Requirement: A recipient takes every delivery pending for it as one prompt

At its next wait a recipient SHALL be handed, as one prompt, every
delivery whose readers name it, owed to it, handed to it by nothing and
claimed for it by nothing, but a drop's or a done's end, in the order
the host's deliveries recorded them, each item the delivery's message
followed by the way that recipient confirms it and the items joined by
one blank line; each SHALL be recorded handed on the send's answer, an
end by `notice` and a message by `message` (43, 214, 215; weir 103).

#### Scenario: Two messages and an end while a role works

- **WHEN** a role's run's agent works, two messages for its address and a fix's end come, and its agent then goes quiet and its place settles
- **THEN** nothing is typed while it works, and the run's session receives one prompt holding the two messages and the end in the order recorded, each with its confirm, each recorded handed to the address (S126)

#### Scenario: Something handed before

- **WHEN** an end was handed to `ops` by its inbox, and a message comes for `ops` after it
- **THEN** the next batch to `ops` holds the message alone

### Requirement: A delivery is claimed before it is handed

A settle SHALL claim every delivery of a batch beside the host's
deliveries records in one write at the version read, read the document
again and drop each item another way handed to the recipient meanwhile,
and on the send's answer add each hand-over and remove the claims in one
write; a send refused or not found SHALL remove the claims and record
nothing; a claim still sending SHALL be recorded by no other settle and
hold its delivery out of every later batch (43, 75, C.7).

#### Scenario: Two settles at once

- **WHEN** two settles read the same batch for `ops` pending at once
- **THEN** one claims it and sends it, the other finds it claimed and sends nothing, and `ops` receives one prompt (75)

#### Scenario: A send refused

- **WHEN** the fake terminal refuses the send of a batch to `ops`
- **THEN** no hand-over is recorded, no claim is left, and the next settle that reads `ops` waiting sends the batch

#### Scenario: A settle ended mid-send

- **WHEN** a claim for `ops` is left sending, and a new end comes for `ops`
- **THEN** the next batch holds the new end alone, and `ops`'s inbox gives both

### Requirement: The deliver part takes the place of the terminal

Where a recipient is a role's run whose live agent's profile offers
deliver, a settle SHALL hand it nothing through the terminal, and the
agent's take SHALL hand it the same batch, claimed and recorded handed
in one write (214, 215).

#### Scenario: A run whose agent takes its batch

- **WHEN** the fake agent of a role's run offers deliver, an end comes for its address while it works, and it goes quiet and takes
- **THEN** no notice goes through the fake terminal, its take answers the end's notice with its confirm, and the end is recorded handed to the address by notice
