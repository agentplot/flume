# Verify: each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer

Checked on 2026-11-05 at
`change/each-stream-has-its-own-session-and-labels-and-the-host-a-local-layer`
d0037d1. The branch last took main at d81d258 (c634247), and main's
head on `origin` is still d81d258, so no merge was needed. The check
covers this change's proposal, design, nine specs and tasks, against the
statement at 2c95a70.

**The work holds, and the change is archived.** Every task is ticked.
The gate passes. Every requirement and every scenario is held by a test
that reads what its scenario says. The archive syncs the nine specs and
leaves `openspec validate --specs --strict` passing 37 of 37.

## Task 7.5

Task 7.5 mends the three things an earlier reading of this report found
at 762059f. Each one now holds:

1. **A stream with no bound: status.** `engine/labels.rs`'s
   `a_stream_with_no_bound` reads the host's status after the order. It
   checks that `personal` gives `reserves` as `Reserves::PassedOver`,
   and as `"passed-over"` in the JSON form. Its scenario cites 70, 181
   and 56 (3750e3d).
2. **The new capabilities' Purpose.** The deltas of `stream-sessions`,
   `host-configuration` and `stream-labels` each open with a
   `## Purpose` that says what the capability is for (4398f1f). The
   archive carries each Purpose into its new main spec, in place of
   openspec's placeholder.
3. **`session` in a declaration.** `kinds_weir.rs`'s
   `a_session_in_an_agents_table_gives_no_setting` reads one
   declaration that names `session` in four places:
   - a repository's table for `change`, a kind the file does not
     declare, for every stage and for `verify`;
   - the declared kind `bump`'s own agents table, for every stage and
     for `code`;
   - `code`'s own agents.

   Each reads as no setting. The model and instructions beside each are
   read (3750e3d). This holds the rest of design 9's half of
   stream-sessions' "A session no other layer gives".

## Tasks

All 23 tasks of the seven groups are ticked, `[x]`, and none is open.
The gate run below backs each group's closing task: 1.3, 2.3, 3.4, 4.4,
5.2, 6.2, 7.4 and 7.5. 7.4 also names `root:specs`, which the gate does
not run, since nothing under `openspec/specs/` changes before the
archive. It passes on the tree at d0037d1, and, over a copy of
`openspec/`, after the archive. Task 7.3 holds: `docs/architecture/`
holds `core.md` alone, and nothing in the change, `docs/` or `crates/`
names `_open.md`.

## Checks

- **The gate.** `sluice exec main` at d0037d1 ran the checks d0037d1
  affects against main at d81d258. It passed 23 tasks of
  `#check-fast` and `#check-tools`, with `site:install` run for them,
  and recorded their result under its key for d0037d1:
  - `seams`, `core`, `fakes`, `conformance` and `flume`: format, lint
    and test, `flume:test` holding `kinds_weir`'s
    `a_session_in_an_agents_table_gives_no_setting`, and
    `conformance:test` the engine's scenarios, the labels' stream with
    no bound among them;
  - `conformance:coverage`: "every clause of Parts A and B, I1 to I8
    and S1 to S85 at 2c95a70 is cited by the suite's 422 scenarios",
    with 183 named unbuilt to the answer to section 10's question;
  - `flume:tools`, 78 of 78, `terminal_herdr`'s
    `s82_two_streams_sessions_under_herdr` and
    `the_pinned_herdr_keeps_its_named_sessions_under_the_session_home`
    among them, under sluice's sandbox;
  - `plugin-claude:test`;
  - `site:prose` and `site:build`;
  - `root:format`, `root:adrs` (33 records, each with its own number)
    and `root:changes`.
- **The archive.** `openspec archive … --yes` (openspec 1.14.3) over a
  copy of `openspec/` at d0037d1 syncs the nine specs with no error: 17
  requirements added, 6 modified, none removed. `openspec validate
  --specs --strict` then passes 37 of 37, and each of the three new
  main specs opens with its delta's Purpose.
- **The statement.** `clauses.rs` reads 2c95a70. Part A, Part B and
  the unbuilt set are as the clause-coverage spec gives them: 183 is
  named unbuilt, by the answer to section 10's question on how a host's
  MCP servers reach an agent, and S1 to S85 are listed.

## Requirements and scenarios

Each test named below exists under its name at d0037d1 and passed in
the gate. I read each one's assertions against its scenario's WHEN and
THEN.

