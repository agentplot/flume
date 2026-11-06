# Verify: an-agents-program-settings-come-from-flumes-layers

Read on 2026-11-06 at `f6cc42f`, the head of the branch
`change/an-agents-program-settings-come-from-flumes-layers`. The check
covers the change's proposal, design, twelve delta specs and tasks. The
branch holds flume's main at `b51498e`. Origin's main, after a fetch on
2026-11-06, is `4b3d2d1`: #127 added one commit, `53a65f8`, which
touches record 0013, core.28, the site's rules page and its roadmap,
and no spec or code. It merges with the branch with no file contested,
and sluice ran the gate's checks against it.

## Verdict

**Holds; archived.**

- Every task of groups 1 to 11 is ticked: 29 of 29.
- Every requirement of the twelve delta specs is built: 40
  requirements and 81 scenarios. Each scenario is held by a test that
  asserts its THEN (below).
- `openspec validate an-agents-program-settings-come-from-flumes-layers
  --strict --no-interactive` passes.
- A trial archive passes 44 of 44 specs. It ran on the head's
  `openspec/`, copied to a scratch directory. The specs it writes are
  byte for byte those of the first archive at `74d7819`: 36
  requirements added, 4 modified and 1 renamed, and the new spec
  `own-settings`. The four it modifies, launching's layers,
  agent-modes' layers and clause-coverage's two, keep every scenario
  main holds and add this change's. clause-coverage's two scenarios
  are rewritten for this change, as each change rewrites them.
- `sluice exec main` at `f6cc42f`, against main at `4b3d2d1`, passes
  all 24 of the check tasks the head affects, flume:tools 105 of 105
  among them. sluice recorded the pass under its keys for `f6cc42f`.
- Task 11.1's mend holds (below). Its conformance scenario fails
  without the mend and passes with it.
- The live check (10.1) passed 7 of 7 at `f6ebc1f8`, run by hand
  outside the gate as `CLAUDE.md` says. Since then only 11.1's mend
  changed code, in the core's delivery. It was not run again here: it
  needs a test account's token, and no branch's gate requires
  `check-live`.

## Task 11.1: the race #126's flume:tools failed on

#126's run on main failed `a_fix_begun_in_another_mode_twice_is_never_begun`:
code read never begun with no `unsettled` in its end. The cause is in
`Engine::deliver_reading` (`crates/flume-core/src/deliver.rs`). One
pass reads the start begun without its settings and records so, in
`read_unsettled`. It then reads the agent not quiet, and reads it
stopped. `never_begun` then built the end from the held state read
before that record, so the end did not name how it began.

The mend reads such a start once more from the held state as it stands
before it reads it never begun. On that second pass:

- where `read_unsettled` reads the agent quiet, it ends it as clause
  161 says: a first start is started afresh, and a second never begun
  with its end naming the variable;
- where the agent is stopped but still not quiet, `never_begun` builds
  the end from the held state that holds the record, so the end names
  the variable.

`read_again` bounds the loop to one more pass. Both outcomes are the
start-integrity spec's: "a start so begun that is not quiet by the
stage's limit SHALL be never begun, naming the same". The new
conformance scenario, `stopped_before_it_reads_quiet` in
`engine/start_integrity.rs`, cites 161, 35 and 43. It is the second of
these: an agent stopped with its own work going on, read in the
launch's own delivery. Its end names `AGENT_HOME`, and no prompt was
sent.

With the mend's twelve lines taken out of `deliver.rs`,
`cargo test -p flume-conformance --test engine` fails 1 of 538 engine
scenarios, this one, with "the end names the variable:
`None where Some(["AGENT_HOME"]) was due`". With them, all 538 pass. In the gate, flume:tools ran
`a_fix_begun_in_another_mode_twice_is_never_begun`, the walkthrough
#126 failed on, and it passed.

The mend changes no requirement: the start-integrity spec holds the
behaviour already, and the change's deltas are unchanged.

## Tasks

All 29 tasks are ticked, `[x]`, and none is open.

- **The record and the rules** (1.1, 5.4): record 0043 is `proposed`.
  It waits on the user's yes once they have seen it as a record, at
  their review of this change. Records 0017, 0020 and 0024 each carry
  the line saying 0043 amends them. core.30 is amended and core.50 is
  drawn into "Stages and agents". Each cites blueprints main at
  `033d887`.
