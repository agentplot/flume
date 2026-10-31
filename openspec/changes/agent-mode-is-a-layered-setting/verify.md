# Verify: agent-mode-is-a-layered-setting

Checked on 2026-10-31 at `change/agent-mode-is-a-layered-setting`
4383e9e, cut from main at 359f8ce, against this change's proposal,
design, twelve specs and tasks.

**The work falls short, so the change is not archived.** Every task is
ticked, every requirement is built, every scenario has a test, and the
gate passes at 4383e9e. But archiving the change breaks the gate (see
below), so it goes back to its code stage. The warnings further down
are about wording and how literal some tests are. None of them leaves
a requirement unmet.

## What fails: the archived specs fail `root:specs`

`openspec archive agent-mode-is-a-layered-setting --yes` syncs the
change's specs into `openspec/specs/`. After that,
`sluice exec main -- moon run root:specs` fails:
`openspec validate --specs --strict` reports 30 passed and 3 failed.
The strict check counts a warning as an error, and each failure is the
warning "Requirement text is very long (>500 characters)". Every
requirement it names is one this change modifies, and each was under
500 characters before the change:

| Spec | Requirement | Before (359f8ce) | After the archive |
|---|---|---|---|
| agent-profile | An agent started as Claude Code with flume's plugin | 493 | 625 |
| start-integrity | A start records what its agent runs under | 426 | 524 |
| start-integrity | The started ack is taken only where the record matches | 347 | 667 |
| store-profile | The host's configuration | 433 | 596 |

`root:changes` passed on the open change at 4383e9e, so the gate does
not catch this until the change is archived. `root:specs` is a
`check-fast` task, which every branch requires, so an archive as the
specs stand cannot land.

**What the code stage needs to do.** Bring each of these four MODIFIED
requirements under 500 characters, keeping its meaning. Move its
examples into scenarios, or split it into requirements that each state
one behaviour. Then check that a trial archive passes
`openspec validate --specs --strict` before the change comes back to
verify.

## Tasks

All 24 tasks of the eight groups are ticked, `[x]`. Each group's
closing check task is backed by the gate run below. Task 6.1 records
`flume:live` passing 4 of 4 at d265124 on 2026-10-31, run by the
operator outside the gate with a test account's token, as `CLAUDE.md`
says. The only commit since d265124 is 4383e9e, which changes
`tasks.md` alone, so that run stands for this head. It was not run
again here: it needs a test account's token, and no branch requires
`check-live`.

## Checks

`devenv shell -- sluice exec main` at 4383e9e passed all 34 tasks of
`#check-fast` and `#check-tools`, and sluice recorded the results under
4383e9e's keys. Those tasks include every one that task 8.3 names:

- `seams`, `fakes`, `core`, `conformance` and `flume`: format, lint and
  test;
- `conformance:coverage` and `flume:tools`, 55 tests among them
  `claude_words::the_pinned_release_holds_the_modes_the_map_rests_on`,
  `walkthroughs::a_hosts_mode_over_the_repositorys_and_none_where_no_layer_gives_one`
  and `walkthroughs::a_fix_begun_in_another_mode_twice_is_never_begun`;
- `plugin-claude`: format, validate and test;
- `site:prose` and `site:build`;
- `root:format`, `root:changes` and `root:specs`.

## Requirements and scenarios

Each scenario was matched to a test and the test's assertions read
against its WHEN and THEN. Short names: SI is
`crates/flume-conformance/src/engine/start_integrity.rs`, AM is
`engine/agent_modes.rs`, PS is `engine/place_status.rs`, HC is
`crates/flume/tests/profiles/hooks_claude.rs`, and AC is
`crates/flume/tests/profiles/agent_claude.rs`.

| Spec | Scenarios | Tests | Met |
|---|---|---|---|
| agent-modes | 13 | AM: bare value, table of programs, S66's host over the repository, a stage's own first, the host's table changing mid-stage, no mode, each mode mapped, a mode the profile cannot honour, a later start refused, no entry for the program, `plan` for every stage, `plan` for a conversation, an agent in `ask` asking; `kinds_weir.rs` for the real reader | yes (W1, W3) |
| launching | 1 new | `engine/agent_settings.rs` `a_mode_beside_a_model` | yes (W3) |
| start-integrity | 7 new or changed, 6 kept | SI: the mode as passed, another mode, another mode at the first prompt, S67's mode disabled by the account's configuration, the fake reading no mode back, and the six kept scenarios; the stand-in walkthroughs | yes (W3) |
| claude-code-plugin | 7 | HC: SessionStart on the program and variables, `manual` read back as `default`, no mode in the record, another mode blocked, no mode in the input, a later prompt and a compaction, an ack by hand; `plugins/claude/tests/hooks.sh` with its `user-prompt-submit-default.json` fixture | yes |
| agent-profile | 6 | AC: the command for a code stage, for a stage in `edits`, each of the four, its own value under its name, the mode read back; `claude_words.rs` for the pin | yes |
| command-line | 3 | `commands/launch.rs` unit tests; `commands.rs` `a_mode_and_a_table_at_the_launch` | yes (W2, W3) |
| kinds-profile | 2 | `kinds_weir.rs` `a_mode_for_every_stage_and_a_table_for_one`, `a_mode_that_is_neither_refuses_the_launch_naming_the_field` | yes |
| store-profile | 2 new | `host/config.rs` unit tests | yes (W3) |
| place-status | 4 | PS: host table, a table's entry, not read back, a start in another mode, in the JSON form; `print.rs` unit tests for the person's form | yes |
| seam-contracts | 7 | `seams/agent.rs` over the fake; `flume-seams/src/round_trip.rs` | yes (W3) |
| seam-fakes | 2 | SI `a_scripted_start_in_another_mode`, `another_mode_at_the_first_prompt`; `fakes.rs` | yes |
| clause-coverage | 3 | `clauses.rs` `the_clauses_listed` and its unbuilt list; `coverage.rs`; `conformance:coverage` | yes |

