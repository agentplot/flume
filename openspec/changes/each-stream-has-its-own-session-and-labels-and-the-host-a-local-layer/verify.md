# Verify: each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer

Checked on 2026-11-05 at
`change/each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`
762059f. The branch last took main at d81d258 (c634247), and main's
head on `origin` is still d81d258, so no merge was needed. The check
covers this change's proposal, design, nine specs and tasks, against the
statement at 2c95a70. agentplot/blueprints main at 4ec26f7 holds the
same `design/flume/requirements.md`.

**The work falls short, and the change is not archived.** Every task is
ticked. The gate passes. Every requirement is built. Three things still
stand, and they send the change back to its code stage:

1. **A stream with no bound: status is never tested.** stream-labels'
   scenario "A stream with no bound" ends: "no reserve is held in
   `personal`, and status names its reserves passed over". No test reads
   the second half. `labels.rs`'s `a_stream_with_no_bound` checks the
   order alone. `Reserves::PassedOver` comes from
   `crates/flume-core/src/room.rs:777-781`, and no test of the core
   reads it. The only test is `print.rs`'s `a_streams_labels`, which
   prints a `StreamStatus` the test builds by hand. The only assertion
   on `reserves` is `Reserves::Held`, in `the_labels_in_status`. **To
   do:** have `a_stream_with_no_bound` also read the host's status.
   Check that `personal` gives `reserves` passed over, as
   `Reserves::PassedOver` and as `"passed-over"` in the JSON form (56,
   181).
2. **The archive leaves `root:specs` failing.** The deltas of the three
   new capabilities, `stream-sessions`, `host-configuration` and
   `stream-labels`, give no `## Purpose`. So `openspec archive`
   (openspec 1.14.3) writes its placeholder into each new main spec.
   `openspec validate --specs --strict`, which is `root:specs` and one
   of the checks task 7.4 names, then fails 3 of 37: "Purpose section
   is still a placeholder". The gate ran no `root:specs`, because
   762059f changes nothing under `openspec/specs/`. **To do:** open
   each of the three deltas with a `## Purpose` that says what the
   capability is for. A trial over a copy of `openspec/`, with a
   Purpose put at the head of each delta, archived and left 37 of 37
   passing, the Purpose carried into the main spec.
3. **`kinds_weir.rs` holds no `session` in a declaration.** Design 9
   and task 3.1 give `crates/flume/tests/profiles/kinds_weir.rs` half
   of stream-sessions' "A session no other layer gives": a stage's
   agents `session` in a declaration gives no setting. No such test is
   there. The scenario itself is held: `sessions.rs`'s
   `a_session_no_other_layer_gives` covers it over the fakes, with the
   engine taking `session` out of the kind's layer, and the weir
   profile's `map_agent` gives `session: None` whatever a declaration
   says (`crates/flume/src/profiles/kinds/weir/mod.rs:450`). **To do:**
   add the test the design names. Read a declaration whose agents
   table, for every stage and for one, names `session`, and check its
   `Settings::session` is none.

## Tasks

All 22 tasks of the seven groups are ticked, `[x]`, and none is open.
The gate run below backs each group's closing task (1.3, 2.3, 3.4, 4.4,
5.2, 6.2, 7.4). Item 2 above is the one exception: `root:specs`, which
7.4 names, passes on the tree as it stands but fails once the change is
archived. Task 7.3 holds:
`docs/architecture/` holds `core.md` alone, and nothing in the change,
`docs/` or `crates/` names `_open.md`.

## Checks

