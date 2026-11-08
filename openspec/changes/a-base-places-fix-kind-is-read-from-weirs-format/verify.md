# Verify: a-base-places-fix-kind-is-read-from-weirs-format

Checked on 2026-11-08 at
`change/a-base-places-fix-kind-is-read-from-weirs-format` 0aafa142, cut
from `bolt/flume-launches-a-conflicting-bolts-fix` at bb2d47b2, the
bolt's head still. The branch holds main at 81d57226, taken in by task
6.1. The check covers the change's proposal, design, its one spec,
`kinds-profile`, and its tasks.

**The whole checks fail, so the change is not archived.** `sluice exec
bolt/flume-launches-a-conflicting-bolts-fix` at 0aafa142 failed one of
its 40 tasks, `nix:test`: the flake that came in with main still fetches
weir's crates at the old revision, `5562af52`, while `Cargo.lock` names
`daacda39`. The work itself holds. Every task is ticked, every
requirement is built, and each of the spec's seven scenarios has a test
that passes in the same run.

## Checks

`sluice exec bolt/flume-launches-a-conflicting-bolts-fix` ran once, with
the worktree clean, as the repository's instructions name the gate for a
place on a bolt: the bolt's `check-fast` layer, `check-tools` left to the
bolt's merge onto main. It ran the checks 0aafa14 affects against the
bolt at bb2d47b and recorded 39 results under their keys for 0aafa14.
Tasks: 39 completed, 1 failed.

Passed:

- `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint and
  test. `flume:test` ran 554 tests with none failed, among them the seven
  this change adds to `kinds_weir.rs`;
- `conformance:coverage`;
- `plugin-claude`: format, typecheck, engine-types, validate and test;
- `plugin-herdr`: manifest, lint and test;
- `site`: install, build, and prose with 0 errors and 0 warnings;
- `nix`: format, lint and typecheck;
- `root`: format, changes, specs (48 of 48), adrs, marketplace, layers,
  versions, toolchain and hooks.

Failed:

- **`nix:test`**, in its check "Every git crate is fetched by a hash
  taken at Cargo.lock's revision":

  ```text
  weir-capture-0.0.0: Cargo.lock names daacda39e59469029b2841f889c6cbd8cd7839d9,
  and nix/packages/git-crates.nix's hash is of 5562af52cb7e065e7de7109135dfd7eecb1d61c5:
  the hash moves with the revision, in one commit
  weir-kinds-0.0.0: Cargo.lock names daacda39e59469029b2841f889c6cbd8cd7839d9,
  and nix/packages/git-crates.nix's hash is of 5562af52cb7e065e7de7109135dfd7eecb1d61c5:
  the hash moves with the revision, in one commit
  ```

  `nix/packages/git-crates.nix` isn't on the bolt. It came in with main
  at 81d57226 (3e81c484, record 0055), pinning both weir crates at
  `5562af52` with that revision's hash. Task 2.1 moved `Cargo.toml` and
  `Cargo.lock` to `daacda39` before the merge, and the merge in 6.1 kept
  both sides, so the flake's revision and hash were never moved with the
  lock. 2.1's check, `git grep -n 5562af5 -- Cargo.toml Cargo.lock`,
  doesn't look at `nix/`. `git grep -n 5562af5` still finds the file's
  two `rev` lines. Both entries need `rev` set to
  `daacda39e59469029b2841f889c6cbd8cd7839d9` and `hash` set to that
  revision's, in one commit, as the flake's design has it
  (`openspec/changes/archive/2026-11-07-flume-publishes-its-own-flake-and-module/design.md`;
  core.22).

As the stage asks, the checks were run once and not again, and nothing
was fixed here.

## Tasks

All 13 tasks of the six groups are ticked, `[x]`, and none is open.
The narrow check each names is covered by the gate run above:
`root:adrs` and `root:format` for 1.1; `flume:test` for 2.1, 3.1 to 3.5,
4.1 and 4.2; `site:prose` for 5.1 to 5.3; `site:build` for 6.1, whose
`flume:tools` half is `check-tools` and isn't run on a bolt. 2.1's
`git grep -n 5562af5 -- Cargo.toml Cargo.lock` finds nothing, but its
intent, weir's crates at `daacda3`, isn't met in the tree, as above.
`openspec validate a-base-places-fix-kind-is-read-from-weirs-format
--strict` passes.

## Requirements and scenarios

Each test was read against its scenario's WHEN and THEN, and each passes
in the gate run. KW is `crates/flume/tests/profiles/kinds_weir.rs`.

| Requirement | Scenario | Test in KW | Met |
|---|---|---|---|
| A kind with no stages names its landing fix through weir's crate | A landing fix and its bound over a scratch repository | `a_kind_with_no_stages_names_its_landing_fix_and_its_bound`: `train` gives `(Some("repo/mend@1"), Some(2))`, `yard` and `mend` `(None, None)` | yes |
| | A package's landing fix | `a_packages_kind_names_its_landing_fix`: `acme/train@1` gives `(Some("flume/fix@1"), Some(1))` | yes |
| A landing fix weir or flume refuses | A landing fix weir refuses | `a_landing_fix_weir_refuses_is_named_in_weirs_words`: `mend` and `repo/mend`, each at its own commit, refused naming `kinds.train.landing_fix` and "is no identity"; `bump`'s two fields each "is on a kind with stages" | yes |
| | A bound past what flume counts | `a_landing_fix_bound_past_what_flume_counts_is_refused`: `4294967296` refused, "kinds.train.max_landing_fixes: is more than flume counts" | yes |
| | The fields' old spellings | `fix_and_max_fixes_are_sections_naming_no_landing_fix`: `train` reads `(None, None)` | yes |
| A base place's launch resolves the landing fix the profile reads | The landing fix held through the profile | `a_base_places_launch_holds_the_landing_fix_the_profile_reads`: held `fixes_as` spelled `repo/mend@1`, from the repository's source, declaration and the launch's revision; status gives that fix kind and `Some(2)` | yes |
| | A landing fix at a version the repository does not declare | `a_landing_fix_at_a_version_not_declared_refuses_the_launch`: refusal names `repo/mend@2` and "declares mend at version 1", the fake queue logs no `open`, the store holds no place | yes |

The code matches the design. `map_kind` in
`crates/flume/src/profiles/kinds/weir/mod.rs` maps `landing_fix` by
weir's `Display` and `max_landing_fixes` through `u32::try_from`, naming
the field where it can't, from nothing else. The docs of `map_kind`, its
module, and `Kind::fix` and `Kind::fix_limit` in
`crates/flume-seams/src/kinds.rs` say so. The kinds guide, the
configuration reference (its `stages` row saying `[]` on a kind that
lands) and the merge-queue guide's "Its fixes" give both fields in
weir's spelling, naming `flume/fix@1`, to any host's operator
(core.35). core.34's rule and the details of core.4 and core.34, the
rules page's mirror and record 0056's amendment cite weir's record 0005.

## What goes back

The code stage moves `nix/packages/git-crates.nix`'s two weir entries
to `daacda39e59469029b2841f889c6cbd8cd7839d9` with that revision's
hash, in one commit, and runs `sluice exec
bolt/flume-launches-a-conflicting-bolts-fix -- moon run nix:test`.
Nothing else in the change needs to be done again.
