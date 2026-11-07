# Tasks

The work is on `chore/a-tidied-place-leaves-no-folder`, cut from
`bolt/flume-tidy-leaves-nothing-behind`. The statement is read on
agentplot/blueprints at 00d90e9 (62, 64 to 66, 81, C.1), C.1 amended
for record 0057 on its branch
`design/flume-a-tidied-place-leaves-no-folder` at 723ab51.

## 1. The profile and the tidy

- [x] 1.1 Have the git worktree profile's remove take what stands at a place's location once git holds no worktree there, after `git worktree remove`, whatever it exited, and for a place read gone, never through a link, each folder made writable where its removal is refused, three times in all; then the place's group folder under flume's own data directory once it is empty; refusing where a `.git` file there names a git directory that still stands (62, 81, C.1). True when `sluice exec bolt/flume-tidy-leaves-nothing-behind -- moon run flume:test` passes, with `tests/profiles/place_git_worktree.rs` holding `a_place_git_half_removes_leaves_no_folder`, `what_a_process_wrote_at_a_gone_places_location_goes`, `a_group_folder_holding_another_place_stays` and `a_worktree_another_repository_holds_at_a_places_location_is_kept`.
- [x] 1.2 Have a tidy hand a place it reads gone to the place seam's remove, a refusal keeping the place named with its step (62, 66, 81). True when `sluice exec bolt/flume-tidy-leaves-nothing-behind -- moon run core:test conformance:test conformance:coverage` passes, with `engine/tidying.rs` holding `a_place_read_gone`.

## 2. The spec and the records

- [x] 2.1 Modify place-profile's "A place is a plain git worktree" and add tidying's "A place read gone is removed all the same". True when `openspec validate a-tidied-place-leaves-no-folder --strict` passes.
- [x] 2.2 Record 0057, proposed, and rule core.18's details in `docs/architecture/core.md`. True when `sluice exec bolt/flume-tidy-leaves-nothing-behind -- moon run root:adrs` passes.

## 3. The gate

- [x] 3.1 Run the gate. True when `sluice exec bolt/flume-tidy-leaves-nothing-behind` passes and `openspec archive a-tidied-place-leaves-no-folder --yes` leaves `openspec validate --specs --strict` passing.
