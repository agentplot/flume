# Verify: a-base-places-fix-kind-is-read-from-weirs-format

Checked on 2026-11-08 at
`change/a-base-places-fix-kind-is-read-from-weirs-format` faa409e3, cut
from `bolt/flume-launches-a-conflicting-bolts-fix` at bb2d47b2, the
bolt's head still. The check covers the change's proposal, design, its
one spec, `kinds-profile`, and its tasks.

**The whole checks fail, so the change is not archived.** `sluice exec
main` at faa409e3 failed two of its 38 tasks: `flume:tools`, on one
walkthrough this change does not touch, and `site:install`, on bun's
temporary folder, which left `site:build` unrun. The work itself holds.
Every task is ticked, every requirement is built, and each of the
spec's seven scenarios has a test that passes in the same run.

## Checks

`sluice exec main` ran once, with the worktree clean. It said the
branch would conflict with main at 81d57226 in
`crates/flume-conformance/src/clauses.rs`,
`crates/flume-seams/src/held.rs`,
`crates/flume/tests/profiles/kinds_weir.rs`, `docs/architecture/core.md`
and `openspec/specs/clause-coverage/spec.md`, and ran the checks
faa409e3 affects against main. It recorded 37 results under their keys
for faa409e. Tasks: 35 completed, 2 failed, 1 skipped.

Passed:

- `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint and
  test. `flume:test` ran 533 tests with none failed, among them the seven
  this change adds to `kinds_weir.rs`;
- `conformance:coverage`;
- `plugin-claude`: format, typecheck, engine-types, validate and test;
- `plugin-herdr`: install, manifest, lint and test;
- `site:prose`, 0 errors and 0 warnings;
- `root`: format, changes, specs (46 of 46), adrs, marketplace, layers,
  versions, toolchain and hooks.

Failed:

- **`flume:tools`**: 132 of 133 passed.
  `walkthroughs::s93_a_question_for_easts_ops_is_answered_once_by_its_woken_run`
  failed at `crates/flume/tests/profiles/tools/walkthroughs.rs:3415`
  after its three minutes, "never came to the run of east/ops answered
  and tidied". `change--s93` was still at construct, run 1, working.
  `ops--east-ops` was at `ops@1 · answer · run 1 · lost`, waiting on
  its caller's retry because its agent was lost. The walkthrough runs a
  role's run under the pinned herdr with the stand-in. This branch
  changes nothing under `crates/flume/tests/profiles/tools/`,
  `crates/flume-core/`, `crates/flume-fakes/` or `plugins/`. It changes
  only the weir pin, the kinds profile's `map_kind`, and the seam's
  docs, and `ops@1` was read and its run started before its agent was
  lost.
- **`site:install`**: `bun install v1.4.2` failed at once, "error: bun
  is unable to write files to tempdir: EPERM". `site:build` depends on
  it and was skipped, so the site's build was not checked. This branch
  changes no file of the site's install. Its site changes are prose in
  four pages, which `site:prose` passed.

As the stage asks, the checks were run once and not again, and nothing
was fixed here.

## Tasks

All ten tasks of the five groups are ticked, `[x]`, and none is open.
The narrow check each names is covered by the gate run above:
`root:adrs` and `root:format` for 1.1; `flume:test` for 2.1, 3.1 to 3.5,
4.1 and 4.2; `site:prose` for 5.1 to 5.3. 2.1's
`git grep -n 5562af5 -- Cargo.toml Cargo.lock` finds nothing.
`openspec validate a-base-places-fix-kind-is-read-from-weirs-format
--strict` passes.

## Requirements and scenarios

Each test was read against its scenario's WHEN and THEN, and each passes
in the gate run. KW is `crates/flume/tests/profiles/kinds_weir.rs`.

| Requirement | Scenario | Test in KW | Met |
|---|---|---|---|
| A kind with no stages names its landing fix through weir's crate | A landing fix and its bound over a scratch repository | `a_kind_with_no_stages_names_its_landing_fix_and_its_bound`: `train` gives `(Some("repo/mend@1"), Some(2))`, `yard` and `mend` `(None, None)` | yes |
| | A package's landing fix | `a_packages_kind_names_its_landing_fix`: `acme/train@1` gives `(Some("flume/fix@1"), Some(1))` | yes |
| A landing fix weir or flume refuses | A landing fix weir refuses | `a_landing_fix_weir_refuses_is_named_in_weirs_words`: `mend` and `repo/mend` each refused naming `kinds.train.landing_fix` and "is no identity"; `bump` refused naming both fields and "is on a kind with stages" | yes |
| | A bound past what flume counts | `a_landing_fix_bound_past_what_flume_counts_is_refused`: "kinds.train.max_landing_fixes: is more than flume counts" | yes |
| | The fields' old spellings | `fix_and_max_fixes_are_sections_naming_no_landing_fix`: `train` gives `(None, None)` | yes |
| A base place's launch resolves the landing fix the profile reads | The landing fix held through the profile | `a_base_places_launch_holds_the_landing_fix_the_profile_reads`: launch taken; `fixes_as` is `repo/mend@1` from the repository's source, its declaration file at the launch's revision; status gives that fix kind with bound 2 | yes |
| | A landing fix at a version the repository does not declare | `a_landing_fix_at_a_version_not_declared_refuses_the_launch`: refused naming `repo/mend@2` and "declares mend at version 1"; no `open` in the fake queue's log; no place held | yes |

## Design and rules

- **Decision 1, core.22.** `Cargo.toml` pins weir-kinds at
  `daacda39e59469029b2841f889c6cbd8cd7839d9`, its comment unchanged.
  `Cargo.lock` moves weir-kinds and weir-capture to it and nothing else.
- **Decision 2, core.4.** `map_kind` sets `fix` from
  `kind.landing_fix` through `ToString`, weir's `Display`, and
  `fix_limit` from `kind.max_landing_fixes` by `u32::try_from`, a
  failure pushing the problem that names the field. Nothing else feeds
  either, and no section is read. One `map_kind` serves a declaration, a
  launch's kinds and a package.
- **Decision 3, core.14.** `Kind::fix` and `Kind::fix_limit` keep their
  names and types. Only their docs change, as design 3 words them.
  Status's `fix_kind` and `fix_limit` are unchanged.
- **Decision 4.** The guides give three for a bound left out, as
  `Defaults::send_backs` holds. The core's `fix_kind_of`
  (`crates/flume-core/src/launch.rs`) refuses a landing fix that
  resolves to no kind, or to a kind that does not land or has no
  stages, before anything is made, as the kinds guide says.
- **Decision 6, core.35.** The kinds guide, the configuration reference
  and the merge-queue guide give both fields in weir's spelling. They
  name `flume/fix@1` as flume's own and `repo/mend@1` as a repository's
  kind, and nothing core.35 rules out. flume's own `fix@1`
  (`crates/flume/src/profiles/kinds/weir/own/fix@1/kind.toml`) lands
  and has stages, so the examples resolve.
- **Decision 7, core.19.** Record 0056's amendment line, core.34's rule
  and details, core.4's details and the site's mirror of core.34 cite
  weir's record 0005, accepted by the user on 2026-11-07 on their card,
  and the statement at agentplot/blueprints 8eec6dd.

## Outside the change

- **The conflict with main.** sluice reports that faa409e3 conflicts
  with main at 81d57226 in five files. This branch lands on its bolt,
  `bolt/flume-launches-a-conflicting-bolts-fix`, not on main. Of the
  five, it touches only `kinds_weir.rs` and `core.md`. The bolt and main
  must be reconciled when the bolt lands on main.
