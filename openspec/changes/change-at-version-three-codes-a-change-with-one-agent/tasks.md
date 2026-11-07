# Tasks

The groups are headings for reading. Each task names the narrowest check
that shows it done, run as written; no task runs the repository's whole
checks or a group's, which the change's verify stage and the merge queue
run (record 0054). The user accepted record 0054 on 2026-11-06, and the
statement carries it on agentplot/blueprints main at 49b9d53 and
9371cda; the build reads the statement there. A test filter after `--`
is passed by moon to the task's `cargo test`.

## 1. The planning rules

- [x] 1.1 In `openspec/config.yaml`'s `rules.tasks`, replace the first rule and the last with design 8's two rules, the narrowest check per task and no task running the whole checks or a group's, keeping the three rules between them as they are (kit:specs.3; record 0054). Check: `devenv shell -- treefmt --ci openspec/config.yaml`.
- [x] 1.2 In `CLAUDE.md`'s paragraph under "Checks", say that `sluice exec main` runs the gate's checks, which a change's verify stage runs once and no task of a change runs, and that a task's own check runs as the task names it, beside `cargo check` and one unit test, as design 8 has it (record 0054). Check: `devenv shell -- treefmt --ci CLAUDE.md`.

## 2. change@3 built in

- [x] 2.1 Add `crates/flume/src/profiles/kinds/weir/own/change@3/kind.toml` and its `prompts/change-construct.md`, `prompts/change-code.md` and `prompts/change-verify.md`, each exactly as design 1 and 2 give them; list `flume/change@3` in `OWN` in `crates/flume/src/profiles/kinds/weir/mod.rs` after change@2, its doc comments naming change@1, change@2 and change@3 side by side; and have `each_versions_prompts_are_its_own` in `crates/flume/tests/profiles/kinds_weir.rs` read the five directories, change@1's and change@2's prompts as before and change@3's three none of theirs (22, 156, 157, C.2). Check: `devenv shell -- moon run flume:test -- --test profiles each_versions_prompts_are_its_own`.
- [x] 2.2 Have `flumes_own_kinds_are_built_in` read change@3 on `change/` with construct, code and verify, its code stage not repeating, landing, its verify's `on_fail` `code` (22, 140, C.2). Check: `devenv shell -- moon run flume:test -- --test profiles flumes_own_kinds_are_built_in`.
- [x] 2.3 Have `flumes_own_as_packages` read `flume/change@3` with its hash, and add change@3's hash to `OWN_HASHES` in `crates/flume/tests/profiles/built_in.rs`, leaving the other four as they are (156, 157). Check: `devenv shell -- moon run flume:test -- --test profiles flumes_own_as_packages` and `devenv shell -- moon run flume:test -- --test profiles the_hashes_recorded`.
- [x] 2.4 Have `no_prompt_of_flumes_own_asks_to_submit` render every prompt of change@1, change@2 and change@3, and `a_code_prompt_sent_back_with_a_conflict_merges_the_base_never_rebasing` change@3's code prompt beside fix@1's, chore@1's and change@2's (28, 71, 112). Check: `devenv shell -- moon run flume:test -- --test profiles no_prompt_of_flumes_own_asks_to_submit` and `devenv shell -- moon run flume:test -- --test profiles a_code_prompt_sent_back_with_a_conflict`.
- [ ] 2.5 Add to `kinds_weir.rs` one test for each of the kinds-profile spec's four added scenarios over change@3's prompts rendered for a place, each named from `change_3s_`: its construct prompt, its code prompt, its code prompt's words last, and its verify prompt (22, 131, C.2, S119; core.33). Check: `devenv shell -- moon run flume:test -- --test profiles change_3s_`.
- [ ] 2.6 Add `a_place_launched_at_change_2_keeps_it` to `kinds_weir.rs`: a place launched by a flume holding change@1 and change@2 alone, settled by one holding all three, keeps change@2's repeating code stage and status names change@2 (21). Check: `devenv shell -- moon run flume:test -- --test profiles a_place_launched_at_change_2_keeps_it`.

## 3. The engine's and the kinds seam's scenarios

- [ ] 3.1 Give `flumes_own()` in `crates/flume-conformance/src/fake_harness.rs` change@3, change@1's stages at version 3 with a code prompt of its own, after change@2, and have the kinds seam's `flumes_own` scenario in `src/seams/kinds.rs` read fix@1, chore@1, change@1, change@2 and change@3, change@2's code alone repeating (22, 140, 157). Check: `devenv shell -- moon run conformance:test -- --test seams the_fake_kinds_pass_the_kinds_scenarios` and `devenv shell -- moon run flume:test -- --test profiles the_kinds_profile_passes_the_kinds_scenarios`.
- [ ] 3.2 Have `engine/launching.rs` hold the launching spec's scenarios: no version runs change@3, not repeating, and status names `flume/change@3`; `change@2` named runs change@2, repeating; `change@1` named as before; `change@4` refused naming versions 1, 2 and 3, nothing made (4, 19, 22, 154); and have every other engine scenario that launches a bare `change` and reads a repeating code stage, a group, `code 1/2`, a repetition's notes or `flume/change@2` launch `change@2` by name, as design 5 has it. Check: `devenv shell -- moon run conformance:test -- --test engine`.
- [ ] 3.3 Add to `engine/running_through.rs` the running-through spec's scenario, citing 22, 154 and S119: a bare `change` runs change@3, its construct writing four task groups, its code stage one start whose status and line give `code` with no group count, its verify archiving the change and the place submitted, as design 6 has it. Check: `devenv shell -- moon run conformance:test -- --test engine`.

## 4. The walkthroughs

- [ ] 4.1 Have `a_change_code_agents_line_names_its_group_its_run_and_its_tasks`, `a_change_of_three_task_groups_takes_three_agents_in_turn` and `a_failed_submission_sends_a_repeating_stage_back_for_one_repetition_with_no_group` in `crates/flume/tests/profiles/tools/walkthroughs.rs` launch `change@2`, and the module's doc comment say so (140 to 142, S50, S52). Check: `devenv shell -- moon run flume:tools -- a_change_code_agents_line_names_its_group`, `devenv shell -- moon run flume:tools -- a_change_of_three_task_groups_takes_three_agents_in_turn` and `devenv shell -- moon run flume:tools -- a_failed_submission_sends_a_repeating_stage_back`.
- [ ] 4.2 Have `a_change_runs_construct_code_and_verify` keep its bare `change` and read that the place runs `flume/change@3` and its code stage started once (S2, S35, S119). Check: `devenv shell -- moon run flume:tools -- a_change_runs_construct_code_and_verify`.

## 5. The clause list

- [ ] 5.1 Have `crates/flume-conformance/src/clauses.rs` read agentplot/blueprints main at 9371cda, or the later revision of main the build reads, as design 7 and the clause-coverage spec have it: every clause the revision states in Part A or Part B, `SCENARIOS` its last scenario, `SKIPPED` only what it skips, each clause and scenario added since c94c496 but S119 named unbuilt by the change that builds it, and the list's own tests at the revision's counts (106; core.19). Check: `devenv shell -- moon run conformance:test -- --lib clauses` and `devenv shell -- moon run conformance:coverage`.

## 6. The site

- [ ] 6.1 In `site/src/content/docs/guides/kinds.mdx`, list flume's own kinds with `flume/change@3`, say a bare `change` takes the newest, `change@3`, unless a lock records another, and say in a sentence each how change@1, change@2 and change@3 code a change, as design 9 has it, to any host's operator (22, 154; core.35). Check: `devenv shell -- moon run site:prose`.
