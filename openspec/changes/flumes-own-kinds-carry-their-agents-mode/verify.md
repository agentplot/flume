# Verify: flumes-own-kinds-carry-their-agents-mode

Verified at `e7bb42d6` on `change/flumes-own-kinds-carry-their-agents-mode`,
against its base `bolt/flumes-settings-read-as-its-records-say` at
`2d180f0b`, which the branch holds.

**Verdict: falls short. Not archived.** The whole checks fail on one
test. `flume:test`'s `kinds_weir::flumes_own_as_packages` still expects
flume's own five packages, and the binary now holds eight. That test is
the kinds-profile scenario "Flume's own as packages", which this change
modifies to name the eight, so that scenario is not met. Every task is
ticked, and every other requirement and scenario is met by a check that
passed.

## The whole checks

`sluice exec bolt/flumes-settings-read-as-its-records-say`, the host's
sluice 0.93.2, run once: **failed**, `exit 1`. sluice ran the checks
`e7bb42d` affects against the base, the bolt's `#check-fast`: 15 tasks,
14 passed and 1 failed (`flume:test`). It recorded the 14 passing
results under their keys for `e7bb42d`.

The run first asked through `devenv shell -- sluice exec …` stopped
before any check ran. Inside the shell, the sluice the tree pins for the
merge-queue profile's tests, 0.71.0, comes first on `PATH`. It refused
the host's `~/.config/sluice/config.toml` (`unknown field credential`,
line 21). No check ran under it. The run above is the host's own sluice,
which enters the devenv toolchain itself, as `.config/sluice.toml`'s
`[tests] toolchain` says.

The failure:

```
---- kinds_weir::flumes_own_as_packages stdout ----
thread 'kinds_weir::flumes_own_as_packages' panicked at crates/flume/tests/profiles/kinds_weir.rs:2937:5:
assertion `left == right` failed
  left: [("flume/change@1", 71), ("flume/change@2", 71), ("flume/change@3", 71), ("flume/change@4", 71), ("flume/chore@1", 71), ("flume/chore@2", 71), ("flume/fix@1", 71), ("flume/fix@2", 71)]
 right: [("flume/change@1", 71), ("flume/change@2", 71), ("flume/change@3", 71), ("flume/chore@1", 71), ("flume/fix@1", 71)]

test result: FAILED. 203 passed; 1 failed (the `profiles` test binary)
```

The tasks that passed: `site:install`, `site:prose` (0 errors, 0
warnings), `site:build`, `root:format`, `root:changes` (2 passed, this
change among them), `nix:typecheck`, `nix:test`, `plugin-claude:test`,
`conformance:format`, `conformance:lint`, `conformance:test`,
`conformance:coverage`, `flume:format` and `flume:lint`. `flume:lint` is
clippy over `--all-targets --all-features`, the live walkthroughs among
them, which is task 4.4's check.

In `flume:test`, the unit tests (286), the binary's own (2) and
`command_line`, `commands` (62, `a_source_file_with_no_kinds_dir` among
them), `host` and `profiles` ran. cargo stopped at the failing
`profiles` binary, so `scratch` and the doc tests did not run.

## The tasks' own checks outside the bolt's gate

The bolt requires `#check-fast` alone. The walkthroughs are
`#check-tools`, which main requires. Tasks 4.2 and 4.3 name them, so
they ran once, through sluice:
`sluice exec bolt/flumes-settings-read-as-its-records-say -- moon run flume:tools`,
**passed**, 170 passed and 0 failed.
`walkthroughs::a_hosts_mode_over_the_repositorys_and_none_where_no_layer_gives_one`,
`walkthroughs::a_change_runs_construct_code_and_verify` and
`walkthroughs::a_fix_agents_line_is_status_line_and_the_fix_lands` passed.

## The specs

### kinds-profile

