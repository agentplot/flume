# Verify: flumes-own-kinds-carry-their-agents-mode

Verified at `23239655` on `change/flumes-own-kinds-carry-their-agents-mode`,
against its base `bolt/flumes-settings-read-as-its-records-say` at
`ea8939fd`. The branch was cut at `2d180f0b`. The bolt has since landed
#73, `a-labels-share-is-owed-and-reserved`. Its spec moves touch
host-configuration, host-module, host-queue, seam-contracts,
stream-labels and streams, none of the five capabilities this change
modifies.

**Verdict: holds. Archived.** The whole checks pass. Every requirement
and scenario of the change's five delta specs is met by a check that
passed, and every task, 1.1 to 5.1, is ticked and done.

## The whole checks

`sluice exec bolt/flumes-settings-read-as-its-records-say`, the host's
sluice 0.93.2, run once: **passed**, `exit 0`. sluice ran the checks
`2323965` affects against the bolt at `ea8939f`, its `#check-fast`. moon
completed 15 tasks with none failed, and sluice recorded 14 results
under their keys for `2323965`.

The tasks: `site:install`, `site:prose` (0 errors, 0 warnings),
`site:build`, `root:format`, `root:changes` (2 passed, this change among
them), `nix:typecheck`, `nix:test`, `plugin-claude:test`,
`conformance:format`, `conformance:lint`, `conformance:test`,
`conformance:coverage`, `flume:format`, `flume:lint` and `flume:test`.
`flume:lint` is clippy over `--all-targets --all-features`, the live
walkthroughs among them, which is task 4.4's check. In `flume:test`, the
unit tests (286), the binary's own (2), `command_line` (4), `commands`
(62), `host` (32), `profiles` (204) and `scratch` (2) all passed.

The run first asked through `devenv shell -- sluice exec …` stopped
before any check ran. Inside the shell, the sluice the tree pins for the
merge-queue profile's tests, 0.71.0, comes first on `PATH`, and it
refused the host's `~/.config/sluice/config.toml` (`unknown field
credential`, line 21). The run above is the host's own sluice, which
enters the devenv toolchain itself, as `.config/sluice.toml`'s `[tests]
toolchain` says.

## The tasks' own checks outside the bolt's gate

The bolt requires `#check-fast` alone. The walkthroughs are
`#check-tools`, which main requires, and tasks 4.2 and 4.3 name them, so
they ran once, through sluice:
`sluice exec bolt/flumes-settings-read-as-its-records-say -- moon run flume:tools`,
**passed**, 170 passed and 0 failed.
`walkthroughs::a_hosts_mode_over_the_repositorys_and_none_where_no_layer_gives_one`,
`walkthroughs::a_change_runs_construct_code_and_verify` and
`walkthroughs::a_fix_agents_line_is_status_line_and_the_fix_lands` are
among them.

`openspec validate flumes-own-kinds-carry-their-agents-mode --strict`:
valid.

## The specs

### kinds-profile

- **flume's newest own versions give full and say their one way to
  ask.** Met. `change@4`, `chore@2` and `fix@2` are each the version
  before it at the next version: their `kind.toml`s differ only in the
  header comment, the version, and `agents = [{ mode = "full" }]` on
  each stage. Each change@4 prompt is change@3's with a blank line and
  the sentence after the line holding `{{needs}}`, byte for byte as
  design section 3 gives it. fix@2's code prompt is fix@1's with "or you
  find work this stage cannot deliver past, " taken out of its last line
  and the sentence after it. chore@2's prompts equal fix@2's. No
  declaration gives a `label`, a `standing` or a kind-level `agents`
  table. Held by `kinds_weir::flumes_own_kinds_are_built_in`,
  `the_newest_own_prompts_say_their_one_way_to_ask`,
  `each_versions_prompts_are_its_own` and
  `built_in::flumes_own_kinds_declare_no_label`, all passed.
- **flume's own three kinds and their prompts.** Met: "The kinds built
  in", "Each version's prompts are its own", "No prompt asks to submit"
  and the places launched at change@1 and change@2, each held by its
  test, which passed.
- **What flume's own prompts ask.** Met:
  `a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing`
  renders fix@2's, chore@2's and change@4's code prompts with a
  conflict's words, and it passed.
- **Packages, identities and locks through weir's crate.** Met. "Flume's
  own as packages" is held by `kinds_weir::flumes_own_as_packages`, which
  names the eight versions in `OWN`'s order, `flume/change@1` to
  `flume/change@4`, `flume/chore@1`, `flume/chore@2`, `flume/fix@1` and
  `flume/fix@2`, each with its hash, and passed. The other four
  scenarios are unchanged and passed.

### agent-modes

- **flume's own kinds give full.** Met. In the conformance suite,
  `a_bare_fix_in_full`, `the_hosts_mode_over_the_own_kinds` and
  `an_earlier_own_version_named` each run in
  `World::with_every_own_version()`. In the profile,
  `kinds_weir::flumes_newest_own_kinds_give_full_from_the_kind` covers
  the same three. Over the binary, the mode walkthrough's third walk
  launches a bare fix, reads `full` from `kind` passed as
  `bypassPermissions`, and lands it. All passed.
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
gate or in the walkthroughs above. Task 1.5, `flumes_own_as_packages`
naming the eight versions, holds the scenario the last verify found
unmet.

## The statement

Design section 7 leaves clause 22's list of flume's own kinds, and the
statement's table naming change@3 the newest, for design to bring to
change@4, chore@2 and fix@2 (core.19). This change cites record 0064 and
the paragraph after the table for them, and changes nothing in the
statement.
