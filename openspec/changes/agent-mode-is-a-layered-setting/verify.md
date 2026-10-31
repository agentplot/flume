# Verify: agent-mode-is-a-layered-setting

Checked on 2026-10-31 at `change/agent-mode-is-a-layered-setting`
4b1e11d, which holds main at df45390, against this change's proposal,
design, twelve specs and tasks.

**The work holds, and the change is archived.** Every task is ticked,
every requirement is built, every scenario has a test, the gate passes
at 4b1e11d, and the archived specs pass the strict check. The warnings
below are about wording and how literal some tests are. None of them
leaves a requirement unmet.

## Tasks

All 25 tasks of the eight groups are ticked, `[x]`. Each group's
closing check task is backed by the gate run below. Task 6.1 records
`flume:live` passing 4 of 4 at d265124 on 2026-10-31. The operator ran
it outside the gate with a test account's token, as `CLAUDE.md` says.
It was not run again here: it needs a test account's token, and no
branch requires `check-live`. Since d265124 the change's side has
changed only its specs and tasks. The code changes since then are main
merged in twice, so the run stands for this change's code.

Task 8.4 is met:

- The four requirements that an archive left over 500 characters are
  each back under it. The mode in each now stands as an ADDED
  requirement with the scenario that shows it: agent-profile's
  "The settings' mode passed by --permission-mode", start-integrity's
  "A start records the mode its profile passes" and "A mode its program
  names only at the first prompt", and store-profile's "A mode in the
  host's agents tables". agent-profile's "An agent started as Claude
  Code with flume's plugin" and start-integrity's "A start records what
  its agent runs under" are no longer modified. They stand in the main
  specs as main has them.
- agent-modes' "Each mode mapped as the profile says" launches `plan`
  on a conversation, as AM `each_mode_mapped_as_the_profile_says` does.
  command-line's "A program's own value given bare" expects
  `--agent mode.<program>=auto`, which is the refusal `commands/launch.rs`
  gives.
- Main is merged at 5d29913 and at df45390, and df45390 is main's head
  today.

## Checks

- **The archive.** `openspec archive agent-mode-is-a-layered-setting --yes`
  over a copy of `openspec/` at 4b1e11d applies 25 added and 7 modified
  requirements. `openspec validate --specs --strict` over the result
  then passes 33 of 33.
- **The modified requirements.** Each of the seven MODIFIED requirements
  was compared with main's text at df45390. Each keeps main's text and
  adds the mode. The exception is clause-coverage's two, which replace
  the revision, the unbuilt list and the clauses this change cites, as
  they are meant to. None of the four main specs that main changed
  after 359f8ce (command-line, history-and-host, kinds-profile and
  terminal-profiles) holds a requirement this change modifies, so the
  archive drops nothing of main's.
- **The gate.** `devenv shell -- sluice exec main` at 4b1e11d, on a
  clean tree, passed the 24 `#check-fast` and `#check-tools` tasks the
  branch affects, and sluice recorded the result under 4b1e11d's key.
  Among them are every task that task 8.3 names except
  `plugin-claude:validate` and `root:specs`, which the branch's diff
  from main does not affect:
  - `seams`, `fakes`, `core`, `conformance` and `flume`: format, lint
    and test;
  - `conformance:coverage`;
  - `flume:tools`, 67 tests. Among them are
    `claude_words::the_pinned_release_holds_the_modes_the_map_rests_on`,
    `walkthroughs::a_hosts_mode_over_the_repositorys_and_none_where_no_layer_gives_one`,
    `walkthroughs::a_fix_begun_in_another_mode_twice_is_never_begun`,
    and main's
    `walkthroughs::an_agent_run_after_its_start_was_read_never_begun_is_begun_and_a_retry_leaves_it`;
  - `plugin-claude`: format and test;
  - `site:prose` and `site:build`;
  - `root:format`, `root:changes` and `root:adrs`.

  `root:specs` runs `openspec validate --specs --strict`, so the trial
  archive above is that check over the archived specs. The archive
  commit makes `root:specs` affected, so the gate runs it there.

## Requirements and scenarios

Each scenario was matched to a test and the test's assertions read
against its WHEN and THEN. The reworded scenarios of task 8.4 were read
again against their tests at 4b1e11d. Every test named below stands at
4b1e11d. Short names: SI is
`crates/flume-conformance/src/engine/start_integrity.rs`, AM is
`engine/agent_modes.rs`, PS is `engine/place_status.rs`, HC is
`crates/flume/tests/profiles/hooks_claude.rs`, and AC is
`crates/flume/tests/profiles/agent_claude.rs`.