- **The clause list** (1.2, 5.4, 9.1): `REVISION` is `fa40c94`, which
  main's #125 set. `fa40c94` is on blueprints main and descends from
  `033d887`. It states 195 in Part A, 196 in Part B and S100 to S102,
  as the clause-coverage delta allows ("or the later one of main the
  build read"). None of the five is named unbuilt, and
  conformance:coverage passes.
- **The seams** (2.1, 2.2): `Settings::programs` with
  `ProgramSettings` and `NamedFile`, `Layer::HostStream` and
  `Layer::Host`, `KindsRead::Read`'s `agents`, the held
  `repository_agents`, `Agent::reserved`, `Agent::own`, and
  `AgentCommand::files`. `Store::write_file` writes a command's files.
  The fake agent scripts its way by `--own <file>` and its reserved
  keys.
- **The readers** (3.1 to 3.4): `crates/flume/src/own.rs` reads an
  entry and its file. The kinds profile reads `programs` and a
  repository's `[agents]`. `host/config.rs` reads `[agents]`,
  `[agents.streams.<stream>]` and `[repos."<repository>".agents]`.
  `commands/launch.rs` reads both `--agent` forms.
- **The core** (4.1, 4.2, 4.4): `crates/flume-core/src/own.rs` leaves
  out reserved keys, refuses written ones, checks for credentials and
  folds key by key. `Engine::layers` gives the eight layers in clause
  32's order: launch, host-repository with its table for every kind
  last, host-kind, host-stream, host, repository with `[agents]` last,
  kind, default. Status gives `programs`, `from` and `own`.
- **The pool's filter** (4.3): moved to record 0026's change, as the
  task says. Neither the spec nor the proposal holds it.
- **Claude Code** (5.1, 5.2): `claude/settings.rs` names the thirteen
  reserved keys the agent-profile delta lists, each with its owner. It
  writes `own-settings.json` and passes `--settings` after
  `--plugin-dir`. `claude_words.rs` finds every fragment in the pinned
  release.
- **The stand-in and the walkthroughs** (6.1): S100, S101 and S102 run
  under the printed command, each landing or refused as its scenario
  says.
- **The live walkthrough** (7.1, 10.1):
  `a_hosts_own_settings_reach_its_fix_agents_session_and_commit` is in
  `live/walkthroughs.rs`, and flume:lint builds it.
- **The site** (8.1): `guides/agent-settings.md`, `guides/kinds.mdx`,
  `reference/configuration.md`, `reference/commands.md` (both
  `--agent` forms), `guides/claude-code.mdx` and `reference/state.md`.
  None names a host, a team or a word of ours (core.35). site:prose
  and site:build pass.
- **The mend** (11.1): above.

## Specs

Each scenario and the test that asserts it. Tests under
`crates/flume-conformance/src/` are named by their function.

- **own-settings**, 15 requirements and 34 scenarios:
  `engine/own_settings.rs` holds 31 of them, from
  `two_programs_entries` to `a_named_file_not_passed`. The readers'
  three refusals are the configuration's tests:
  - a program's key beside the file, and a date, are in
    `host/config.rs`'s `a_programs_that_is_no_table_of_entries`;
  - a host's file that cannot be read is in
    `a_hosts_file_flume_cannot_read`.

  A file outside the repository is also read over a scratch repository
  in `kinds_weir.rs`'s
  `a_settings_file_outside_the_repository_or_not_at_the_revision_refuses_the_launch`.
- **launching**, 1 requirement and 5 scenarios: main's three, and
  `engine/agent_settings.rs`'s scenarios of a stream and every agent,
  and of every kind of a repository.
- **agent-modes**, 1 requirement and 4 scenarios: main's three, and
  `engine/agent_modes.rs`'s stream's mode over the kind's.
- **store-profile**, 4 requirements and 7 scenarios:
  `host/config.rs`'s `settings_for_every_agent_and_for_a_stream`,
  `a_table_for_every_agent_or_a_stream_cannot_hold`,
  `own_settings_for_a_kind_and_for_a_repositorys_stage`,
  `a_programs_that_is_no_table_of_entries`,
  `a_hosts_file_flume_cannot_read`, `a_forks_table_for_every_kind` and
  `a_repositorys_table_that_names_a_stage`. Beside them,
  `a_hosts_file_read_where_its_link_leads` and
  `own_settings_set_locally_key_by_key`.
- **streams**, 1 requirement and 1 scenario: `engine/streams.rs`'s
  place in a stream taking its stream's settings over every agent's,
  citing S100 and 143.
- **kinds-profile**, 4 requirements and 5 scenarios: `kinds_weir.rs`'s
  `own_settings_for_every_stage_and_for_one`,
  `a_launchs_declaration_reads_its_settings_file_on_disk`,
  `a_settings_file_outside_the_repository_or_not_at_the_revision_refuses_the_launch`,
  `a_repositorys_agents_for_every_kind` and
  `a_repositorys_agents_table_naming_a_stage_or_an_environment_refuses_the_launch`.
  `own.rs`'s `a_shape_flume_cannot_take_is_refused_naming_its_key`
  holds an entry flume cannot take.
- **command-line**, 2 requirements and 3 scenarios:
  `commands/launch.rs`'s `own_settings_at_the_launch`,
  `a_key_given_twice_is_refused_naming_it` and `a_file_at_the_launch`.
  `tests/commands.rs`'s `own_settings_and_a_file_at_the_launch` runs
  over a scratch repository.
- **agent-profile**, 4 requirements and 9 scenarios:
  `agent_claude.rs`'s `the_command_for_a_stage_with_own_settings`,
  `the_command_for_a_stage_with_none`, `each_reserved_key`,
  `a_peoples_settings_file`, `keys_taken_as_written` and
  `a_repositorys_tagline_written_as_given`. The fork, the repository's
  own words and a repository nothing names are in
  `walkthroughs.rs`'s
  `a_forks_attribution_is_the_hosts_a_repositorys_its_own_and_another_its_accounts`.
  The pinned release is read in `claude_words.rs`'s
  `the_pinned_release_takes_own_settings_as_the_profile_passes_them`.
  fix@1 and chore@1 each have one stage, so "every stage of each" is
  code.
- **place-status**, 1 requirement and 3 scenarios:
  `engine/place_status.rs`'s three own-settings scenarios. The keys
  left out are named with their owners, `the account` and `flume's
  mode`. The person's form is in `commands/print.rs`'s
  `an_agent_with_own_settings_from_a_file_and_inline`,
  `a_long_own_setting_is_elided` and `an_agent_with_no_own_settings`.
- **seam-contracts**, 4 requirements and 6 scenarios:
  `seams/agent.rs`'s reserved, own and command scenarios, which run
  over the fake scripted both ways. `round_trip.rs` holds
  `a_profile_that_passes_no_own_settings`,
  `a_held_state_with_own_settings` and
  `a_held_state_with_a_repositorys_settings_for_every_kind`.
  `engine/starts.rs` holds the two files.
- **seam-fakes**, 1 requirement and 1 scenario:
  `engine/own_settings.rs`'s `a_scripted_way_and_a_reserved_key`, and
  `fakes.rs`'s `the_agents_own_settings_are_scripted`.
- **clause-coverage**, 2 requirements and 3 scenarios: `clauses.rs`'s
  `the_clauses_listed` and
  `the_clauses_unbuilt_are_named_by_the_changes_that_build_them`, and
  conformance:coverage.

## The checks

`sluice exec main` at `f6cc42f` ran the checks the head affects
against main at `4b3d2d1`, in 8m 19s, and passed these 24 tasks:

- conformance: coverage, format, lint, test
- core: format, lint, test
- fakes: format, lint, test
- flume: format, lint, test, tools
- plugin-claude: test
- root: adrs, changes, format, versions
- seams: format, lint, test
- site: build, prose

These cover task 11.1's own checks: core:test, conformance:test,
conformance:coverage and flume:tools. moon warned that it could not
write its output cache and that it had no internet connection for its
dependency install. Neither warning touches a task's result.

## Outside this repository

- Clause 128 and C.7's store row give the store a file written into
  the place's folder. They are amended at blueprints `352aa32` on the
  branch `design/flume-the-store-writes-a-starts-files`, as task 4.3
  asked. Record 0043 and design 8 cite it there. The branch is on this
  host's clone alone: origin does not hold it and blueprints main does
  not contain it, as read on 2026-11-06 after a fetch. It waits on the
  orchestrator to push and land it.
- Record 0043 stays `proposed` until the user's yes at their review.