Short names: SE is `crates/flume-conformance/src/engine/sessions.rs`,
LB `engine/labels.rs`, HL `engine/host_layers.rs`, HQ
`engine/host_queue.rs`, PL `engine/places.rs`, ST `engine/streams.rs`,
CF `crates/flume/src/host/config.rs`, PR `commands/print.rs`, LA
`commands/launch.rs`, CM `crates/flume/tests/commands.rs`, HT
`tests/host.rs`, BI `tests/profiles/built_in.rs`, KW
`tests/profiles/kinds_weir.rs`, TH
`tests/profiles/tools/terminal_herdr.rs`, HM
`src/profiles/terminal/herdr/mod.rs`, RT
`crates/flume-seams/src/round_trip.rs`, and FK
`crates/flume-fakes/tests/fakes.rs`.

| Spec | Requirement | Scenarios | Tests | Met |
|---|---|---|---|---|
| stream-sessions | A stream's agents open in its own session | 4 | SE `s82_each_stream_in_its_own_session`, `a_stream_that_names_its_session`, `the_default_stream_that_names_one`, `a_session_changed_between_starts` | yes |
| stream-sessions | A launch names its place's session | 4 | SE `a_launchs_session_over_its_streams`, `one_stages_session`, `a_pane_before_a_session`, `a_session_no_other_layer_gives`; KW `a_session_in_an_agents_table_gives_no_setting`; CF `a_session_in_a_streams_table_and_none_in_an_agents_table` | yes |
| stream-sessions | Status gives each agent's session and each stream's | 1 | SE `the_sessions_in_status` (JSON form); PR `each_agents_session` | yes |
| host-configuration | A managed layer and a local layer over it | 4 | CF `a_streams_bound_and_priority_set_locally`, `a_kinds_settings_key_by_key`, `no_local_file`; HL `s83_both_layers_in_status` (the next start's room); CM `flume_writes_neither_configuration_file`; HT `the_host_reads_both_layers_and_writes_neither` | yes |
| host-configuration | The local layer sets only C.7's keys | 4 | CF `a_local_file_that_sets_a_pool`, `a_streams_pool_set_locally`, `a_local_file_at_fault`, `a_stream_only_the_local_layer_names`, `each_key_the_local_layer_may_not_set_named_once` | yes |
| host-configuration | The managed layer pins keys | 3 | CF `one_key_pinned`, `a_table_pinned`, `a_pin_of_nothing_the_local_layer_sets`, `a_pin_of_a_repositorys_key_with_the_home_or_tilde` | yes |
| host-configuration | Status gives every value with its layer | 2 | HL `s83_both_layers_in_status`, `a_local_layer_not_taken`; CM `s83_the_local_layer_through_the_binary`; CF `every_value_as_toml_writes_it_and_no_variables`; PR `the_hosts_local_layer` | yes |
| stream-labels | A place's label | 4 | LB `s85_each_source_of_a_label`, `a_label_changed_between_starts`; BI `flumes_own_kinds_declare_no_label`; CF `a_label_that_is_no_word_is_refused_naming_its_key`, `a_label_by_repository_and_kind_then_kind` | yes |
| stream-labels | A label's reserve and the slots it keeps free | 5 | LB `s84_a_reserve_lent_and_a_slot_kept_free`, `one_settle_ranks_again`, `a_reserve_where_the_host_has_no_room`, `a_stream_with_no_bound` (its order and its reserves passed over in status and the JSON form), `a_reserve_set_locally`; CF `labels_sessions_and_a_kinds_label_set_locally` | yes |
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
  tests above read. **Decision 9**'s test list is met, `kinds_weir.rs`'s
  declaration's `session` among it.
- **core.1.** Nothing this change adds to `flume-core`, `flume-seams`,
  `flume-fakes` or `flume-conformance` names herdr, Claude Code, sluice,
  worktrunk, OpenSpec or any program in an agent profile's table.
- **core.4.** The weir profile reads `session` in a declaration through
  weir's crate and gives it no setting. It gives `Kind::label` none
  until weir states and reads a kind's label.
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
- **The user's two rulings of 2026-11-05** came in while this check
  ran, in the Inbox thread on this change's two questions. At 16:25Z a
  kind's label became `label` in both formats, with weir 94's display
  words renamed. At 16:37Z, clause 183 got Option A: servers declared in
  flume's host configuration under `[mcp.<server>]` with `labels`, and
  passed to each agent at its start. Neither changes what this change
  builds against the statement at 2c95a70:
  - a kind's own label still waits on weir's statement and crate
    reading the field;
  - 183 stays unbuilt until the statement states the answer and a
    change builds it.

  They are written into the statement and record 0033 apart from this
  archive.
