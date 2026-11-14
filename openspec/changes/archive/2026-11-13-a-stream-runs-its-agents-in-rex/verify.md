# Verify: a-stream-runs-its-agents-in-rex

Verified at `85439b66` on `change/a-stream-runs-its-agents-in-rex`,
against its base `bolt/rex-is-flumes-second-terminal` at `b54e8908`,
which the branch holds.

**Verdict: holds. Archived.** The whole checks pass. Every requirement
and scenario of the change's six specs is met by a check that passed.
Every task is ticked done, and every check a task names exists and
passed.

## The whole checks

`sluice exec bolt/rex-is-flumes-second-terminal`, the host's sluice
0.93.2, run once: **passed**, `exit 0`. sluice ran the checks `85439b6`
affects against the base, the bolt's `#check-fast`: 38 tasks completed
and 37 passed. It recorded the 37 results under their keys for
`85439b6`.

The run first asked through `devenv shell -- sluice exec …` stopped
before any check: inside the shell the sluice the tree pins for the
merge-queue profile's tests, 0.71.0, comes first on `PATH`, and it
refused the host's `~/.config/sluice/config.toml` (`unknown field
credential`, line 21). No check ran under it. The run above is the
host's own sluice, which enters the devenv toolchain itself as
`.config/sluice.toml`'s `[tests] toolchain` says.

The tasks it ran: `root:format`, `root:hooks`, `root:adrs` (65 records,
each with a number of its own), `root:specs` (49 passed), `root:changes`
(2 passed, this change among them), `root:marketplace` and the root's
other tasks; `seams`, `core`, `fakes`, `conformance` and `flume`, each
`format`, `lint` and `test`; `conformance:coverage`; `nix:format`,
`nix:lint`, `nix:typecheck`, `nix:test`; `plugin-claude:validate`,
`format`, `typecheck`, `engine-types`, `test`; `plugin-herdr:manifest`,
`lint`, `test`; `site:install`, `site:prose` (0 errors, 0 warnings) and
`site:build` (every internal link valid).

The run's warnings are moon's: its output archives could not be written
from the gate's sandbox. No task failed for them.

Of note:

- `conformance:test`: `no_tool_is_named_outside_a_profile` (core.1) ok.
- `flume:test`: `built_in::terminals_and_windows_are_given_by_name`,
  `commands::a_launch_from_a_rex_block_names_its_block`,
  `commands::a_streams_multiplexer_flume_does_not_carry`, every
  `profiles::terminal::rex::tests::*` and
  `profiles::terminal::common::*` ok.
- `openspec validate a-stream-runs-its-agents-in-rex --strict`: valid.

## The tasks' own checks outside the bolt's gate

The bolt requires `#check-fast` alone; the Rex profile's tools tests
and the walkthrough are `#check-tools`, which main requires. The
checks tasks 1.1, 3.1 to 3.16, 4.1, 4.2 and 5.1 name were run as they
name them, once, in the devenv shell with the pinned Rex
0.1.0-preview.14:

`devenv shell -- cargo test -p flume --features tools --test tools --
terminal_rex:: walkthroughs::a_fix_on_a_stream_under_rex_lands_through_the_binary
merge_queue_sluice::a_submission_from_a_rex_block_names_no_session`:
**passed**, `exit 0`, 32 passed and 0 failed in 33.81 seconds: the 30
`terminal_rex::*` tests, the Rex walkthrough and the sluice profile's
submission from a Rex block.

## The spec: terminal-profiles

