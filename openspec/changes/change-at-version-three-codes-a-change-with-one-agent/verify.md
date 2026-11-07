# Verify: change-at-version-three-codes-a-change-with-one-agent

The verify stage's report on
`change/change-at-version-three-codes-a-change-with-one-agent` at
838bb9f, against the change's proposal, design, delta specs and tasks.

## Verdict

The work holds. Every requirement and scenario of the change's five
delta specs is met by a test that asserts it, every task is ticked, and
the gate passes over the branch's head. The change is archived with
this report.

## The gate

`sluice exec main`, run once at 838bb9f, passed 14 tasks with exit 0:
conformance:format, conformance:lint, conformance:test,
conformance:coverage, flume:format, flume:lint, flume:test, flume:tools,
plugin-claude:test, root:format, root:specs, root:changes, site:build and
site:prose. flume:tools passed 108 of 108. conformance:coverage reads
every clause of Parts A and B, I1 to I8 and S1 to S119 at 9371cda as
cited by the suite's 680 scenarios, but those it names unbuilt.
`openspec validate change-at-version-three-codes-a-change-with-one-agent --strict`
passes.

The tests each task names ran inside those tasks and passed, among them
`change_3s_construct_prompt_gives_each_task_its_own_check`,
`change_3s_code_prompt_runs_each_tasks_own_check_alone`,
`change_3s_code_prompt_puts_a_retrys_words_last`,
`change_3s_verify_prompt_runs_the_whole_checks_once_first`,
`a_place_launched_at_change_2_keeps_it`, `the_hashes_recorded`,
`the_kinds_profile_passes_the_kinds_scenarios`,
`the_engine_passes_its_scenarios`, the four walkthroughs of group 4 and
`clauses::tests`.

## The statement and the base

- `crates/flume-conformance/src/clauses.rs` reads agentplot/blueprints
  at 9371cda, which is on blueprints main and descends from 49b9d53.
  The statement there, read for this report, numbers 146 clauses in
  Part A and 60 in Part B, and the list's `part_a()` and `part_b()`
  give the same sets exactly. Section 11 runs S1 to S119 with no number
  skipped, so `SCENARIOS` is 119 and `SKIPPED` is empty. Blueprints main
  has since moved to 9c060ab (record 0055), and its clause and scenario
  numbers are the same as 9371cda's.
- The clauses named unbuilt are attributed by the statement's own
  commits: 193 and S93, S94, S98 and S99 to record 0039's roles
  (blueprints 3394602); 202 to 206, 208, S109 to S112 and S118 to
  record 0051's base places and the host's watch (51b6316); 209 to
  record 0053's rename (ec4806b). S119 is cited by
  `s119_one_agent_codes_the_change` and is not named unbuilt.
- Record 0054 is `accepted`, on the user's "yes to 54" of 2026-11-06.
  The change touches no rule of `docs/architecture/core.md`.
- The base has moved since the cut. The bolt
  `bolt/flume-codes-a-change-with-one-agent`, cut at 5668b5d, now
  stands at 6d82383, where #1 (3177b9e) rewrites the same two rules of
  `openspec/config.yaml`'s `rules.tasks` that task 1.1 rewrites, in
  other words. A trial merge conflicts in that one file. That is the
  merge queue's to report at the submission, which sends the place back
  to code with the conflict's words (112). It is not a shortfall of the
  change's work, so this report does not stop short on it.

## Tasks

All 16 tasks of groups 1 to 6 are ticked. The changes read for this
report:

- change@3's three prompts are byte for byte design 2's texts, each
  ending in one newline. Its `kind.toml` declares design 1's stages,
  with `code` not repeating and `verify`'s `on_fail` `code`. `OWN` lists
  `flume/change@3` after change@2, and the core's `own_kind` sorts by
  version, so a bare `change` takes change@3.
- `OWN_HASHES` gains change@3's hash, and the other four entries are
  unchanged.
- The walkthroughs' `Walk::launch` now reads the place's location from
  the launch's answer instead of building `<kind>--<name>`. A kind
  named as `change@2` needs that, since its place is `change--<name>`.
  This came with task 4.1 (e4ad448).

