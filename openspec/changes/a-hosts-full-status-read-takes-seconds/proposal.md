# A host's full status read takes seconds

## Why

On deneb, with 58 places, `flume status --json` took 4 min 50 s on
2026-11-05 while five of herdr's watches read status too, and 2 min 2 s
and 4 min 10 s earlier. A board that reads every host's places within
30 s never showed deneb's.

A status asked the merge queue once for each place, though C.6 reads
read-submission "once for each repository at each ask": `sluice
overview --json` and `sluice status <base> --all --json` for each base,
again for every place. It read the places one after another, and each
live agent twice, once for its place and once for its base's count
(record 0050).

## What Changes

- **Each repository's merge queue is read once for a status (99, C.6).**
  The merge-queue seam answers read-submission for several branches of
  one repository from one read, and read-submission of one branch is
  that read of the one branch. A status asks each repository once for
  all its places. Nothing is kept between asks (58; core.15).
- **The places are read side by side (58).** A status reads its places,
  its repositories' merge queues and the agents it did not read for a
  place on up to eight threads, and `status --line` its lines the same
  way.
- **Each live agent is read once for a status (139).** The base counts
  take the state status read for each place's agent.
- **The answer is the same.** No field changes, and on a fixture of 58
  places the before and after binaries gave equal JSON.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `merge-queue-profile`: read-submission runs sluice's overview and each
  base's status once for every branch of a repository asked at once.
- `place-status`: status reads its places side by side, each
  repository's merge queue once and each live agent once.
- `seam-contracts`: the merge-queue seam reads several branches at once.

## Impact

- `crates/flume-seams`: `MergeQueue::read_submissions`, and
  `read_submission` given by it.
- `crates/flume-core`: `side_by_side`; status's `submissions`, its
  places read side by side and its agents' states handed to the base
  counts; `lines` side by side.
- `crates/flume`: the sluice profile's `read_submissions`; a tools test
  counting sluice's commands.
- `crates/flume-fakes`: the fake merge queue's `read_submissions`.
- `crates/flume-conformance`: the seam's scenario of two branches at
  once, and status's of three places of one repository.
- `docs/`: record 0050, proposed; rule core.15's details.
- No new crate, plugin or site part, so no new moon project, and no new
  dependency or tool. The statement is unchanged: C.6 and 58 already
  say this.
