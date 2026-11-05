# Verify: a-places-readiness-is-one-design-across-the-tools

The work holds. It was read at `change/a-places-readiness-is-one-design-across-the-tools`
79ff4ec, which has main at 7e7eb88 merged in, against the proposal, the
design, the four delta specs and the tasks. Every requirement and
scenario is met, every task is ticked, and the checks the tasks name
pass.

## Checks

- `sluice exec main` at 79ff4ec, the main gate's `#check-fast` and
  `#check-tools`, under the pinned sluice 0.67.0, passed 14 tasks:
  `conformance:format`, `lint`, `test` and `coverage`; `flume:format`,
  `lint`, `test` and `tools`, the last running 85 tests in 4m12s;
  `plugin-claude:test`; `site:install`, `prose` and `build`; and
  `root:format`, `adrs` and `changes`.
- `sluice exec main -- moon run core:test root:specs` passed. sluice
  left these two out of the gate, because nothing this branch changes
  affects them. Task 4.2 names them, so they were run here.
- `openspec validate a-places-readiness-is-one-design-across-the-tools --strict`
  passed. Every requirement the deltas modify or remove is a header in
  `openspec/specs/`. The scenarios that each modified requirement holds
  on main are kept in the delta, so the archive drops none of them.

## Specs

### merge-queue-profile

- **Read, base and the place's data.** `for_place` gives `equip`,
  `sluice equip <base> --at <revision> --json`, built for the revision
  given, along with `sluice release` and the base's remote. It reads the
  policy only for where the base lives. `ForPlace` has no `toolchain`,
  `install` or `setup` field, and `PINS` is gone.
  - "A queue nothing drives" and "A fetch that fails" are held by the
    tools tests as before.
  - "What the policy says of a place" is held by
    `tools/merge_queue_sluice.rs` `what_the_policy_says_of_a_place`,
    against the pinned sluice. It checks one readiness step, its
    reading, the teardown and `origin`, and then the pinned sluice
    equipping a place and its answer being read.
  - "The steps sluice equipped with" is held by the unit test
    `the_steps_sluice_equipped_with`: four steps in order, the last
    one's words cut by `shell::tail`.
  - "An answer with no steps" is held by the unit test
    `an_answer_with_no_steps`, which covers a refusal's
    `error.message`, an unreadable answer, and a sluice that could not
    run, each as one failed `sluice equip` step.
- **REMOVED: A moon repository's pins installed before its setup.**
  #96's install and its two `place_git_worktree.rs` tests are gone. Its
  scenario is held as S89.

### place-profile

- **Readiness and teardown as the merge queue declares.** `Step` carries
  an optional `Reading`, and `prints_environment`, `Step::toolchain`
  and `TOOLCHAIN` are gone. `ready` runs each step under `bash -c` in
  the place, its environment cleared to the login environment, and
  extends the readiness by the reading. `BoundPlace::ready` builds the
  data for the revision it is given, through `place_data`.
  - "Readiness that fails at its toolchain" is held by
    `s16_readiness_that_fails_at_its_toolchain`: only the toolchain step
    with its words, no later step run, and nothing of the caller's
    environment.
  - "No merge queue bound" is held by
    `with_no_merge_queue_bound_nothing_runs`.
  - "Readied at the revision the launch read" is held by
    `a_place_is_readied_at_the_revision_its_launch_read`. It runs a
    stand-in sluice on the login `PATH`, moves main on, and checks that
    both asks were `equip main --at <launched> --json`.
  - "Nothing kept in the place's git directory" is held by
    `nothing_is_kept_in_the_places_git_directory`.
  - "A fresh clone's pins installed by the pinned sluice" is held by
    `tools/place_git_worktree.rs`
    `a_fresh_clones_pins_are_installed_by_the_pinned_sluice`, which runs
    on a fresh clone with stand-in proto and moon and no moon of the
    host's. Its readiness passes `toolchain`, `proto install`,
    `moon --version` and `moon setup`, and moon is installed under the
    clone's proto home. The companion test
    `without_the_install_moon_is_not_there_to_answer_its_version` fails
    at `moon --version`, as deneb's place did.
- The place seam's scenarios, `ready_installs_the_pins` among them, are
  unchanged and pass over the fake. `crates/flume-conformance/src/seams`,
  `flume-fakes`, `flume-seams` and `flume-core` have no diff against
  main, as the design requires (core.14).

### launching

- **Readiness that fails.** The core already passes the launch's
  revision at a retry: `acts.rs` passes `held.place.base_revision`.
  - "A change's place waits, a fix's starts" is held by
    `readiness_fails`, unchanged.
  - "A retry readies at the launch's revision" is held by the new
    scenario `a_retry_readies_at_the_launchs_revision`, citing 12 and
    20. It checks both `ready` lines at the launch's revision after
    main moved on, and that construct is running.
  - "Each step named as it passed" is held by the new scenario
    `each_step_named_as_it_passed`, citing S89 and 12. It checks four
    steps in order with their words, and that construct is running.

### clause-coverage

- `REVISION` is 6188a10 and `SCENARIOS` is 91. At
  agentplot/blueprints, 6188a10 is on main. It states S89, has no
  scenario after S91, and holds decision 7's amendment: the C.1 ready
  column, the C.2 comment and the C.6 place row naming `sluice equip`.
- `READINESS` is gone, and S89 is out of the unbuilt list. The unbuilt
  list names 186 to 190, S90 and S91 under record 0037's change, as
  main's does. `conformance:coverage` passes with S89 cited by the
  launching scenario.

## Rules and records

- Record 0036 is accepted, with the user's words and the date. Its
  consequences give blueprints 9f1f0fd.
- core.47 is in `docs/architecture/core.md` with its details as task 1.2
  words them. `site/.../architecture/rules.md` mirrors it word for word.
  core.3's details now let the sluice profile run `sluice equip` and
  `sluice release` in a place.
- core.1 and core.2 hold. `sluice` and `EQUIP` are named only in the
  sluice profile and the resolver (`profiles.rs`), as before. The place
  profile reads nothing of sluice's form: the reading is handed to it as
  a function.
- core.47 holds. No toolchain, install or setup is named or run by
  flume's code. No `[tests]` key or `.prototools` is read for a place.
  Nothing is kept under `<git-dir>/flume`. A retry readies at the
  launch's revision.
- core.35 holds. The site's changed pages name sluice, proto and moon as
  tools flume integrates with, and no host, team or word of ours. The
  places guide's example status shows sluice's four steps.

## Tasks

Groups 1 to 4 are all ticked: 1.1 to 1.3, 2.1 to 2.7, 3.1 and 3.2, and
4.1 and 4.2. Each is matched by the code, tests and documents above.
The checks each one names are covered by the runs above.

## Outside this repository

The proposal names changes in agentplot/sluice, afterthought/crew,
agentplot/boomdrive and afterthought/kit-standard. They are launched in
those repositories and are not part of this change's tasks. sluice's
part is released as 0.67.0, and this tree pins that release.

## Notes

- The merge-queue-profile delta rewrites the citations of "Read, base
  and the place's data". Main's version cited 9, 99, 125, 126, C.1, C.6
  and core.3. The delta's cites 9, 11, 20, 126, C.1, C.6 and core.3, so
  99 and 125 are dropped. The requirement still reads a branch's newest
  entry (99), and its scenario "A queue nothing drives" still cites 124
  and 125. No requirement goes unmet. A later change to this
  requirement can restore 99 and 125 to its citations (core.19).
- `openspec archive` warned that the proposal's Why section exceeds
  1000 characters. `--strict` validation passes.
