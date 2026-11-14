# Verify: flume-reports-each-place-as-a-trace

Verified on `change/flume-reports-each-place-as-a-trace` at 0c906871,
cut from `bolt/flume-shows-its-places-as-traces` (2d180f0b).

**Outcome: the whole checks pass; the work falls short in two places,
so the change is not archived and goes back to its code stage.**

## The whole checks

`sluice exec bolt/flume-shows-its-places-as-traces`, run once with the
host's sluice (0.93.2): exit 0. sluice ran the checks 0c90687 affects
against the base at 2d180f0, the bolt's required `#check-fast`: 30 tasks
completed, 29 results recorded under their keys for 0c90687, none
failed. Among them `seams`, `core`, `fakes`, `conformance` and `flume`
format, lint and test; `conformance:coverage`; `nix` format, lint,
typecheck and test; `site:prose` and `site:build`; `plugin-claude:test`,
`plugin-herdr:manifest` and `plugin-herdr:test`; `root:changes`,
`root:adrs` and `root:versions`. moon warned it could not archive task
outputs into `.moon/cache/outputs`, which fails nothing.

The first attempt, `devenv shell -- sluice exec …`, ran no check: the
devenv's sluice is the 0.71.0 `devenv.nix` pins for the profile tests,
and it refused the host's `~/.config/sluice/config.toml` (`unknown field
credential`) before running anything. The run above is the checks' one
run.

`openspec validate flume-reports-each-place-as-a-trace --strict`: valid.

## Tasks

Every task of `tasks.md`, 1.1 to 7.1, is ticked, and every test a task
names exists under the name it gives: 24 named tests across
`flume-seams`, `flume-core`, `flume-fakes`, `flume` and its `host`,
`profiles` and `telemetry` test binaries, the seven `telemetry_…` tests
of `host::config`, the module's `collectorOffTheLoopback` case in
`nix/tests/fails.nix` and `scripts.sh`, and the conformance scenarios of
`engine/telemetry.rs`. Each passed in the run above.

Task 4.7 is not met in full; see finding 1.

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
| telemetry | Every signal names the place and none of its words | conformance S129's words, a warning in flume's words; `no_signal_holds_words_or_a_path`, `a_warning_is_scrubbed_of_paths_and_credentials` — **short: finding 1** |
| telemetry | Each agent is given its place and its start | conformance: the agent's variables, traces off, a command nests under its start; `the_agents_resource_keeps_its_own_pairs_before_the_places` |
| telemetry | Metrics by kind and stage, never by place | conformance: a start's time, the host's places; `metrics_name_no_place_intent_mark_conversation_or_account`, `the_hosts_reading_names_no_place` |
| host-configuration | The managed layer's telemetry table | `telemetry_a_table_that_sends`, `telemetry_a_collector_on_the_loopback` |
| host-configuration | A telemetry table that does not read is refused | `telemetry_a_table_that_sends_names_its_collector`, `telemetry_a_collector_off_the_loopback`, `telemetry_a_key_it_does_not_read` |
| host-configuration | The local layer sets no telemetry | `telemetry_set_by_the_local_layer` |
| host-module | settings declares the telemetry table | `nix:test`'s "A collector off the loopback"; `the_modules_files_read_whole` |
| held-state | A launch names the place's trace | conformance: a launch names the trace, a document written before; `a_held_state_written_before_its_trace` |
| held-state | Each start and each wait is held with its span | conformance: a start's span before its command; `a_held_state_with_its_trace_and_waits` |
| start-integrity | What flume sets for an agent's telemetry is in no record | conformance "start integrity"; `the_record_covers_the_environment_and_flumes_variables_and_no_terminals_or_withheld` — **short: finding 2** |
| seam-fakes | The fake telemetry keeps every signal | `the_fake_telemetry_keeps_every_signal_in_order`; conformance "seam fakes" |
| agent-profile | Claude Code's conversation read from its started word | `the_conversation_is_the_session_its_started_word_names` |

## Findings

1. **One line of flume's log does not go as a warning.**
   `crates/flume-core/src/settle.rs:240` writes "the clues of {id} did
   not show: {reason}" through `self.host.log` directly, not through
   `Engine::log` (`crates/flume-core/src/telemetry.rs:88`), so it reaches
   the host's log and never the telemetry. Task 4.7 asks that each core
   call of `Host::log` also go as a `Signal::Warning`, design 11 that
   `flume.log`'s lines go as warnings whichever command writes them, and
   the telemetry spec that flume's own warnings and errors go as records
   (218). Every other core call goes through `Engine::log`. The fix is
   the call through `self.log`, and a test that a clue that does not
   show goes as a warning.

2. **The start-integrity requirement and the code disagree on an
   account's variable.** The delta spec
   (`specs/start-integrity/spec.md`) says "a variable of either name
   that the start's account gives SHALL be recorded as any account's
   variable is", and an account's variable is recorded by its digest.
   The code takes `OTEL_RESOURCE_ATTRIBUTES` and `TRACEPARENT` out of
   every start's record whatever gives them (`TELEMETRYS_OWN` in
   `crates/flume-core/src/start_record.rs`), and its test asserts an
   account's `team=ops` is in no record. Record 0067's code-stage
   amendment chose this ("Not taken: its digest in the record, which an
   agent started on a host that sends would never match"), but the
   change's spec was not amended with it. One of them must move: the
   delta spec carrying the record's choice, or the code recording an
   account's value as the spec says.

## Decisions of this stage

None. The run with the host's sluice rather than the devenv's is how
the repository's instructions name the gate (`sluice exec <base>`); the
devenv's sluice refusing the host's configuration is a fact of the
host's tools, noted above for whoever keeps `CLAUDE.md`'s Checks.