| Spec | Scenarios | Tests | Met |
|---|---|---|---|
| agent-modes | 13 | AM: bare value, table of programs, S66's host over the repository, a stage's own first, the host's table changing mid-stage, no mode, each mode mapped (`plan` on a conversation), a mode the profile cannot honour, a later start refused, no entry for the program, `plan` for every stage, `plan` for a conversation, an agent in `ask` asking; `kinds_weir.rs` for the real reader | yes |
| launching | 1 new | `engine/agent_settings.rs` `a_mode_beside_a_model` | yes (W1) |
| start-integrity | 7 new or changed, 4 kept | SI: the mode as passed, another mode, another mode at the first prompt, S67's mode disabled by the account's configuration, the fake reading no mode back, everything matching, no prompt to the agent ended, and the four kept scenarios; the stand-in walkthroughs | yes (W1) |
| claude-code-plugin | 7 | HC: SessionStart on the program and variables, `manual` read back as `default`, no mode in the record, another mode blocked, no mode in the input, a later prompt and a compaction, an ack by hand; `plugins/claude/tests/hooks.sh` with its `user-prompt-submit-default.json` fixture | yes |
| agent-profile | 6 | AC: `the_command_for_a_code_stage` (no `--permission-mode`), `the_command_for_a_stage_in_edits`, `each_of_the_four`, `its_own_value_under_its_name`, `the_mode_read_back`; `claude_words.rs` for the pin | yes |
| command-line | 3 | `commands/launch.rs` unit tests; `commands.rs` `a_mode_and_a_table_at_the_launch` | yes (W1) |
| kinds-profile | 2 | `kinds_weir.rs` `a_mode_for_every_stage_and_a_table_for_one`, `a_mode_that_is_neither_refuses_the_launch_naming_the_field` | yes |
| store-profile | 2 new | `host/config.rs` `a_mode_for_a_kind_and_a_table_for_a_repositorys_stage`, `a_mode_that_is_neither_is_refused_naming_the_table_and_mode` | yes (W1) |
| place-status | 4 | PS: host table, a table's entry, not read back, a start in another mode, in the JSON form; `print.rs` unit tests for the person's form | yes |
| seam-contracts | 7 | `seams/agent.rs` over the fake; `flume-seams/src/round_trip.rs` | yes (W1) |
| seam-fakes | 2 | SI `a_scripted_start_in_another_mode`, `another_mode_at_the_first_prompt`; `fakes.rs` | yes |
| clause-coverage | 3 | `clauses.rs` `the_clauses_listed` and its unbuilt list; `coverage.rs`; `conformance:coverage` | yes |

The clause list reads revision 5caff76, Part A and Part B exactly as the
spec gives them, and S1 to S75. Its unbuilt set is exactly the spec's
34 clauses, each under its record's change. S64 is cited by
`caller_waits.rs` `s64_a_fix_of_its_bases_failed_checks`. Clauses 162,
169, S66 and S67 are each cited by this change's scenarios.

## Design, rules and the merges

- **Decisions 1 to 6.** `ModeSetting`, `ModeEntry` and `ModeBegan` are
  built as specified. `Settings::under` takes the mode whole. In
  `Engine::resolve`, `from["mode"]` is the layer that gave the mode,
  with the mode as written. `mode_for` refuses in design 5's three
  wordings. Where no layer gives a mode it returns none and maps
  nothing. `Engine::launch` asks `resolution` of every stage before
  anything is made (core.8).
- **Decisions 7 and 8.** SessionStart compares the program and the
  variables alone. UserPromptSubmit compares the mode only at the
  start's first prompt. That is the first prompt with no received ack,
  where no word that the agent began otherwise stands. Where the mode
  differs it answers `{"decision":"block"}` naming both modes, writes
  `unsettled.json` and starts a settle detached.
- **The merge at df45390.** `deliver.rs` holds both sides. A start begun
  otherwise is read while Starting or Started, ended once quiet and
  never sent its prompt again. Main's side holds too: the folder-trust
  question is answered only while Starting, a dialog shown holds the
  start from being read never begun, and a started ack clears the
  dialog. The walkthroughs hold main's `an_agent_run_after_its_start_was_read_never_begun_is_begun_and_a_retry_leaves_it`
  beside this change's two. `state.md`'s agent row names both `mode`
  and `dialog`.
- **Decision 10.** `mode::fragments()` holds the eight fragments design
  10 lists. A unit test pins their text, and `claude_words.rs` finds
  each of them in the pinned release.
- **core.1 and core.2.** No line this change adds outside a profile
  directory names a program. `--permission-mode` and Claude Code's
  choices appear only in `profiles/agent/claude/`. The refusal of a bare
  value names `<program>`, not a program.
- **core.14.** `AgentStatus::mode`, `ModeStatus`, `Settings::mode`,
  `StartRecord::mode` and `Unsettled::mode` are added fields, each read
  with a default.
- **core.19.** Records 0024, 0025 and 0026 read accepted. core.10's
  detail reads the mode at the start's first prompt. `_open.md` names
  the mode built, the plan stage and points still to build, and the
  statement's SessionStart reading as an open item.
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
