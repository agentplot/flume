# Verify: an-agents-effort-is-a-setting-of-flumes-layers

Read on 2026-11-07 at `6a2427a`, the head of the branch
`change/an-agents-effort-is-a-setting-of-flumes-layers`. The check
covers the change's proposal, design, ten delta specs and tasks. The
branch holds flume's main at `f6a5ba7`, merged by task 8.1 after the
merge queue found the first archive conflicting with #131 to #141.
Origin's main, fetched on 2026-11-07, is still at `f6a5ba7`.

## Verdict

**Holds; archived.**

- Every task of groups 1 to 8 is ticked: 21 of 21.
- Every requirement of the ten delta specs is built, and each scenario
  is held by a test that asserts its THEN (below). Each test named here
  is in the tree at the head.
- `openspec validate an-agents-effort-is-a-setting-of-flumes-layers
  --strict` passes.
- `openspec/specs/` at the head is main's at `f6a5ba7`, byte for byte,
  and `agent-effort` is not among them until the archive.
- A trial archive passes 45 of 45 specs. It ran on the head's
  `openspec/`, copied to a scratch directory, and applied 16
  requirements added and 7 modified. It made the new spec
  `agent-effort`. No requirement or scenario heading main holds at
  `f6a5ba7` is lost from any spec. Launching and clause-coverage, whose
  requirements the change only modifies, come out as the first archive
  at `7a5bfc4` made them. Place-status, seam-contracts and seam-fakes
  hold the effort's requirements beside the standing address's, as the
  merge's resolution did, the effort's after them.
- `sluice exec main` at `6a2427a`, against main at `f6a5ba7`, passes
  all 23 of the check tasks the head affects, and sluice recorded the
  23 results under their keys for `6a2427a`. check-fast and check-tools
  are among them: flume:test's suites, its largest 226 of 226, and
  flume:tools 117 of 117, which runs S103's walkthrough, the efforts'
  check of the pinned Claude Code and #141's standing agent under
  herdr. conformance:coverage finds every clause it holds at `c94c496`
  cited by the suite's 718 scenarios, 197 and S103 among them.

## The merge of main at f6a5ba7

The merge commit `bfdf502` took #131 to #141. Git found five files
contested, and each keeps both sides:

- `crates/flume-conformance/src/engine/place_status.rs` and
  `crates/flume/src/commands/print.rs`: the conflict was each file's
  import list alone. Each now imports `EffortStatus` beside
  `AddressStatus`, `StandingFound` and `PendingEnd`, and no other hunk
  of either was contested.
- `openspec/specs/place-status`, `seam-contracts` and `seam-fakes`:
  each held the effort's requirements, from the first archive, beside
  the standing address's from #141. Commit `6a2427a` then moved the
  change out of the archive and gave back `openspec/specs/` as main
  holds it, so the archive above applies the change's deltas afresh.

Both sides also changed twelve other files, which git merged
untouched: the conformance's `answers.rs` and `fake_harness.rs`, the
core's `answer.rs`, the fakes' tests, the seams' `round_trip.rs`,
`commands/mod.rs`, `host/config.rs`, the tools' `walkthroughs.rs`,
`core.md`, and the site's commands, configuration and state references.
Each still holds the effort's lines against main, and the gate above
runs each test among them. Record 0044, core.30, core.39 and the site's
rules page still carry the effort, and the clause list still drops 197
and S103 from the unbuilt.

## The scenarios and their tests

### agent-effort (new)

Each scenario is a conformance scenario in
`crates/flume-conformance/src/engine/agent_effort.rs`, run against the
fakes, citing 197 with its other clauses.

| Scenario | Test |
|---|---|
| A word kept as written | `a_word_kept_as_written`: the settings give `xhigh` from `kind`, and the command passes it |
| A table refused | `a_table_refused`, over a declaration the fake reads as failing on `kinds.fix.agents.effort`, nothing made; the real read is `kinds_weir.rs`'s `an_effort_that_is_no_word_refuses_the_launch_naming_the_field` |
| The launch's over the kind's | `the_launchs_over_the_kinds`: construct and verify at `high` from `kind`, code at `max` from `launch` |
| The host's over the repository's | `the_hosts_over_the_repositorys`: `medium` from `host-kind` |
| An effort for a repository, a stream and every agent | `for_a_repository_a_stream_and_every_agent`: `xhigh` from `host-repository`, `high` from `host-stream` in `work`, `medium` from `host` in `default` |
| The host's table changed after the launch | `the_hosts_table_changed_after_the_launch`: construct and code keep `high` in their held starts; verify starts at `xhigh`, each from `host-kind` |
| No effort in any layer | `no_effort_in_any_layer`: no `--effort`, no `effort` in the fake's log, and none in status or its layers |
| A word the fake takes and one it does not | `a_word_taken_and_one_refused`: `high` asked at the launch and at the start and passed; `med` refused naming `med`, `agent`, `code`, `launch`, `low` and `high`, nothing made |
| A refusal at the launch | `a_refusal_at_the_launch`: `ultracode` for verify refuses the change's launch, naming `agent`, `verify` and `launch`; no agent opened, nothing made |
| A refusal at a later start | `a_refusal_at_a_later_start`: `turbo` from `host-kind` leaves code refused, no agent opened; a retry after the table is mended starts it |