## Requirements and scenarios

None of the seven modified requirements drops a scenario its base
requirement in `openspec/specs/` holds. Each of the 27 scenarios of the
delta specs' 11 requirements maps to a test that asserts its THEN, each
found on the branch's head for this report. Profile tests are in
`crates/flume/tests/profiles/`. Engine scenarios are in
`crates/flume-conformance/src/engine/`.

| spec | scenario | test |
|---|---|---|
| kinds-profile | change@3's construct prompt rendered | `kinds_weir.rs` `change_3s_construct_prompt_gives_each_task_its_own_check` |
| kinds-profile | change@3's code prompt rendered | `kinds_weir.rs` `change_3s_code_prompt_runs_each_tasks_own_check_alone` |
| kinds-profile | A retry's words come last | `kinds_weir.rs` `change_3s_code_prompt_puts_a_retrys_words_last` |
| kinds-profile | change@3's verify prompt rendered | `kinds_weir.rs` `change_3s_verify_prompt_runs_the_whole_checks_once_first` |
| kinds-profile | The kinds built in | `kinds_weir.rs` `flumes_own_kinds_are_built_in` |
| kinds-profile | Each version's prompts are its own | `kinds_weir.rs` `each_versions_prompts_are_its_own` |
| kinds-profile | No prompt asks to submit | `kinds_weir.rs` `no_prompt_of_flumes_own_asks_to_submit`, versions 1, 2 and 3 |
| kinds-profile | A place launched at change@1 | `kinds_weir.rs` `a_place_launched_at_change_1_keeps_it` |
| kinds-profile | A place launched at change@2 | `kinds_weir.rs` `a_place_launched_at_change_2_keeps_it` |
| kinds-profile | A code prompt sent back with a conflict | `kinds_weir.rs` `a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing`, fix@1's, chore@1's, change@2's and change@3's |
| kinds-profile | A package's hash wherever it lies | `kinds_weir.rs` `a_packages_hash_wherever_it_lies` |
| kinds-profile | A source's listing | `kinds_weir.rs` `a_sources_listing` |
| kinds-profile | The repository's lock at a revision | `kinds_weir.rs` `the_repositorys_lock_at_a_revision` |
| kinds-profile | A lock written and read back | `kinds_weir.rs` `a_lock_written_and_read_back` |
| kinds-profile | Flume's own as packages | `kinds_weir.rs` `flumes_own_as_packages` |
| kind-sources | The hashes recorded | `built_in.rs` `the_hashes_recorded` |
| kind-sources | A lock of flume's own at another hash | `kind_sources.rs` `a_lock_of_flumes_own_at_another_hash` |
| launching | No version named | `launching.rs` `no_version_named`, `the_identity_status_names` |
| launching | change@2 named | `launching.rs` `change_2_named` |
| launching | change@1 named | `launching.rs` `change_1_named` |
| launching | A version flume does not hold | `launching.rs` `a_version_flume_does_not_hold`, change@4 refused naming versions 1, 2, 3 |
| launching | A version the repository's declaration does not hold | `launching.rs` `a_version_the_repository_does_not_hold` |
| launching | A kind of a host's source named whole | `launching.rs` `a_kind_of_a_hosts_source_named_whole` |
| running-through | A bare change runs change@3 through | `running_through.rs` `s119_one_agent_codes_the_change`; `tools/walkthroughs.rs` `a_change_runs_construct_code_and_verify` (check-tools) |
| clause-coverage | The clauses listed | `clauses.rs` `the_clauses_listed`, `the_clauses_unbuilt_are_named_by_the_changes_that_build_them` |
| clause-coverage | A clause left out | `coverage.rs` `a_clause_left_out_is_named` |
| clause-coverage | The suite as this change leaves it | conformance:coverage, passing at 838bb9f |

The engine's other scenarios that read a repeating code stage launch
`change@2` by name (`place_line.rs`, `place_status.rs`), and `list.rs`
reads a bare change's kind as `change@3`. The kinds seam's
`flumes_own` scenario reads five kinds, change@2's code alone repeating.
All of them pass under conformance:test and, through the kinds profile,
flume:test.
