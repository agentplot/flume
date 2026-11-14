# deliveries Specification

## Purpose
flume's store of weir's deliveries, the form of what a place's end owes
each recipient: the host's one document of weir's records, written
through `weir-delivery` by compare-and-set and held to weir's pack, and
an end's delivery key, `<kind>--<name>/<token>/<end key>` (43, C.7;
weir 100 to 113, C.8).

## Requirements

### Requirement: The host's deliveries are one document of weir's records

flume SHALL keep every record of weir's C.8 for the ends of the host's
places in one host document, in the order added, each spelled by
`weir-delivery`'s JSON and read back through it, written whole by
compare-and-set, every record or none; and SHALL write no record but
through weir's `record`, `hand` and `confirm` over the records it read
at the version it writes at (C.7; weir 100, 112; core.4, core.16).

#### Scenario: A delivery, a hand-over and a confirmation round-trip

- **WHEN** a delivery, a hand-over, a confirmation and a known name are written to the host's deliveries document and read back
- **THEN** each reads back as written, in order, each naming its format, and the document's JSON holds each record as weir's `json::write` spells it (C.8)

#### Scenario: Two writes at one version

- **WHEN** two engines over one store each confirm an end at the version both read
- **THEN** one write lands, the other reads the version moved, reads again and writes the records weir's `confirm` gives over what it read, so the store holds one confirmation and one known name (75; weir 101, 102)

### Requirement: flume's store of deliveries passes weir's pack

flume's store of deliveries, weir's `Store` over the store seam, SHALL
pass every delivery case of weir's pack, over the fakes' store and over
the host's state directory (weir 87, 112, 113).

#### Scenario: The pack over the fakes' store

- **WHEN** `weir_pack::conform_deliveries` runs every delivery case of `Pack::embedded()` through flume's store of deliveries over a fresh fake store per case
- **THEN** every case passes

#### Scenario: The pack over the state directory

- **WHEN** the same cases run through flume's store of deliveries over a fresh scratch state directory per case
- **THEN** every case passes, and each document is readable by the host's user alone

### Requirement: An end's delivery key

An end's delivery SHALL be keyed `<kind>--<name>/<token>/<end key>`, the
token eight hex characters a place's launch took and kept on its record,
so no two launches of one place share a key; a place's document written
with no token SHALL be given one by the first settle that writes it
(43, C.7; weir 101).

#### Scenario: A place launched twice

- **WHEN** a fix is launched, its code delivers, it lands and is tidied with nothing pending, and a fix of the same name is launched and its code delivers
- **THEN** the two code ends have two delivery keys, each `fix--<name>/<token>/stage/code/1/delivered` with a token of its own, and the second's delivery is recorded owed to its recipients (weir 101)

#### Scenario: A place written before the token

- **WHEN** a live place's document written with no token is settled
- **THEN** that settle's write gives it a token, and its ends' deliveries are keyed by it

### Requirement: Status names only a recorded delivery key

Status, the inbox and a wait SHALL give an end's `delivery`, the key a
recipient confirms it by, only where the host's deliveries record the
end's delivery under it, and none while none is recorded; an end with a
recipient whose delivery is not recorded SHALL read pending for each of
its recipients with no key and no expiry (43, 56; weir 101, 102).

#### Scenario: A delivery not yet recorded

- **WHEN** a fix's code delivers for `ops` while the host's deliveries refuse every write, and status is asked
- **THEN** code's end is pending for `ops` with no `delivery` and no `expires_at`; once the next settle records it, status names its key, and `ops` confirming that key is taken

### Requirement: A place's deliveries are let go of only once no document of it stands

flume SHALL let go of a delivery only once every reader confirmed it and
no document of its place stands: by the act that removes the place's
last document, or by a host's settle that reads no document of its place
in two reads of every place, the second begun once the first ended. A
tidy SHALL remove a place's own document only once the document kept
for its pending ends stands, and otherwise keep it, flume's log saying
why, for the next settle to tidy again (43, 62; weir 107, I9).

#### Scenario: Kept until each recipient confirms

- **WHEN** a fix for `ops` and `boomdrive` lands, both confirm its code end and `ops` its landing, and the host is settled and tidied again and again
- **THEN** the tidied document stays pending for `boomdrive`, both deliveries stay recorded and status names each key, code's confirmed by both and the landing pending for `boomdrive`, no second notice goes to `ops`, and nothing is let go of until `boomdrive` confirms the landing, whereupon the next settle removes the document and lets go of both

#### Scenario: A document a tidy moves while every place is read

- **WHEN** a host's let-go of the gone reads every place, and between its listing and its reads a tidy writes the document it keeps for the place's pending ends and removes the place's own, whose done delivery the host's deliveries hold
- **THEN** the delivery is kept; once no document of the place stands, the next let-go lets go of it

#### Scenario: A kept document not written

- **WHEN** a fix for `ops` lands while no pane holds `ops` and the document kept for its ends cannot be written
- **THEN** the place's own document stays, status giving its ends pending for `ops` with their keys and flume's log saying why, and the next settle writes the document kept, pending for `ops`, and removes the place's own

### Requirement: flume's log names each delivery recorded and let go of

flume's log SHALL name each delivery a write of the host's deliveries
records, with its place, its readers and its expiry, and each delivery
one lets go of, with the readers that confirmed it and why, each once
the write lands, and nothing of a delivery's message (43, 62; weir 101,
107).

#### Scenario: Each delivery logged as it is recorded and let go of

- **WHEN** a fix for `ops` lands while no pane holds `ops`, is settled twice more, and `ops` then confirms its ends and the host is settled
- **THEN** the log holds one line for each of the code end and the landing as recorded, naming its key, its place, `ops` and its expiry, none more for the settles after, and one line for each as let go of, naming its key, `ops` and the removed document

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
