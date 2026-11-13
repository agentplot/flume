# A place keeps its deliveries until each is confirmed

## Why

Under deneb's runner, a landed chore's ends could not be confirmed as
`boomdrive`: flume refused the delivery key
`chore--denebs-runner-config-names-its-forge/3c57fed9/stage/code/2/delivered`,
"no delivery is recorded under" it (the runner's log, 2026-11-13
02:24:00Z; signal 2026-11-13-agentplot-ops-f9cf9852/01). The runner
copies `ends[].delivery` from `flume status`, and status named a key the
host's deliveries did not hold: status gives an end's key whenever the
end has a recipient, recorded or not, and an end whose delivery is not
recorded reads pending for each recipient, so a key no settle recorded,
or one let go of, reads owed and confirmable. A host's settle also lets
go of a done delivery whose place it reads no document of, from one read
of every place, and a tidy that moves a place's document to the one kept
for its ends while that read lists and reads them leaves the place out
of it. flume's log said nothing of either.

The design ruled: status names only delivery keys flume has recorded; a
tidied place keeps its deliveries until each recipient confirms them
(43, 62); and flume's log names each delivery as it is recorded and as
it is let go of, a test for each. The statement holds already: 43
records each end once as a delivery, owed until each recipient confirms
it, and 62 keeps a tidied place's document while an end is pending and
lets go of its deliveries once the document goes. This change makes the
build hold to them.

## What Changes

- **Status names only a recorded key (43, 56; weir 101, 102).** An
  end's `delivery` in status, the inbox and a wait is the key of its
  delivery as the host's deliveries hold it, and none while none is
  recorded. The form gains and loses no field (61).
- **A place's deliveries go only with its last document (62; weir 107,
  I9).** A host's settle lets go of a done delivery only where two reads
  of every place, the second begun once the first ended, both lack its
  place's document. A tidy removes a place's own document only once the
  document kept for its pending ends stands, and otherwise keeps it and
  says why in flume's log.
- **flume's log names each delivery.** Each delivery a settle records,
  with its place, its readers and its expiry; and each delivery let go
  of, with the readers that confirmed it and why.
- **The fake store** moves a document inside a read of every place, and
  refuses one place's next write, so the suite can show both.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `deliveries`: status names only a recorded key; a place's deliveries
  are let go of only once no document of it stands; flume's log names
  each delivery recorded and let go of.
- `seam-fakes`: the fake store moves a document inside a read of every
  place and refuses one place's next write.

## Impact

- `crates/flume-core`: `answer.rs` (`EndStatus::of`), `deliveries.rs`
  (the record's and the let-go's log lines, the let-go of the gone's two
  reads), `ends.rs` (`keep_for_ends` says why it kept nothing),
  `tidy.rs` (the place's own document kept while the one kept for its
  ends cannot be written).
- `crates/flume-fakes`: `store.rs`.
- `crates/flume-conformance`: four end-delivery scenarios.
- A caller that read an end's `delivery` before its delivery was
  recorded reads none until the next settle records it. The runner
  leaves out an end with no `delivery` (boomdrive's `facts::owed`), so
  it confirms only what flume recorded.
