# Tasks

The work is on `change/a-place-whose-folder-git-dropped-is-removed`,
cut from `bolt/flumes-places-need-no-hand`, the base every `sluice
exec` below names. The statement is read at boomdrivedev/blueprints
main 81288ba (64 to 66, 74, 76, 80, 81, C.1). Record 0057's amendment
of 2026-11-14 and rule core.18's details are written with this change.
A task whose code departs from them amends them in its own commit.

Commit by pathspec, Conventional Commits subjects scoped to the part of
the tree touched, citing record 0057's amendment and the clauses.

## 1. The place profile

- [x] 1.1 In `crates/flume/src/profiles/place/git_worktree/mod.rs`, have `remove` refuse at `git worktree remove` where git's removal failed and git then lists no worktree at the location, its words git's and then that git no longer lists the place and flume removes what is left of it at the next settle, removing nothing more in that call; a removal that passed still calls `left` (design D1, Task notes 1.1). Replace `a_place_git_half_removes_leaves_no_folder` in `crates/flume/tests/profiles/place_git_worktree.rs` with `what_git_left_of_a_failed_removal_goes_at_the_next_remove`, as Task notes 1.1 give it. True when `sluice exec bolt/flumes-places-need-no-hand -- cargo test -p flume --test profiles place_git_worktree::what_git_left_of_a_failed_removal_goes_at_the_next_remove` passes.
- [ ] 1.2 Have `left` remove what stands at a location only where its parent is a group folder directly under flume's own `places/` or is `<repository>/places`, and where it holds no `.git` folder and no `.git` file but one naming the repository's own worktree entry under `<common>/worktrees` that no longer stands, refusing otherwise with D3's words (design D3, Task notes 1.2). Add `only_a_folder_flume_made_goes_from_a_places_location` in `crates/flume/tests/profiles/place_git_worktree.rs`, as Task notes 1.2 give it. True when `sluice exec bolt/flumes-places-need-no-hand -- cargo test -p flume --test profiles place_git_worktree::` passes, the new test and the module's tests of places read gone and of places an earlier flume made among them.

## 2. The tidy

- [ ] 2.1 In `crates/flume-conformance/src/engine/tidying.rs`, have `a_place_read_gone` remove the place at its second remove by a settle, `world.engine.settle(Some(&id))`, its answer naming the place removed after two removes, its name gaining "and removed by the next settle" and its cites `76` (design D2, Task notes 2.1). True when `sluice exec bolt/flumes-places-need-no-hand -- cargo test -p flume-conformance --test engine` passes.

## 3. The site

- [ ] 3.1 Say it in `site/src/content/docs/reference/commands.md` (`flume tidy`, the worktree's bullet) and `site/src/content/docs/guides/places-and-herdr.mdx` ("Tidying", step 2), as Task notes 3.1 give it. True when `sluice exec bolt/flumes-places-need-no-hand -- moon run site:prose` passes.

## 4. The spec

- [ ] 4.1 Archive the change with `openspec archive a-place-whose-folder-git-dropped-is-removed --yes`. True when `sluice exec bolt/flumes-places-need-no-hand -- moon run root:specs` passes.
