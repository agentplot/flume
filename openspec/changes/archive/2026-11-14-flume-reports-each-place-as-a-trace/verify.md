# Verify: flume-reports-each-place-as-a-trace

Verified on `change/flume-reports-each-place-as-a-trace` at 2229ddde,
cut from `bolt/flume-shows-its-places-as-traces` (2d180f0b).

**Outcome: the whole checks pass and the work holds; the change is
archived.**

## The whole checks

`sluice exec bolt/flume-shows-its-places-as-traces`, run once with the
host's sluice (0.93.2): exit 0. sluice ran the checks 2229ddd affects
against the base at 2d180f0, the bolt's required `#check-fast`: 30 tasks
completed, 29 results recorded under their keys for 2229ddd, none
failed. Among them `seams`, `core`, `fakes`, `conformance` and `flume`
format, lint and test; `conformance:coverage`; `nix` format, lint,
typecheck and test; `site:install`, `site:prose` and `site:build`;
`plugin-claude:test`, `plugin-herdr:manifest` and `plugin-herdr:test`;
`root:format`, `root:changes`, `root:adrs` and `root:versions`. moon
warned it could not archive task outputs into `.moon/cache/outputs`,
which fails nothing.

`devenv shell -- sluice exec …` ran no check: the devenv's sluice is the
0.71.0 `devenv.nix` pins for the profile tests, and it refused the
host's `~/.config/sluice/config.toml` (`unknown field credential`)
before running anything. The run above is the checks' one run.

`openspec validate flume-reports-each-place-as-a-trace --strict`: valid.

## Tasks

Every task of `tasks.md`, 1.1 to 7.1, is ticked, and every test a task
names exists under the name it gives and passed in the run above: the
named tests across `flume-seams`, `flume-core`, `flume-fakes`, `flume`
and its `host`, `profiles` and `telemetry` test binaries, the seven
`telemetry_…` tests of `host::config`, the module's
`collectorOffTheLoopback` case in `nix/tests/fails.nix` and
`scripts.sh`, and the conformance scenarios of `engine/telemetry.rs`.
Among them task 4.8's
`telemetry::tests::a_clue_that_does_not_show_goes_as_a_warning` and task
4.9's `start_record::tests::an_accounts_telemetry_variables_are_in_no_record`.

Task 4.7 holds whole: every core line of flume's log goes through
`Engine::log` (`crates/flume-core/src/telemetry.rs:88`), the one call of
`Host::log` in the core, so each line reaches the host's log and goes as
a `Signal::Warning`; the settle whose clues the terminal refuses among
them (`crates/flume-core/src/settle.rs:240`).

## Requirements and scenarios

| spec | requirement | met by |
|---|---|---|
| telemetry | Telemetry is the managed layer's, off without it | `nothing_is_sent_without_a_table`; conformance "seam fakes: … sends nothing" |
| telemetry | OpenTelemetry's variables only turn signals off | `a_callers_collector_is_never_flumes`, `otels_variables_turn_signals_off_and_never_on`, `telemetry::tests::the_signals_the_table_and_otels_variables_leave_on` |
| telemetry | Nothing flume decides reads its telemetry back | `a_collector_that_never_answers_holds_no_answer`, `a_refusing_collector_changes_nothing`, `status_and_the_list_send_nothing`; conformance "status and the list hand on nothing" |
| telemetry | One trace per place | conformance: the place's span, a dropped place, two settles read one end, a span read after a restart; `a_write_refused_for_its_version_sends_nothing` |
| telemetry | A span for each start of a stage | conformance S129, a lost start, a repeating stage's repetitions |
| telemetry | A span for each wait | conformance: a wait that stalls |
| telemetry | A wait names whom it waits on | conformance S129 (caller 40 minutes, merge queue 50 minutes, `landed`), a wait for room, a person while the agent asks |
| telemetry | A record at each change | conformance: a stage running now, a hold put |
| telemetry | Every signal names the place and none of its words | conformance S129's words, a warning in flume's words; `no_signal_holds_words_or_a_path`, `a_log_line_goes_as_a_warning`, `a_clue_that_does_not_show_goes_as_a_warning`, `a_warning_is_scrubbed_of_paths_and_credentials` |
| telemetry | Each agent is given its place and its start | conformance: the agent's variables, traces off, a command nests under its start; `the_agents_resource_keeps_its_own_pairs_before_the_places` |
| telemetry | Metrics by kind and stage, never by place | conformance: a start's time, the host's places; `metrics_name_no_place_intent_mark_conversation_or_account`, `the_hosts_reading_names_no_place` |
| host-configuration | The managed layer's telemetry table | `telemetry_a_table_that_sends`, `telemetry_a_collector_on_the_loopback` |
| host-configuration | A telemetry table that does not read is refused | `telemetry_a_table_that_sends_names_its_collector`, `telemetry_a_collector_off_the_loopback`, `telemetry_a_key_it_does_not_read` |
| host-configuration | The local layer sets no telemetry | `telemetry_set_by_the_local_layer` |
| host-module | settings declares the telemetry table | `nix:test`'s "A collector off the loopback"; `the_modules_files_read_whole` |
| held-state | A launch names the place's trace | conformance: a launch names the trace, a document written before; `a_held_state_written_before_its_trace` |
| held-state | Each start and each wait is held with its span | conformance: a start's span before its command; `a_held_state_with_its_trace_and_waits` |
| start-integrity | What flume sets for an agent's telemetry is in no record | "The ack taken whatever telemetry gave": conformance "start integrity", `the_record_covers_the_environment_and_flumes_variables_and_no_terminals_or_withheld`; "An account's variable of either name is in no record": `an_accounts_telemetry_variables_are_in_no_record` |
| seam-fakes | The fake telemetry keeps every signal | `the_fake_telemetry_keeps_every_signal_in_order`; conformance "seam fakes" |
| agent-profile | Claude Code's conversation read from its started word | `the_conversation_is_the_session_its_started_word_names` |

The start-integrity requirement's account half holds through the
engine: `start_record` leaves both names out of every record whatever
gives them (`TELEMETRYS_OWN`, `crates/flume-core/src/start_record.rs:30`),
and the start writes each variable its account gives into the start's
`from` under `env.<NAME>` with the layer `account`
(`crates/flume-core/src/start.rs:673`), from which status names it with
`<set>` (`crates/flume-core/src/status.rs:309`). Design 9 and the delta
spec say the same, as record 0067 chose.

## Findings

None.

## Decisions of this stage

None. The run with the host's sluice rather than the devenv's is how
the repository's instructions name the gate (`sluice exec <base>`); the
devenv's sluice refusing the host's configuration is a fact of the
host's tools, noted above for whoever keeps `CLAUDE.md`'s Checks.
