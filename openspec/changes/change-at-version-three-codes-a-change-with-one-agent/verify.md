# Verify: change-at-version-three-codes-a-change-with-one-agent

The verify stage's report on
`change/change-at-version-three-codes-a-change-with-one-agent` at
af086c8, against the change's proposal, design, delta specs and tasks.
This is the second verify. The first, at 838bb9f, archived the change,
and the merge queue sent the place back on #2's conflict in
`openspec/config.yaml` with the base at 6d82383. The code stage merged
the base, took the change out of the archive with `openspec/specs` the
base's, and added and ticked task 7.1.

## Verdict

The work holds. The whole checks pass at af086c8. Every requirement and
scenario of the change's five delta specs is met by a test that asserts
it, and all 16 tasks are ticked. The change is archived with this
report.

## The gate

`sluice exec main` ran once at af086c8 and exited 0, passing 14 tasks:
conformance:format, conformance:lint, conformance:test,
conformance:coverage, flume:format, flume:lint, flume:test, flume:tools,
plugin-claude:test, root:format, root:specs, root:changes, site:build
and site:prose.

- flume:tools passed 108 of 108.
- root:specs passed 44 of 44.
- conformance:coverage passed. Every clause of Parts A and B, I1 to I8
  and S1 to S119 at 9371cda is cited by the suite's 680 scenarios, apart
  from the ones the list names unbuilt.

sluice did not record the pass against af086c8, because this report was
in the worktree, not yet committed, when the run ended. That changes
nothing about the outcome.

Every test the tasks name ran inside those tasks and passed. Among them:

- the four `change_3s_` tests
- `a_place_launched_at_change_2_keeps_it` and `the_hashes_recorded`
- `the_kinds_profile_passes_the_kinds_scenarios` and
  `the_engine_passes_its_scenarios`
- the four walkthroughs of group 4
- `clauses::tests`

## What moved since the first verify

`git diff 838bb9f af086c8` touches the change's own design, proposal,
tasks and report, and `openspec/config.yaml`; no code, test, prompt,
site page or `CLAUDE.md` line. The send-back's work holds:

- The merge 43c72ec takes the base's words for record 0054's two rules
  of `rules.tasks` whole: `git diff 6d82383 af086c8 -- openspec/config.yaml`
  is empty. Design 8 quotes those two rules as the file holds them, and
  names this branch's earlier wording as the path not taken. The
  proposal's impact names them the base's.
- `CLAUDE.md`'s paragraph under "Checks" is task 1.2's, and agrees with
  the base's rules: `sluice exec main` runs the gate's checks, which a
  change's verify stage runs once and no task runs, and a task's own
  check runs as the task names it.
- `openspec/specs` is the base's: `git diff 6d82383 af086c8 -- openspec/specs`
  is empty, so archiving applies the five delta specs over the base's
  specs once.
- `openspec validate change-at-version-three-codes-a-change-with-one-agent --strict`
  passes.

Tasks 2.1 to 6.1 name checks as `moon run <project>:test -- <filter>`,
the wording design 8 now names as the path not taken. They were written
and run before the base's rules landed, and they ran as written. The
base's rules bind the changes written after them, so this is not a
shortfall.

## Tasks

All 16 tasks of groups 1 to 7 are ticked, none open. Read for this
report:

- change@3's `kind.toml` declares construct, code and verify on
  `change/`, landing, `code` not repeating and `verify`'s `on_fail`
  `code`. Its three prompts are design 2's. The code prompt runs each
  task's own check alone and puts a retry's words last. The verify
  prompt runs the whole checks once before it checks the specs and the
  tasks.
- `OWN` in `crates/flume/src/profiles/kinds/weir/mod.rs` lists
  `flume/change@3` after change@2, its three prompts included from its
  own directory.
- Task 7.1's merge and words are as above.

## The statement

`crates/flume-conformance/src/clauses.rs` reads agentplot/blueprints at
9371cda, which is on blueprints main. `SCENARIOS` is 119 and `SKIPPED`
is empty. S119 is cited by `s119_one_agent_codes_the_change` and is not
named unbuilt. 193, S93, S94, S98 and S99 are named for record 0039's
roles; 202 to 206, 208, S109 to S112 and S118 for record 0051; 209 for
record 0053's rename.

