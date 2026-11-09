## ADDED Requirements

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
