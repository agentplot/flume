## ADDED Requirements

### Requirement: A message's delivery key

A message to a role's address SHALL be recorded as one delivery keyed
`message/<stream>/<role>/<token>`, the token eight hex characters drawn
as a launch's is, its one reader the address, its message a first line
naming who sent it, where the sending command reads one, and the words
as given, and its expiry `[host] ends_expire` after it was recorded, a
week where that is unset (53, 214, C.7; weir 101, 104).

#### Scenario: A message recorded

- **WHEN** the pane `ops` messages `east/planner` "the nightly job failed" on a host whose `ends_expire` is `48h`
- **THEN** the host's deliveries hold one `weir-delivery/1` keyed `message/east/planner/` and eight hex characters, its readers `east/planner` alone, its message `ops messages east/planner:` and the words on the next line, expiring 48 hours after it was recorded (C.7; weir 101)

#### Scenario: A message from no one the command names

- **WHEN** a command that reads no recipient of its own messages `east/planner`
- **THEN** the delivery's message begins `A message to east/planner:`

### Requirement: The claims kept beside the host's deliveries

The host's deliveries document SHALL hold, beside its records, each
claim of a delivery being handed: its delivery's key, its recipient,
when it was claimed and whether its send is unanswered, written with the
records by compare-and-set and read with none in a document written
before; weir's pack SHALL still pass over the document's records, and
no claim SHALL be spelled as a weir record (75, C.7; weir 112, 113;
core.4).

#### Scenario: A claim round-trips

- **WHEN** a deliveries document holding two records and a claim still sending is written and read back
- **THEN** it reads back equal, and a document written before, with no claims, reads with none

#### Scenario: weir's pack over a document with claims

- **WHEN** weir's pack runs over flume's store of deliveries holding claims
- **THEN** it passes, reading the records alone (weir 113)

### Requirement: A message's delivery is let go of once its reader confirmed it

flume SHALL let go of a message's delivery, with its hand-overs and its
confirmations, once its one reader has confirmed it, at the next settle
of every place, and SHALL keep it while it is pending, expired or not;
its log SHALL name it as it names an end's (43; weir 107, I9).

#### Scenario: Confirmed, then let go of

- **WHEN** a message at `east/planner` is handed to the address, confirmed by it, and the host is settled
- **THEN** the delivery and its records are gone from the host's deliveries, its `weir-known/1` kept, and the log names it let go of with `east/planner`

#### Scenario: Expired and kept

- **WHEN** a message at `east/planner` is past its expiry and never confirmed, and the host is settled
- **THEN** it stays pending, status giving it expired with its reader
