# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on main and on the
groups before it. A group's last task runs the checks it touched
through sluice. Every conformance scenario a group adds cites its
clauses as data (106).

Group 1 is built once the user says yes to the design and record 0036.
Groups 2 to 4 are built once three things hold:

- flume's and sluice's statements are amended on agentplot/blueprints
  main as design 7 words them;
- agentplot/sluice has released `sluice equip` as design 4 has it;
- this repository's `devenv.nix` pins that release, every command and
  field C.6 names checked against it (core.22; 2.7).

Tests reach no real account, no repository of the user's and no network.
Every git call in a test runs under the scratch environment that
`crates/flume/tests/common` gives. A proto, a moon or a sluice a test
puts on a step's path is a stand-in of the test's own, but in the tools
layer, where sluice is the one the tree pins.

## 1. The decision recorded

- [x] 1.1 Accept record 0036. Its status reads accepted by the user, with
  their words and the date. Its deciders name the user's yes. Its
  consequences give the blueprints revision that holds the statements'
  amendment (11, 20, 79, C.1, C.2, C.6, C.7, S89; core.19). True when
  `sluice exec main -- moon run root:adrs root:format` passes.
- [x] 1.2 Draw core.47 in `docs/architecture/core.md` under "Branches,
  submissions and landings", the next free id when this lands, citing
  11, 12, 20, 79, C.1 and C.6 and record 0036. The rule: "A place is made
  ready only by its merge queue's equipping of it, asked at the revision
  its launch read; flume runs no readiness step of its own, reads no
  tool's step from the merge queue's policy, and keeps nothing a step
  makes." Its details:
  - Rule out a toolchain, an install or a runner's setup named or run by
    flume's code; a policy key of `[tests]` read for a place; a
    readiness read at the base's revision at the moment of a retry rather
    than the launch's; and a step's environment kept in the place.
  - Do not rule out the sluice profile declaring `sluice equip` and
    reading its steps, or the teardown `sluice release`.

  Amend core.3's details, "the sluice profile reading
  `.config/sluice.toml` for where the base lives and the readiness, once
  bound", to "for where the base lives, once bound, and running
  `sluice equip` and `sluice release` in a place". Have
  `site/src/content/docs/architecture/rules.md` mirror core.47 as
  written (core.19, core.35). True when
  `sluice exec main -- moon run root:format site:prose site:build`
  passes.
