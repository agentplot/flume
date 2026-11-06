# Tasks

The work is on `fix/a-hosts-full-status-read-takes-seconds`. It is
measured on a fixture of 58 places in four scratch repositories, each
with two bases holding 30 landed submissions, under the printed command,
real git, sluice 0.67.0 and no account.

## 1. The seam and the profile

- [x] 1.1 Add `MergeQueue::read_submissions(repository, branches)`, each branch's newest submission in the order asked from one read, and give `read_submission` by it; have the sluice profile run `sluice overview --json` and each base's `sluice status <base> --all --json` once for every branch asked, and the fake read once for them all (99, C.6). True when `sluice exec main -- moon run seams:test fakes:test conformance:test flume:test flume:tools` passes, with the seam's scenario of two branches at once run against the fake and sluice, and `tools/merge_queue_sluice.rs` counting one `overview` and one `status` for three branches.

## 2. Status

- [x] 2.1 Have status ask each repository's merge queue once for all its places, read its places and repositories side by side on up to eight threads, and hand the agents' states it read to the base counts, which read the agents it did not read side by side; read `status --line`'s lines side by side (58, 139). True when `sluice exec main -- moon run core:test conformance:test conformance:coverage` passes, with `engine/place_status.rs` holding the place-status spec's three places of one repository.

## 3. The measure and the records

- [x] 3.1 Measure a full status on the fixture before and after, alone, six at once and beside five loops of a full settle and a full status, with the processes one read runs, and compare the two answers; give the numbers in the commit. Record 0050, proposed, and rule core.15's details in `docs/architecture/core.md`. True when `sluice exec main -- moon run root:adrs` passes and the answers are equal.

## 4. The gate

- [x] 4.1 Run the checks of the projects this change touched. True when `sluice exec main` passes.