### ADDED

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| The terminal through Rex | Seam scenarios against Rex | `terminal_rex::the_rex_profile_passes_the_terminal_scenarios` | yes |
| | A quiet block with nothing delivered | `terminal_rex::a_done_agent_with_nothing_delivered_waits_and_no_settle_ends_it` | yes |
| Every Rex call is bounded and names its server | A Rex call that never answers | `terminal_rex::a_rex_call_that_never_answers_is_refused_within_its_bound` | yes |
| | A call from inside a Rex block | `terminal_rex::a_call_from_inside_another_servers_block_reaches_its_own_server_alone` | yes |
| Rex's servers are flume's own | A session's server started where none answers | `terminal_rex::a_sessions_server_is_started_where_none_answers` | yes |
| | A server stopped and started again | `terminal_rex::a_server_started_again_restores_nothing`: the first agent stopped, the second in a new Rex session of its base | yes |
| A session's name Rex cannot hold is refused | A session name the socket cannot hold | `rex::tests::a_session_name_the_socket_cannot_hold_is_refused`; `runs` refusing `../x` in `runs_reads_three_parts_at_once_and_starts_none` | yes |
| Where Rex opens an agent | Places on two bases | `terminal_rex::places_on_two_bases_open_in_two_rex_sessions` | yes |
| | A stage of two agents | `terminal_rex::a_second_agent_of_a_place_splits_its_window` | yes |
| A block is labelled its agent's Rex name | A long place's Rex name | `rex::tests::a_long_places_rex_name_is_128_characters_ending_in_its_digest` | yes |
| Rex opens beside a named block | Beside a named block | `terminal_rex::an_agent_opens_beside_a_named_block` | yes |
| Rex starts the agent by a script of the start's own | No variable in any process's arguments | `terminal_rex::no_variable_of_the_agent_stands_in_any_processs_arguments`; `rex::tests::the_start_script_holds_every_variable_and_no_argument_does` | yes |
| Rex's ready is the program's first report | A program that never reports | `terminal_rex::a_program_that_never_reports_is_never_begun_with_the_screen` | yes |
| | A program that exits as it starts | `terminal_rex::a_program_that_exits_as_it_starts_is_never_begun_at_once` | yes |
| Rex claims the identity before it makes a block | Two opens of one identity | `terminal_rex::two_opens_of_one_identity_leave_one_block` | yes |
| Rex runs only a program that reports its state to it | A program that reports nothing to Rex | `rex::tests::a_kind_that_reports_nothing_to_rex_is_refused` | yes |
| Rex runs the program the settings name | A program that runs another in its place | `terminal_rex::a_program_that_runs_another_in_its_place_is_refused_naming_it` | yes |
| Rex answers the folder-trust question from the screen | The folder-trust question at the start | `terminal_rex::s39_the_folder_trust_question_is_answered_and_the_agent_comes_up_under_rex` | yes |
| | The question drawn after the open returned | `terminal_rex::s39_a_folder_trust_question_drawn_after_the_open_is_answered_at_a_settle_under_rex` | yes |
| Rex's send leaves nothing typed | A prompt left typed | `terminal_rex::s43_a_prompt_left_typed_is_cleared_before_each_sending_under_rex` | yes |
| | A working agent sent a tell | `terminal_rex::a_working_agent_is_sent_a_tell_with_no_ctrl_c` | yes |
| | A text past sixty thousand bytes | `terminal_rex::a_text_past_sixty_thousand_bytes_arrives_whole`; `rex::tests::a_text_is_cut_between_characters_and_a_screen_is_its_last_rows` | yes |
| What Rex reads of an agent | Rex's readings as the profile reads them | `terminal_rex::rexs_readings_as_the_profile_reads_them`; `rex::tests::the_state_is_read_from_the_programs_newest_report` | yes |
| | The screen | `terminal_rex::the_screen_is_the_blocks_last_rows` | yes |
| Rex's close reads that the block is gone | A close that does not happen | `terminal_rex::a_close_rex_cannot_make_is_refused` | yes |
| Rex tidies what it made | The last place on a base | `terminal_rex::s47_the_last_place_on_a_base_closes_its_window_and_rex_session` | yes |
| | A person's block keeps the Rex session | `terminal_rex::s48_a_persons_block_keeps_the_rex_session_named` | yes |
| Rex reaches a session by its name | A recipient's block | `terminal_rex::a_recipient_block_is_sent_to_and_read_by_its_name` | yes |
| | A role run's address | `terminal_rex::a_role_runs_address_reaches_its_block` | yes |
| | A herdr session's name | `rex::tests::a_herdr_sessions_name_is_none_of_rexs` | yes |
| Rex gives a session's agents by name | A standing agent by its label | `terminal_rex::a_standing_agent_is_given_by_its_label` | yes |
| Rex's runs asks each server | Three parts in one reading | `terminal_rex::runs_reads_three_parts_at_once_and_starts_none` | yes |
| Rex's attach and show | A Rex stream's view | `rex::tests::attach_refuses_both_views_and_show_shows_none` | yes |

### MODIFIED

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| The caller's session as the terminal names it | A launch from a herdr pane | `built_in::a_recipient_unnamed_is_the_herdr_pane_the_command_runs_in`, unchanged but for `REX_*` stripped | yes |
| | A launch from a Rex block | `commands::a_launch_from_a_rex_block_names_its_block`; `rex::tests::a_command_run_in_a_rex_block_names_its_block` | yes |

