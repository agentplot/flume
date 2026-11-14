# Verify: a-place-whose-folder-git-dropped-is-removed

Verified at `dd20da83` on
`change/a-place-whose-folder-git-dropped-is-removed`, against its base
`bolt/flumes-places-need-no-hand` at `4e7fa04d`, which the branch holds.

**Verdict: holds. Archived.** The whole checks pass. Every requirement
and scenario of the change's two specs is met by a check that passed.
Every task is ticked done, and every test a task names is defined once
in the tree. The code stage archived the change as its task 4.1 asks,
at `dd20da83`. This report is committed into the archived change's
folder.

## The whole checks

`sluice exec bolt/flumes-places-need-no-hand`, the host's sluice
0.93.2, run once: **passed**, `exit 0`. sluice ran the checks `dd20da8`
affects against the base, the bolt's `#check-fast`: 17 tasks completed
and 16 passed. It recorded the 16 results under their keys for
`dd20da8`.

The run first asked through `devenv shell -- sluice exec …` stopped
before any check. Inside the shell, the sluice the tree pins, 0.71.0,
comes first on `PATH`, and it refused the host's
`~/.config/sluice/config.toml` (`unknown field credential`, line 21).
No check ran under it. The run above is the host's own sluice, which
enters the devenv toolchain itself, as `.config/sluice.toml`'s
`[tests] toolchain` says.

The tasks it ran:

- `root:format` (345 files, 0 changed), `root:adrs` (67 records, each
  with a number of its own), `root:specs` (49 passed, `tidying` and
  `place-profile` among them) and `root:changes` (1 passed).
- `flume:format`, `flume:lint` and `flume:test`.
- `conformance:format`, `conformance:lint`, `conformance:test` and
  `conformance:coverage`. Every clause of Parts A and B, I1 to I8 and
  S1 to S120 is cited by the suite's 931 scenarios.
- `nix:typecheck` and `nix:test`, and `plugin-claude:test`.
- `site:install`, `site:prose` (0 errors, 0 warnings) and `site:build`
  (every internal link valid).

`seams`, `core` and `fakes` were not affected, since the change touches
none of them.

The run's warnings are moon's: its output archives could not be written
from the gate's sandbox. No task failed for them.

Of note, in `flume:test`, every `place_git_worktree::*` test ran ok:
`what_git_left_of_a_failed_removal_goes_at_the_next_remove`,
`only_a_folder_flume_made_goes_from_a_places_location`,
`what_a_process_wrote_at_a_gone_places_location_goes`,
`a_group_folder_holding_another_place_stays`,
`a_worktree_another_repository_holds_at_a_places_location_is_kept`,
`a_place_made_inside_the_main_worktree_is_read_and_removed_where_it_stands`,
`a_place_made_beside_a_bare_repositorys_worktrees_is_read_and_removed_where_it_stands`
and `the_place_profile_passes_the_place_scenarios`.

## The spec: place-profile

### ADDED

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| What git left of a failed removal goes at a later remove | A failed removal refused | `what_git_left_of_a_failed_removal_goes_at_the_next_remove`. The first remove is refused at `git worktree remove`, its words holding git's `failed to delete` and `no longer lists`. The location stands, the place reads `Gone` and the teardown's mark holds one line. The second and third removes are `Removed`, and the mark still holds one line. | yes |
| Only a folder flume made goes from a place's location | A folder flume did not make | `only_a_folder_flume_made_goes_from_a_places_location`, parts 1 to 3. A location outside every `places/` reads `Gone` and is refused naming it and `no folder flume makes places in`, with `kept` standing. Another repository's dropped worktree reads `Gone` and is refused naming `another repository`, its `README` standing. A repository of its own reads `Gone` and is refused, `mine` standing. | yes |
| | The place's own dropped worktree | the same test, part 4. A place made by `made`, its entry dropped and a file written, reads `Gone`, and its remove is `Removed` with nothing standing at the location. | yes |