The core asks the profile in `effort_taken`
(`crates/flume-core/src/settings.rs`), called from
`Engine::resolution`, which the launch runs for every stage before
anything is made and each start runs again. It asks nothing where no
layer gives an effort, and its refusal is in design 8's words.
`Settings::under` takes the effort whole, and `Engine::resolve` folds
it as the model is, `from["effort"]` naming its layer.

### agent-profile

| Scenario | Test |
|---|---|
| The command for a stage with no effort | `agent_claude.rs`, `the_command_for_a_stage_with_no_effort` |
| The command for a stage at xhigh | `agent_claude.rs`, `the_command_for_a_stage_at_xhigh`: `--model opus --effort xhigh`, the effort right after the model |
| Each of the five, and two the release reads otherwise | `agent_claude.rs`, `each_of_the_five_and_two_the_release_reads_otherwise`, and the variable `CLAUDE_CODE_EFFORT_LEVEL` |
| effortLevel written in a table | `agent_claude.rs`, `effort_level_written_in_a_table`; conformance `own_settings.rs`, `an_efforts_key_written_in_a_table` |
| effortLevel in a named file | `agent_claude.rs`, `effort_level_in_a_named_file`: the file passed holds `maxEffortLevel` and `tui`, and status names `effortLevel` left out; conformance `an_efforts_key_in_a_named_file` |
| The pinned release's efforts read | `tools/claude_words.rs`, `the_pinned_release_takes_the_efforts_the_profile_passes`, over `effort::fragments()`, and `effortLevel`'s declaration among `settings.rs`'s reserved fragments |
| Each reserved key (modified) | `agent_claude.rs`'s reserved keys test, `effortLevel` with the owner flume's `effort` |
| A people's settings file, Keys taken as written (modified) | main's tests, unchanged and passing |

`crates/flume/src/profiles/agent/claude/effort.rs` holds the five
words, the refusal, the variable and the release's fragments. Its unit
tests hold the fragments as written and the refusal's words. Claude
Code's `effort` and `effort_variable` come from it, and its command
passes `--effort <word>` after `--model`.

### place-status

| Scenario | Test |
|---|---|
| An effort from the launch | conformance `place_status.rs`, `an_effort_from_the_launch`; `print.rs`, `an_effort_from_the_launch`; and S103's walkthrough, which reads both forms |
| An account's variable that outranks it | conformance `an_effort_outranked_by_the_account`; `print.rs`, `an_effort_the_accounts_variable_outranks`; `agent_claude.rs`, `an_effort_the_accounts_variable_outranks`, on an account whose `env` gives `CLAUDE_CODE_EFFORT_LEVEL` |
| No effort | conformance `no_effort_in_status`, its JSON holding no `effort`; `print.rs`, `no_effort` |

`effort_status` (`crates/flume-core/src/status.rs`) names the variable
only where the start's `from` holds `env.<NAME>` from `account`, so
never a value. `EffortStatus` is only added to `AgentStatus` (core.14).

### command-line

| Scenario | Test |
|---|---|
| An effort for every stage and for one | `commands.rs`, `an_effort_for_every_stage_and_for_one_at_the_launch`, over a scratch repository: held settings give `high` and verify `max`, and each agent's effort is from `launch`; `launch.rs` unit test `an_effort_for_every_stage_and_for_one` |
| The settings named in a refusal | `launch.rs`, `a_refusal_names_the_effort_among_the_settings`; `commands.rs`'s refusal test now requires `effort` in the words, and `--help` names it |

### kinds-profile

| Scenario | Test (`crates/flume/tests/profiles/kinds_weir.rs`) |
|---|---|
| An effort for every stage and one for a stage | `an_effort_for_every_stage_and_one_for_a_stage` |
| An effort for every kind of a repository | `an_effort_for_every_kind_of_a_repository` |
| An effort that is no word | `an_effort_that_is_no_word_refuses_the_launch_naming_the_field`: a number and a table, in the repository's `[agents]`, its table for a kind, a stage's table and a stage's `agents`, each named, and the launch refused |
| A repository's agents for every kind (modified) | main's test, unchanged and passing |