- **flume's newest own versions give full and say their one way to
  ask.** Met. The three declarations give each stage
  `agents = [{ mode = "full" }]`. Each new prompt is its earlier
  version's with the sentence added as a paragraph of its own after the
  line holding `{{needs}}`, byte for byte as design section 3 gives it.
  fix@2's and chore@2's are equal and drop "or you find work this stage
  cannot deliver past, ". No declaration gives a `label`, a `standing`
  or a kind-level `agents` table. Each scenario is held by a test that
  passed: `kinds_weir::flumes_own_kinds_are_built_in`,
  `the_newest_own_prompts_say_their_one_way_to_ask`,
  `each_versions_prompts_are_its_own` and
  `built_in::flumes_own_kinds_declare_no_label`.
- **flume's own three kinds and their prompts.** Met: "The kinds built
  in", "Each version's prompts are its own", "No prompt asks to submit"
  and the places launched at change@1 and change@2, each held by its
  test, which passed.
- **What flume's own prompts ask.** Met:
  `a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing`
  renders fix@2's, chore@2's and change@4's code prompts with a
  conflict's words, and it passed.
- **Packages, identities and locks through weir's crate.** **Not met.**
  The scenario "Flume's own as packages" says each package is one of
  `flume/fix@1`, `flume/fix@2`, `flume/chore@1`, `flume/chore@2` and
  `flume/change@1` to `flume/change@4`, with its hash.
  `kinds_weir::flumes_own_as_packages`, its test, still lists the five
  before this change and fails. The binary holds the eight, as the test's
  own output shows, so the code is right and the test was not brought to
  the scenario. No task of `tasks.md` names this test. The requirement's
  other four scenarios are unchanged and passed.

### agent-modes

- **flume's own kinds give full.** Met. In the conformance suite,
  `a_bare_fix_in_full`, `the_hosts_mode_over_the_own_kinds` and
  `an_earlier_own_version_named` each run in
  `World::with_every_own_version()`. In the profile,
  `kinds_weir::flumes_newest_own_kinds_give_full_from_the_kind` covers
  the same three. Over the binary, the mode walkthrough's third walk
  covers a bare fix. All passed.
- **No mode where no layer gives one.** Met. `no_mode_in_any_layer`
  launches `fix@1` by name, and the mode walkthrough's second walk does
  the same.

### launching

- **A launch may name its kind's version.** Met. "No version named"
  runs change@4, its code stage one agent, each stage in `full` from
  `kind`, and status names `flume/change@4`. "A version flume does not
  hold" refuses `change@5` naming versions 1, 2, 3 and 4. The other
  scenarios are unchanged. All passed in `conformance:test`.

### running-through

- **change@3 codes its change with one agent.** Met. The locked
  change@3 scenario runs with `FakeKinds::set_lock` recording
  `flume/change@3`. Its twin runs change@4 through, in `full` from
  `kind`, its line `change--plan · change@4 · code · run 1 · tasks 0/7 ·
  working`. Over the binary, `a_change_runs_construct_code_and_verify`
  reads `flume/change@4`.

### kind-sources

- **The host declares each source in a file of its own.** Met. "No
  sources directory" runs `flume/fix@2` for a bare `fix`.
- **A bare name keeps its meaning.** Met. "A source that holds fix"
  passes in the conformance suite (`flume/fix@2` and `acme/fix@1`), and
  over the binary in `commands::a_source_file_with_no_kinds_dir`.
- **flume's own kinds are packages whose content never changes.** Met.
  `built_in::the_hashes_recorded` records the eight, the five earlier
  hashes unchanged, and it passed.

## The tasks

Every task, 1.1 to 5.1, is ticked. Each task's change is on the branch
as the task says, and each check a task names exists and passed, in the
gate or in the walkthroughs above.

## What goes back to code

1. Bring `kinds_weir::flumes_own_as_packages`
   (`crates/flume/tests/profiles/kinds_weir.rs:2925`) to the
   kinds-profile scenario "Flume's own as packages", listing
   `flume/change@1` to `flume/change@4`, `flume/chore@1`,
   `flume/chore@2`, `flume/fix@1` and `flume/fix@2` in `OWN`'s order,
   and record it as a ticked task of the change naming that test as its
   check.
