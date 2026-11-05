# Tasks

Each group's work is on `fix/a-places-readiness-installs-its-repositorys-pinned-tools`:
groups 1 and 2 at 0a3f138, group 3 at 42152b9. Every conformance
scenario cites its clauses as data (106), and names no tool (core.1).
Every git call in a test runs under the scratch environment
`crates/flume/tests/common` gives, and no test reaches the host's own
proto or moon.

## 1. The install the policy names

- [x] 1.1 Have `for_place` in `crates/flume/src/profiles/merge_queue/sluice/mod.rs` give `ForPlace`'s `install`, `proto install --config-mode upwards`, where the policy's runner is moon and the revision `policy_at` read the policy at holds `PINS`, `.prototools`, and none under another runner or where that revision pins nothing. The pins are read at the base's revision, never from a place's tree, and the module's comment says so (11, 20, C.6; core.3). True when `sluice exec main -- moon run flume:tools` passes with `what_the_policy_says_of_a_place` in `crates/flume/tests/profiles/tools/merge_queue_sluice.rs` holding no install under the command runner, none under moon with nothing pinned, and the install once main's `.prototools` pins moon.
- [x] 1.2 Have the resolver's `place_data` in `crates/flume/src/profiles.rs`, which `BoundPlace::data` calls, give the place profile the toolchain, the install and the runner's setup as three steps, each of its own, the install after the toolchain so it runs under the toolchain's environment, and none where no merge queue is bound (11, 12, 79; core.2). True when `sluice exec main -- moon run flume:test` passes with `pins_that_cannot_be_installed_stop_the_readiness_at_the_install_with_its_words` and `a_repository_that_pins_nothing_runs_no_install` in `crates/flume/tests/profiles/place_git_worktree.rs`, the merge-queue-profile spec's scenarios of an install that fails and of a repository that pins nothing.

## 2. Conformance

- [x] 2.1 Add to the place seam's scenarios in `crates/flume-conformance/src/seams/place.rs` `ready_installs_the_pins`, citing 79, 11 and 12: a repository whose pins nothing installed is made ready, every step passing, the install a step of its own whose words say what it installed, and a step after it. Give `PlaceHarness` `pinned` and its answer `Pinned`, the fake's in `fake_harness.rs` and the lax harness's in `suite.rs` (core.2). True when `sluice exec main -- moon run conformance:test conformance:coverage` passes.
- [x] 2.2 Have the git worktree profile's harness in `crates/flume/tests/profiles/place_git_worktree.rs` answer `pinned` with `moon_repository`: a sluice policy whose runner is moon, a toolchain that prints the environment devenv's does, proto's home in the git common directory and its installed tools on `PATH`, installing none, a stand-in for proto that installs a stand-in moon, and no moon of the host's on the toolchain's `PATH`, bound by `place_data` as the resolver binds it. This is the merge-queue-profile spec's scenario of pins nothing installed (11, 12, 79). True when `sluice exec main -- moon run flume:test` passes with `the_place_profile_passes_the_place_scenarios`.

## 3. The site

- [x] 3.1 List the install in `site/src/content/docs/guides/places-and-herdr.mdx` between the toolchain and `moon setup`, when the runner is moon and the base holds a `.prototools`, run under the toolchain's environment as sluice's moon runner runs it. Have the moon runner's row in `site/src/content/docs/guides/merge-queue.md` say the install comes first. Write both to any host's operator, naming no host, team or word of ours (core.35). True when `sluice exec main -- moon run site:prose site:build` passes.

## 4. The gate

- [x] 4.1 Run the checks of the projects this change touched. True when `sluice exec main -- moon run flume:format flume:lint flume:test flume:tools conformance:format conformance:lint conformance:test conformance:coverage site:prose site:build root:format root:changes root:specs` passes.
