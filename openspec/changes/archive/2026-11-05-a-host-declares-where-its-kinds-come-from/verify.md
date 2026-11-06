# Verify: a-host-declares-where-its-kinds-come-from

Read on 2026-11-05 at `59e3fc8`, the head of the branch
`change/a-host-declares-where-its-kinds-come-from`. The check covers the
change's proposal, design, eight delta specs and tasks. The branch
merged main at `9691c5b` in `7c7696d`. origin's main is now `f871275`,
three commits later (#113). Those commits touch only the-gate's spec
and CI, and merge with this branch with no conflict: `git merge-tree`
writes the tree `8ebf47b`.

## Verdict

**Holds; archived.**

- Every task is ticked.
- Every requirement of the eight delta specs is built, and each of their
  53 scenarios is held by a test that asserts its THEN.
- `sluice exec main` passes at `59e3fc8` against main at `f871275`,
  flume:tools among it.
- The archive leaves `openspec validate --specs --strict` passing, both
  on the branch and over the branch merged with main at `f871275`.

## Tasks

All 28 tasks of groups 1 to 9 are ticked, `[x]`, and none is open.
`openspec validate a-host-declares-where-its-kinds-come-from --strict`
passes.

- **Record 0040** is `accepted`. It quotes the user's words of
  2026-11-05, "Start its code" (1.1).
- **core.34's details** cite record 0040 for the following (1.1):
  - a bare name never resolving to a kind of a source the host
    declares;
  - a source's file, or anything in a `kinds_dir`, written by flume;
  - a source's fault refusing only the launches that name its source;
  - a program that manages the host writing a source's file and running
    `flume kinds lock --host --hash`.
- **The clause list** is at blueprints' `ff616eb`. It holds 191 and
  192, and S95 to S97 with no S92 to S94. 155, 156, 158 to 160, 191,
  192, S59 to S63 and S95 to S97 are built, and `PACKAGES` is gone
  (1.3, 8.1, 9.1).
- **weir-kinds** is pinned at weir's `b431004`. `Cargo.toml` moves that
  pin alone (2.1).
- **The merge of main at `9691c5b`** (9.1) is `7c7696d`, whose parents
  are `984665e` and `9691c5b`. Its resolution keeps main's side:
  - `REVISION` moves from main's `6188a10` to `ff616eb`, the scenarios
    run to S97 with S92 to S94 skipped, and main's 180 to 190 and S82
    to S91 stay built;
  - a launch reads the base through main's `base_now`, resolves its kind
    by identity, and refuses a stream's multiplexer flume carries no
    profile for before anything is made, in main's order (190; core.8);
  - `HostConfig::read` and `HostConfig::read_layers` both read
    `sources/` beside the managed layer.
- **The local layer declares no source** (9.2).
  `tests/host.rs`'s `the_local_layer_declares_no_source` reads the
  source in `sources/` through `read_layers` in two cases. Under a
  local layer that is taken, `acme` alone is declared. Under a local
  layer naming `[sources.beta]`, that layer is not taken, the refused
  keys begin with `sources`, and `acme` alone is still declared.
  core.45's details rule out a source declared by the local layer,
  citing 191 and record 0040.
- **The new capability's Purpose** (9.3). `specs/kind-sources/spec.md`
  opens with a `## Purpose` of more than 50 characters, citing 19, 21,
  154 to 160, 191 and 192.
- **Each group's closing task** (3.3, 4.4, 5.4, 6.4, 7.3, 8.2, 9.4)
  names the checks it ran through sluice. The gate run below backs them
  all at the branch's head.

## Requirements and scenarios

Each of the eight specs' 21 requirements is built, and each of their 53
scenarios is held by a test that asserts its THEN. Every test named
below was found by name at `59e3fc8`. The conformance scenarios run over
the fakes. The profile, host, command and walkthrough tests run over
scratch directories, repositories and hosts.

### kind-sources (10 requirements, 29 scenarios)

| Requirement | Held by |
|---|---|
| The host declares each source in a file of its own | `tests/host.rs`: `a_source_written_beside_and_renamed`, `entries_that_are_no_source`, `no_sources_directory`; the local layer declaring none, `the_local_layer_declares_no_source` |
| A source's fault refuses only the launches naming it | `engine/kind_sources.rs`: `a_file_with_no_kinds_dir`, `a_file_taking_flumes_name`, `a_kinds_dir_not_yet_written`; `tests/host.rs`'s `each_fault_is_its_sources_own`, one case per fault the requirement lists; `tests/commands.rs`'s `a_source_file_with_no_kinds_dir`, through the binary |
| A source holds packages side by side | `two_versions_side_by_side`, `a_part_outside_and_an_environment`, `a_package_at_another_identity`; at the profile, `kinds_weir.rs`'s `a_package_at_another_identity_is_refused_naming_both_versions` and `a_part_outside_the_package_an_environment_and_a_program_by_a_location` |
| A launch names a kind by its identity | `a_source_the_host_does_not_declare`, `a_version_the_source_does_not_hold`, `the_repositorys_and_the_launchs_by_source` |
| A bare name keeps its meaning | `s97_a_source_that_holds_fix`, `s62_a_name_only_the_hosts_sources_hold` |
| A kind from a source runs only as a lock records it | `s60_unlocked_then_locked_by_the_repository`, `s61_an_edited_prompt`, `two_locks_that_disagree`, `a_lock_that_cannot_be_read`; through the binary, `tests/commands.rs`'s `unlocked_then_locked_by_the_repository` and `a_lock_that_cannot_be_read` |
| A launch naming no version takes the locks' version | `s59_the_repository_locks_change_1`, `a_version_not_yet_locked`, `the_newest_held_unlocked` |
| flume's own kinds are packages whose content never changes | `tests/profiles/built_in.rs`'s `the_hashes_recorded`; `a_lock_of_flumes_own_at_another_hash` |
| flume kinds lock records a kind's content | `tests/commands.rs`: `kinds_lock_writes_the_hosts_lock`, `kinds_lock_writes_a_working_copys_lock`, `kinds_lock_refuses_a_hash_the_content_does_not_have`, `kinds_lock_refuses_an_identity_no_lock_records`, and `kinds_lock_two_writers_at_once_and_a_removal`, which starts its two writers as two processes together; over the fakes, `kinds_locked`, `a_hash_the_content_does_not_have`, `an_identity_no_lock_records`, `a_removal_run_twice` and `a_lock_kinds_lock_cannot_read` |
| A place keeps the kind its source gave at its launch | `s95_the_source_moved_under_a_place`; the walkthrough `a_hosts_source_runs_its_locked_kind_and_a_place_keeps_its_version` |

The scenario *No sources directory* has two halves.
`no_sources_directory` holds the first: the host declares no source.
Every existing launch of `fix` holds the second, since the fake host and
every scratch host of the walkthroughs declare no source.

### The modified and added capabilities

- **launching** (5 scenarios), in `engine/launching.rs`:
  - `the_identity_status_names` holds *No version named* and *change@1
    named*;
  - the existing scenarios holding *A version flume does not hold* and
    *A version the repository's declaration does not hold* pass
    unchanged;
  - `a_kind_of_a_hosts_source_named_whole` holds the fifth.
- **kinds-profile** (5), in `tests/profiles/kinds_weir.rs`:
  `a_packages_hash_wherever_it_lies`, `a_sources_listing`,
  `the_repositorys_lock_at_a_revision`, `a_lock_written_and_read_back`
  and `flumes_own_as_packages`. The profile reads, spells and hashes
  kinds only through `weir_kinds` (core.4).
- **place-status** (4): `engine/place_status.rs`'s
  `a_kind_of_a_hosts_source`, `the_repositorys_a_launchs_and_flumes_own`,
  `a_place_launched_before` and `a_source_and_a_fault` hold all four.
  `print.rs`'s unit tests hold the person's form:
  `a_kind_of_a_hosts_source`,
  `the_repositorys_a_launchs_and_flumes_own`,
  `a_place_launched_before_this_change` and
  `the_hosts_sources_with_a_fault`.
- **role-waking** (2), in `engine/role_waking.rs`:
  `a_role_woken_to_a_hosts_source` and `a_roles_kind_unlocked`.
- **seam-contracts** (4):
  - `seams/kinds.rs`'s scenarios, each read twice to one answer;
  - `a_kinds_text_read_by_the_seam`;
  - `round_trip.rs`'s `a_held_state_with_where_its_kind_came_from`;
  - `sources_scripted_on_the_host`.
- **seam-fakes** (1): `a_package_edited_after_a_launch`.
- **clause-coverage** (3), in `clauses.rs`:
  - its two tests hold *The clauses listed*: Part A and Part B 1 to 192
    without 100 and 150, S92 to S94 absent, S91 and S95 to S97 present,
    and none of 154 to 160, 191, 192, S59 to S63 or S95 to S97 named
    unbuilt;
  - `coverage.rs` holds *A clause left out*;
  - `conformance:coverage` in the gate holds *The suite as this change
    leaves it*.

## Code against the design

`resolve_kind` in `crates/flume-core/src/launch.rs` follows design 3 to
5:

- **A source named** is the only one searched.
- **A host's source's fault** refuses in `host_source` before any of
  its packages is read.
- **A bare name** goes through the launch's declaration, then the
  repository's, then flume's own, and never reaches a host's source.
- **The locks** are read only where the kind is flume's own or a
  host's source, so a lock that cannot be read refuses no launch of the
  repository's own `bump` (core.34).
- **A package** is read before its locks are checked, so the hash
  checked is the one just read (156).

`crates/flume/src/host/config.rs` reads `sources/` beside the managed
layer's file, and only there. A source's fault stays with that source.
A `sources/` that cannot be listed fails the configuration, as design 1
has it.

The lines this change adds to the site, the README, the plugins and the
commands' output were searched for each word core.35's details list.
None of them appears.

## Checks

- **The gate.** `sluice exec main` at `59e3fc8` ran the checks `59e3fc8`
  affects against main at `f871275`. All 38 tasks completed, and sluice
  recorded 37 passes under their keys for `59e3fc8`:
  - `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint
    and test;
  - `conformance:coverage`;
  - `flume:tools`, 94 of 94 in 438.86 s, the walkthrough
    `a_hosts_source_runs_its_locked_kind_and_a_place_keeps_its_version`
    among them;
  - `plugin-claude`: format, validate, engine-types, typecheck and test;
  - `plugin-herdr`: manifest, lint and test;
  - `site:prose` and `site:build`;
  - `root`: format, hooks, toolchain, adrs, changes, specs, layers,
    versions and marketplace.
- **The change.** `openspec validate
  a-host-declares-where-its-kinds-come-from --strict` passes.
- **The archive over main.** `openspec archive
  a-host-declares-where-its-kinds-come-from --yes` (openspec 1.14.3) was
  run on a copy of `openspec/` from the tree `8ebf47b`, which is
  `59e3fc8` merged with main at `f871275`. It syncs the eight specs
  with no error, adding 18 requirements and modifying 3:
  - kind-sources' 10 are new;
  - kinds-profile, role-waking and seam-fakes each gain 1;
  - place-status gains 2, and seam-contracts 3;
  - launching's one requirement and clause-coverage's two are modified.

  `openspec validate --specs --strict` over that copy then passes 42 of
  42. `openspec/specs/kind-sources/spec.md` carries the delta's Purpose.
- **The archive on the branch.** It is run in the commit that archives
  the change, with the same totals and `openspec validate --specs
  --strict` passing over the specs it leaves.