Blueprints main has since moved to b0df70b, which takes clause 209 out
because the user rejected record 0053 on 2026-11-07. The clause-coverage
spec reads "9371cda, or the later one of main the build read", and the
build reads 9371cda, so this change meets its spec. Moving the list to
b0df70b, with 209 out of Part B and out of the unbuilt list, belongs to
the change that adopts that revision (106; core.19). It is not this
change's work.

## Requirements and scenarios

None of the seven modified requirements drops a scenario that its base
requirement in `openspec/specs/` holds. Each of the 27 scenarios of the
delta specs' 11 requirements maps to a test that asserts its THEN. Each
test was found by name on af086c8 for this report. Profile tests are in
`crates/flume/tests/profiles/`, and engine scenarios in
`crates/flume-conformance/src/engine/`.

| spec | scenario | test |
|---|---|---|
| kinds-profile | change@3's construct prompt rendered | `kinds_weir.rs` `change_3s_construct_prompt_gives_each_task_its_own_check` |
| kinds-profile | change@3's code prompt rendered | `kinds_weir.rs` `change_3s_code_prompt_runs_each_tasks_own_check_alone` |
| kinds-profile | A retry's words come last | `kinds_weir.rs` `change_3s_code_prompt_puts_a_retrys_words_last` |
| kinds-profile | change@3's verify prompt rendered | `kinds_weir.rs` `change_3s_verify_prompt_runs_the_whole_checks_once_first` |
| kinds-profile | The kinds built in | `kinds_weir.rs` `flumes_own_kinds_are_built_in` |
| kinds-profile | Each version's prompts are its own | `kinds_weir.rs` `each_versions_prompts_are_its_own` |
| kinds-profile | No prompt asks to submit | `kinds_weir.rs` `no_prompt_of_flumes_own_asks_to_submit` |
| kinds-profile | A place launched at change@1 | `kinds_weir.rs` `a_place_launched_at_change_1_keeps_it` |
| kinds-profile | A place launched at change@2 | `kinds_weir.rs` `a_place_launched_at_change_2_keeps_it` |
| kinds-profile | A code prompt sent back with a conflict | `kinds_weir.rs` `a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing` |
| kinds-profile | A package's hash wherever it lies | `kinds_weir.rs` `a_packages_hash_wherever_it_lies` |
| kinds-profile | A source's listing | `kinds_weir.rs` `a_sources_listing` |
| kinds-profile | The repository's lock at a revision | `kinds_weir.rs` `the_repositorys_lock_at_a_revision` |
| kinds-profile | A lock written and read back | `kinds_weir.rs` `a_lock_written_and_read_back` |
| kinds-profile | Flume's own as packages | `kinds_weir.rs` `flumes_own_as_packages` |
| kind-sources | The hashes recorded | `built_in.rs` `the_hashes_recorded` |
| kind-sources | A lock of flume's own at another hash | `kind_sources.rs` `a_lock_of_flumes_own_at_another_hash` |
| launching | No version named | `launching.rs` `no_version_named` |
| launching | change@2 named | `launching.rs` `change_2_named` |
| launching | change@1 named | `launching.rs` `change_1_named` |
| launching | A version flume does not hold | `launching.rs` `a_version_flume_does_not_hold` |
| launching | A version the repository's declaration does not hold | `launching.rs` `a_version_the_repository_does_not_hold` |
| launching | A kind of a host's source named whole | `launching.rs` `a_kind_of_a_hosts_source_named_whole` |
| running-through | A bare change runs change@3 through | `running_through.rs` `s119_one_agent_codes_the_change`; `tools/walkthroughs.rs` `a_change_runs_construct_code_and_verify` |
| clause-coverage | The clauses listed | `clauses.rs` `the_clauses_listed`, `the_clauses_unbuilt_are_named_by_the_changes_that_build_them` |
| clause-coverage | A clause left out | `coverage.rs` `a_clause_left_out_is_named` |
| clause-coverage | The suite as this change leaves it | conformance:coverage |

`s119_one_agent_codes_the_change` launches a bare `change`, which status
gives as `flume/change@3`. Its code stage starts once across four task
groups. Status gives that stage as `code` with no group count, and the
place's line as `change--plan · change@3 · code · run 1 · tasks 0/7 ·
working`. The place is submitted once, after verify archives the
change. `no_version_named` reads change@3's code stage as not
repeating.
