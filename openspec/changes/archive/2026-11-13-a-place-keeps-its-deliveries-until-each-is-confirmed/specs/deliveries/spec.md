## ADDED Requirements

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
