# Verify: agent-mode-is-a-layered-setting

Checked on 2026-10-31 at `change/agent-mode-is-a-layered-setting`
355ebbc, which holds main at 55a3666 (#77). Main's head is still
55a3666. The check covers this change's proposal, design, twelve specs
and tasks.

**The work holds, and the change is archived.** Every task is ticked
and every requirement is built. Every scenario has a test, the gate
passes at 355ebbc, `flume:tools` passes twice in a row, and the
archived specs pass the strict check. The warnings below are about
wording, how literal some tests are, and how far a stand-in reaches.
None of them leaves a requirement unmet.

## Tasks

All 27 tasks of the eight groups are ticked, `[x]`. The gate run below
backs each group's closing check task.

Task 6.1 records `flume:live` passing 4 of 4 at d265124 on 2026-10-31.
The operator ran it outside the gate with a test account's token, as
`CLAUDE.md` says. It was not run again here, because it needs a test
account's token and no branch requires `check-live` (W5).

Task 8.6 is met:

- **Out of the archive.** b72d89e's tree is 48ec24e's, the tree from
  before the archive, byte for byte. At 355ebbc, `openspec/specs/` and
  `openspec/changes/archive/` are main's at 55a3666, so the specs are
  unsynced.
- **What failed.** Sluice entry 79 tested c649d4e, main at 55a3666
  with bc04493, and failed `flume:tools`, 66 of 67. Its one failure was
  `a_conflict_sluice_answers_at_the_submit_goes_back_to_code_and_lands`.
  The send-back prompt held sluice's notice,
  `[sluice] #1 conflicts on main in work.rs. sluice wait 1 shows it.`,
  not the record's `conflict on main: work.rs`. Sluice marks the
  failure flaky, since the same task passed at bc04493 in this place
  minutes before.
- **What 459ace5 changes.** It changes two test files of the tools
  layer and nothing else. No code of flume's and no test of this
  change's own moved:
  - `walk.rs` `stand_in_sluice`, main's stand-in for sluice, answers
    `inbox` with `{"outcomes": []}` while its first submit runs, its
    `--no-wait` dropped. That is the shape the sluice profile reads
    from `inbox --json`. Every later command is the pinned sluice's.
  - `merge_queue_sluice.rs` gains
    `a_settle_beside_the_stand_ins_submit_collects_nothing_its_answer_delivers`.
    It steps main and collects the inbox beside that submit until the
    submit answers. Then it asserts that the inbox gave nothing, that
    the submit answered the conflict on `work.rs` in the record's
    words, and that the inbox is empty after.
- **Twice in a row.** The gate below ran `flume:tools` at 355ebbc, 68
  of 68. `sluice exec main -- moon run flume:tools --force` then ran it
  again, 68 of 68. Both runs hold the walkthrough and the new scenario,
  and both pass `instructions_of_twenty_thousand_characters_reach_the_agent_whole`.
- **Failing against the old stand-in.** `walk.rs` was put back to
  459ace5^'s with the new scenario kept, and the scenario run alone
  three times. It failed each time, its inbox collecting
  `Conflict { files: ["work.rs"], .. }` with entry 79's notice words.
  `walk.rs` was then restored to 355ebbc's.

## Checks

- **The archive.** `openspec archive agent-mode-is-a-layered-setting --yes`
  ran over a copy of `openspec/` at 355ebbc. It applies 25 added and 7
  modified requirements, and `openspec validate --specs --strict` over
  the result then passes 33 of 33. The specs it gives are bc04493's
  byte for byte.
- **Main's specs kept.** #77 changed command-line, place-profile and
  seam-contracts. This change only adds to command-line and does not
  touch place-profile. In seam-contracts it modifies "The agent seam
  reads a spent account and a start without its settings", which #77
  did not change. #77's new "The place seam says where a place is made"
  stands in the archived seam-contracts. The archive drops nothing of
  main's.
- **The gate.** `devenv shell -- sluice exec main` at 355ebbc, with the
  worktree clean, passed the 24 `#check-fast` and `#check-tools` tasks
  the branch affects, none from cache. Among the 24 is every task that
  tasks 8.3 to 8.6 name, except `plugin-claude:validate` and
  `root:specs`, which the branch's diff from main does not affect:
  - `seams`, `fakes`, `core`, `conformance` and `flume`: format, lint
    and test;
  - `conformance:coverage`;
  - `flume:tools`, 68 tests. Among them are
    `claude_words::the_pinned_release_holds_the_modes_the_map_rests_on`,
    `walkthroughs::a_hosts_mode_over_the_repositorys_and_none_where_no_layer_gives_one`,
    `walkthroughs::a_fix_begun_in_another_mode_twice_is_never_begun`,
    `walkthroughs::a_conflict_sluice_answers_at_the_submit_goes_back_to_code_and_lands`
    and the new `merge_queue_sluice` scenario;
  - `plugin-claude`: format and test;
  - `site:prose` and `site:build`;
  - `root:format`, `root:changes` and `root:adrs`.

  `root:specs` runs `openspec validate --specs --strict`, so the trial
  archive above is that check over the archived specs. The archive
  commit makes `root:specs` affected, and the gate runs on that commit
  too.

## Requirements and scenarios

The verify at 4b1e11d matched each scenario to a test and read the
test's assertions against its WHEN and THEN. Since then, this change's
own code has changed only in 643ae69's three place lookups, and 459ace5
touches main's stand-in for sluice. Each test named below was found
again at 355ebbc, and each passes in the gate run above,
`s66_a_fix_begins_in_the_mode_its_host_gives_over_the_repositorys_own`
aside, which is a live walkthrough (W5). The scenario counts are the
specs' own at 355ebbc.

Short names: SI is
`crates/flume-conformance/src/engine/start_integrity.rs`, AM is
`engine/agent_modes.rs`, PS is `engine/place_status.rs`, HC is
`crates/flume/tests/profiles/hooks_claude.rs`, and AC is
`crates/flume/tests/profiles/agent_claude.rs`.

| Spec | Scenarios | Tests | Met |
|---|---|---|---|
| agent-modes | 13 | AM: bare value, table of programs, S66's host over the repository, a stage's own first, the host's table changing mid-stage, no mode, each mode mapped (`plan` on a conversation), a mode the profile cannot honour, a later start refused, no entry for the program, `plan` for every stage, `plan` for a conversation, an agent in `ask` asking; `kinds_weir.rs` for the real reader | yes |
| launching | 3 | `engine/agent_settings.rs` `a_mode_beside_a_model` and the requirement's existing scenarios | yes (W1) |
| start-integrity | 11 | SI: the mode as passed, another mode, another mode at the first prompt, S67's mode disabled by the account's configuration, the fake reading no mode back, everything matching, no prompt to the agent ended, and the four kept scenarios; the stand-in walkthroughs | yes (W1) |
| claude-code-plugin | 7 | HC: SessionStart on the program and variables, `manual` read back as `default`, no mode in the record, another mode blocked, no mode in the input, a later prompt and a compaction, an ack by hand; `plugins/claude/tests/hooks.sh` with its `user-prompt-submit-default.json` fixture | yes |
| agent-profile | 6 | AC: `the_command_for_a_code_stage` (no `--permission-mode`), `the_command_for_a_stage_in_edits`, `each_of_the_four`, `its_own_value_under_its_name`, `the_mode_read_back`; `claude_words.rs` for the pin | yes |
| command-line | 3 | `commands/launch.rs` unit tests; `commands.rs` `a_mode_and_a_table_at_the_launch` | yes (W1) |
| kinds-profile | 2 | `kinds_weir.rs` `a_mode_for_every_stage_and_a_table_for_one`, `a_mode_that_is_neither_refuses_the_launch_naming_the_field` | yes |
| store-profile | 6 | `host/config.rs` `a_mode_for_a_kind_and_a_table_for_a_repositorys_stage`, `a_mode_that_is_neither_is_refused_naming_the_table_and_mode`, and the modified requirement's existing scenarios | yes (W1) |
| place-status | 4 | PS: host table, a table's entry, not read back, a start in another mode, in the JSON form; `print.rs` unit tests for the person's form | yes |
| seam-contracts | 7 | `seams/agent.rs` over the fake; `flume-seams/src/round_trip.rs` | yes (W1) |
| seam-fakes | 2 | SI `a_scripted_start_in_another_mode`, `another_mode_at_the_first_prompt`; `fakes.rs` | yes |
| clause-coverage | 3 | `clauses.rs` `the_clauses_listed` and its unbuilt list; `coverage.rs`; `conformance:coverage` | yes |

The clause list reads revision 5caff76, Part A and Part B exactly as the
spec gives them, and S1 to S75. Its unbuilt set is exactly the spec's
34 clauses, each under its record's change. S64 is cited by
`caller_waits.rs` `s64_a_fix_of_its_bases_failed_checks`. This change's
scenarios cite 162, 169, S66 and S67. Neither #77 nor 459ace5 touches
the clause list.

## Design, rules and the merges

- **Decisions 1 to 6.** `ModeSetting`, `ModeEntry` and `ModeBegan` are
  built as specified. `Settings::under` takes the mode whole. In
  `Engine::resolve`, `from["mode"]` is the layer that gave the mode,
  with the mode as written. `mode_for` refuses in design 5's three
  wordings. Where no layer gives a mode it returns none and maps
  nothing.
- **core.8 over #77.** #77 has `Engine::launch` ask the place seam for
  the location where it once built `<repository>/places/<kind>--<name>`.
  `Engine::launch` still asks `resolution` of every stage before
  anything is made. `a_mode_and_a_table_at_the_launch` shows each mode
  refused at the launch leaves nothing at the place's location.
- **Decisions 7 and 8.** SessionStart compares the program and the
  variables alone. UserPromptSubmit compares the mode only at the
  start's first prompt. That is the first prompt with no received ack,
  where no word that the agent began otherwise stands. Where the mode
  differs it answers `{"decision":"block"}` naming both modes, writes
  `unsettled.json` and starts a settle detached.
- **The merges.** Main is merged at 5d29913, df45390 and 55a3666.
  `deliver.rs` holds both sides of df45390. A start begun otherwise is
  read while Starting or Started, ended once quiet and never sent its
  prompt again. Main's dialog is read while Starting. 55a3666 merged
  clean.
- **Decision 10.** `mode::fragments()` holds the eight fragments design
  10 lists. A unit test pins their text, and `claude_words.rs` finds
  each of them in the pinned release.
- **core.1 and core.2.** No line this change adds outside a profile
  directory names a program. `--permission-mode` and Claude Code's
  choices appear only in `profiles/agent/claude/`. The refusal of a bare
  value names `<program>`, not a program. 459ace5's lines name sluice
  only in the sluice profile's tests and the walkthroughs' harness.
- **core.6.** 459ace5 leaves the outcome read from the merge queue's
  record. The new scenario asserts the conflict reaches flume once, in
  the submit's answer, and that no inbox gives it again.
- **core.14.** `AgentStatus::mode`, `ModeStatus`, `Settings::mode`,
  `StartRecord::mode` and `Unsettled::mode` are added fields, each read
  with a default.
- **core.19.** Records 0024, 0025 and 0026 read accepted. core.10's
  detail reads the mode at the start's first prompt. `_open.md` names
  the mode built, the plan stage and points still to build, and the
  statement's SessionStart reading as an open item. #77's own item on
  C.1 and record 0027 stands beside them. 459ace5 and 355ebbc cite 112
  and 133.
- **core.35.** The site's diff names none of the words core.35 rules
  out.

## Warnings

- **W1. Tests narrower than their scenarios.** Each of these behaviours
  is shown by another test or follows from shared code, so none leaves
  a requirement unmet:
  - start-integrity's "Everything matches" (SI `everything_matches`)
    gives no mode. A matching mode being taken is shown by HC
    `manual_read_back_as_default`, by the S66 stand-in walkthrough and
    by the live walkthrough.
  - `commands.rs` `a_mode_and_a_table_at_the_launch` reads the mode
    from the place's held launch settings, which are the `launch` layer
    by construction. It does not check status's `from.mode`.
  - store-profile's "A mode that is neither" asserts that
    `HostConfig::parse` refuses naming `kinds.fix.agents` and `mode`.
    It runs no launch over that configuration. The refusal of every
    launch is the existing path for an unreadable configuration.
  - AM `a_programs_own_value_given_bare` scripts the fake kinds
    profile's refusal words. The real reader is
    `kinds_weir.rs` `a_mode_that_is_neither_refuses_the_launch_naming_the_field`,
    and the words "under its program's name" are shown by
    `round_trip.rs` `a_bare_word_of_a_programs_own`.
  - `agent_settings.rs` `a_mode_beside_a_model` uses one repository for
    both `/r` and `/s`, changing the host's table between its two
    launches.
  - `seams/agent.rs` `begun_without_and_no_turn` asserts that
    `started_without` is some, but not that it names the variable. The
    scenario was there before this change.
- **W2. Stale task wording.** Task 3.2 says "design 10's four"
  fragments. Design 10 and `mode::fragments()` hold eight, and all
  eight are pinned and pass.
- **W3. Clause 170 in comments.** The doc of `Agent::mode` and the
  comment on `round_trip.rs` `a_profile_that_maps_no_mode` cite 170,
  which the clause list names unbuilt under record 0026. They are
  comments, not cite data, so coverage passes.
- **W4. The statement.** C.4's started and received rows, and 161's
  word that the agent started, still read the mode at SessionStart at
  blueprints 5caff76. The operator ruled on 2026-10-31 to read it at
  the start's first prompt. `docs/architecture/_open.md` holds the item
  until the statement says so (core.19).
- **W5. The live walkthroughs since d265124.** `flume:live` last passed
  at d265124, before #77 moved a clone's places under flume's data
  directory. Since then the live harness has changed in two ways. Main
  finds its walkthroughs' places through `walk.rs`, and this change's
  S66 reads its place from its launch's answer. Both build under
  `flume:lint`, and the stand-in walkthroughs pass through the same
  `walk.rs` in `flume:tools`. Nothing has run them under herdr with the
  pinned Claude Code since. That run is the operator's to make by hand,
  and no branch requires it.
- **W6. The stand-in for sluice hides a window.** The stand-in now
  answers its inbox with nothing for as long as its first submit runs.
  It rests on the pinned sluice delivering an entry it decides as it
  takes it in with that submit's answer. Sluice's `report_delivering`
  marks the outcome delivered once the answer is written and flushed.
  That was read in a sluice worktree on this host, not in the pinned
  0.57.1's source. No test now shows what flume does with an outcome an
  inbox gives in the instant between the decision and its mark. That
  window is sluice's and the walkthrough is main's, not this change's.
  Either way, entry 79's prompt shows the send-back still names the
  files contested and the base's revision, which come from the
  outcome's own fields.