- **The gate.** `sluice exec main` at 762059f ran the checks 762059f
  affects against main at d81d258. It passed all 23 `#check-fast` and
  `#check-tools` tasks, replayed none, and recorded them under their
  keys for 762059f:
  - `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint
    and test, and `flume:proto`;
  - `conformance:coverage`: "every clause of Parts A and B, I1 to I8
    and S1 to S85 at 2c95a70 is cited by the suite's 422 scenarios";
  - `flume:tools`, 78 of 78, among them `terminal_herdr`'s
    `s82_two_streams_sessions_under_herdr`,
    `a_session_name_herdr_refuses_refuses_the_open_in_its_words`,
    `a_place_tidied_in_two_sessions_closes_its_tab_and_workspace_in_each`,
    `a_place_recorded_before_records_were_kept_by_server_is_tidied_as_before`
    and `the_pinned_herdr_keeps_its_named_sessions_under_the_session_home`,
    run under sluice's sandbox as the user ruled on 2026-11-05;
  - `plugin-claude:test`;
  - `site:install`, `site:prose` and `site:build`;
  - `root:format`, `root:changes` and `root:adrs`.
- **The archive.** `openspec archive … --yes` over a copy of
  `openspec/` at 762059f syncs the nine specs with no error: 17
  requirements added, 6 modified, none removed. Every scenario heading
  main's six modified specs hold is in the synced specs. The counts go
  from 5 to 5 in clause-coverage, 13 to 15 in command-line, 12 to 14 in
  host-queue, 33 to 36 in seam-contracts, 11 to 13 in seam-fakes and 36
  to 41 in terminal-profiles. The new specs hold 13 (host-configuration),
  15 (stream-labels) and 9 (stream-sessions). The strict check fails as
  item 2 says, and on nothing else.
- **The statement.** `clauses.rs` reads 2c95a70. Part A, Part B and
  the unbuilt set are as the clause-coverage spec gives them: 183 is
  named unbuilt, by the answer to section 10's question on how a host's
  MCP servers reach an agent, and S1 to S85 are listed.

## Requirements and scenarios

Each test named below exists under its name at 762059f and passed in
the gate. I read each one's assertions against its scenario's WHEN and
THEN.

Short names: SE is `crates/flume-conformance/src/engine/sessions.rs`,
LB `engine/labels.rs`, HL `engine/host_layers.rs`, HQ
`engine/host_queue.rs`, PL `engine/places.rs`, ST `engine/streams.rs`,
CF `crates/flume/src/host/config.rs`, PR `commands/print.rs`, LA
`commands/launch.rs`, CM `crates/flume/tests/commands.rs`, HT
`tests/host.rs`, BI `tests/profiles/built_in.rs`, TH
`tests/profiles/tools/terminal_herdr.rs`, HM
`src/profiles/terminal/herdr/mod.rs`, RT
`crates/flume-seams/src/round_trip.rs`, and FK
`crates/flume-fakes/tests/fakes.rs`.

| Spec | Requirement | Scenarios | Tests | Met |
|---|---|---|---|---|
| stream-sessions | A stream's agents open in its own session | 4 | SE `s82_each_stream_in_its_own_session`, `a_stream_that_names_its_session`, `the_default_stream_that_names_one`, `a_session_changed_between_starts` | yes |
| stream-sessions | A launch names its place's session | 4 | SE `a_launchs_session_over_its_streams`, `one_stages_session`, `a_pane_before_a_session`, `a_session_no_other_layer_gives`; CF `a_session_in_a_streams_table_and_none_in_an_agents_table` | yes, but see item 3 |
| stream-sessions | Status gives each agent's session and each stream's | 1 | SE `the_sessions_in_status` (JSON form); PR `each_agents_session` | yes |
| host-configuration | A managed layer and a local layer over it | 4 | CF `a_streams_bound_and_priority_set_locally`, `a_kinds_settings_key_by_key`, `no_local_file`; HL `s83_both_layers_in_status` (the next start's room); CM `flume_writes_neither_configuration_file`; HT `the_host_reads_both_layers_and_writes_neither` | yes |
| host-configuration | The local layer sets only C.7's keys | 4 | CF `a_local_file_that_sets_a_pool`, `a_streams_pool_set_locally`, `a_local_file_at_fault`, `a_stream_only_the_local_layer_names`, `each_key_the_local_layer_may_not_set_named_once` | yes |
| host-configuration | The managed layer pins keys | 3 | CF `one_key_pinned`, `a_table_pinned`, `a_pin_of_nothing_the_local_layer_sets`, `a_pin_of_a_repositorys_key_with_the_home_or_tilde` | yes |
| host-configuration | Status gives every value with its layer | 2 | HL `s83_both_layers_in_status`, `a_local_layer_not_taken`; CM `s83_the_local_layer_through_the_binary`; CF `every_value_as_toml_writes_it_and_no_variables`; PR `the_hosts_local_layer` | yes |
| stream-labels | A place's label | 4 | LB `s85_each_source_of_a_label`, `a_label_changed_between_starts`; BI `flumes_own_kinds_declare_no_label`; CF `a_label_that_is_no_word_is_refused_naming_its_key`, `a_label_by_repository_and_kind_then_kind` | yes |
| stream-labels | A label's reserve and the slots it keeps free | 5 | LB `s84_a_reserve_lent_and_a_slot_kept_free`, `one_settle_ranks_again`, `a_reserve_where_the_host_has_no_room`, `a_stream_with_no_bound`, `a_reserve_set_locally`; CF `labels_sessions_and_a_kinds_label_set_locally` | **no**: "A stream with no bound"'s status half, item 1 |
| stream-labels | What a stream's labels refuse | 4 | LB `a_pool_in_a_labels_table`, `reserves_past_the_bound`, `more_kept_free_than_owed`, `the_configuration_mended`; CF `a_labels_table_holding_a_pool_is_read_with_the_pool_unknown` | yes |
| stream-labels | Status gives labels and the room they keep | 2 | LB `the_labels_in_status`, `waiting_on_a_slot_kept_free`; PR `a_places_label`, `a_streams_labels` | yes |
| host-queue | A stage starts only under the host's bound | 5 | HQ `a_slot_another_label_keeps_free` and the four as before | yes |
| host-queue | What waits starts by priority | 5 | HQ `a_label_below_its_reserve_before_a_places_priority` and the four as before | yes |
| terminal-profiles | Where herdr opens an agent | 4 | TH `s82_two_streams_sessions_under_herdr`, `a_session_name_herdr_refuses_refuses_the_open_in_its_words`, and the two as before | yes |
| terminal-profiles | herdr tidies a place in every session | 2 | TH `a_place_tidied_in_two_sessions_closes_its_tab_and_workspace_in_each`, `a_place_recorded_before_records_were_kept_by_server_is_tidied_as_before` | yes |
| terminal-profiles | The base's count on each of its workspaces | 1 | PL `one_base_in_two_sessions`; HM `a_place_in_two_sessions_shows_on_its_live_agents_server_and_its_base_on_each` | yes |
| seam-contracts | The terminal seam's open, tidy and show | 2 | FK `the_terminal_logs_where_each_agent_opens` and the one as before | yes |
| seam-contracts | The host's facts give sessions, labels and the layers | 1 | ST `the_hosts_sessions_labels_and_layers` | yes |
| seam-contracts | The types of a session, a label and the layers | 1 | RT `a_held_state_with_a_session_and_a_label`, `where_an_agent_opens_and_its_session`, `a_label_and_its_share`, `the_hosts_configuration_and_its_layers` | yes |
| seam-fakes | The fakes script sessions, labels and the layers | 2 | FK `a_scripted_session`, `a_scripted_label`, `the_hosts_labels_shares_and_layers_are_scripted`, `a_workspace_per_session_and_base` | yes |
| command-line | launch names a session by --agent | 2 | CM `a_session_for_every_stage_and_for_one_at_the_launch`, `the_settings_agent_gives_are_named_in_its_refusal`; LA `a_session_for_every_stage_and_one_for_a_stage`; `--agent`'s help names `session` (`commands/mod.rs:271`) | yes |
| clause-coverage | The suite knows the statement's clauses; A clause cited by nothing fails the suite | 3 | `clauses.rs`'s `the_clauses_listed` and its unbuilt list; `coverage.rs`; `conformance:coverage` | yes |

## Design and rules

- **Decisions 1 to 8** are built as the design says, in the code the
  tests above read. Decisions 6 and 7 carry the build's settlements
  that 762059f wrote back into the design.
- **core.1.** Nothing this change adds to `flume-core`, `flume-seams`,
  `flume-fakes` or `flume-conformance` names herdr, Claude Code, sluice,
  worktrunk, OpenSpec or any program in an agent profile's table.
- **core.14.** The JSON form only gains fields: a place's `label`, an
  agent's `session`, a stream's `session`, `labels`, `reserves` and
  `refused`, a queued stage's `label` and `kept-free`, and the host's
  `configuration`.
- **core.16 and core.45.** No command writes either layer's file:
  CM `flume_writes_neither_configuration_file` and HT
  `the_host_reads_both_layers_and_writes_neither`.
- **core.30.** An account's variables are given by name and `<set>`.
  The binary's status never holds the variable's value
  (`s83_the_local_layer_through_the_binary`).
- **core.35.** None of the words core.35 rules out appears in this
  change's diff of `site/`, the README or `plugins/`.
- **Records 0031 to 0033** are proposed. Each waits on the user's yes,
  on their review of this change. The archive accepts none of them. The
  rules they draw, core.44 to core.46, and the amended core.26 are in
  `core.md` and in the site's `rules.md` word for word.