The sounding-shaped test that read `effort = "high"` as the runner's
now asserts it read as the stage's effort.

### store-profile

| Scenario | Test (`crates/flume/src/host/config.rs`) |
|---|---|
| An effort for a kind and one for a repository's stage | `an_effort_for_a_kind_and_one_for_a_repositorys_stage`, through `Host::settings` at the host-kind and host-repository layers |
| An effort for every agent from the local layer | `an_effort_for_every_agent_set_locally`: `high` from the local layer, nothing passed over |
| An effort that is no word | `an_effort_that_is_no_word_is_refused_naming_the_table_and_effort`, in the kind's table and in `[agents]`, `[agents.streams.work]` and `[repos."/r".agents]` |
| The three modified requirements' scenarios | main's tests, unchanged and passing |

### launching

The modified requirement adds the effort to the eight layers' list. Its
five scenarios are main's, unchanged and passing; the effort through
every layer is agent-effort's scenarios above.

### seam-contracts and seam-fakes

| Scenario | Test |
|---|---|
| A profile that names no way to give an effort | `round_trip.rs`, `a_profile_that_names_no_way_to_give_an_effort` |
| The fake's efforts over the seam's scenarios | `seams/agent.rs`, `effort_takes_its_programs_words` and `the_command_passes_the_effort`, generic over the harness, run for the fake and for Claude Code |
| A held state with an effort | `round_trip.rs`, `a_held_state_with_an_effort`, which also reads a state written before the effort with none and writes none back |
| A scripted effort taken and passed | `fakes.rs`, `the_agents_efforts_are_scripted`; conformance `a_word_taken_and_one_refused` reads the log at the launch and the start |

### clause-coverage

`crates/flume-conformance/src/clauses.rs` drops the change's unbuilt
entry for 197 and S103 and counts two fewer unbuilt. Both are cited by
this change's scenarios, 197 throughout and S103 by
`the_launchs_over_the_kinds`, status's `an_effort_from_the_launch` and
the stand-in's walkthrough. The list's revision stays `c94c496`, which
holds both.

## The walkthroughs

- **S103 under the printed command**
  (`tools/walkthroughs.rs`,
  `a_kinds_effort_and_a_launchs_for_review_start_each_agent_at_its_own_and_the_place_lands`).
  A repository's `bump` gives `xhigh` and the launch gives review
  `max`. Code is printed with `--effort xhigh` from `kind` and review
  with `--effort max` from `launch`. Each stand-in says it began there,
  each form of status names it, and the place lands. The stand-in reads
  `--effort` as 2.1.296 does, warning of an unknown word and beginning
  at its default (`tools/stand_in.rs`).
- **The live fix** (`live/walkthroughs.rs`) launches with
  `--agent effort=low` and reads `low` from `launch`. Task 5.2's commit
  `c0222a20` records the operator's run by hand in this place at
  `691d6a2e`: `flume:live` 7 passed, 0 failed, on 2026-11-07. The
  live layer is required by no branch, so this verify did not run it
  again.

## The record, the rules and the site

- Record 0044 is accepted, on the user's approval of the design on
  2026-11-06. core.30 gains the effort's sentence and its details, and
  core.39 an effort its profile cannot honour; the site's rules page
  mirrors both.
- The site writes the effort into the agent-settings, kinds and Claude
  Code guides and the configuration, commands and state references. No
  added line names a tool, team or host of ours (core.35).
- The statement's 197 and S103 are on blueprints main at `cd8fd73`;
  the clause list reads `c94c496`, which holds them.

## Observations

These do not hold the change back.

- `crates/flume/src/commands/print.rs`'s unit tests name
  `CLAUDE_CODE_EFFORT_LEVEL` as the variable in their sample status, as
  the same file's tests already name `CLAUDE_CONFIG_DIR`. The printing
  code names no program; it prints the variable status gives.
- The stand-in reads `ultracode` as its own level where the design says
  2.1.296 reads it as `xhigh` with dynamic workflows on. No test leans
  on it: Claude Code's profile refuses `ultracode` before any command
  is built.
- Record 0055, accepted on main, names a repository by
  its address. The host's `[repos."<repository>".agents]`, which this
  change reads `effort` in, is keyed by location as main's code keys it
  today; moving that key is 0055's own change.

## Decisions in this stage

None. The verify read what the change built and archived it.
