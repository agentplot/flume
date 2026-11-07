# Tasks

Each group is one session's work for an agent that reads this change
(proposal, design, specs) and its group, and builds on the place's base
and on the groups before it. The statement's amendment is on
agentplot/blueprints main at 734d60c (41, 48, 131), and the build reads
the statement there. A group's last task runs the checks it touched
through sluice. Every conformance scenario a group adds cites its
clauses as data (106). The engine's scenarios name no tool (core.1).
Tests use fixtures only: no real account, configuration home, herdr
session or host configuration of the user's.

## 1. The decision recorded

- [x] 1.1 Amend record 0011 with its amendment of 2026-11-06, the design agent's: a found goes back only to a fix round before the stage that found, a found from a last stage that is its kind's fix round is taken as needing an answer, and every agent is told a found is work a stage before its own must do again, with its deciders, its sources (the statement on blueprints main at 734d60c; crew's `fix--a-landing-in-another-hosts-queue-reads-as-in-the-queue` at flume main 376c17ca), context, five considered options, outcome, why not the others and consequences (41, 42, 48, 49, 131, 156; core.19). True when `sluice exec main -- moon run root:adrs root:format` passes.
- [x] 1.2 Amend core.27 in `docs/architecture/core.md` as the amendment has it, its details ruling out a found sent back to the stage that found and words that call a found anything but work a stage before the agent's own must do again, and citing record 0011 as amended 2026-11-06, and have `site/src/content/docs/architecture/rules.md` mirror it (core.19, core.35). True when `sluice exec main -- moon run root:format site:prose site:build` passes.
- [x] 1.3 Run the checks of the projects this group touched. True when `openspec validate a-found-at-its-kinds-own-fix-round-waits-for-an-answer --strict` and `sluice exec main -- moon run root:format root:adrs root:changes site:prose site:build` pass.

## 2. The core

- [x] 2.1 In `crates/flume-core/src/submit.rs`, add `fix_round_of(held) -> Option<usize>`, the stage the kind's last stage names in `send_back_to`, else that last stage, and have `fix_round` take its target from it and `waiting_on`'s past-bound branch in `status.rs` its stage (design 1; 23, 112, 131). True when `sluice exec main -- moon run core:test conformance:test` passes with every existing scenario unchanged, `engine/running_through.rs` holding the running-through spec's "A fix's failed submission" as `a_fixs_failed_submission`.
- [x] 2.2 Have `sent_back_by` hold only where `fix_round_of` is a stage before the one that found, and `waiting_on`'s `EndedShort` branch answer nothing only where `sent_back_by` holds, a retry of the stage waiting on an answer with its words otherwise (design 2, 3; 41, 42, 48, 130, 131; core.27). True when `sluice exec main -- moon run core:test conformance:test` passes, `engine/running_through.rs` holding the running-through spec's "A fix's code finds work" as `a_fixs_code_finds_work`, "The retry that answers a fix's found" as `the_retry_that_answers_a_fixs_found` and "A last stage that names no stage for a failed submission" as `a_last_stage_naming_none_finds_work`, and `verify_finds_work` citing 48.
- [x] 2.3 Update the module documentation of `engine/running_through.rs`, `found_work`'s and `sent_back_by`'s in `submit.rs` and `needs`' in `agent_acts.rs` to say that a found goes back only to a fix round before the stage that found and is otherwise taken as needing an answer (131). True when `sluice exec main -- moon run core:lint conformance:lint` passes.
- [x] 2.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run core:format core:lint core:test conformance:format conformance:lint conformance:test conformance:coverage` passes.

## 3. What every agent is told

- [x] 3.1 Give Claude Code's `stop_short` in `crates/flume/src/profiles/agent/claude/mod.rs` design 5's words, and hold them in `crates/flume/tests/profiles/agent_claude.rs` as the agent-profile spec's scenario has it (28, 41, 131; core.31). True when `sluice exec main -- moon run flume:test` passes.
- [x] 3.2 Give `flume needs --found` the help "The stage found work a stage before its own must do again." in `crates/flume/src/commands/mod.rs`, held by a test in `crates/flume/tests/commands.rs` as the command-line spec's scenario has it; say the same in `StopKind`'s and `StopShort`'s documentation in `crates/flume-seams/src/agent.rs`; and give the fake agent's `stop_short` in `crates/flume-fakes/src/agent.rs` the same sense (41, 92, C.4). True when `sluice exec main -- moon run seams:test fakes:test flume:test` passes.
- [x] 3.3 Give `walkthroughs.rs`'s `STOP_SHORT` the new words, and add a walkthrough in the `check-tools` layer under the stand-in for Claude Code: a fix whose code runs `flume needs --found '<findings>'` waits on an answer with one agent opened and its line `waiting on an answer`; its caller's `flume retry --stage code --words '<answer>'` starts code once more, its prompt holding the findings and then the answer; and the fix lands (41, 42, 48, 49, 131). True when `sluice exec main -- moon run flume:tools` passes.
- [x] 3.4 Run the checks of the projects this group touched. True when `sluice exec main -- moon run seams:format seams:lint seams:test fakes:format fakes:lint fakes:test flume:format flume:lint flume:test flume:tools` passes.

## 4. The site

- [x] 4.1 Say in `site/src/content/docs/guides/kinds.mdx`'s "How a kind runs through" and its `on_fail` row, `guides/merge-queue.md`'s count of goings-back and `reference/configuration.md`'s `on_fail` row that found work from the last stage sends the place back only where `on_fail` names a stage before it, and otherwise waits on an answer, as fix's and chore's code does; in `guides/callers-and-holds.md`'s table of ends and its stop-short section that a found is work a stage before the agent's own must do again, and when it waits on an answer; and give `reference/prompt-parts.md`'s way to stop short and `reference/commands.md`'s `flume needs` row the new words (28, 41, 131; core.35). True when `sluice exec main -- moon run site:prose site:build` passes.
- [ ] 4.2 Run the checks of the projects this change touched. True when `openspec validate a-found-at-its-kinds-own-fix-round-waits-for-an-answer --strict` and `sluice exec main -- moon run root:format root:adrs root:changes core:test conformance:test conformance:coverage flume:test flume:tools site:prose site:build` pass.
