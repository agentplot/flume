# Verify: a-base-places-fix-kind-is-read-from-weirs-format

Checked on 2026-11-08 at
`change/a-base-places-fix-kind-is-read-from-weirs-format` 1afee23e, cut
from `bolt/flume-launches-a-conflicting-bolts-fix` at bb2d47b2, the
bolt's head still. The branch holds main at 81d57226, taken in by task
6.1. The check covers the change's proposal, design, its one spec,
`kinds-profile`, and its tasks.

**The whole checks pass and the work holds, so the change is archived.**
`sluice exec bolt/flume-launches-a-conflicting-bolts-fix` at 1afee23e
completed all 40 of its tasks with none failed. Every task is ticked,
every requirement is built, and each of the spec's seven scenarios has a
test that passes in the same run.

## Checks

`sluice exec bolt/flume-launches-a-conflicting-bolts-fix` ran once, with
the worktree clean, as the repository's instructions name the gate for a
place on a bolt: the bolt's `check-fast` layer, `check-tools` left to the
bolt's merge onto main. It ran against the bolt at bb2d47b and recorded
39 results under their keys for 1afee23, exiting 0. Tasks: 40 completed,
39 checks passed, none failed.

Passed:

- `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint and
  test. The crates ran 794 tests with none failed: `seams` 103, `core`
  47, `fakes` 65, `conformance` 25 and `flume` 554, the last among them
  the seven this change adds to `kinds_weir.rs`;
- `conformance:coverage`: every clause of Parts A and B, I1 to I8 and S1
  to S120 at 31f9f1c cited by the suite's 896 scenarios, or left to the
  change it names;
- `plugin-claude`: format, typecheck, engine-types, validate and test;
- `plugin-herdr`: manifest, lint and test;
- `site`: install, build (19 pages), and prose with 0 errors and 0
  warnings;
- `nix`: format, lint, typecheck and test, the last's "every case holds"
  taking in "Every git crate is fetched by a hash taken at Cargo.lock's
  revision", which `nix/packages/git-crates.nix` at daacda39 now meets;
- `root`: format, changes, specs (48 of 48), adrs, marketplace, layers,
  versions, toolchain and hooks.

moon warned that it could not write its shared cache's blobs or archive
task outputs, and that it skipped the Rust dependency install with no
internet connection. None of these is a check, and none failed one.

## Tasks

All 14 tasks of the six groups are ticked, `[x]`, and none is open.
The narrow check each names is covered by the gate run above:
`root:adrs` and `root:format` for 1.1; `flume:test` for 2.1, 3.1 to 3.5,
4.1 and 4.2; `site:prose` for 5.1 to 5.3; `site:build` for 6.1, whose
`flume:tools` half is `check-tools` and isn't run on a bolt; `nix:test`
for 6.2. weir's crates are at `daacda39e59469029b2841f889c6cbd8cd7839d9`
in `Cargo.toml`, both of `Cargo.lock`'s weir entries and both of
`nix/packages/git-crates.nix`'s, each with that revision's hash, and
`git grep -l 5562af5` finds the old revision only in this change's
design, tasks and report and two archived changes' designs, each telling
where the pin stood. `openspec validate
a-base-places-fix-kind-is-read-from-weirs-format --strict` passes.

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

## Decisions

This stage made none. It ran the checks, read the work against the
change and archived it, which the change's tasks and the stage's
instructions decide.
