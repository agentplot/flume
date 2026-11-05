# Tasks

The work is on `fix/a-place-is-cut-from-origins-copy-of-its-base`. Every
test that reads git runs on scratch repositories under the environment
`crates/flume/tests/common` gives, its origin a bare clone beside it.

## 1. The seams and the fakes

- [x] 1.1 Add to `flume-seams`: `BaseRevision::Unlanded`; `OriginBranch` and `History::origin_branch`; `BaseFrom` and `PlaceRecord::base_from`, read with none where a document lacks it and written with none skipped. Have the fake git hold each repository's `origin`, its branches or why its fetch fails, and the fake history read it (9, 56, 126, C.1). True when `sluice exec main -- moon run seams:test fakes:test` passes, with `round_trip.rs` holding each new type and a place held before it said where its base was read.

## 2. The launch

- [x] 2.1 Have the launch read its base by `base_now`: the merge queue's answer where it lands the base, else `origin_branch`, else the host's own branch where origin is missing or lacks it, a failed fetch refusing the launch; keep `base_from` on the place and give it in `PlaceStatus` (9, 56, 126, C.1; core.7). True when `sluice exec main -- moon run core:test conformance:test conformance:coverage` passes, with `engine/launching.rs` holding the launching spec's scenarios over the fakes and `seams/history.rs` and `seams/merge_queue.rs` the history-and-host and seam-contracts scenarios.

## 3. git and sluice

- [x] 3.1 Have `GitHistory::origin_branch` fetch by `Git::fetch_branch`, the one fetch of a branch the binding's read, the default branch's read and sluice's base now share, and tell a branch origin lacks by `Git::remote_holds`, `ls-remote --exit-code` exiting 2. Have sluice's `base_revision` answer `Unlanded` where the policy names no table for the base or no revision of it holds one (9, 20, 126, C.1, C.2, C.6). True when `sluice exec main -- moon run flume:test flume:tools` passes, with `tests/host.rs` holding the history scenarios over git with a stand-in origin, `tests/commands.rs` the launching spec's scenarios through the binary with a clone whose `main` lags origin's, and `tools/merge_queue_sluice.rs` the merge-queue-profile spec's scenario of a base the policy names no table for.
- [x] 3.2 Print where the base was read on the person's status line beside the base (56). True when `print.rs` unit tests hold each of the four and none.

## 4. The site and the records

- [x] 4.1 Say in `site/src/content/docs/guides/places-and-herdr.mdx` where a place is cut, and in `site/src/content/docs/reference/commands.md` that status gives where the base was read, to any host's operator (core.35). Record 0041, proposed, and rule core.7 and its details in `docs/architecture/core.md` and the site's mirror. True when `sluice exec main -- moon run site:prose site:build root:adrs` passes.

## 5. The gate

- [x] 5.1 Run the checks of the projects this change touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test core:format core:lint core:test fakes:format fakes:lint fakes:test conformance:format conformance:lint conformance:test conformance:coverage flume:format flume:lint flume:test flume:tools site:prose site:build root:format root:adrs root:changes root:specs` passes.