- [x] 1.3 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run root:format root:adrs site:prose site:build`
  passes.

## 2. A place's readiness is sluice's equip

- [x] 2.1 Have the suite's clause list read the statement at the revision
  of agentplot/blueprints main that holds the amendment and states S89:
  `REVISION` that revision, and `SCENARIOS` 89 or what that revision
  holds. Parts A and B are as main's list holds them when this group
  builds. Name S89 unbuilt by this change, in a constant beside
  `PROFILES`, until 2.4's scenario cites it (clause-coverage spec;
  core.19). True when
  `sluice exec main -- moon run conformance:test conformance:coverage`
  passes.
- [x] 2.2 In `crates/flume/src/profiles/merge_queue/sluice/mod.rs`:
  - Have `for_place` give the place's readiness as design 5 has it: one
    step, `sluice equip <base> --at <revision> --json`, built for the
    revision `ready` is given, with the reading `equipped`. `ForPlace`
    loses `toolchain`, #96's `install` and `setup`, `PINS` goes, and no
    `[tests]` key and no `.prototools` is read for a place.
  - Have `equipped` read `steps[]` as `ReadyStep`s, each one's words cut
    as `git_worktree`'s `tail` cuts them.
  - Have it read a refusal, an answer it cannot read, or a sluice that
    could not run as one failed step, `sluice equip`, with sluice's
    words.
  - Update the module's comment to say what sluice equips and at which
    revision (11, 12, 20, 79, C.1, C.6; core.1, core.3).

  True when `sluice exec main -- moon run flume:test` passes, with unit
  tests holding the merge-queue-profile spec's scenarios "The steps
  sluice equipped with" and "An answer with no steps". It must also
  pass `sluice exec main -- moon run flume:tools`, with
  `tools/merge_queue_sluice.rs`'s `what_the_policy_says_of_a_place`
  holding the spec's scenario of that name against the pinned sluice,
  in place of #96's test of the install under the command runner and
  with no pins.
- [x] 2.3 In `crates/flume/src/profiles/place/git_worktree/mod.rs` and
  `crates/flume/src/profiles.rs`:
  - Have `Step` carry an optional reading of what the step printed, and
    lose `prints_environment`, `Step::toolchain` and `TOOLCHAIN`.
  - Have `GitWorktree::ready` run the readiness step as it runs every
    step, under `bash -c` in the place, with the login environment.
    Where the step has a reading, the readiness is the steps the reading
    gives.
  - Have `BoundPlace::ready` build the place's data for the revision it
    is given, through `place_data`, which takes that revision beside its
    repository, merge queue and base. The profile tests bind with it, as
    the binary does.
  - Update both modules' comments, and C.1's and C.7's citations in them
    (11, 12, 20, 79, C.1, C.7; core.2, core.17).

  True when `sluice exec main -- moon run flume:test` passes. That
  includes `place_git_worktree.rs` holding the place-profile spec's
  scenarios "Readiness that fails at its toolchain", over a stand-in
  step that answers its toolchain failed and records its environment;
  "No merge queue bound"; "Readied at the revision the launch read",
  over a stand-in `sluice` on the login path that records what it was
  asked, with the scratch repository's main moved on; and "Nothing kept
  in the place's git directory", which replaces
  `the_toolchain_environment_is_kept_in_the_places_git_directory`.
  #96's `pins_that_cannot_be_installed_stop_the_readiness_at_the_install_with_its_words`
  and `a_repository_that_pins_nothing_runs_no_install` go, since whether
  to install is sluice's to decide. The harness's `pinned` gives none
  here, where no sluice runs, and 2.5 holds it under the pinned sluice.
  The place seam's scenarios, `ready_installs_the_pins` among them, must
  pass unchanged over the fake (design 5).
- [x] 2.4 Add to `crates/flume-conformance/src/engine/launching.rs` the
  launching spec's scenarios:
  - "A retry readies at the launch's revision", citing 12 and 20, which
    reads the fake place's log of `ready <location> <revision>`;
  - "Each step named as it passed", citing S89 and 12.

  Take S89 out of the unbuilt list. True when
  `sluice exec main -- moon run core:test conformance:test conformance:coverage`
  passes, with `readiness_fails` passing unchanged.
- [x] 2.5 Add to `crates/flume/tests/profiles/tools/` the place-profile
  spec's scenario "A fresh clone's pins installed by the pinned sluice".
  A scratch moon repository on main has:
  - `.config/sluice.toml` naming the moon runner and a toolchain that
    prints a path holding a stand-in `proto`;
  - a `.prototools` pinning moon at 2.6.1;
  - a stand-in `proto` whose `install` makes a stand-in `moon` that
    answers `moon 2.6.1` and passes `setup`;
  - no moon of the host's on the step's path.

  It is cloned fresh, and a place of the clone is readied through
  `place_data` and the git worktree profile under the pinned sluice. Its
  readiness gives `toolchain`, `proto install`, `moon --version` and
  `moon setup`, each passed, as the pinned sluice names them, and the
  stand-in moon is installed. Without the install, its `moon --version`
  fails as deneb's place did (S89, 11, 79, C.1, C.6). True when
  `sluice exec main -- moon run flume:tools` passes, and every existing
  tools test passes unchanged.
- [x] 2.6 Run the checks of the projects this group touched. True when
  `sluice exec main -- moon run core:test conformance:format conformance:lint conformance:test conformance:coverage flume:format flume:lint flume:test flume:tools`
  passes.
- [x] 2.7 Pin sluice 0.67.0, the release that has `sluice equip`
  (sluice #379), in `devenv.nix` with its three release assets' hashes,
  main merged in first, as the answer to this stage's stop asked. Check
  equip's command and answer at that tag: `sluice equip <branch> [--at
  <revision>] [--json]` in the worktree it equips, answering `{branch,
  revision, steps: [{step, passed, words}]}`, exiting 0 where every step
  passed or none ran and 1 where one failed, a refusal `{error: {kind,
  message, next}}` exiting 2, and `[tests] toolchain` read only as a list
  of words. Every sluice a tools test starts keeps its scratch under
  `SLUICE_USER_SCRATCH`, a folder of the test's own, since the gate's
  sandbox refuses it `/tmp/sluice-<uid>` (sluice 35). The walkthroughs'
  host user's login shell is the walk's own, giving the walk's `PATH`, so
  a place's steps, equip and release, run the pinned sluice and not the
  one the host's login `PATH` names (11, 79, C.6; core.22). True when
  `sluice exec main -- moon run flume:tools` passes.

## 3. The site

- [x] 3.1 Write design 5's readiness into the site, to any host's
  operator, naming sluice, proto and moon as tools flume integrates with
  and no host, team or word of ours (core.35):
  - `guides/places-and-herdr.mdx`: a place's readiness is
    `sluice equip`, asked at the revision its launch read, and its
    example status shows sluice's four steps.
  - `guides/merge-queue.md`: the policy rows say sluice equips each new
    place, the toolchain and the runner's install and setup, and flume
    reads no `[tests]` key itself. This replaces #96's line on the
    install, and the places guide's step 2 that #96 added goes the same
    way.
  - `reference/state.md`: loses the `flume/toolchain.sh` row.
  - `architecture/overview.md`: says what the merge queue's profile gives
    a place.

  True when `sluice exec main -- moon run site:prose site:build`
  passes.
- [x] 3.2 Run the checks of the project this group touched. True when
  `sluice exec main -- moon run site:prose site:build root:format`
  passes.

## 4. Coverage and the gate

- [x] 4.1 Have the suite's clause list name nothing of this change
  unbuilt, with its constant gone and S89 cited by 2.4's scenario, as
  the clause-coverage spec's scenarios have it (core.19). True when
  `sluice exec main -- moon run conformance:test conformance:coverage`
  passes.
- [x] 4.2 Run the checks of the projects this change touched. True when
  `sluice exec main -- moon run core:test conformance:format conformance:lint conformance:test conformance:coverage flume:format flume:lint flume:test flume:tools site:prose site:build root:format root:adrs root:changes root:specs`
  passes.
