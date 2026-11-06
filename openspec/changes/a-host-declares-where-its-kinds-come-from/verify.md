# Verify: a-host-declares-where-its-kinds-come-from

Read on 2026-11-05 at `94570ca`, the branch
`change/a-host-declares-where-its-kinds-come-from`. The check covers the
change's proposal, design, eight delta specs and tasks. The branch last
took main at `201086c`, and origin's main is now `9691c5b`, nine
landings later (#98 to #112).

## Verdict

**Falls short; not archived.** Every task is ticked. Every scenario of
the eight delta specs is held by a test that asserts its THEN, and
`sluice exec main` passes at `94570ca`. But the change cannot yet be
archived to a tree the gate passes.

- **What fails.** The delta spec `specs/kind-sources/spec.md` adds a
  new capability with no `## Purpose`. Archived, it leaves
  `openspec/specs/kind-sources/spec.md` with openspec's placeholder
  Purpose. `openspec validate --specs --strict`, which is `root:specs`,
  then fails 34 of 35 on `spec/kind-sources`.
- **The fix.** A `## Purpose` at the head of the delta, as main's new
  capabilities carry one (`stream-views`, `stream-terminals`), of 50
  characters or more. A trial archive on a copy with one added passes
  35 of 35. Editing the archived spec by hand is no fix, since
  `openspec/specs/` changes only through a change.

The place goes back to its code stage with these words. That round can
also merge main, which the landing needs anyway (see *The merge with
main*).

## Tasks

All 24 tasks of groups 1 to 8 are ticked, `[x]`, and none is open.
`openspec validate a-host-declares-where-its-kinds-come-from --strict`
passes.

- **Record 0040** is `accepted` and quotes the user's words of
  2026-11-05, "Start its code" (1.1).
- **core.34's details** name the following, each citing record 0040
  (1.1):
  - a bare name never resolving to a kind of a source the host
    declares;
  - a source's file, or anything in a `kinds_dir`, written by flume;
  - a source's fault refusing only the launches that name its source;
  - a program that manages the host writing a source's file and running
    `flume kinds lock --host --hash`.
- **The clause list** is at blueprints' `ff616eb`, the revision that
  states the user's ruling: 191 and 192, and S95 to S97 with no S92 to
  S94 (1.3, 8.1).
- **weir-kinds** is pinned at weir's main `b431004`, and nothing else
  moved (2.1).
- **Each group's closing task** (3.3, 4.4, 5.4, 6.4, 7.3, 8.2) names
  the checks it ran through sluice. The gate run below backs them all
  at the branch's head.

## Requirements and scenarios

Each of the eight specs' 21 requirements is built, and each of their 53
scenarios is held by a test that asserts its THEN. The conformance
scenarios run over the fakes. The profile, host, command and walkthrough
tests run over scratch directories, repositories and hosts.

### kind-sources (10 requirements, 29 scenarios)

| Requirement | Held by |
|---|---|
| The host declares each source in a file of its own | `tests/host.rs`: `a_source_written_beside_and_renamed`, `entries_that_are_no_source` (a link reads as its file), `no_sources_directory` |
| A source's fault refuses only the launches naming it | `engine/kind_sources.rs`: `a_file_with_no_kinds_dir`, `a_file_taking_flumes_name`, `a_kinds_dir_not_yet_written`; `tests/host.rs`: `each_fault_is_its_sources_own`, covering each fault the requirement lists; `tests/commands.rs`: `a_source_file_with_no_kinds_dir`, through the binary |
| A source holds packages side by side | `two_versions_side_by_side`, `a_part_outside_and_an_environment`, `a_package_at_another_identity`; at the profile, `kinds_weir.rs`'s `a_package_at_another_identity_is_refused_naming_both_versions` and `a_part_outside_the_package_an_environment_and_a_program_by_a_location` |
| A launch names a kind by its identity | `a_source_the_host_does_not_declare`, `a_version_the_source_does_not_hold`, `the_repositorys_and_the_launchs_by_source` |
| A bare name keeps its meaning | `s97_a_source_that_holds_fix`, `s62_a_name_only_the_hosts_sources_hold` |
| A kind from a source runs only as a lock records it | `s60_unlocked_then_locked_by_the_repository`, `s61_an_edited_prompt`, `two_locks_that_disagree`, `a_lock_that_cannot_be_read`; through the binary, `tests/commands.rs`'s `unlocked_then_locked_by_the_repository` and `a_lock_that_cannot_be_read` |
| A launch naming no version takes the locks' version | `s59_the_repository_locks_change_1`, `a_version_not_yet_locked`, `the_newest_held_unlocked` |
| flume's own kinds are packages whose content never changes | `tests/profiles/built_in.rs`: `the_hashes_recorded`; `a_lock_of_flumes_own_at_another_hash` |
| flume kinds lock records a kind's content | `tests/commands.rs`: `kinds_lock_writes_the_hosts_lock`, `kinds_lock_writes_a_working_copys_lock`, `kinds_lock_refuses_a_hash_the_content_does_not_have`, `kinds_lock_refuses_an_identity_no_lock_records`, and `kinds_lock_two_writers_at_once_and_a_removal`, whose two writers are two processes started together, four rounds over; over the fakes, `kinds_locked`, `a_hash_the_content_does_not_have`, `an_identity_no_lock_records`, `a_removal_run_twice` and `a_lock_kinds_lock_cannot_read` |
| A place keeps the kind its source gave at its launch | `s95_the_source_moved_under_a_place`; the walkthrough `a_hosts_source_runs_its_locked_kind_and_a_place_keeps_its_version` |

The scenario *No sources directory* has two halves.
`no_sources_directory` holds the first: the host declares no source.
Every existing launch of `fix` holds the second, since the fake host and
every scratch host of the walkthroughs declare no source.

The requirement that the local layer declares no source holds on this
branch because the branch has no local layer at all. The local layer,
`config.local.toml`, came to main after `201086c`. A source is a file
in `sources/`, not a key of any TOML file, so no key of the local
layer can declare one. The merge with main should keep it so (see
*The merge with main*).

### The modified and added capabilities

- **launching** (5 scenarios), in `engine/launching.rs`:
  - `the_identity_status_names` holds *No version named* and *change@1
    named*;
  - the existing scenarios holding *A version flume does not hold* and
    *A version the repository's declaration does not hold* pass
    unchanged;
  - `a_kind_of_a_hosts_source_named_whole` holds the fifth.
- **kinds-profile** (5), in `tests/profiles/kinds_weir.rs`:
  - `a_packages_hash_wherever_it_lies`, through a link, the hash moving
    with one byte;
  - `a_sources_listing`;
  - `the_repositorys_lock_at_a_revision`;
  - `a_lock_written_and_read_back`, a failed write leaving the file as
    it was;
  - `flumes_own_as_packages`.

  The profile reads, spells and hashes only through `weir_kinds`:
  - `read_named`, `read_source`, `read_package`, the locks and
    `write_lock` each call weir-kinds;
  - the core's one `sha256` is the existing intent digest, which hashes
    no kind (core.4).
- **place-status** (4):
  - `engine/place_status.rs` holds all four;
  - `print.rs`'s unit tests `a_kind_of_a_hosts_source`,
    `the_repositorys_a_launchs_and_flumes_own`,
    `a_place_launched_before_this_change` and
    `the_hosts_sources_with_a_fault` hold the person's form.
- **role-waking** (2), in `engine/role_waking.rs`:
  - `a_role_woken_to_a_hosts_source` runs `acme/turn@2` and names the
    source's file;
  - `a_roles_kind_unlocked` makes no place.
- **seam-contracts** (4):
  - `seams/kinds.rs`'s eight scenarios each read twice to one answer;
  - `a_kinds_text_read_by_the_seam`;
  - `round_trip.rs`'s `a_held_state_with_where_its_kind_came_from`;
  - `sources_scripted_on_the_host`, which reads no package of `bad`.
- **seam-fakes** (1): `a_package_edited_after_a_launch`, one
  `read_package` per launch.
- **clause-coverage** (3), in `clauses.rs`:
  - its two tests hold *The clauses listed*: Part A and Part B 1 to 192
    without 100 and 150, S92 to S94 absent, and none of 154 to 160,
    191, 192, S59 to S63 or S95 to S97 named unbuilt;
  - `coverage.rs` holds *A clause left out*, clause 70;
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
  `resolves_to_none` lists the host's sources only to name them in the
  refusal.
- **The locks** are read only where the kind is flume's own or a
  host's source. A repository's `bump` reads no lock, so a lock that
  cannot be read refuses it not (core.34).
- **A package** is read before its locks are checked, so the hash
  checked is the one just read (156).
- **A role's wake** resolves its `kind` through the same path, and
  `run_of` finds its run by the name the kinds seam reads.

The site's and `/launch`'s additions name no team, host or tool of
ours (core.35). A search of their added lines for each word core.35's
details list finds none.

## Checks

- **The gate.** `sluice exec main` at `94570ca` ran the checks
  `94570ca` affects against main at `9691c5b`. All 36 tasks completed,
  and sluice recorded 35 passes under their keys for `94570ca`:
  - `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint
    and test;
  - `conformance:coverage`;
  - `flume:tools`, 74 of 74, the walkthrough
    `a_hosts_source_runs_its_locked_kind_and_a_place_keeps_its_version`
    among them;
  - `plugin-claude`: format, validate and test;
  - `plugin-herdr`: manifest, lint and test;
  - `site:prose` and `site:build`;
  - `root`: format, hooks, toolchain, adrs, changes, specs, layers,
    versions and marketplace.

  `root:specs` passed 34 of 34 over the specs as the branch holds them,
  before any archive.
- **The change.** `openspec validate
  a-host-declares-where-its-kinds-come-from --strict` passes.
- **The archive.** `openspec archive
  a-host-declares-where-its-kinds-come-from --yes` (openspec 1.14.3)
  ran over a copy of `openspec/` at `94570ca`. It syncs the eight specs
  with no error, adding 18 requirements and modifying 3:
  - kind-sources' 10 are new;
  - kinds-profile, role-waking and seam-fakes each gain 1;
  - place-status gains 2, and seam-contracts 3;
  - launching's one requirement and clause-coverage's two are modified.

  The strict check over the specs it leaves then fails 34 of 35. Its
  one failure is `spec/kind-sources`: "Purpose section is still a
  placeholder rather than a Purpose anyone wrote". With a `## Purpose`
  at the delta's head, the archive carries that Purpose into the new
  spec. A Purpose of fewer than 50 characters fails as "too brief", and
  one of 50 or more passes 35 of 35.

## The merge with main

A submission of `94570ca` would conflict. `git merge-tree` of it with
origin's main at `9691c5b` conflicts in 25 files, and `sluice exec
main` names the same 25:

- 22 under `crates/`: conformance's `clauses.rs`, `engine/answers.rs`,
  `engine/launching.rs` and `engine/mod.rs`; core's `answer.rs`,
  `launch.rs`, `lib.rs` and `room.rs`; the fakes' `host.rs`, `kinds.rs`
  and `tests/fakes.rs`; the seams' `held.rs` and `host.rs`; flume's
  `commands/mod.rs`, `commands/print.rs`, `host/config.rs`,
  `host/mod.rs`, `tests/commands.rs`, `tests/host.rs`,
  `tests/profiles/built_in.rs`, `tests/profiles/tools/walk.rs` and
  `tests/profiles/tools/walkthroughs.rs`;
- 3 under the site's `reference/`: `commands.md`, `configuration.md`
  and `state.md`.

No task of the change asks for this merge, and nothing in its specs
fails for main having moved, so it is no part of the verdict. But the
place must merge main to land. If the round the Purpose sends it back
for leaves main out, the merge queue's conflict sends it back to code
again (`on_fail = "code"`; 27, 112; core.27). Merged before the next
verify, the archive then syncs over main's specs as they stand. main's
archive of flume-opens-a-streams-multiplexer synced `launching`,
`seam-contracts`, `seam-fakes` and `clause-coverage`, which this
change's deltas also touch.

The merge should:

- take main's clause list and the scenarios its revision holds, with
  186 to 190 and S90 and S91 built;
- keep main's local layer, `config.local.toml`, declaring no source;
- run the gate again over the merged tree.