## The spec: stream-terminals

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| A stream naming Rex runs its places in Rex (ADDED) | A fix on a stream under Rex | `walkthroughs::a_fix_on_a_stream_under_rex_lands_through_the_binary`: the block at `trial.sock`, the Rex session `<repository> main`, the window `fix--one`, status `{"profile": "rex", "from": "stream"}`, landed, then no Rex session and the server running | yes |
| A stream whose multiplexer flume carries no profile for is refused alone (ADDED) | A stream naming cmux | conformance `a_stream_naming_cmux` | yes |
| | A stream changed to cmux while it waits | conformance `a_stream_changed_to_cmux_while_it_waits` | yes |
| A stream whose multiplexer names no profile is refused alone (REMOVED) | | its scenarios name `cmux` in the requirement above; no scenario holds `rex` uncarried | yes |

## The spec: launching

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| A launch whose binding or settings flume cannot run is refused whole (MODIFIED) | A stream's multiplexer flume does not carry | `commands::a_streams_multiplexer_flume_does_not_carry`: refused naming `trial`, `cmux` and `rex` among the profiles; conformance `launching.rs` on `cmux` | yes |
| | The other three scenarios | unchanged, their tests passing in `flume:test` and `conformance:test` | yes |

## The spec: the-list

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| The list gives a line for each stream the host declares (MODIFIED) | Three streams over the fake terminal | unchanged, conformance `list.rs` | yes |
| | A multiplexer flume carries no profile for | conformance `list.rs`: `trial · session trial · no profile for cmux`; `flume-core` `list::tests`; `commands::print::tests::the_list_by_repository_then_the_streams` | yes |

## The spec: seam-contracts

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| The resolver gives a terminal and a window by name (MODIFIED) | Named terminals and windows | `fakes::the_resolver_holds_terminals_and_windows_by_name`, asking for `cmux` | yes |
| | The terminals built in | `built_in::terminals_and_windows_are_given_by_name`: `herdr` twice one, `rex` given, nothing under the state, `cmux` none, the list `printed`, `herdr`, `rex` | yes |

## The spec: merge-queue-profile

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| Submitted through sluice with no wait (MODIFIED) | A submission from a herdr pane's hook | unchanged, its tools test | yes |
| | A submission from a Rex block | `merge_queue_sluice::a_submission_from_a_rex_block_names_no_session`; `a_session` in `profiles/merge_queue/sluice/mod.rs` takes `REX_` | yes |

## The tasks

All 40 tasks are ticked `[x]`, and every test a task names is defined
once in the tree.

- 1.1: Rex pinned from its flake's 0.1.0-preview.14 URL in `devenv.yaml`, locked in `devenv.lock`, `rex` in `devenv.nix`'s packages where the flake packages it; `rex` resolves in the devenv shell.
- 1.2: `profiles/terminal/common.rs` holds the folder-trust reading, the program check, the base's label, the digest and the records; herdr's module calls it, and neither profile calls the other.
- 2.1 to 2.10: the profile named `rex`, registered third in `terminal_profiles()`, built over `<state>/terminal/rex` running nothing, with its socket rule, Rex name, session form, start script, state reading, pieces, kind refusal and attach; each unit test passed in `flume:test`.
- 3.1 to 3.16, 4.1, 4.2: the profile against the pinned Rex and through the binary, each passing in the tools run above.
- 5.1 to 5.3: `REX_*` stripped by the sluice profile and by the tests' lists, `flume:lint`, `flume:test` and `plugin-claude:test` passing in the gate.
- 6.1 to 6.5: `cmux` the uncarried multiplexer in `commands.rs`, `list.rs`, `print.rs`, `fakes.rs` and the conformance scenarios; no `"rex"` stands uncarried in `flume-core`, `flume-fakes` or `flume-conformance`.
- 7.1: record 0068, proposed; core.29 reads as the task gives it, its details naming `--autostart=false`, the socket and the caller's `REX_*`; core.1's details name `rex` and the common module; `root:adrs` passed.
- 7.2: the site names `rex` among `multiplexer`'s values with its socket, the roadmap's terminals row and paragraph, and a section "Places under Rex" in the guide; `site:prose` and `site:build` passed.

## Of note, not findings

- Record 0068 is proposed, awaiting the operator's yes, as the rules
  ask of a record; core.29's amendment cites it as proposed.
- The site's guide says flume refuses a session's name that "holds a
  `/`" beside its 104-byte socket rule; the profile also refuses an
  empty name, `.`, `..` and a NUL, which the spec states and the guide
  leaves out as no reader's case.
