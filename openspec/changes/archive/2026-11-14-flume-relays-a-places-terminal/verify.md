# Verify: flume-relays-a-places-terminal

Verified at `319fbe4e` on `change/flume-relays-a-places-terminal`,
against its base `bolt/boomdrives-runner-reaches-a-terminal-through-flume`
at `2d180f0b`, which the branch holds.

**Verdict: holds. Archived.** The whole checks pass. Every requirement
and scenario of the change's five specs is held by a check that passed,
and all 29 tasks are ticked done, each naming a test that exists.
`openspec validate flume-relays-a-places-terminal --strict` passes.

## The whole checks

`sluice exec bolt/boomdrives-runner-reaches-a-terminal-through-flume`,
the host's sluice 0.93.2, run once: **passed**, `exit 0`. sluice ran
the checks `319fbe4` affects against the base, the bolt's
`#check-fast`: 25 tasks completed, 24 passed and none failed. It
recorded the 24 results under their keys for `319fbe4`.

The tree pins sluice 0.71.0 in the devenv shell, which refuses the
host's `~/.config/sluice/config.toml` (`unknown field credential`) and
runs no check, so the run is the host's own sluice, as the verifies of
this bolt's earlier changes ran it.

The tasks that passed: `root:format`, `root:adrs` (68 records, each
with a number of its own), `root:changes` (2 passed, this change among
them); `seams`, `core`, `fakes`, `conformance` and `flume`, each
`format`, `lint` and `test`; `conformance:coverage` (every clause at
`31f9f1c` cited by the suite's 945 scenarios); `nix:typecheck`,
`nix:test`; `plugin-claude:test` (10 passed); `site:install`,
`site:prose` (0 errors, 0 warnings, 117 suggestions) and `site:build`.

`conformance:test` ran every binary: `no_tool_is_named_outside_a_profile`
passed (core.1), as did `the_engine_passes_its_scenarios`, which runs
the place-relay scenarios, and the fake terminal's relay tests.

The run's warnings are moon's: its output archives could not be written
from the gate's sandbox, and its hasher passed over folders among a
task's inputs. No task failed for them.

## The checks the bolt's gate does not run

Tasks 5.3, 6.4, 6.5's tools test and 7.1 name `#check-tools` tests,
which the bolt's gate leaves to its merge onto main.
`devenv shell -- cargo test -p flume --features tools --test tools --
terminal_herdr:: terminal_rex::the_rex_profile_passes_the_terminal_scenarios
walkthroughs::a_person_reaches_a_fixs_agent_through_a_relay` passed at
`9f63f863`, 62 passed and 0 failed against the pinned herdr and Rex.
Between `9f63f863` and `319fbe4e` the tree's code differs only in
`crates/flume/tests/command_line.rs`, a test binary the tools run does
not build, so that run stands for this revision.

## The spec: place-relay

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| A relay reaches the place's live agent through the terminal its start recorded | The current stage's agent | conformance `engine/relay.rs` `the_current_stages_agent` | yes |
| | A start recorded in another terminal | `a_start_recorded_in_another_terminal`: typed through `other`, the host's own terminal's log holds no `relay` | yes |
| A relay of a place with no live agent, or of a terminal that offers none, is refused | A place waiting for room | `a_place_waiting_for_room`: the terminal's log empty | yes |
| | A terminal that offers no relay | `a_terminal_that_offers_no_relay` | yes |
| A relay gives the screen at once and again only when it changes | A screen given once while it stands | `a_screen_given_once_while_it_stands` | yes |
| | A read the terminal does not answer | `a_read_the_terminal_does_not_answer`: the call after the second read is `state` | yes |
| A relay types the person's input as typed | Text onto what was left typed, then enter | `text_onto_what_was_left_typed`: `draft more` submitted, cleared 0 times, no `send` | yes |
| | An input the terminal refuses | `an_input_the_terminal_refuses` | yes |
| A relay ends at its caller's close or once the agent's terminal goes | The caller closes | `the_caller_closes`, and a link that takes no more lines | yes |
| | The agent stops while its screen still reads | `the_agent_stops_while_its_screen_reads` | yes |
| | The agent closed | `the_agent_closed` | yes |
| A relay writes, settles and keeps nothing | Nothing written | `nothing_written`: the store asked `read` alone, the version and the state unchanged, status the same, no document holding either secret | yes |
| | Through the binary under herdr | `walkthroughs::a_person_reaches_a_fixs_agent_through_a_relay` (tools run above) | yes |

Each engine scenario passed in `conformance:test`'s
`the_engine_passes_its_scenarios`, and cites only clauses at `31f9f1c`,
naming 221 or 222 in its words (design section 10).

## The spec: command-line

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| relay writes JSON lines | Lines out and in | `command_line::relay_writes_screens_and_types_each_line_of_input` | yes |
| | A refusal is the only line | `command_line::relay_gives_back_what_it_cannot_type_and_a_refusal_alone` | yes |
| relay reads JSON lines | Input flume cannot read | the same test; `commands::relay::tests::a_line_of_input_read_as_text_keys_or_why_not` | yes |
| relay's help says what it speaks | The help | `command_line::relays_help_names_its_lines_and_no_caller`: the input forms, the JSON lines and both ends named, no caller of flume's; a terminal's name in the help's source left to `no_tool_is_named_outside_a_profile`, which passed | yes |

## The spec: seam-contracts

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| The terminal seam's relay | The relay among the terminal's scenarios | `the_fake_terminal_passes_the_terminal_scenarios`; `terminal_herdr::the_herdr_profile_passes_the_terminal_scenarios` (tools run above) | yes |
| | The relay refused among the terminal's scenarios | `terminal_printed::the_printed_command_passes_the_terminal_scenarios`; `terminal_rex::the_rex_profile_passes_the_terminal_scenarios` (tools run above) | yes |
| A relay refuses a key its terminal cannot type | A key the terminal cannot type | `herdr::tests::relay_refuses_a_key_herdr_cannot_type_and_types_the_keys_after_it`, herdr being the one profile with such keys; the fake types every key | yes |
| A terminal that builds no relay refuses it | A terminal that builds no relay | `terminal::tests::a_terminal_that_builds_no_relay_refuses_it` | yes |
| A person's input is text or keys | Input read and written back | `round_trip::a_persons_typed_input` | yes |
| flume names a person's keys | Key names | `terminal::tests::key_names_are_read_and_written_as_flume_names_them` | yes |

## The spec: seam-fakes

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| The fake terminal relays an open agent | A relay of the fake | `the_fake_terminals_relay_echoes_what_is_typed` | yes |
| The fake terminal scripts its relay | Scripted refusals | `the_fake_terminal_scripts_its_relay` | yes |

## The spec: terminal-profiles

| Requirement | Scenario | Held by | Met |
|---|---|---|---|
| herdr relays an agent's pane | Reads and types over a stand-in herdr | `herdr::tests::relay_reads_the_pane_as_ansi_and_types_by_send_text_and_send_keys` | yes |
| | The terminal's scenarios against the pinned herdr | `terminal_herdr::the_herdr_profile_passes_the_terminal_scenarios` (tools run above) | yes |
| herdr's relay refuses the keys herdr 0.9.5 cannot type | Every key name against the pinned herdr | `terminal_herdr::relay_types_each_of_flumes_key_names_into_a_pane` (tools run above) | yes |
| | A key herdr cannot type among keys it can | `herdr::tests::relay_refuses_a_key_herdr_cannot_type_and_types_the_keys_after_it` | yes |
| herdr's relay finds a moved pane again and reads a gone one gone | A pane moved, then gone | `herdr::tests::relay_finds_a_moved_pane_again_and_reads_a_gone_one_gone` | yes |
| | A server not running | the same test | yes |
| herdr's relay calls are bounded at one second | A read that does not answer | `herdr::tests::a_relay_read_that_does_not_answer_is_unread_naming_one_second` | yes |
| The printed command refuses relay | A relay under the printed command | `terminal_printed::relay_under_the_printed_command_is_refused_and_runs_nothing` | yes |
| Rex refuses relay | A relay under Rex | `rex::tests::relay_is_refused_and_calls_nothing` | yes |
| | The terminal's scenarios against the pinned Rex | `terminal_rex::the_rex_profile_passes_the_terminal_scenarios` (tools run above) | yes |

## The tasks

All 29 tasks are ticked `[x]`, and every test a task names is defined
once in the tree.

- 1.1 to 1.3: passed in `seams:test`.
- 2.1 to 2.3: passed in `conformance:test`.
- 3.1: passed in `core:test`; 3.2 to 3.6 in `conformance:test`'s engine scenarios; 3.7 `conformance:coverage` passed.
- 4.1 to 4.3: passed in `flume:test`. 4.4: `relays_help_names_its_lines_and_no_caller` passed in `flume:test` and `conformance:test` passed with it.
- 5.1 and 5.2: passed in `flume:test`. 5.3: passed in the tools run above.
- 6.1 to 6.3 and 6.5's unit test: passed in `flume:test`. 6.4 and 6.5's tools test: passed in the tools run above.
- 7.1: passed in the tools run above.
- 8.1: record 0070, proposed; `root:adrs` passed. 8.2: core.57 under "Stages and agents" and core.11's details amended, each citing record 0070, mirrored on the site's rules page; `root:format` and `site:prose` passed. 8.3: `flume relay`'s row, its section and its place beside `flume open`; `site:prose` and `site:build` passed.

## Of note, not findings

- Record 0070 and rule core.57 are proposed, awaiting the operator's
  yes, as the rules ask of a record.
- 222 and C.5's relay row are read at boomdrivedev/blueprints
  `b54a9b4`, on its branch `design/a-relay-refuses-a-key-it-cannot-type`,
  not yet on its main, as record 0070 says.
- The tree pins sluice 0.71.0 in the devenv shell, which cannot read
  the host's sluice configuration, so `devenv shell -- sluice exec`
  runs no check on this host.