The clause list reads revision 5caff76, Part A and Part B exactly as the
spec gives them, and S1 to S75. Its unbuilt set is exactly the spec's
34 clauses, each under its record's change. S64 is cited by
`caller_waits.rs` `s64_a_fix_of_its_bases_failed_checks`: a fix
launched with the failure's words as its brief and its submission held,
whose code delivers, which waits on the hold, and which a retry naming
the submission submits and lands. Clauses 162, 169, S66 and S67 are
each cited by this change's scenarios.

## Design and rules

- **Decisions 1 to 6.** `ModeSetting`, `ModeEntry` and `ModeBegan` are
  built as specified. `Settings::under` takes the mode whole. In
  `Engine::resolve`, `from["mode"]` is the layer that gave the mode,
  with the mode as written. `mode_for` refuses in design 5's three
  wordings, and returns none, mapping nothing, where no layer gives a
  mode. `Engine::launch` asks `resolution` of every stage before
  anything is made (core.8).
- **Decisions 7 and 8.** SessionStart compares the program and the
  variables alone. UserPromptSubmit compares the mode only at the
  start's first prompt, the first with no received ack where no
  unsettled word stands. Where the mode differs it answers
  `{"decision":"block"}` naming both modes, writes `unsettled.json` and
  starts a settle detached. `deliver.rs` reads `started_without` while
  a start is starting and while it is started. A start read so is never
  sent its prompt again, and the stage's limit still applies to it.
- **Decision 10.** `mode::fragments()` holds the eight fragments design
  10 lists, byte for byte. A unit test pins their text, and
  `claude_words.rs` finds each of them in the pinned release.
- **core.1 and core.2.** No line this change adds outside a profile
  directory names a program. `--permission-mode` and Claude Code's
  choices appear only in `profiles/agent/claude/`, and
  `crates/flume/src/mode.rs` reads the TOML without naming one.
- **core.14.** `AgentStatus::mode`, `ModeStatus`, `Settings::mode`,
  `StartRecord::mode` and `Unsettled::mode` are added fields, each read
  with a default.
- **core.35.** The site's diff names none of the words core.35 rules
  out.

## Warnings

- **W1. A scenario that cannot be met as written.** agent-modes' "Each
  mode mapped as the profile says" launches fix's code stage once in
  each mode, `plan` among them. The same spec's "plan only where its
  stage can deliver in it" refuses `plan` there. AM
  `each_mode_mapped_as_the_profile_says` therefore launches `plan` on a
  conversation. The test is right. The scenario's wording should name a
  conversation for `plan` the next time agent-modes is touched.
- **W2. A refusal's example.** command-line's "A program's own value
  given bare" expects the refusal to say "as `mode.claude=auto`". The
  refusal comes from `ModeSetting::from_word` in the seams. It says
  `--agent mode.<program>=<value>`, as `mode.<program>=auto`, since
  code outside a profile names no program (core.1). The tests use
  `careful` in place of `auto`. The scenario's example should read
  `mode.<program>=auto`.
- **W3. Tests narrower than their scenarios.** Every one of these
  behaviours is shown by another test or follows from shared code, so
  none leaves a requirement unmet:
  - start-integrity's "Everything matches" (SI `everything_matches`)
    gives no mode. A matching mode being taken is shown by HC
    `manual_read_back_as_default`, by the S66 stand-in walkthrough and
    by the live walkthrough. In the conformance suite it is shown only
    where the fake's word names no mode.
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
    launches. It reads the model from status's `from.model`, not from
    the command.
  - `seams/agent.rs` `begun_without_and_no_turn` asserts that
    `started_without` is some, but not that it names the variable. The
    scenario was there before this change.
- **W4. Stale task wording.** Task 3.2 says "design 10's four"
  fragments. After task 6.2, design 10 and `mode::fragments()` hold
  eight, and all eight are pinned and pass.
- **W5. Clause 170 in comments.** The doc of `Agent::mode` and the
  comment on `round_trip.rs` `a_profile_that_maps_no_mode` cite 170,
  which the clause list names unbuilt under record 0026. They are
  comments, not cite data, so coverage passes.
- **W6. The statement.** C.4's started and received rows, and 161's
  word that the agent started, still read the mode at SessionStart at
  blueprints 5caff76. The operator ruled on 2026-10-31 to read it at
  the start's first prompt. `docs/architecture/_open.md` holds the item
  until the statement says so (core.19).
- **W7. The branch against main.** main has moved past 359f8ce. A trial
  merge (`git merge-tree`) conflicts in
  `crates/flume/src/commands/print.rs`, `docs/architecture/_open.md`
  and `docs/architecture/core.md`. On main, 45191ad accepts records
  0024 to 0026, while this branch's copies of core.md and record 0024
  still say "proposed". Nothing was merged here: a conflicting
  submission goes back to the kind's send-back stage with the merge
  queue's words (core.5, core.27).