### MODIFIED

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| Nothing of a removed place stands at its location | What git could not delete | `what_git_left_of_a_failed_removal_goes_at_the_next_remove`. `devenv_writes` adds the link to `outputs` and the read-only `.devenv/state/stuck`. The first remove is refused with the folder standing. The second passes: nothing stands at the location, the empty group folder is gone, and `outputs/archive` stands. | yes |
| | What a process wrote at a gone place's location | `what_a_process_wrote_at_a_gone_places_location_goes`, unchanged | yes |
| | A group folder holding another place | `a_group_folder_holding_another_place_stays`, unchanged | yes |
| | Another repository's worktree at a place's location | `a_worktree_another_repository_holds_at_a_places_location_is_kept`, unchanged. `claimed` gives the same words for a `.git` file whose git directory stands. | yes |

The code: in `GitWorktree::remove`, the arm `(_, Ok(None)) =>
self.left(place)` is split. `(Err(words), Ok(None))` is refused at
`git worktree remove` with git's words, then `; git no longer lists
<location> as a worktree, and flume removes what is left of it at the
next settle` (D1). `(Ok(_), Ok(None))` calls `left` as before (record
0057). `left` asks `foreign` first, which takes over `linked`.
`in_places` holds a location directly in a group folder under
`<own>/places`, under any id, or directly in `<repository>/places`,
compared by `canonical`. `claimed` reads the `.git` there as D3 gives
it. Where nothing stands, it goes on to the group folder as before.

## The spec: tidying

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| A place read gone is removed all the same (MODIFIED) | A place read gone | conformance `a_place_read_gone` in `crates/flume-conformance/src/engine/tidying.rs`, passing in `conformance:test`. The done keeps the place after one remove. After `mend_teardown`, `world.engine.settle(Some(&id))` answers `Answer::Settled` with `tidy.removed` naming the place, and the fake place's log counts two removes. The scenario's name gains "and removed by the next settle", and its cites `76`. | yes |

No code of the core changed (D2). The settle's step of a place whose
work is over tidies a kept place again with no caller's word, and the
conformance scenario shows it.

## The tasks

All 5 tasks are ticked `[x]`.

- 1.1: `remove` refuses as D1 gives it, and the module's and `remove`'s
  comments say so, citing record 0057's amendment.
  `a_place_git_half_removes_leaves_no_folder` is gone from the tree,
  and `what_git_left_of_a_failed_removal_goes_at_the_next_remove`,
  defined once, passed.
- 1.2: `foreign`, `in_places` and `claimed` sit beside `clear`, and
  `left` asks `foreign` before it removes anything.
  `only_a_folder_flume_made_goes_from_a_places_location`, defined once,
  passed, and so did every other `place_git_worktree::` test, the
  places an earlier flume made among them.
- 2.1: `a_place_read_gone` settles rather than tidies, as Task notes
  2.1 give it. `conformance:test` and `conformance:coverage` passed.
- 3.1: `reference/commands.md` (`flume tidy`, the worktree's bullet)
  and `guides/places-and-herdr.mdx` ("Tidying", step 2) say it in the
  reader's words, naming git and no tool or host of ours (core.35).
  `site:prose` and `site:build` passed.
- 4.1: archived at `dd20da83`. `openspec/specs/place-profile/spec.md`
  and `openspec/specs/tidying/spec.md` hold the delta, and `root:specs`
  passed.

Record 0057's amendment of 2026-11-14, proposed, and core.18's details
are at `743861a5`. They cite clauses 64 to 66, 76, 80 and 81 and
sluice's record 0115, as the code's comments and the commits do.
`root:adrs` passed.

## Of note, not findings

- Record 0057's amendment is proposed, awaiting the operator's yes, as
  the rules ask of a record. core.18's details cite it.
- In part 3 of `only_a_folder_flume_made_goes_from_a_places_location`,
  a location holding a repository of its own is refused by `remove`'s
  own guard, which predates this change. Its words are `<location> is
  the repository's main worktree, which flume never removes`, not D3's
  `<location> holds a repository of its own`. Task notes 1.2 ask only
  that it be refused and its file stand, and both hold. `claimed`'s
  `.git` folder case is reached only where a `.git` folder appears
  after that guard, so it guards in depth.
